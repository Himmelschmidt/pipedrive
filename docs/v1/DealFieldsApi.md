# \DealFieldsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_deal_field**](DealFieldsApi.md#add_deal_field) | **POST** /dealFields | Add a new deal field
[**delete_deal_field**](DealFieldsApi.md#delete_deal_field) | **DELETE** /dealFields/{id} | Delete a deal field
[**delete_deal_fields**](DealFieldsApi.md#delete_deal_fields) | **DELETE** /dealFields | Delete multiple deal fields in bulk
[**get_deal_field**](DealFieldsApi.md#get_deal_field) | **GET** /dealFields/{id} | Get one deal field
[**get_deal_fields**](DealFieldsApi.md#get_deal_fields) | **GET** /dealFields | Get all deal fields
[**update_deal_field**](DealFieldsApi.md#update_deal_field) | **PUT** /dealFields/{id} | Update a deal field



## add_deal_field

> models::GetFieldResponse add_deal_field(create_field_request)
Add a new deal field

Adds a new deal field. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/adding-a-new-custom-field\" target=\"_blank\" rel=\"noopener noreferrer\">adding a new custom field</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_field_request** | Option<[**CreateFieldRequest**](CreateFieldRequest.md)> |  |  |

### Return type

[**models::GetFieldResponse**](GetFieldResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_deal_field

> models::DeleteResponse delete_deal_field(id)
Delete a deal field

Marks a field as deleted. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/deleting-a-custom-field\" target=\"_blank\" rel=\"noopener noreferrer\">deleting a custom field</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the field | [required] |

### Return type

[**models::DeleteResponse**](DeleteResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_deal_fields

> models::DeleteFieldsResponse delete_deal_fields(ids)
Delete multiple deal fields in bulk

Marks multiple deal fields as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | The comma-separated field IDs to delete | [required] |

### Return type

[**models::DeleteFieldsResponse**](DeleteFieldsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal_field

> models::GetFieldResponse get_deal_field(id)
Get one deal field

Returns data about a specific deal field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the field | [required] |

### Return type

[**models::GetFieldResponse**](GetFieldResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal_fields

> models::FieldsResponse get_deal_fields(start, limit)
Get all deal fields

Returns data about all deal fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**models::FieldsResponse**](FieldsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_deal_field

> models::GetFieldResponse update_deal_field(id, update_field_request)
Update a deal field

Updates a deal field. For more information, see the tutorial for <a href=\" https://pipedrive.readme.io/docs/updating-custom-field-value \" target=\"_blank\" rel=\"noopener noreferrer\">updating custom fields' values</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the field | [required] |
**update_field_request** | Option<[**UpdateFieldRequest**](UpdateFieldRequest.md)> |  |  |

### Return type

[**models::GetFieldResponse**](GetFieldResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

