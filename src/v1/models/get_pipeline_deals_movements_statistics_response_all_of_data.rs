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

/// GetPipelineDealsMovementsStatisticsResponseAllOfData : The pipeline object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetPipelineDealsMovementsStatisticsResponseAllOfData {
    #[serde(rename = "movements_between_stages", skip_serializing_if = "Option::is_none")]
    pub movements_between_stages: Option<Box<models::GetPipelineDealsMovementsStatisticsResponseAllOfDataMovementsBetweenStages>>,
    #[serde(rename = "new_deals", skip_serializing_if = "Option::is_none")]
    pub new_deals: Option<Box<models::GetPipelineDealsMovementsStatisticsResponseAllOfDataNewDeals>>,
    #[serde(rename = "deals_left_open", skip_serializing_if = "Option::is_none")]
    pub deals_left_open: Option<Box<models::GetPipelineDealsMovementsStatisticsResponseAllOfDataNewDeals>>,
    #[serde(rename = "won_deals", skip_serializing_if = "Option::is_none")]
    pub won_deals: Option<Box<models::GetPipelineDealsMovementsStatisticsResponseAllOfDataNewDeals>>,
    #[serde(rename = "lost_deals", skip_serializing_if = "Option::is_none")]
    pub lost_deals: Option<Box<models::GetPipelineDealsMovementsStatisticsResponseAllOfDataNewDeals>>,
    #[serde(rename = "average_age_in_days", skip_serializing_if = "Option::is_none")]
    pub average_age_in_days: Option<Box<models::GetPipelineDealsMovementsStatisticsResponseAllOfDataAverageAgeInDays>>,
}

impl GetPipelineDealsMovementsStatisticsResponseAllOfData {
    /// The pipeline object
    pub fn new() -> GetPipelineDealsMovementsStatisticsResponseAllOfData {
        GetPipelineDealsMovementsStatisticsResponseAllOfData {
            movements_between_stages: None,
            new_deals: None,
            deals_left_open: None,
            won_deals: None,
            lost_deals: None,
            average_age_in_days: None,
        }
    }
}

