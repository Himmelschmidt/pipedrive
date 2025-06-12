# \StagesApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_stage**](StagesApi.md#add_stage) | **POST** /stages | Add a new stage
[**delete_stage**](StagesApi.md#delete_stage) | **DELETE** /stages/{id} | Delete a stage
[**delete_stages**](StagesApi.md#delete_stages) | **DELETE** /stages | Delete multiple stages in bulk
[**get_stage**](StagesApi.md#get_stage) | **GET** /stages/{id} | Get one stage
[**get_stage_deals**](StagesApi.md#get_stage_deals) | **GET** /stages/{id}/deals | Get deals in a stage
[**get_stages**](StagesApi.md#get_stages) | **GET** /stages | Get all stages
[**update_stage**](StagesApi.md#update_stage) | **PUT** /stages/{id} | Update stage details



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


## delete_stages

> models::DeleteStagesResponse delete_stages(ids)
Delete multiple stages in bulk

Marks multiple stages as deleted. <br>This endpoint has been deprecated. Please use <a href=\"https://developers.pipedrive.com/docs/api/v1/Stages#deleteStage\" target=\"_blank\" rel=\"noopener noreferrer\">DELETE /api/v2/stages/{id}</a> instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | The comma-separated stage IDs to delete | [required] |

### Return type

[**models::DeleteStagesResponse**](DeleteStagesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stage

> models::GetStageResponse get_stage(id)
Get one stage

Returns data about a specific stage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the stage | [required] |

### Return type

[**models::GetStageResponse**](GetStageResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stage_deals

> models::GetStageDealsResponse get_stage_deals(id, filter_id, user_id, everyone, start, limit)
Get deals in a stage

Lists deals in a specific stage. If no parameters are provided open deals owned by the authorized user will be returned. <br>This endpoint has been deprecated. Please use <a href=\"https://developers.pipedrive.com/docs/api/v1/Deals#getDeals\" target=\"_blank\" rel=\"noopener noreferrer\">GET /api/v2/deals?stage_id={id}</a> instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the stage | [required] |
**filter_id** | Option<**i32**> | If supplied, only deals matching the given filter will be returned |  |
**user_id** | Option<**i32**> | If supplied, `filter_id` will not be considered and only deals owned by the given user will be returned. If omitted, deals owned by the authorized user will be returned. |  |
**everyone** | Option<**f64**> | If supplied, `filter_id` and `user_id` will not be considered – instead, deals owned by everyone will be returned |  |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**models::GetStageDealsResponse**](GetStageDealsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stages

> models::GetStagesResponse get_stages(pipeline_id, start, limit)
Get all stages

Returns data about all stages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | Option<**i32**> | The ID of the pipeline to fetch stages for. If omitted, stages for all pipelines will be fetched. |  |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

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

