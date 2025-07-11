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
pub struct MailMessageDataFromInner {
    /// ID of the mail participant
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Mail address of the mail participant
    #[serde(rename = "email_address", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Name of the mail participant
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ID of the linked person to the mail message
    #[serde(rename = "linked_person_id", skip_serializing_if = "Option::is_none")]
    pub linked_person_id: Option<i32>,
    /// Name of the linked person to the mail message
    #[serde(rename = "linked_person_name", skip_serializing_if = "Option::is_none")]
    pub linked_person_name: Option<String>,
    /// ID of the mail message participant
    #[serde(rename = "mail_message_party_id", skip_serializing_if = "Option::is_none")]
    pub mail_message_party_id: Option<i32>,
}

impl MailMessageDataFromInner {
    pub fn new() -> MailMessageDataFromInner {
        MailMessageDataFromInner {
            id: None,
            email_address: None,
            name: None,
            linked_person_id: None,
            linked_person_name: None,
            mail_message_party_id: None,
        }
    }
}

