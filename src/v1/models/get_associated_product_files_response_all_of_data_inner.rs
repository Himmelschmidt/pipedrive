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

/// GetAssociatedProductFilesResponseAllOfDataInner : The file data
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAssociatedProductFilesResponseAllOfDataInner {
    /// The ID of the file
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The ID of the product associated with the file
    #[serde(rename = "product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i32>,
    /// The UTC date time when the file was uploaded. Format: YYYY-MM-DD HH:MM:SS
    #[serde(rename = "add_time", skip_serializing_if = "Option::is_none")]
    pub add_time: Option<String>,
    /// The UTC date time when the file was last updated. Format: YYYY-MM-DD HH:MM:SS
    #[serde(rename = "update_time", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// The original name of the file
    #[serde(rename = "file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// The size of the file in bytes
    #[serde(rename = "file_size", skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i32>,
    /// Whether the user is active or not.
    #[serde(rename = "active_flag", skip_serializing_if = "Option::is_none")]
    pub active_flag: Option<bool>,
    /// Whether the file was uploaded as inline or not
    #[serde(rename = "inline_flag", skip_serializing_if = "Option::is_none")]
    pub inline_flag: Option<bool>,
    /// The location type to send the file to. Only googledrive is supported at the moment.
    #[serde(rename = "remote_location", skip_serializing_if = "Option::is_none")]
    pub remote_location: Option<String>,
    /// The ID of the remote item
    #[serde(rename = "remote_id", skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    /// The location of the cloud storage
    #[serde(rename = "s3_bucket", skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    /// The name of the product associated with the file
    #[serde(rename = "product_name", skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// The URL to download the file
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The visible name of the file
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the file
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl GetAssociatedProductFilesResponseAllOfDataInner {
    /// The file data
    pub fn new() -> GetAssociatedProductFilesResponseAllOfDataInner {
        GetAssociatedProductFilesResponseAllOfDataInner {
            id: None,
            product_id: None,
            add_time: None,
            update_time: None,
            file_name: None,
            file_size: None,
            active_flag: None,
            inline_flag: None,
            remote_location: None,
            remote_id: None,
            s3_bucket: None,
            product_name: None,
            url: None,
            name: None,
            description: None,
        }
    }
}

