# UpdateActivityRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**due_date** | Option<[**String**](string.md)> | The due date of the activity. Format: YYYY-MM-DD | [optional]
**due_time** | Option<**String**> | The due time of the activity in UTC. Format: HH:MM | [optional]
**duration** | Option<**String**> | The duration of the activity. Format: HH:MM | [optional]
**deal_id** | Option<**i32**> | The ID of the deal this activity is associated with | [optional]
**lead_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The ID of the lead in the UUID format this activity is associated with | [optional]
**person_id** | Option<**i32**> | The ID of the person this activity is associated with | [optional]
**project_id** | Option<**i32**> | The ID of the project this activity is associated with | [optional]
**org_id** | Option<**i32**> | The ID of the organization this activity is associated with | [optional]
**location** | Option<**String**> | The address of the activity. | [optional]
**public_description** | Option<**String**> | Additional details about the activity that is synced to your external calendar. Unlike the note added to the activity, the description is publicly visible to any guests added to the activity. | [optional]
**note** | Option<**String**> | The note of the activity (HTML format) | [optional]
**subject** | Option<**String**> | The subject of the activity | [optional]
**r#type** | Option<**String**> | The type of the activity. This is in correlation with the `key_string` parameter of ActivityTypes. | [optional]
**user_id** | Option<**i32**> | The ID of the user whom the activity is assigned to | [optional]
**participants** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | List of multiple persons (participants) this activity is associated with. It requires a structure as follows: `[{\"person_id\":1,\"primary_flag\":true}]` | [optional]
**busy_flag** | Option<**bool**> | Set the activity as 'Busy' or 'Free'. If the flag is set to `true`, your customers will not be able to book that time slot through any Scheduler links. The flag can also be unset by never setting it or overriding it with `null`. When the value of the flag is unset (`null`), the flag defaults to 'Busy' if it has a time set, and 'Free' if it is an all-day event without specified time. | [optional]
**attendees** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | The attendees of the activity. This can be either your existing Pipedrive contacts or an external email address. It requires a structure as follows: `[{\"email_address\":\"mail@example.org\"}]` or `[{\"person_id\":1, \"email_address\":\"mail@example.org\"}]` | [optional]
**done** | Option<**f64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


