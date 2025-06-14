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
pub struct GetProductFollowersResponseAllOfDataInner {
    /// The ID of the user
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    /// The ID of the user follower
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The ID of the product
    #[serde(rename = "product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i32>,
    /// The date and time when the follower was added to the person
    #[serde(rename = "add_time", skip_serializing_if = "Option::is_none")]
    pub add_time: Option<String>,
}

impl GetProductFollowersResponseAllOfDataInner {
    pub fn new() -> GetProductFollowersResponseAllOfDataInner {
        GetProductFollowersResponseAllOfDataInner {
            user_id: None,
            id: None,
            product_id: None,
            add_time: None,
        }
    }
}

