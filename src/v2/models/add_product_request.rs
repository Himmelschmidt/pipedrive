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
pub struct AddProductRequest {
    /// The name of the product. Cannot be an empty string
    #[serde(rename = "name")]
    pub name: String,
    /// The product code
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The product description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The unit in which this product is sold
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// The tax percentage
    #[serde(rename = "tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    /// The category of the product
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<f64>,
    /// The ID of the user who will be marked as the owner of this product. When omitted, the authorized user ID will be used
    #[serde(rename = "owner_id", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i32>,
    /// Whether this product can be added to a deal or not
    #[serde(rename = "is_linkable", skip_serializing_if = "Option::is_none")]
    pub is_linkable: Option<bool>,
    #[serde(rename = "visible_to", skip_serializing_if = "Option::is_none")]
    pub visible_to: Option<VisibleTo>,
    /// An array of objects, each containing: `currency` (string), `price` (number), `cost` (number, optional), `direct_cost` (number, optional). Note that there can only be one price per product per currency. When `prices` is omitted altogether, a default price of 0 and the user's default currency will be assigned.
    #[serde(rename = "prices", skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<serde_json::Value>>,
    /// Only available in Advanced and above plans  How often a customer is billed for access to a service or product 
    #[serde(rename = "billing_frequency", skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<BillingFrequency>,
    /// Only available in Advanced and above plans  The number of times the billing frequency repeats for a product in a deal  When `billing_frequency` is set to `one-time`, this field must be `null`  When `billing_frequency` is set to `weekly`, this field cannot be `null`  For all the other values of `billing_frequency`, `null` represents a product billed indefinitely  Must be a positive integer less or equal to 208 
    #[serde(rename = "billing_frequency_cycles", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub billing_frequency_cycles: Option<Option<i32>>,
}

impl AddProductRequest {
    pub fn new(name: String) -> AddProductRequest {
        AddProductRequest {
            name,
            code: None,
            description: None,
            unit: None,
            tax: None,
            category: None,
            owner_id: None,
            is_linkable: None,
            visible_to: None,
            prices: None,
            billing_frequency: None,
            billing_frequency_cycles: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VisibleTo {
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "3")]
    Variant3,
    #[serde(rename = "5")]
    Variant5,
    #[serde(rename = "7")]
    Variant7,
}

impl Default for VisibleTo {
    fn default() -> VisibleTo {
        Self::Variant1
    }
}
/// Only available in Advanced and above plans  How often a customer is billed for access to a service or product 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BillingFrequency {
    #[serde(rename = "one-time")]
    OneTime,
    #[serde(rename = "annually")]
    Annually,
    #[serde(rename = "semi-annually")]
    SemiAnnually,
    #[serde(rename = "quarterly")]
    Quarterly,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "weekly")]
    Weekly,
}

impl Default for BillingFrequency {
    fn default() -> BillingFrequency {
        Self::OneTime
    }
}

