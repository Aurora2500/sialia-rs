//! Model for a tweet post.

use std::hash::{Hash, Hasher};

use serde::Deserialize;

use crate::model::id::*;

/// A tweet object
#[derive(Deserialize, Debug)]
pub struct Tweet {
    /// The ID of the tweet.
    pub id: TweetId,
    /// The ID of the User that tweeted.
    pub author_id: UserId,
    /// The content of the tweet.
    pub text: String,
}

impl PartialEq for Tweet {
    fn eq(&self, rhs: &Self) -> bool {
        self.id == rhs.id
    }
}

impl Eq for Tweet {}

impl Hash for Tweet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
