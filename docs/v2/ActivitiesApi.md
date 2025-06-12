# \ActivitiesApi

All URIs are relative to *https://api.pipedrive.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_activity**](ActivitiesApi.md#add_activity) | **POST** /activities | Add a new activity
[**delete_activity**](ActivitiesApi.md#delete_activity) | **DELETE** /activities/{id} | Delete an activity
[**get_activities**](ActivitiesApi.md#get_activities) | **GET** /activities | Get all activities
[**get_activity**](ActivitiesApi.md#get_activity) | **GET** /activities/{id} | Get details of an activity
[**update_activity**](ActivitiesApi.md#update_activity) | **PATCH** /activities/{id} | Update an activity



## add_activity

> models::UpsertActivityResponse add_activity(add_activity_request)
Add a new activity

Adds a new activity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_activity_request** | Option<[**AddActivityRequest**](AddActivityRequest.md)> |  |  |

### Return type

[**models::UpsertActivityResponse**](UpsertActivityResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_activity

> models::DeleteActivityResponse delete_activity(id)
Delete an activity

Marks an activity as deleted. After 30 days, the activity will be permanently deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the activity | [required] |

### Return type

[**models::DeleteActivityResponse**](DeleteActivityResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_activities

> models::GetActivitiesResponse get_activities(filter_id, ids, owner_id, deal_id, lead_id, person_id, org_id, done, updated_since, updated_until, sort_by, sort_direction, include_fields, limit, cursor)
Get all activities

Returns data about all activities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_id** | Option<**i32**> | If supplied, only activities matching the specified filter are returned |  |
**ids** | Option<**String**> | Optional comma separated string array of up to 100 entity ids to fetch. If filter_id is provided, this is ignored. If any of the requested entities do not exist or are not visible, they are not included in the response. |  |
**owner_id** | Option<**i32**> | If supplied, only activities owned by the specified user are returned. If filter_id is provided, this is ignored. |  |
**deal_id** | Option<**i32**> | If supplied, only activities linked to the specified deal are returned. If filter_id is provided, this is ignored. |  |
**lead_id** | Option<**String**> | If supplied, only activities linked to the specified lead are returned. If filter_id is provided, this is ignored. |  |
**person_id** | Option<**i32**> | If supplied, only activities whose primary participant is the given person are returned. If filter_id is provided, this is ignored. |  |
**org_id** | Option<**i32**> | If supplied, only activities linked to the specified organization are returned. If filter_id is provided, this is ignored. |  |
**done** | Option<**bool**> | If supplied, only activities with specified 'done' flag value are returned |  |
**updated_since** | Option<**String**> | If set, only activities with an `update_time` later than or equal to this time are returned. In RFC3339 format, e.g. 2025-01-01T10:20:00Z. |  |
**updated_until** | Option<**String**> | If set, only activities with an `update_time` earlier than this time are returned. In RFC3339 format, e.g. 2025-01-01T10:20:00Z. |  |
**sort_by** | Option<**String**> | The field to sort by. Supported fields: `id`, `update_time`, `add_time`, `due_date`. |  |[default to id]
**sort_direction** | Option<**String**> | The sorting direction. Supported values: `asc`, `desc`. |  |[default to asc]
**include_fields** | Option<**String**> | Optional comma separated string array of additional fields to include |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |

### Return type

[**models::GetActivitiesResponse**](GetActivitiesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_activity

> models::UpsertActivityResponse get_activity(id, include_fields)
Get details of an activity

Returns the details of a specific activity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the activity | [required] |
**include_fields** | Option<**String**> | Optional comma separated string array of additional fields to include |  |

### Return type

[**models::UpsertActivityResponse**](UpsertActivityResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_activity

> models::UpsertActivityResponse update_activity(id, add_activity_request)
Update an activity

Updates the properties of an activity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the activity | [required] |
**add_activity_request** | Option<[**AddActivityRequest**](AddActivityRequest.md)> |  |  |

### Return type

[**models::UpsertActivityResponse**](UpsertActivityResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

