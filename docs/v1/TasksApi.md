# \TasksApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_task**](TasksApi.md#add_task) | **POST** /tasks | Add a task
[**delete_task**](TasksApi.md#delete_task) | **DELETE** /tasks/{id} | Delete a task
[**get_task**](TasksApi.md#get_task) | **GET** /tasks/{id} | Get details of a task
[**get_tasks**](TasksApi.md#get_tasks) | **GET** /tasks | Get all tasks
[**update_task**](TasksApi.md#update_task) | **PUT** /tasks/{id} | Update a task



## add_task

> models::AddTaskResponse add_task(add_task_request)
Add a task

Adds a new task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_task_request** | Option<[**AddTaskRequest**](AddTaskRequest.md)> |  |  |

### Return type

[**models::AddTaskResponse**](AddTaskResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_task

> models::DeleteTaskResponse delete_task(id)
Delete a task

Marks a task as deleted. If the task has subtasks then those will also be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the task | [required] |

### Return type

[**models::DeleteTaskResponse**](DeleteTaskResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_task

> models::GetTaskResponse get_task(id)
Get details of a task

Returns the details of a specific task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the task | [required] |

### Return type

[**models::GetTaskResponse**](GetTaskResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tasks

> models::GetTasksResponse get_tasks(cursor, limit, assignee_id, project_id, parent_task_id, done)
Get all tasks

Returns all tasks. This is a cursor-paginated endpoint. For more information, please refer to our documentation on <a href=\"https://pipedrive.readme.io/docs/core-api-concepts-pagination\" target=\"_blank\" rel=\"noopener noreferrer\">pagination</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, up to 500 items will be returned. |  |
**assignee_id** | Option<**i32**> | If supplied, only tasks that are assigned to this user are returned |  |
**project_id** | Option<**i32**> | If supplied, only tasks that are assigned to this project are returned |  |
**parent_task_id** | Option<**i32**> | If `null` is supplied then only parent tasks are returned. If integer is supplied then only subtasks of a specific task are returned. By default all tasks are returned. |  |
**done** | Option<**f64**> | Whether the task is done or not. `0` = Not done, `1` = Done. If not omitted then returns both done and not done tasks. |  |

### Return type

[**models::GetTasksResponse**](GetTasksResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_task

> models::UpdateTaskResponse update_task(id, update_project_request1)
Update a task

Updates a task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the task | [required] |
**update_project_request1** | Option<[**UpdateProjectRequest1**](UpdateProjectRequest1.md)> |  |  |

### Return type

[**models::UpdateTaskResponse**](UpdateTaskResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

