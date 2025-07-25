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
pub struct DealCollectionResponseObject {
    /// The ID of the deal
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The ID of the deal creator
    #[serde(rename = "creator_user_id", skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<i32>,
    /// The ID of the user
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    /// The ID of the person associated with the deal
    #[serde(rename = "person_id", skip_serializing_if = "Option::is_none")]
    pub person_id: Option<i32>,
    /// The ID of the organization associated with the deal
    #[serde(rename = "org_id", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i32>,
    /// The ID of the deal stage
    #[serde(rename = "stage_id", skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<i32>,
    /// The title of the deal
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The value of the deal
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// The currency associated with the deal
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// The creation date and time of the deal in UTC. Format: YYYY-MM-DD HH:MM:SS.
    #[serde(rename = "add_time", skip_serializing_if = "Option::is_none")]
    pub add_time: Option<String>,
    /// The last update date and time of the deal in UTC. Format: YYYY-MM-DD HH:MM:SS.
    #[serde(rename = "update_time", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// The status of the deal
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The success probability percentage of the deal
    #[serde(rename = "probability", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub probability: Option<Option<f64>>,
    /// The reason for losing the deal
    #[serde(rename = "lost_reason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub lost_reason: Option<Option<String>>,
    /// The visibility of the deal
    #[serde(rename = "visible_to", skip_serializing_if = "Option::is_none")]
    pub visible_to: Option<String>,
    /// The date and time of closing the deal in UTC. Format: YYYY-MM-DD HH:MM:SS.
    #[serde(rename = "close_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub close_time: Option<Option<String>>,
    /// The ID of the pipeline associated with the deal
    #[serde(rename = "pipeline_id", skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<i32>,
    /// The date and time of changing the deal status to won in UTC. Format: YYYY-MM-DD HH:MM:SS.
    #[serde(rename = "won_time", skip_serializing_if = "Option::is_none")]
    pub won_time: Option<String>,
    /// The date and time of changing the deal status to lost in UTC. Format: YYYY-MM-DD HH:MM:SS.
    #[serde(rename = "lost_time", skip_serializing_if = "Option::is_none")]
    pub lost_time: Option<String>,
    /// The expected close date of the deal
    #[serde(rename = "expected_close_date", skip_serializing_if = "Option::is_none")]
    pub expected_close_date: Option<String>,
    /// The label or multiple labels assigned to the deal
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl DealCollectionResponseObject {
    pub fn new() -> DealCollectionResponseObject {
        DealCollectionResponseObject {
            id: None,
            creator_user_id: None,
            user_id: None,
            person_id: None,
            org_id: None,
            stage_id: None,
            title: None,
            value: None,
            currency: None,
            add_time: None,
            update_time: None,
            status: None,
            probability: None,
            lost_reason: None,
            visible_to: None,
            close_time: None,
            pipeline_id: None,
            won_time: None,
            lost_time: None,
            expected_close_date: None,
            label: None,
        }
    }
}

