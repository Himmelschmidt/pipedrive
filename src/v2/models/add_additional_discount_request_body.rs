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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddAdditionalDiscountRequestBody {
    /// The name of the discount.
    #[serde(rename = "description")]
    pub description: String,
    /// The discount amount. Must be a positive number (excluding 0).
    #[serde(rename = "amount")]
    pub amount: f64,
    /// Determines whether the discount is applied as a percentage or a fixed amount.
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl AddAdditionalDiscountRequestBody {
    pub fn new(description: String, amount: f64, r#type: Type) -> AddAdditionalDiscountRequestBody {
        AddAdditionalDiscountRequestBody {
            description,
            amount,
            r#type,
        }
    }
}
/// Determines whether the discount is applied as a percentage or a fixed amount.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "percentage")]
    Percentage,
    #[serde(rename = "amount")]
    Amount,
}

impl Default for Type {
    fn default() -> Type {
        Self::Percentage
    }
}

