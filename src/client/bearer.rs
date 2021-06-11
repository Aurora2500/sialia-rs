//! Bearer client

use crate::model::prelude::*;
use crate::error::*;

use serde::de::IntoDeserializer;
use serde::de::Deserialize;
use reqwest::Client;

/// Client that only needs the bearer token to function.
///
/// The bearer token only allows you to look up and read from twitter, but no
/// other types of interraction.
pub struct BearerClient {
    token: String,
    client: Client,
}

impl BearerClient {
    /// Generate a bearer client from a bearer token
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: Client::new(),
        }
    }

    /// Get a tweet by it's ID
    pub async fn get_tweet_by_id(&self, id: TweetId) -> Result<Tweet> {
        let url = format!("https://api.twitter.com/2/tweets/{}", id.0);
        let response = self.client
            .get(url)
            .bearer_auth(&self.token)
            .query(&[("tweet.fields", "author_id")])
            .send()
            .await?;
        let de = response.json::<serde_json::Value>()
            .await?
            .get_mut("data").ok_or(Error::JsonError(None))?
            .take()
            .into_deserializer();

        Ok(Tweet::deserialize(de)?)
    }

    /// Get multiple tweets by their ID
    pub async fn get_tweets_by_ids(&self, ids: &[TweetId]) -> Result<Vec<Tweet>> {
        if ids.len() > 100 {
            todo!()
        }

        let response = self.client
            .get("https://api.twitter.com/2/tweets/")
            .bearer_auth(&self.token)
            .query(&[("ids", TweetId::join(ids, ",")),("expansions", "author_id".to_owned())])
            .send()
            .await?;
        println!("Caught tweets");
        let mut val = response.json::<serde_json::Value>()
            .await?;
        println!("{:?}", val);
        let de = val
            .get_mut("data").ok_or(Error::JsonError(None))?
            .take()
            .into_deserializer();
        println!("{:?}", de);
        Ok(Vec::<Tweet>::deserialize(de)?)
    }
}
