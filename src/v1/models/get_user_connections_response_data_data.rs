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

/// GetUserConnectionsResponseDataData : The object of UserConnections
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUserConnectionsResponseDataData {
    /// The third party ID or false in case the ID is not found
    #[serde(rename = "google", skip_serializing_if = "Option::is_none")]
    pub google: Option<String>,
}

impl GetUserConnectionsResponseDataData {
    /// The object of UserConnections
    pub fn new() -> GetUserConnectionsResponseDataData {
        GetUserConnectionsResponseDataData {
            google: None,
        }
    }
}

