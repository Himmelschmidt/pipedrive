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
pub struct MergeDealsRequest {
    /// The ID of the deal that the deal will be merged with
    #[serde(rename = "merge_with_id")]
    pub merge_with_id: i32,
}

impl MergeDealsRequest {
    pub fn new(merge_with_id: i32) -> MergeDealsRequest {
        MergeDealsRequest {
            merge_with_id,
        }
    }
}

