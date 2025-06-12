# TaskResponseObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the task, generated when the task was created | [optional]
**title** | Option<**String**> | The title of the task | [optional]
**project_id** | Option<**f64**> | The ID of the project this task is associated with | [optional]
**description** | Option<**String**> | The description of the task | [optional]
**parent_task_id** | Option<**f64**> | The ID of a parent task. Can not be ID of a task which is already a subtask. | [optional]
**assignee_id** | Option<**f64**> | The ID of the user who will be the assignee of the task | [optional]
**done** | Option<**f64**> |  | [optional]
**due_date** | Option<[**String**](string.md)> | The due date of the task. Format: YYYY-MM-DD. | [optional]
**creator_id** | Option<**f64**> | The creator of a task | [optional]
**add_time** | Option<**String**> | The creation date and time of the task in UTC. Format: YYYY-MM-DD HH:MM:SS. | [optional]
**update_time** | Option<**String**> | The update date and time of the task in UTC. Format: YYYY-MM-DD HH:MM:SS. | [optional]
**marked_as_done_time** | Option<**String**> | The marked as done date and time of the task in UTC. Format: YYYY-MM-DD HH:MM:SS. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


