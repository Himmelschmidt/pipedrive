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

/// GoalTypeParams : The parameters that accompany the goal type
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GoalTypeParams {
    /// The IDs of pipelines of the goal
    #[serde(rename = "pipeline_id", skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<Vec<i32>>,
    /// The IDs of activity types of the goal
    #[serde(rename = "activity_type_id", skip_serializing_if = "Option::is_none")]
    pub activity_type_id: Option<Vec<i32>>,
}

impl GoalTypeParams {
    /// The parameters that accompany the goal type
    pub fn new() -> GoalTypeParams {
        GoalTypeParams {
            pipeline_id: None,
            activity_type_id: None,
        }
    }
}

