//! User model
use std::hash::{Hash, Hasher};

use serde::Deserialize;

use super::prelude::*;

/// A representation of an User.
#[derive(Deserialize, Clone)]
pub struct User {
    /// The ID of the User.
    pub id: UserId,
    /// The name of the User.
    pub name: String,
    /// The handler of the User.
    #[serde(rename = "screen_name")]
    pub handler: String,
    /// Is this account protected.
    pub protected: bool,
    /// Is this account verified.
    pub verified: bool,
    /// The number of followers this User has.
    #[serde(rename = "followers_count")]
    pub followers: u32,
    /// The number of accounts this User is following.
    #[serde(rename = "friends_count")]
    pub following: u32,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}

impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
