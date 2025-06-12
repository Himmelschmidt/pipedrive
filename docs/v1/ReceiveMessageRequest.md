# ReceiveMessageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the message | 
**channel_id** | **String** | The channel ID as in the provider | 
**sender_id** | **String** | The ID of the provider's user that sent the message | 
**conversation_id** | **String** | The ID of the conversation | 
**message** | **String** | The body of the message | 
**status** | **String** | The status of the message | 
**created_at** | **String** | The date and time when the message was created in the provider, in UTC. Format: YYYY-MM-DD HH:MM | 
**reply_by** | Option<**String**> | The date and time when the message can no longer receive a reply, in UTC. Format: YYYY-MM-DD HH:MM | [optional]
**conversation_link** | Option<**String**> | A URL that can open the conversation in the provider's side | [optional]
**attachments** | Option<[**Vec<models::ReceiveMessageRequestAttachmentsInner>**](receiveMessage_request_attachments_inner.md)> | The list of attachments available in the message | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


