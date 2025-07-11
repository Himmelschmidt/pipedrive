/*
 * Pipedrive API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

/// ActivityItemLocation : Location of the activity
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityItemLocation {
    /// The full address of the activity
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Country of the activity
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Admin area level 1 (e.g. state) of the activity
    #[serde(rename = "admin_area_level_1", skip_serializing_if = "Option::is_none")]
    pub admin_area_level_1: Option<String>,
    /// Admin area level 2 (e.g. county) of the activity
    #[serde(rename = "admin_area_level_2", skip_serializing_if = "Option::is_none")]
    pub admin_area_level_2: Option<String>,
    /// Locality (e.g. city) of the activity
    #[serde(rename = "locality", skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// Sublocality (e.g. neighborhood) of the activity
    #[serde(rename = "sublocality", skip_serializing_if = "Option::is_none")]
    pub sublocality: Option<String>,
    /// Route (e.g. street) of the activity
    #[serde(rename = "route", skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// Street number of the activity
    #[serde(rename = "street_number", skip_serializing_if = "Option::is_none")]
    pub street_number: Option<String>,
    /// Postal code of the activity
    #[serde(rename = "postal_code", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
}

impl ActivityItemLocation {
    /// Location of the activity
    pub fn new() -> ActivityItemLocation {
        ActivityItemLocation {
            value: None,
            country: None,
            admin_area_level_1: None,
            admin_area_level_2: None,
            locality: None,
            sublocality: None,
            route: None,
            street_number: None,
            postal_code: None,
        }
    }
}

