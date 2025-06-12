# \StagesApi

All URIs are relative to *https://api.pipedrive.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_stage**](StagesApi.md#add_stage) | **POST** /stages | Add a new stage
[**delete_stage**](StagesApi.md#delete_stage) | **DELETE** /stages/{id} | Delete a stage
[**get_stage**](StagesApi.md#get_stage) | **GET** /stages/{id} | Get one stage
[**get_stages**](StagesApi.md#get_stages) | **GET** /stages | Get all stages
[**update_stage**](StagesApi.md#update_stage) | **PATCH** /stages/{id} | Update stage details



## add_stage

> models::UpsertStageResponse add_stage(add_stage_request)
Add a new stage

Adds a new stage, returns the ID upon success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_stage_request** | Option<[**AddStageRequest**](AddStageRequest.md)> |  |  |

### Return type

[**models::UpsertStageResponse**](UpsertStageResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_stage

> models::DeleteStageResponse delete_stage(id)
Delete a stage

Marks a stage as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the stage | [required] |

### Return type

[**models::DeleteStageResponse**](DeleteStageResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stage

> models::UpsertStageResponse get_stage(id)
Get one stage

Returns data about a specific stage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the stage | [required] |

### Return type

[**models::UpsertStageResponse**](UpsertStageResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stages

> models::GetStagesResponse get_stages(pipeline_id, sort_by, sort_direction, limit, cursor)
Get all stages

Returns data about all stages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | Option<**i32**> | The ID of the pipeline to fetch stages for. If omitted, stages for all pipelines will be fetched. |  |
**sort_by** | Option<**String**> | The field to sort by. Supported fields: `id`, `update_time`, `add_time`, `order_nr`. |  |[default to id]
**sort_direction** | Option<**String**> | The sorting direction. Supported values: `asc`, `desc`. |  |[default to asc]
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |

### Return type

[**models::GetStagesResponse**](GetStagesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_stage

> models::UpsertStageResponse update_stage(id, update_stage_request)
Update stage details

Updates the properties of a stage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the stage | [required] |
**update_stage_request** | Option<[**UpdateStageRequest**](UpdateStageRequest.md)> |  |  |

### Return type

[**models::UpsertStageResponse**](UpsertStageResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

