/*
 * Pipedrive API v1
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::v1::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddPersonFollowerResponseAllOfData {
    /// The ID of the user who was added as a follower to a person
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    /// The ID of the follower
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The ID of the person to whom the follower was added
    #[serde(rename = "person_id", skip_serializing_if = "Option::is_none")]
    pub person_id: Option<i32>,
    /// The date and time when the follower was added to a person. Format: YYYY-MM-DD HH:MM:SS
    #[serde(rename = "add_time", skip_serializing_if = "Option::is_none")]
    pub add_time: Option<String>,
}

impl AddPersonFollowerResponseAllOfData {
    pub fn new() -> AddPersonFollowerResponseAllOfData {
        AddPersonFollowerResponseAllOfData {
            user_id: None,
            id: None,
            person_id: None,
            add_time: None,
        }
    }
}

