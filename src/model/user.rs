//! All kinds of user object
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::HashMap;

use super::image::Image;
use crate::model::Type;
/// [Public user object](https://developer.spotify.com/web-api/object-model/#user-object-public)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicUser {
    pub display_name: Option<String>,
    pub external_urls: HashMap<String, String>,
    pub followers: Option<HashMap<String, Option<Value>>>,
    pub href: String,
    pub id: String,
    pub images: Option<Vec<Image>>,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

/// [Private user object](https://developer.spotify.com/documentation/web-api/reference/users-profile/get-current-users-profile)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivateUser {
    pub birthdate: Option<NaiveDate>,
    pub country: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub external_urls: HashMap<String, String>,
    pub followers: Option<HashMap<String, Option<Value>>>,
    pub href: String,
    pub id: String,
    pub images: Option<Vec<Image>>,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}
