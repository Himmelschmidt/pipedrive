# GetMailThreadMessagesResponseAllOfDataInner

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
**from** | Option<[**Vec<models::MailThreadAllOfPartiesToInner>**](MailThread_allOf_parties_to_inner.md)> | Senders of the mail thread | [optional]
**to** | Option<[**Vec<models::MailThreadAllOfPartiesToInner>**](MailThread_allOf_parties_to_inner.md)> | Recipients of the mail thread | [optional]
**cc** | Option<[**Vec<models::MailThreadAllOfPartiesToInner>**](MailThread_allOf_parties_to_inner.md)> | Participants of the Cc | [optional]
**bcc** | Option<[**Vec<models::MailThreadAllOfPartiesToInner>**](MailThread_allOf_parties_to_inner.md)> | Participants of the Bcc | [optional]
**body_url** | Option<**String**> | A link to the mail thread message | [optional]
**mail_thread_id** | Option<**i32**> | ID of the mail thread | [optional]
**draft** | Option<**String**> | If the mail message has a draft status then the value is the mail message object as JSON formatted string, otherwise `null`. | [optional]
**has_body_flag** | Option<**f64**> |  | [optional][default to Variant0]
**sent_flag** | Option<**f64**> |  | [optional][default to Variant0]
**sent_from_pipedrive_flag** | Option<**f64**> |  | [optional][default to Variant0]
**message_time** | Option<**String**> | The time when the mail message was received or created | [optional]
**add_time** | Option<**String**> | The time when the mail message was inserted to database | [optional]
**update_time** | Option<**String**> | The time when the mail message was updated in database received | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


