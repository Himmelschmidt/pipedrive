# \ProjectTemplatesApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_project_template**](ProjectTemplatesApi.md#get_project_template) | **GET** /projectTemplates/{id} | Get details of a template
[**get_project_templates**](ProjectTemplatesApi.md#get_project_templates) | **GET** /projectTemplates | Get all project templates
[**get_projects_board**](ProjectTemplatesApi.md#get_projects_board) | **GET** /projects/boards/{id} | Get details of a board
[**get_projects_phase**](ProjectTemplatesApi.md#get_projects_phase) | **GET** /projects/phases/{id} | Get details of a phase



## get_project_template

> models::GetProjectTemplateResponse get_project_template(id)
Get details of a template

Returns the details of a specific project template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the project template | [required] |

### Return type

[**models::GetProjectTemplateResponse**](GetProjectTemplateResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_templates

> models::GetProjectTemplatesResponse get_project_templates(cursor, limit)
Get all project templates

Returns all not deleted project templates. This is a cursor-paginated endpoint. For more information, please refer to our documentation on <a href=\"https://pipedrive.readme.io/docs/core-api-concepts-pagination\" target=\"_blank\" rel=\"noopener noreferrer\">pagination</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, up to 500 items will be returned. |  |

### Return type

[**models::GetProjectTemplatesResponse**](GetProjectTemplatesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_board

> models::GetProjectBoardResponse get_projects_board(id)
Get details of a board

Returns the details of a specific project board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the project board | [required] |

### Return type

[**models::GetProjectBoardResponse**](GetProjectBoardResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_phase

> models::GetProjectPhaseResponse get_projects_phase(id)
Get details of a phase

Returns the details of a specific project phase.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the project phase | [required] |

### Return type

[**models::GetProjectPhaseResponse**](GetProjectPhaseResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

