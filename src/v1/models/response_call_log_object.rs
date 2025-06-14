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
pub struct ResponseCallLogObject {
    /// The ID of the owner of the call log. Please note that a user without account settings access cannot create call logs for other users.
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    /// If specified, this activity will be converted into a call log, with the information provided. When this field is used, you don't need to specify `deal_id`, `person_id` or `org_id`, as they will be ignored in favor of the values already available in the activity. The `activity_id` must refer to a `call` type activity.
    #[serde(rename = "activity_id", skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<i32>,
    /// The name of the activity this call is attached to
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// The duration of the call in seconds
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Describes the outcome of the call
    #[serde(rename = "outcome")]
    pub outcome: Outcome,
    /// The number that made the call
    #[serde(rename = "from_phone_number", skip_serializing_if = "Option::is_none")]
    pub from_phone_number: Option<String>,
    /// The number called
    #[serde(rename = "to_phone_number")]
    pub to_phone_number: String,
    /// The date and time of the start of the call in UTC. Format: YYYY-MM-DD HH:MM:SS.
    #[serde(rename = "start_time")]
    pub start_time: String,
    /// The date and time of the end of the call in UTC. Format: YYYY-MM-DD HH:MM:SS.
    #[serde(rename = "end_time")]
    pub end_time: String,
    /// The ID of the person this call is associated with
    #[serde(rename = "person_id", skip_serializing_if = "Option::is_none")]
    pub person_id: Option<i32>,
    /// The ID of the organization this call is associated with
    #[serde(rename = "org_id", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i32>,
    /// The ID of the deal this call is associated with. A call log can be associated with either a deal or a lead, but not both at once.
    #[serde(rename = "deal_id", skip_serializing_if = "Option::is_none")]
    pub deal_id: Option<i32>,
    /// The ID of the lead in the UUID format this call is associated with. A call log can be associated with either a deal or a lead, but not both at once.
    #[serde(rename = "lead_id", skip_serializing_if = "Option::is_none")]
    pub lead_id: Option<uuid::Uuid>,
    /// The note for the call log in HTML format
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// The call log ID, generated when the call log was created
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// If the call log has an audio recording attached, the value should be true
    #[serde(rename = "has_recording", skip_serializing_if = "Option::is_none")]
    pub has_recording: Option<bool>,
    /// The company ID of the owner of the call log
    #[serde(rename = "company_id", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<i32>,
}

impl ResponseCallLogObject {
    pub fn new(outcome: Outcome, to_phone_number: String, start_time: String, end_time: String) -> ResponseCallLogObject {
        ResponseCallLogObject {
            user_id: None,
            activity_id: None,
            subject: None,
            duration: None,
            outcome,
            from_phone_number: None,
            to_phone_number,
            start_time,
            end_time,
            person_id: None,
            org_id: None,
            deal_id: None,
            lead_id: None,
            note: None,
            id: None,
            has_recording: None,
            company_id: None,
        }
    }
}
/// Describes the outcome of the call
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Outcome {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "no_answer")]
    NoAnswer,
    #[serde(rename = "left_message")]
    LeftMessage,
    #[serde(rename = "left_voicemail")]
    LeftVoicemail,
    #[serde(rename = "wrong_number")]
    WrongNumber,
    #[serde(rename = "busy")]
    Busy,
}

impl Default for Outcome {
    fn default() -> Outcome {
        Self::Connected
    }
}

