# \ActivitiesApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_activity**](ActivitiesApi.md#add_activity) | **POST** /activities | Add an activity
[**delete_activities**](ActivitiesApi.md#delete_activities) | **DELETE** /activities | Delete multiple activities in bulk
[**delete_activity**](ActivitiesApi.md#delete_activity) | **DELETE** /activities/{id} | Delete an activity
[**get_activities**](ActivitiesApi.md#get_activities) | **GET** /activities | Get all activities assigned to a particular user
[**get_activities_collection**](ActivitiesApi.md#get_activities_collection) | **GET** /activities/collection | Get all activities collection
[**get_activity**](ActivitiesApi.md#get_activity) | **GET** /activities/{id} | Get details of an activity
[**update_activity**](ActivitiesApi.md#update_activity) | **PUT** /activities/{id} | Update an activity



## add_activity

> models::AddActivityResponse add_activity(add_activity_request)
Add an activity

Adds a new activity. Includes `more_activities_scheduled_in_context` property in response's `additional_data` which indicates whether there are more undone activities scheduled with the same deal, person or organization (depending on the supplied data). For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/adding-an-activity\" target=\"_blank\" rel=\"noopener noreferrer\">adding an activity</a>. <br /> <br /> ***Starting from 30.09.2024, activity attendees will receive updates only if the activity owner has an active calendar sync***

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_activity_request** | Option<[**AddActivityRequest**](AddActivityRequest.md)> |  |  |

### Return type

[**models::AddActivityResponse**](AddActivityResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_activities

> models::DeleteActivitiesResponse delete_activities(ids)
Delete multiple activities in bulk

Marks multiple activities as deleted. After 30 days, the activities will be permanently deleted. <br>This endpoint has been deprecated. Please use <a href=\"https://developers.pipedrive.com/docs/api/v1/Activities#deleteActivity\" target=\"_blank\" rel=\"noopener noreferrer\">DELETE /api/v2/activities/{id}</a> instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | The comma-separated IDs of activities that will be deleted | [required] |

### Return type

[**models::DeleteActivitiesResponse**](DeleteActivitiesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
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

> models::GetActivitiesResponse get_activities(user_id, filter_id, r#type, limit, start, start_date, end_date, done)
Get all activities assigned to a particular user

Returns all activities assigned to a particular user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**i32**> | The ID of the user whose activities will be fetched. If omitted, the user associated with the API token will be used. If 0, activities for all company users will be fetched based on the permission sets. |  |
**filter_id** | Option<**i32**> | The ID of the filter to use (will narrow down results if used together with `user_id` parameter) |  |
**r#type** | Option<**String**> | The type of the activity, can be one type or multiple types separated by a comma. This is in correlation with the `key_string` parameter of ActivityTypes. |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. |  |
**start** | Option<**i32**> | For pagination, the position that represents the first result for the page |  |
**start_date** | Option<**String**> | Use the activity due date where you wish to begin fetching activities from. Insert due date in YYYY-MM-DD format. |  |
**end_date** | Option<**String**> | Use the activity due date where you wish to stop fetching activities from. Insert due date in YYYY-MM-DD format. |  |
**done** | Option<**f64**> | Whether the activity is done or not. 0 = Not done, 1 = Done. If omitted returns both done and not done activities. |  |

### Return type

[**models::GetActivitiesResponse**](GetActivitiesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_activities_collection

> models::GetActivitiesCollectionResponse get_activities_collection(cursor, limit, since, until, user_id, done, r#type)
Get all activities collection

Returns all activities. Please note that only global admins (those with global permissions) can access this endpoint. Users with regular permissions will receive a 403 response. Read more about global permissions <a href=\"https://support.pipedrive.com/en/article/global-user-management\" target=\"_blank\" rel=\"noopener noreferrer\">here</a>. <br>This endpoint has been deprecated. Please use <a href=\"https://developers.pipedrive.com/docs/api/v1/Activities#getActivities\" target=\"_blank\" rel=\"noopener noreferrer\">GET /api/v2/activities</a> instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**since** | Option<**String**> | The time boundary that points to the start of the range of data. Datetime in ISO 8601 format. E.g. 2022-11-01 08:55:59. Operates on the `update_time` field. |  |
**until** | Option<**String**> | The time boundary that points to the end of the range of data. Datetime in ISO 8601 format. E.g. 2022-11-01 08:55:59. Operates on the `update_time` field. |  |
**user_id** | Option<**i32**> | The ID of the user whose activities will be fetched. If omitted, all activities are returned. |  |
**done** | Option<**bool**> | Whether the activity is done or not. `false` = Not done, `true` = Done. If omitted, returns both done and not done activities. |  |
**r#type** | Option<**String**> | The type of the activity, can be one type or multiple types separated by a comma. This is in correlation with the `key_string` parameter of ActivityTypes. |  |

### Return type

[**models::GetActivitiesCollectionResponse**](GetActivitiesCollectionResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_activity

> models::GetActivityResponse get_activity(id)
Get details of an activity

Returns the details of a specific activity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the activity | [required] |

### Return type

[**models::GetActivityResponse**](GetActivityResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_activity

> models::UpdateActivityResponse update_activity(id, update_activity_request)
Update an activity

Updates an activity. Includes `more_activities_scheduled_in_context` property in response's `additional_data` which indicates whether there are more undone activities scheduled with the same deal, person or organization (depending on the supplied data). <br /> <br /> ***Starting from 30.09.2024, activity attendees will receive updates only if the activity owner has an active calendar sync***

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the activity | [required] |
**update_activity_request** | Option<[**UpdateActivityRequest**](UpdateActivityRequest.md)> |  |  |

### Return type

[**models::UpdateActivityResponse**](UpdateActivityResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

