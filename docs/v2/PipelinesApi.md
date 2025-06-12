# \PipelinesApi

All URIs are relative to *https://api.pipedrive.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_pipeline**](PipelinesApi.md#add_pipeline) | **POST** /pipelines | Add a new pipeline
[**delete_pipeline**](PipelinesApi.md#delete_pipeline) | **DELETE** /pipelines/{id} | Delete a pipeline
[**get_pipeline**](PipelinesApi.md#get_pipeline) | **GET** /pipelines/{id} | Get one pipeline
[**get_pipelines**](PipelinesApi.md#get_pipelines) | **GET** /pipelines | Get all pipelines
[**update_pipeline**](PipelinesApi.md#update_pipeline) | **PATCH** /pipelines/{id} | Update a pipeline



## add_pipeline

> models::UpsertPipelineResponse add_pipeline(add_pipeline_request)
Add a new pipeline

Adds a new pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_pipeline_request** | Option<[**AddPipelineRequest**](AddPipelineRequest.md)> |  |  |

### Return type

[**models::UpsertPipelineResponse**](UpsertPipelineResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pipeline

> models::DeletePipelineResponse delete_pipeline(id)
Delete a pipeline

Marks a pipeline as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the pipeline | [required] |

### Return type

[**models::DeletePipelineResponse**](DeletePipelineResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline

> models::UpsertPipelineResponse get_pipeline(id)
Get one pipeline

Returns data about a specific pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the pipeline | [required] |

### Return type

[**models::UpsertPipelineResponse**](UpsertPipelineResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipelines

> models::GetPipelinesResponse get_pipelines(sort_by, sort_direction, limit, cursor)
Get all pipelines

Returns data about all pipelines.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort_by** | Option<**String**> | The field to sort by. Supported fields: `id`, `update_time`, `add_time`. |  |[default to id]
**sort_direction** | Option<**String**> | The sorting direction. Supported values: `asc`, `desc`. |  |[default to asc]
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |

### Return type

[**models::GetPipelinesResponse**](GetPipelinesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pipeline

> models::UpsertPipelineResponse update_pipeline(id, update_pipeline_request)
Update a pipeline

Updates the properties of a pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the pipeline | [required] |
**update_pipeline_request** | Option<[**UpdatePipelineRequest**](UpdatePipelineRequest.md)> |  |  |

### Return type

[**models::UpsertPipelineResponse**](UpsertPipelineResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

