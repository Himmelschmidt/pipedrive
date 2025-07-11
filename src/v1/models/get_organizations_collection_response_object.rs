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
pub struct GetOrganizationsCollectionResponseObject {
    /// The full address of the organization
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The sub-premise of the organization location
    #[serde(rename = "address_subpremise", skip_serializing_if = "Option::is_none")]
    pub address_subpremise: Option<String>,
    /// The street number of the organization location
    #[serde(rename = "address_street_number", skip_serializing_if = "Option::is_none")]
    pub address_street_number: Option<String>,
    /// The route of the organization location
    #[serde(rename = "address_route", skip_serializing_if = "Option::is_none")]
    pub address_route: Option<String>,
    /// The sub-locality of the organization location
    #[serde(rename = "address_sublocality", skip_serializing_if = "Option::is_none")]
    pub address_sublocality: Option<String>,
    /// The locality of the organization location
    #[serde(rename = "address_locality", skip_serializing_if = "Option::is_none")]
    pub address_locality: Option<String>,
    /// The level 1 admin area of the organization location
    #[serde(rename = "address_admin_area_level_1", skip_serializing_if = "Option::is_none")]
    pub address_admin_area_level_1: Option<String>,
    /// The level 2 admin area of the organization location
    #[serde(rename = "address_admin_area_level_2", skip_serializing_if = "Option::is_none")]
    pub address_admin_area_level_2: Option<String>,
    /// The country of the organization location
    #[serde(rename = "address_country", skip_serializing_if = "Option::is_none")]
    pub address_country: Option<String>,
    /// The postal code of the organization location
    #[serde(rename = "address_postal_code", skip_serializing_if = "Option::is_none")]
    pub address_postal_code: Option<String>,
    /// The formatted organization location
    #[serde(rename = "address_formatted_address", skip_serializing_if = "Option::is_none")]
    pub address_formatted_address: Option<String>,
    /// The ID of the organization
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Whether the organization is active or not
    #[serde(rename = "active_flag", skip_serializing_if = "Option::is_none")]
    pub active_flag: Option<bool>,
    /// The ID of the owner
    #[serde(rename = "owner_id", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i32>,
    /// The name of the organization
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The last updated date and time of the organization. Format: YYYY-MM-DD HH:MM:SS
    #[serde(rename = "update_time", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// The date and time this organization was deleted. Format: YYYY-MM-DD HH:MM:SS
    #[serde(rename = "delete_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<Option<String>>,
    /// The date and time when the organization was added/created. Format: YYYY-MM-DD HH:MM:SS
    #[serde(rename = "add_time", skip_serializing_if = "Option::is_none")]
    pub add_time: Option<String>,
    /// The visibility group ID of who can see the organization
    #[serde(rename = "visible_to", skip_serializing_if = "Option::is_none")]
    pub visible_to: Option<String>,
    /// The label assigned to the organization. When the `label` field is updated, the `label_ids` field value will be overwritten by the `label` field value.
    #[serde(rename = "label", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub label: Option<Option<i32>>,
    /// The IDs of labels assigned to the organization. When the `label_ids` field is updated, the `label` field value will be set to the first value of the `label_ids` field.
    #[serde(rename = "label_ids", skip_serializing_if = "Option::is_none")]
    pub label_ids: Option<Vec<i32>>,
    /// The BCC email associated with the organization
    #[serde(rename = "cc_email", skip_serializing_if = "Option::is_none")]
    pub cc_email: Option<String>,
}

impl GetOrganizationsCollectionResponseObject {
    pub fn new() -> GetOrganizationsCollectionResponseObject {
        GetOrganizationsCollectionResponseObject {
            address: None,
            address_subpremise: None,
            address_street_number: None,
            address_route: None,
            address_sublocality: None,
            address_locality: None,
            address_admin_area_level_1: None,
            address_admin_area_level_2: None,
            address_country: None,
            address_postal_code: None,
            address_formatted_address: None,
            id: None,
            active_flag: None,
            owner_id: None,
            name: None,
            update_time: None,
            delete_time: None,
            add_time: None,
            visible_to: None,
            label: None,
            label_ids: None,
            cc_email: None,
        }
    }
}

