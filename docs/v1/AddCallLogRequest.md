# AddCallLogRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | Option<**i32**> | The ID of the owner of the call log. Please note that a user without account settings access cannot create call logs for other users. | [optional]
**activity_id** | Option<**i32**> | If specified, this activity will be converted into a call log, with the information provided. When this field is used, you don't need to specify `deal_id`, `person_id` or `org_id`, as they will be ignored in favor of the values already available in the activity. The `activity_id` must refer to a `call` type activity. | [optional]
**subject** | Option<**String**> | The name of the activity this call is attached to | [optional]
**duration** | Option<**String**> | The duration of the call in seconds | [optional]
**outcome** | **String** | Describes the outcome of the call | 
**from_phone_number** | Option<**String**> | The number that made the call | [optional]
**to_phone_number** | **String** | The number called | 
**start_time** | **String** | The date and time of the start of the call in UTC. Format: YYYY-MM-DD HH:MM:SS. | 
**end_time** | **String** | The date and time of the end of the call in UTC. Format: YYYY-MM-DD HH:MM:SS. | 
**person_id** | Option<**i32**> | The ID of the person this call is associated with | [optional]
**org_id** | Option<**i32**> | The ID of the organization this call is associated with | [optional]
**deal_id** | Option<**i32**> | The ID of the deal this call is associated with. A call log can be associated with either a deal or a lead, but not both at once. | [optional]
**lead_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The ID of the lead in the UUID format this call is associated with. A call log can be associated with either a deal or a lead, but not both at once. | [optional]
**note** | Option<**String**> | The note for the call log in HTML format | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


