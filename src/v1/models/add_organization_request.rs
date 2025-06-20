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
pub struct AddOrganizationRequest {
    /// The name of the organization
    #[serde(rename = "name")]
    pub name: String,
    /// The optional creation date & time of the organization in UTC. Format: YYYY-MM-DD HH:MM:SS
    #[serde(rename = "add_time", skip_serializing_if = "Option::is_none")]
    pub add_time: Option<String>,
    /// The ID of the user who will be marked as the owner of this organization. When omitted, the authorized user ID will be used.
    #[serde(rename = "owner_id", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i32>,
    /// The label assigned to the organization. When the `label` field is updated, the `label_ids` field value will be overwritten by the `label` field value.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<i32>,
    /// The IDs of labels assigned to the organization. When the `label_ids` field is updated, the `label` field value will be set to the first value of the `label_ids` field.
    #[serde(rename = "label_ids", skip_serializing_if = "Option::is_none")]
    pub label_ids: Option<Vec<i32>>,
    #[serde(rename = "visible_to", skip_serializing_if = "Option::is_none")]
    pub visible_to: Option<VisibleTo>,
}

impl AddOrganizationRequest {
    pub fn new(name: String) -> AddOrganizationRequest {
        AddOrganizationRequest {
            name,
            add_time: None,
            owner_id: None,
            label: None,
            label_ids: None,
            visible_to: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VisibleTo {
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "3")]
    Variant3,
    #[serde(rename = "5")]
    Variant5,
    #[serde(rename = "7")]
    Variant7,
}

impl Default for VisibleTo {
    fn default() -> VisibleTo {
        Self::Variant1
    }
}

