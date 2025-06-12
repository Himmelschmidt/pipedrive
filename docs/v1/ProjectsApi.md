# \ProjectsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_project**](ProjectsApi.md#add_project) | **POST** /projects | Add a project
[**archive_project**](ProjectsApi.md#archive_project) | **POST** /projects/{id}/archive | Archive a project
[**delete_project**](ProjectsApi.md#delete_project) | **DELETE** /projects/{id} | Delete a project
[**get_project**](ProjectsApi.md#get_project) | **GET** /projects/{id} | Get details of a project
[**get_project_activities**](ProjectsApi.md#get_project_activities) | **GET** /projects/{id}/activities | Returns project activities
[**get_project_groups**](ProjectsApi.md#get_project_groups) | **GET** /projects/{id}/groups | Returns project groups
[**get_project_plan**](ProjectsApi.md#get_project_plan) | **GET** /projects/{id}/plan | Returns project plan
[**get_project_tasks**](ProjectsApi.md#get_project_tasks) | **GET** /projects/{id}/tasks | Returns project tasks
[**get_projects**](ProjectsApi.md#get_projects) | **GET** /projects | Get all projects
[**get_projects_boards**](ProjectsApi.md#get_projects_boards) | **GET** /projects/boards | Get all project boards
[**get_projects_phases**](ProjectsApi.md#get_projects_phases) | **GET** /projects/phases | Get project phases
[**put_project_plan_activity**](ProjectsApi.md#put_project_plan_activity) | **PUT** /projects/{id}/plan/activities/{activityId} | Update activity in project plan
[**put_project_plan_task**](ProjectsApi.md#put_project_plan_task) | **PUT** /projects/{id}/plan/tasks/{taskId} | Update task in project plan
[**update_project**](ProjectsApi.md#update_project) | **PUT** /projects/{id} | Update a project



## add_project

> models::AddProjectResponse add_project(add_project_request)
Add a project

Adds a new project. Note that you can supply additional custom fields along with the request that are not described here. These custom fields are different for each Pipedrive account and can be recognized by long hashes as keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_project_request** | Option<[**AddProjectRequest**](AddProjectRequest.md)> |  |  |

### Return type

[**models::AddProjectResponse**](AddProjectResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## archive_project

> models::UpdateProjectResponse archive_project(id)
Archive a project

Archives a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the project | [required] |

### Return type

[**models::UpdateProjectResponse**](UpdateProjectResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project

> models::DeleteProjectResponse delete_project(id)
Delete a project

Marks a project as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the project | [required] |

### Return type

[**models::DeleteProjectResponse**](DeleteProjectResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project

> models::GetProjectResponse get_project(id)
Get details of a project

Returns the details of a specific project. Also note that custom fields appear as long hashes in the resulting data. These hashes can be mapped against the `key` value of project fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the project | [required] |

### Return type

[**models::GetProjectResponse**](GetProjectResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_activities

> models::GetActivitiesCollectionResponse get_project_activities(id)
Returns project activities

Returns activities linked to a specific project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the project | [required] |

### Return type

[**models::GetActivitiesCollectionResponse**](GetActivitiesCollectionResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_groups

> models::GetProjectGroupsResponse get_project_groups(id)
Returns project groups

Returns all active groups under a specific project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the project | [required] |

### Return type

[**models::GetProjectGroupsResponse**](GetProjectGroupsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_plan

> models::GetProjectPlanResponse get_project_plan(id)
Returns project plan

Returns information about items in a project plan. Items consists of tasks and activities and are linked to specific project phase and group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the project | [required] |

### Return type

[**models::GetProjectPlanResponse**](GetProjectPlanResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_tasks

> models::GetTasksResponse get_project_tasks(id)
Returns project tasks

Returns tasks linked to a specific project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the project | [required] |

### Return type

[**models::GetTasksResponse**](GetTasksResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects

> models::GetProjectsResponse get_projects(cursor, limit, filter_id, status, phase_id, include_archived)
Get all projects

Returns all projects. This is a cursor-paginated endpoint. For more information, please refer to our documentation on <a href=\"https://pipedrive.readme.io/docs/core-api-concepts-pagination\" target=\"_blank\" rel=\"noopener noreferrer\">pagination</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. |  |
**filter_id** | Option<**i32**> | The ID of the filter to use |  |
**status** | Option<**String**> | If supplied, includes only projects with the specified statuses. Possible values are `open`, `completed`, `canceled` and `deleted`. By default `deleted` projects are not returned. |  |
**phase_id** | Option<**i32**> | If supplied, only projects in specified phase are returned |  |
**include_archived** | Option<**bool**> | If supplied with `true` then archived projects are also included in the response. By default only not archived projects are returned. |  |

### Return type

[**models::GetProjectsResponse**](GetProjectsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_boards

> models::GetProjectBoardsResponse get_projects_boards()
Get all project boards

Returns all projects boards that are not deleted.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetProjectBoardsResponse**](GetProjectBoardsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_phases

> models::GetProjectPhasesResponse get_projects_phases(board_id)
Get project phases

Returns all active project phases under a specific board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | ID of the board for which phases are requested | [required] |

### Return type

[**models::GetProjectPhasesResponse**](GetProjectPhasesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_project_plan_activity

> models::UpdateActivityPlanItemResponse put_project_plan_activity(id, activity_id, project_put_plan_item_body_object)
Update activity in project plan

Updates an activity phase or group in a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the project | [required] |
**activity_id** | **i32** | The ID of the activity | [required] |
**project_put_plan_item_body_object** | Option<[**ProjectPutPlanItemBodyObject**](ProjectPutPlanItemBodyObject.md)> |  |  |

### Return type

[**models::UpdateActivityPlanItemResponse**](UpdateActivityPlanItemResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_project_plan_task

> models::UpdateTaskPlanItemResponse put_project_plan_task(id, task_id, project_put_plan_item_body_object)
Update task in project plan

Updates a task phase or group in a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the project | [required] |
**task_id** | **i32** | The ID of the task | [required] |
**project_put_plan_item_body_object** | Option<[**ProjectPutPlanItemBodyObject**](ProjectPutPlanItemBodyObject.md)> |  |  |

### Return type

[**models::UpdateTaskPlanItemResponse**](UpdateTaskPlanItemResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project

> models::UpdateProjectResponse update_project(id, update_project_request)
Update a project

Updates a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the project | [required] |
**update_project_request** | Option<[**UpdateProjectRequest**](UpdateProjectRequest.md)> |  |  |

### Return type

[**models::UpdateProjectResponse**](UpdateProjectResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

