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
pub struct AddTeamRequest {
    /// The team name
    #[serde(rename = "name")]
    pub name: String,
    /// The team description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The team manager ID
    #[serde(rename = "manager_id")]
    pub manager_id: i32,
    /// The list of user IDs
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<i32>>,
}

impl AddTeamRequest {
    pub fn new(name: String, manager_id: i32) -> AddTeamRequest {
        AddTeamRequest {
            name,
            description: None,
            manager_id,
            users: None,
        }
    }
}

