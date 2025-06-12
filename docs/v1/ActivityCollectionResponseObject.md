# ActivityCollectionResponseObject

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
**id** | Option<**i32**> | The ID of the activity, generated when the activity was created | [optional]
**done** | Option<**bool**> | Whether the activity is done or not | [optional]
**subject** | Option<**String**> | The subject of the activity | [optional]
**r#type** | Option<**String**> | The type of the activity. This is in correlation with the `key_string` parameter of ActivityTypes. | [optional]
**user_id** | Option<**i32**> | The ID of the user whom the activity is assigned to | [optional]
**busy_flag** | Option<**bool**> | Marks if the activity is set as 'Busy' or 'Free'. If the flag is set to `true`, your customers will not be able to book that time slot through any Scheduler links. The flag can also be unset. When the value of the flag is unset (`null`), the flag defaults to 'Busy' if it has a time set, and 'Free' if it is an all-day event without specified time. | [optional]
**company_id** | Option<**i32**> | The user's company ID | [optional]
**conference_meeting_client** | Option<**String**> | The ID of the Marketplace app, which is connected to this activity | [optional]
**conference_meeting_url** | Option<**String**> | The link to join the meeting which is associated with this activity | [optional]
**conference_meeting_id** | Option<**String**> | The meeting ID of the meeting provider (Zoom, MS Teams etc.) that is associated with this activity | [optional]
**add_time** | Option<**String**> | The creation date and time of the activity in UTC. Format: YYYY-MM-DD HH:MM:SS. | [optional]
**marked_as_done_time** | Option<**String**> | The date and time this activity was marked as done. Format: YYYY-MM-DD HH:MM:SS. | [optional]
**active_flag** | Option<**bool**> | Whether the activity is active or not | [optional]
**update_time** | Option<**String**> | The last update date and time of the activity. Format: YYYY-MM-DD HH:MM:SS. | [optional]
**update_user_id** | Option<**i32**> | The ID of the user who was the last to update this activity | [optional]
**source_timezone** | Option<**String**> | The timezone the activity was created in an external calendar | [optional]
**location_subpremise** | Option<**String**> | A subfield of the location field. Indicates apartment/suite number. | [optional]
**location_street_number** | Option<**String**> | A subfield of the location field. Indicates house number. | [optional]
**location_route** | Option<**String**> | A subfield of the location field. Indicates street name. | [optional]
**location_sublocality** | Option<**String**> | A subfield of the location field. Indicates district/sublocality. | [optional]
**location_locality** | Option<**String**> | A subfield of the location field. Indicates city/town/village/locality. | [optional]
**location_admin_area_level_1** | Option<**String**> | A subfield of the location field. Indicates state/county. | [optional]
**location_admin_area_level_2** | Option<**String**> | A subfield of the location field. Indicates region. | [optional]
**location_country** | Option<**String**> | A subfield of the location field. Indicates country. | [optional]
**location_postal_code** | Option<**String**> | A subfield of the location field. Indicates ZIP/postal code. | [optional]
**location_formatted_address** | Option<**String**> | A subfield of the location field. Indicates full/combined address. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


