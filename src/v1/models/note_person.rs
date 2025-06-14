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

/// NotePerson : The person the note is attached to
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotePerson {
    /// The name of the person the note is attached to
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl NotePerson {
    /// The person the note is attached to
    pub fn new() -> NotePerson {
        NotePerson {
            name: None,
        }
    }
}

