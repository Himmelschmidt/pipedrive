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
pub struct CreateFieldRequest {
    /// The name of the field
    #[serde(rename = "name")]
    pub name: String,
    /// When `field_type` is either set or enum, possible options must be supplied as a JSON-encoded sequential array of objects. Example: `[{\"label\":\"New Item\"}]`
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<serde_json::Value>>,
    /// Whether the field is available in the 'add new' modal or not (both in the web and mobile app)
    #[serde(rename = "add_visible_flag", skip_serializing_if = "Option::is_none")]
    pub add_visible_flag: Option<bool>,
    /// The type of the field<table><tr><th>Value</th><th>Description</th></tr><tr><td>`address`</td><td>Address field</td></tr><tr><td>`date`</td><td>Date (format YYYY-MM-DD)</td></tr><tr><td>`daterange`</td><td>Date-range field (has a start date and end date value, both YYYY-MM-DD)</td></tr><tr><td>`double`</td><td>Numeric value</td></tr><tr><td>`enum`</td><td>Options field with a single possible chosen option</td></tr><tr></tr><tr><td>`monetary`</td><td>Monetary field (has a numeric value and a currency value)</td></tr><tr><td>`org`</td><td>Organization field (contains an organization ID which is stored on the same account)</td></tr><tr><td>`people`</td><td>Person field (contains a person ID which is stored on the same account)</td></tr><tr><td>`phone`</td><td>Phone field (up to 255 numbers and/or characters)</td></tr><tr><td>`set`</td><td>Options field with a possibility of having multiple chosen options</td></tr><tr><td>`text`</td><td>Long text (up to 65k characters)</td></tr><tr><td>`time`</td><td>Time field (format HH:MM:SS)</td></tr><tr><td>`timerange`</td><td>Time-range field (has a start time and end time value, both HH:MM:SS)</td></tr><tr><td>`user`</td><td>User field (contains a user ID of another Pipedrive user)</td></tr><tr><td>`varchar`</td><td>Text (up to 255 characters)</td></tr><tr><td>`varchar_auto`</td><td>Autocomplete text (up to 255 characters)</td></tr><tr><td>`visible_to`</td><td>System field that keeps item's visibility setting</td></tr></table>
    #[serde(rename = "field_type")]
    pub field_type: FieldType,
}

impl CreateFieldRequest {
    pub fn new(name: String, field_type: FieldType) -> CreateFieldRequest {
        CreateFieldRequest {
            name,
            options: None,
            add_visible_flag: None,
            field_type,
        }
    }
}
/// The type of the field<table><tr><th>Value</th><th>Description</th></tr><tr><td>`address`</td><td>Address field</td></tr><tr><td>`date`</td><td>Date (format YYYY-MM-DD)</td></tr><tr><td>`daterange`</td><td>Date-range field (has a start date and end date value, both YYYY-MM-DD)</td></tr><tr><td>`double`</td><td>Numeric value</td></tr><tr><td>`enum`</td><td>Options field with a single possible chosen option</td></tr><tr></tr><tr><td>`monetary`</td><td>Monetary field (has a numeric value and a currency value)</td></tr><tr><td>`org`</td><td>Organization field (contains an organization ID which is stored on the same account)</td></tr><tr><td>`people`</td><td>Person field (contains a person ID which is stored on the same account)</td></tr><tr><td>`phone`</td><td>Phone field (up to 255 numbers and/or characters)</td></tr><tr><td>`set`</td><td>Options field with a possibility of having multiple chosen options</td></tr><tr><td>`text`</td><td>Long text (up to 65k characters)</td></tr><tr><td>`time`</td><td>Time field (format HH:MM:SS)</td></tr><tr><td>`timerange`</td><td>Time-range field (has a start time and end time value, both HH:MM:SS)</td></tr><tr><td>`user`</td><td>User field (contains a user ID of another Pipedrive user)</td></tr><tr><td>`varchar`</td><td>Text (up to 255 characters)</td></tr><tr><td>`varchar_auto`</td><td>Autocomplete text (up to 255 characters)</td></tr><tr><td>`visible_to`</td><td>System field that keeps item's visibility setting</td></tr></table>
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FieldType {
    #[serde(rename = "address")]
    Address,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "daterange")]
    Daterange,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "enum")]
    Enum,
    #[serde(rename = "monetary")]
    Monetary,
    #[serde(rename = "org")]
    Org,
    #[serde(rename = "people")]
    People,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "time")]
    Time,
    #[serde(rename = "timerange")]
    Timerange,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "varchar")]
    Varchar,
    #[serde(rename = "varchar_auto")]
    VarcharAuto,
    #[serde(rename = "visible_to")]
    VisibleTo,
}

impl Default for FieldType {
    fn default() -> FieldType {
        Self::Address
    }
}

