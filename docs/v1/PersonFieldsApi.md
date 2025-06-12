# \PersonFieldsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_person_field**](PersonFieldsApi.md#add_person_field) | **POST** /personFields | Add a new person field
[**delete_person_field**](PersonFieldsApi.md#delete_person_field) | **DELETE** /personFields/{id} | Delete a person field
[**delete_person_fields**](PersonFieldsApi.md#delete_person_fields) | **DELETE** /personFields | Delete multiple person fields in bulk
[**get_person_field**](PersonFieldsApi.md#get_person_field) | **GET** /personFields/{id} | Get one person field
[**get_person_fields**](PersonFieldsApi.md#get_person_fields) | **GET** /personFields | Get all person fields
[**update_person_field**](PersonFieldsApi.md#update_person_field) | **PUT** /personFields/{id} | Update a person field



## add_person_field

> models::GetFieldResponse add_person_field(create_field_request)
Add a new person field

Adds a new person field. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/adding-a-new-custom-field\" target=\"_blank\" rel=\"noopener noreferrer\">adding a new custom field</a>.

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


## delete_person_field

> models::DeleteResponse delete_person_field(id)
Delete a person field

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


## delete_person_fields

> models::DeleteFieldsResponse delete_person_fields(ids)
Delete multiple person fields in bulk

Marks multiple fields as deleted.

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


## get_person_field

> models::GetFieldResponse get_person_field(id)
Get one person field

Returns data about a specific person field.

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


## get_person_fields

> models::FieldsResponse get_person_fields(start, limit)
Get all person fields

Returns data about all person fields.<br>If a company uses the [Campaigns product](https://pipedrive.readme.io/docs/campaigns-in-pipedrive-api), then this endpoint will also return the `data.marketing_status` field.

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


## update_person_field

> models::GetFieldResponse update_person_field(id, update_field_request)
Update a person field

Updates a person field. For more information, see the tutorial for <a href=\" https://pipedrive.readme.io/docs/updating-custom-field-value \" target=\"_blank\" rel=\"noopener noreferrer\">updating custom fields' values</a>.

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

