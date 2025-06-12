# \RecentsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_recents**](RecentsApi.md#get_recents) | **GET** /recents | Get recents



## get_recents

> models::GetRecentsResponse get_recents(since_timestamp, items, start, limit)
Get recents

Returns data about all recent changes occurred after the given timestamp.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since_timestamp** | **String** | The timestamp in UTC. Format: YYYY-MM-DD HH:MM:SS. | [required] |
**items** | Option<**String**> | Multiple selection of item types to include in the query (optional) |  |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**models::GetRecentsResponse**](GetRecentsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

