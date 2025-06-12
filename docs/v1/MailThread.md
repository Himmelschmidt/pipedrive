# MailThread

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | ID of the mail thread | [optional]
**account_id** | Option<**String**> | The connection account ID | [optional]
**user_id** | Option<**i32**> | ID of the user whom mail message will be assigned to | [optional]
**subject** | Option<**String**> | The subject | [optional]
**snippet** | Option<**String**> | A snippet | [optional]
**read_flag** | Option<**f64**> |  | [optional][default to Variant0]
**mail_tracking_status** | Option<**String**> | Mail tracking status | [optional]
**has_attachments_flag** | Option<**f64**> |  | [optional][default to Variant0]
**has_inline_attachments_flag** | Option<**f64**> |  | [optional][default to Variant0]
**has_real_attachments_flag** | Option<**f64**> |  | [optional][default to Variant0]
**deleted_flag** | Option<**f64**> |  | [optional][default to Variant0]
**synced_flag** | Option<**f64**> |  | [optional][default to Variant0]
**smart_bcc_flag** | Option<**f64**> |  | [optional][default to Variant0]
**mail_link_tracking_enabled_flag** | Option<**f64**> |  | [optional][default to Variant0]
**parties** | Option<[**models::MailThreadAllOfParties**](MailThread_allOf_parties.md)> |  | [optional]
**drafts_parties** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Parties of the drafted mail thread | [optional]
**folders** | Option<**Vec<String>**> | Folders in which messages from thread are being stored | [optional]
**version** | Option<**f64**> | Version | [optional]
**snippet_draft** | Option<**String**> | A snippet from a draft | [optional]
**snippet_sent** | Option<**String**> | A snippet from a message sent | [optional]
**message_count** | Option<**i32**> | An amount of messages | [optional]
**has_draft_flag** | Option<**f64**> |  | [optional][default to Variant0]
**has_sent_flag** | Option<**f64**> |  | [optional][default to Variant0]
**archived_flag** | Option<**f64**> |  | [optional][default to Variant0]
**shared_flag** | Option<**f64**> |  | [optional][default to Variant0]
**external_deleted_flag** | Option<**f64**> |  | [optional][default to Variant0]
**first_message_to_me_flag** | Option<**f64**> |  | [optional][default to Variant0]
**last_message_timestamp** | Option<**String**> | Last message timestamp | [optional]
**first_message_timestamp** | Option<**String**> | The time when the mail thread has had the first message received or created | [optional]
**last_message_sent_timestamp** | Option<**String**> | The last time when the mail thread has had a message sent | [optional]
**last_message_received_timestamp** | Option<**String**> | The last time when the mail thread has had a message received | [optional]
**add_time** | Option<**String**> | The time when the mail thread was inserted to database | [optional]
**update_time** | Option<**String**> | The time when the mail thread was updated in database received | [optional]
**deal_id** | Option<**i32**> | The ID of the deal | [optional]
**deal_status** | Option<**String**> | Status of the deal | [optional]
**lead_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The ID of the lead | [optional]
**all_messages_sent_flag** | Option<**f64**> |  | [optional][default to Variant0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


