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

/// GetPipelineDealsMovementsStatisticsResponseAllOfDataAverageAgeInDaysByStagesInner : The moved deals average age by the stage
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetPipelineDealsMovementsStatisticsResponseAllOfDataAverageAgeInDaysByStagesInner {
    /// The stage ID
    #[serde(rename = "stage_id", skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<i32>,
    /// The average deals age in specific stage
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

impl GetPipelineDealsMovementsStatisticsResponseAllOfDataAverageAgeInDaysByStagesInner {
    /// The moved deals average age by the stage
    pub fn new() -> GetPipelineDealsMovementsStatisticsResponseAllOfDataAverageAgeInDaysByStagesInner {
        GetPipelineDealsMovementsStatisticsResponseAllOfDataAverageAgeInDaysByStagesInner {
            stage_id: None,
            value: None,
        }
    }
}

