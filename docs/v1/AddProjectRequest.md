# AddProjectRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** | The title of the project | 
**board_id** | **f64** | The ID of a project board | 
**phase_id** | **f64** | The ID of a phase on a project board | 
**description** | Option<**String**> | The description of the project | [optional]
**status** | Option<**String**> | The status of the project | [optional]
**owner_id** | Option<**f64**> | The ID of a project owner | [optional]
**start_date** | Option<[**String**](string.md)> | The start date of the project. Format: YYYY-MM-DD. | [optional]
**end_date** | Option<[**String**](string.md)> | The end date of the project. Format: YYYY-MM-DD. | [optional]
**deal_ids** | Option<**Vec<i32>**> | An array of IDs of the deals this project is associated with | [optional]
**org_id** | Option<**f64**> | The ID of the organization this project is associated with | [optional]
**person_id** | Option<**f64**> | The ID of the person this project is associated with | [optional]
**labels** | Option<**Vec<i32>**> | An array of IDs of the labels this project has | [optional]
**template_id** | Option<**f64**> | The ID of the template the project will be based on | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


