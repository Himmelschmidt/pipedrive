/*
 * Pipedrive API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityItemAttendeesInner {
    /// The email address of the attendee
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The name of the attendee
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The status of the attendee
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Whether the attendee is the organizer or not
    #[serde(rename = "is_organizer", skip_serializing_if = "Option::is_none")]
    pub is_organizer: Option<bool>,
    /// The ID of the person if the attendee has a person record
    #[serde(rename = "person_id", skip_serializing_if = "Option::is_none")]
    pub person_id: Option<i32>,
    /// The ID of the user if the attendee is a user
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
}

impl ActivityItemAttendeesInner {
    pub fn new() -> ActivityItemAttendeesInner {
        ActivityItemAttendeesInner {
            email: None,
            name: None,
            status: None,
            is_organizer: None,
            person_id: None,
            user_id: None,
        }
    }
}

