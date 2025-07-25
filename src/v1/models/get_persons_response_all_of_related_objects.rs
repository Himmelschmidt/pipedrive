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
pub struct GetPersonsResponseAllOfRelatedObjects {
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::RelatedOrganizationDataWithActiveFlag>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::GetActivitiesResponseRelatedObjectsUser>>,
}

impl GetPersonsResponseAllOfRelatedObjects {
    pub fn new() -> GetPersonsResponseAllOfRelatedObjects {
        GetPersonsResponseAllOfRelatedObjects {
            organization: None,
            user: None,
        }
    }
}

