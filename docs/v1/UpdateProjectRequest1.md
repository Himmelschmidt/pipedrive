# UpdateProjectRequest1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | The title of the task | [optional]
**project_id** | Option<**f64**> | The ID of the project this task is associated with | [optional]
**description** | Option<**String**> | The description of the task | [optional]
**parent_task_id** | Option<**f64**> | The ID of a parent task. Can not be ID of a task which is already a subtask. | [optional]
**assignee_id** | Option<**f64**> | The ID of the user who will be the assignee of the task | [optional]
**done** | Option<**f64**> |  | [optional]
**due_date** | Option<[**String**](string.md)> | The due date of the task. Format: YYYY-MM-DD. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


