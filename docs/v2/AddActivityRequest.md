# AddActivityRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subject** | Option<**String**> | The subject of the activity | [optional]
**r#type** | Option<**String**> | The type of the activity | [optional]
**owner_id** | Option<**i32**> | The ID of the user who owns the activity | [optional]
**deal_id** | Option<**i32**> | The ID of the deal linked to the activity | [optional]
**lead_id** | Option<**String**> | The ID of the lead linked to the activity | [optional]
**person_id** | Option<**i32**> | The ID of the person linked to the activity | [optional]
**org_id** | Option<**i32**> | The ID of the organization linked to the activity | [optional]
**project_id** | Option<**i32**> | The ID of the project linked to the activity | [optional]
**due_date** | Option<**String**> | The due date of the activity | [optional]
**due_time** | Option<**String**> | The due time of the activity | [optional]
**duration** | Option<**String**> | The duration of the activity | [optional]
**busy** | Option<**bool**> | Whether the activity marks the assignee as busy or not in their calendar | [optional]
**done** | Option<**bool**> | Whether the activity is marked as done or not | [optional]
**location** | Option<[**models::ActivityItemLocation**](ActivityItem_location.md)> |  | [optional]
**participants** | Option<[**Vec<models::ActivityItemParticipantsInner>**](ActivityItem_participants_inner.md)> | The participants of the activity | [optional]
**attendees** | Option<[**Vec<models::ActivityItemAttendeesInner>**](ActivityItem_attendees_inner.md)> | The attendees of the activity | [optional]
**public_description** | Option<**String**> | The public description of the activity | [optional]
**priority** | Option<**i32**> | The priority of the activity. Mappable to a specific string using activityFields API. | [optional]
**note** | Option<**String**> | The note of the activity | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


