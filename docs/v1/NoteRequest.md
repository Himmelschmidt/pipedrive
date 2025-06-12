# NoteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | Option<**String**> | The content of the note in HTML format. Subject to sanitization on the back-end. | [optional]
**lead_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The ID of the lead the note will be attached to | [optional]
**deal_id** | Option<**i32**> | The ID of the deal the note will be attached to | [optional]
**person_id** | Option<**i32**> | The ID of the person the note will be attached to | [optional]
**org_id** | Option<**i32**> | The ID of the organization the note will be attached to | [optional]
**project_id** | Option<**i32**> | The ID of the project the note will be attached to | [optional]
**user_id** | Option<**i32**> | The ID of the user who will be marked as the author of the note. Only an admin can change the author. | [optional]
**add_time** | Option<**String**> | The optional creation date & time of the note in UTC. Can be set in the past or in the future. Format: YYYY-MM-DD HH:MM:SS | [optional]
**pinned_to_lead_flag** | Option<**f64**> |  | [optional]
**pinned_to_deal_flag** | Option<**f64**> |  | [optional]
**pinned_to_organization_flag** | Option<**f64**> |  | [optional]
**pinned_to_person_flag** | Option<**f64**> |  | [optional]
**pinned_to_project_flag** | Option<**f64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


