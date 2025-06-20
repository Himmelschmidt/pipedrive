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
pub struct AddRecurringSubscriptionRequest {
    /// The ID of the deal this recurring subscription is associated with
    #[serde(rename = "deal_id")]
    pub deal_id: i32,
    /// The currency of the recurring subscription. Accepts a 3-character currency code.
    #[serde(rename = "currency")]
    pub currency: String,
    /// The description of the recurring subscription
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The interval between payments
    #[serde(rename = "cadence_type")]
    pub cadence_type: CadenceType,
    /// Shows how many payments the subscription has. Note that one field must be set: `cycles_count` or `infinite`. If `cycles_count` is set, then `cycle_amount` and `start_date` are also required.
    #[serde(rename = "cycles_count", skip_serializing_if = "Option::is_none")]
    pub cycles_count: Option<i32>,
    /// The amount of each payment
    #[serde(rename = "cycle_amount")]
    pub cycle_amount: i32,
    /// The start date of the recurring subscription. Format: YYYY-MM-DD
    #[serde(rename = "start_date")]
    pub start_date: String,
    /// This indicates that the recurring subscription will last until it's manually canceled or deleted. Note that only one field must be set: `cycles_count` or `infinite`.
    #[serde(rename = "infinite", skip_serializing_if = "Option::is_none")]
    pub infinite: Option<bool>,
    /// Array of additional payments. It requires a minimum structure as follows: [{ amount:SUM, description:DESCRIPTION, due_at:PAYMENT_DATE }]. Replace SUM with a payment amount, DESCRIPTION with an explanation string, PAYMENT_DATE with a date (format YYYY-MM-DD).
    #[serde(rename = "payments", skip_serializing_if = "Option::is_none")]
    pub payments: Option<Vec<serde_json::Value>>,
    /// Indicates that the deal value must be set to recurring subscription's MRR value
    #[serde(rename = "update_deal_value", skip_serializing_if = "Option::is_none")]
    pub update_deal_value: Option<bool>,
}

impl AddRecurringSubscriptionRequest {
    pub fn new(deal_id: i32, currency: String, cadence_type: CadenceType, cycle_amount: i32, start_date: String) -> AddRecurringSubscriptionRequest {
        AddRecurringSubscriptionRequest {
            deal_id,
            currency,
            description: None,
            cadence_type,
            cycles_count: None,
            cycle_amount,
            start_date,
            infinite: None,
            payments: None,
            update_deal_value: None,
        }
    }
}
/// The interval between payments
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CadenceType {
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "quarterly")]
    Quarterly,
    #[serde(rename = "yearly")]
    Yearly,
}

impl Default for CadenceType {
    fn default() -> CadenceType {
        Self::Weekly
    }
}

