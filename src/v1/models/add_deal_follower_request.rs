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
pub struct AddDealFollowerRequest {
    /// The ID of the user
    #[serde(rename = "user_id")]
    pub user_id: i32,
}

impl AddDealFollowerRequest {
    pub fn new(user_id: i32) -> AddDealFollowerRequest {
        AddDealFollowerRequest {
            user_id,
        }
    }
}

