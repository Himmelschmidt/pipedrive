# \FiltersApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_filter**](FiltersApi.md#add_filter) | **POST** /filters | Add a new filter
[**delete_filter**](FiltersApi.md#delete_filter) | **DELETE** /filters/{id} | Delete a filter
[**delete_filters**](FiltersApi.md#delete_filters) | **DELETE** /filters | Delete multiple filters in bulk
[**get_filter**](FiltersApi.md#get_filter) | **GET** /filters/{id} | Get one filter
[**get_filter_helpers**](FiltersApi.md#get_filter_helpers) | **GET** /filters/helpers | Get all filter helpers
[**get_filters**](FiltersApi.md#get_filters) | **GET** /filters | Get all filters
[**update_filter**](FiltersApi.md#update_filter) | **PUT** /filters/{id} | Update filter



## add_filter

> models::AddFiltersResponse add_filter(add_filter_request)
Add a new filter

Adds a new filter, returns the ID upon success. Note that in the conditions JSON object only one first-level condition group is supported, and it must be glued with 'AND', and only two second level condition groups are supported of which one must be glued with 'AND' and the second with 'OR'. Other combinations do not work (yet) but the syntax supports introducing them in future. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/adding-a-filter\" target=\"_blank\" rel=\"noopener noreferrer\">adding a filter</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_filter_request** | Option<[**AddFilterRequest**](AddFilterRequest.md)> |  |  |

### Return type

[**models::AddFiltersResponse**](AddFiltersResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_filter

> models::DeleteFilterResponse delete_filter(id)
Delete a filter

Marks a filter as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the filter | [required] |

### Return type

[**models::DeleteFilterResponse**](DeleteFilterResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_filters

> models::DeleteFiltersResponse delete_filters(ids)
Delete multiple filters in bulk

Marks multiple filters as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | The comma-separated filter IDs to delete | [required] |

### Return type

[**models::DeleteFiltersResponse**](DeleteFiltersResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_filter

> models::GetFiltersResponse1 get_filter(id)
Get one filter

Returns data about a specific filter. Note that this also returns the condition lines of the filter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the filter | [required] |

### Return type

[**models::GetFiltersResponse1**](GetFiltersResponse_1.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_filter_helpers

> serde_json::Value get_filter_helpers()
Get all filter helpers

Returns all supported filter helpers. It helps to know what conditions and helpers are available when you want to <a href=\"/docs/api/v1/Filters#addFilter\">add</a> or <a href=\"/docs/api/v1/Filters#updateFilter\">update</a> filters. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/adding-a-filter\" target=\"_blank\" rel=\"noopener noreferrer\">adding a filter</a>.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_filters

> models::GetFiltersResponse get_filters(r#type)
Get all filters

Returns data about all filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The types of filters to fetch |  |

### Return type

[**models::GetFiltersResponse**](GetFiltersResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_filter

> models::AddFiltersResponse update_filter(id, update_filter_request)
Update filter

Updates an existing filter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the filter | [required] |
**update_filter_request** | Option<[**UpdateFilterRequest**](UpdateFilterRequest.md)> |  |  |

### Return type

[**models::AddFiltersResponse**](AddFiltersResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

