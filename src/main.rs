#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate diesel;

extern crate chrono;
extern crate dotenv;
extern crate egg_mode;
extern crate regex;

use chrono::prelude::*;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use dotenv::dotenv;
use std::env;
use regex::Regex;
use models::{Entry, NewEntry};
use schema::entries;

pub mod models;
pub mod schema;

fn main() {
    let connection = establish_connection();
    let timeline = fetch_tweets();

    for tweet in &timeline.unwrap().response {
        let tweet_id = tweet.id as i64;

        if !entry_exists(&connection, tweet_id) {
            create_entry(&connection, convert_tweet_to_entry(&tweet));
        }
    }
}

pub fn fetch_tweets() -> egg_mode::WebResponse<Vec<egg_mode::tweet::Tweet>> {
    let consumer_key = env::var("TWITTER_CONSUMER_KEY").unwrap();
    let consumer_secret = env::var("TWITTER_CONSUMER_SECRET").unwrap();
    let access_token = env::var("TWITTER_ACCESS_TOKEN").unwrap();
    let access_token_secret = env::var("TWITTER_ACCESS_TOKEN_SECRET").unwrap();

    let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);
    let access_token = egg_mode::KeyPair::new(access_token, access_token_secret);
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };

    let mut timeline = egg_mode::tweet::user_timeline("wiraufweltreise", false, false, &token);

    timeline.start()
}

pub fn entry_exists(conn: &PgConnection, tweet_id: i64) -> bool {
    use self::entries::dsl::*;
    use diesel::expression::dsl::exists;

    diesel::select(exists(entries.filter(id.eq(tweet_id))))
        .get_result::<bool>(conn)
        .expect("Error checking for existing Tweet")
}

pub fn create_entry(conn: &PgConnection, new_entry: NewEntry) -> Entry {
    diesel::insert(&new_entry)
        .into(entries::table)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn find_image_url(media: Option<Vec<egg_mode::entities::MediaEntity>>) -> String {
    match media {
        Some(media) => media[0].media_url_https.clone(),
        _ => "".to_string(),
    }
}

pub fn convert_tweet_to_entry(tweet: &egg_mode::tweet::Tweet) -> NewEntry {
    let text = tweet.text.clone();
    let (latitude, longitude) = tweet.coordinates.unwrap_or((0.0, 0.0));
    let tweet_id = tweet.id as i64;

    let re = Regex::new(r"https://t.co.\w{10}$").unwrap();
    let result = re.replace_all(&text, "");

    NewEntry {
        id: tweet_id,
        longitude: longitude,
        latitude: latitude,
        description: result.to_string(),
        image_url: find_image_url(tweet.entities.media.clone()),
        created_at: Utc::now().naive_utc(),
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
