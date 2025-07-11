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
pub struct Comment {
    /// The ID of the note
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<uuid::Uuid>,
    /// Whether the note is active or deleted
    #[serde(rename = "active_flag", skip_serializing_if = "Option::is_none")]
    pub active_flag: Option<bool>,
    /// The creation date and time of the note
    #[serde(rename = "add_time", skip_serializing_if = "Option::is_none")]
    pub add_time: Option<String>,
    /// The creation date and time of the note
    #[serde(rename = "update_time", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// The content of the note in HTML format. Subject to sanitization on the back-end.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The ID of the object that the comment is attached to, will be the id of the note
    #[serde(rename = "object_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    /// The type of object that the comment is attached to, will be \"note\"
    #[serde(rename = "object_type", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    /// The ID of the user who created the comment
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    /// The ID of the user who last updated the comment
    #[serde(rename = "updater_id", skip_serializing_if = "Option::is_none")]
    pub updater_id: Option<i32>,
    /// The ID of the company
    #[serde(rename = "company_id", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<i32>,
}

impl Comment {
    pub fn new() -> Comment {
        Comment {
            uuid: None,
            active_flag: None,
            add_time: None,
            update_time: None,
            content: None,
            object_id: None,
            object_type: None,
            user_id: None,
            updater_id: None,
            company_id: None,
        }
    }
}

