# ProjectResponseObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the project, generated when the task was created | [optional]
**title** | Option<**String**> | The title of the project | [optional]
**board_id** | Option<**f64**> | The ID of the board this project is associated with | [optional]
**phase_id** | Option<**f64**> | The ID of the phase this project is associated with | [optional]
**description** | Option<**String**> | The description of the project | [optional]
**status** | Option<**String**> | The status of the project | [optional]
**owner_id** | Option<**f64**> | The ID of a project owner | [optional]
**start_date** | Option<[**String**](string.md)> | The start date of the project. Format: YYYY-MM-DD. | [optional]
**end_date** | Option<[**String**](string.md)> | The end date of the project. Format: YYYY-MM-DD. | [optional]
**deal_ids** | Option<**Vec<i32>**> | An array of IDs of the deals this project is associated with | [optional]
**org_id** | Option<**f64**> | The ID of the organization this project is associated with | [optional]
**person_id** | Option<**f64**> | The ID of the person this project is associated with | [optional]
**labels** | Option<**Vec<i32>**> | An array of IDs of the labels this project has | [optional]
**add_time** | Option<**String**> | The creation date and time of the project in UTC. Format: YYYY-MM-DD HH:MM:SS. | [optional]
**update_time** | Option<**String**> | The update date and time of the project in UTC. Format: YYYY-MM-DD HH:MM:SS. | [optional]
**status_change_time** | Option<**String**> | The status changed date and time of the project in UTC. Format: YYYY-MM-DD HH:MM:SS. | [optional]
**archive_time** | Option<**String**> | The archived date and time of the project in UTC. Format: YYYY-MM-DD HH:MM:SS. If not archived then 'null'. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


