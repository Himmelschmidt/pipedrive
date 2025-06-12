# \ActivityTypesApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_activity_type**](ActivityTypesApi.md#add_activity_type) | **POST** /activityTypes | Add new activity type
[**delete_activity_type**](ActivityTypesApi.md#delete_activity_type) | **DELETE** /activityTypes/{id} | Delete an activity type
[**delete_activity_types**](ActivityTypesApi.md#delete_activity_types) | **DELETE** /activityTypes | Delete multiple activity types in bulk
[**get_activity_types**](ActivityTypesApi.md#get_activity_types) | **GET** /activityTypes | Get all activity types
[**update_activity_type**](ActivityTypesApi.md#update_activity_type) | **PUT** /activityTypes/{id} | Update an activity type



## add_activity_type

> models::UpsertActivityTypeResponse add_activity_type(add_activity_type_request)
Add new activity type

Adds a new activity type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_activity_type_request** | Option<[**AddActivityTypeRequest**](AddActivityTypeRequest.md)> |  |  |

### Return type

[**models::UpsertActivityTypeResponse**](UpsertActivityTypeResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_activity_type

> models::UpsertActivityTypeResponse delete_activity_type(id)
Delete an activity type

Marks an activity type as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the activity type | [required] |

### Return type

[**models::UpsertActivityTypeResponse**](UpsertActivityTypeResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_activity_types

> models::DeleteActivityTypesResponse delete_activity_types(ids)
Delete multiple activity types in bulk

Marks multiple activity types as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | The comma-separated activity type IDs | [required] |

### Return type

[**models::DeleteActivityTypesResponse**](DeleteActivityTypesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_activity_types

> models::GetActivityTypesResponse get_activity_types()
Get all activity types

Returns all activity types.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetActivityTypesResponse**](GetActivityTypesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_activity_type

> models::UpsertActivityTypeResponse update_activity_type(id, update_activity_type_request)
Update an activity type

Updates an activity type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the activity type | [required] |
**update_activity_type_request** | Option<[**UpdateActivityTypeRequest**](UpdateActivityTypeRequest.md)> |  |  |

### Return type

[**models::UpsertActivityTypeResponse**](UpsertActivityTypeResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

