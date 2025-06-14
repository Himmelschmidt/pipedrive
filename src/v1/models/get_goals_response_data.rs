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
pub struct GetGoalsResponseData {
    #[serde(rename = "goals", skip_serializing_if = "Option::is_none")]
    pub goals: Option<Vec<models::Goal>>,
}

impl GetGoalsResponseData {
    pub fn new() -> GetGoalsResponseData {
        GetGoalsResponseData {
            goals: None,
        }
    }
}

