# \ProductFieldsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_product_field**](ProductFieldsApi.md#add_product_field) | **POST** /productFields | Add a new product field
[**delete_product_field**](ProductFieldsApi.md#delete_product_field) | **DELETE** /productFields/{id} | Delete a product field
[**delete_product_fields**](ProductFieldsApi.md#delete_product_fields) | **DELETE** /productFields | Delete multiple product fields in bulk
[**get_product_field**](ProductFieldsApi.md#get_product_field) | **GET** /productFields/{id} | Get one product field
[**get_product_fields**](ProductFieldsApi.md#get_product_fields) | **GET** /productFields | Get all product fields
[**update_product_field**](ProductFieldsApi.md#update_product_field) | **PUT** /productFields/{id} | Update a product field



## add_product_field

> models::GetProductFieldResponse add_product_field(add_product_field_request)
Add a new product field

Adds a new product field. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/adding-a-new-custom-field\" target=\"_blank\" rel=\"noopener noreferrer\">adding a new custom field</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_product_field_request** | Option<[**AddProductFieldRequest**](AddProductFieldRequest.md)> |  |  |

### Return type

[**models::GetProductFieldResponse**](GetProductFieldResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_product_field

> models::DeleteProductFieldResponse delete_product_field(id)
Delete a product field

Marks a product field as deleted. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/deleting-a-custom-field\" target=\"_blank\" rel=\"noopener noreferrer\">deleting a custom field</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product field | [required] |

### Return type

[**models::DeleteProductFieldResponse**](DeleteProductFieldResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_product_fields

> models::DeleteProductFieldsResponse delete_product_fields(ids)
Delete multiple product fields in bulk

Marks multiple fields as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | The comma-separated field IDs to delete | [required] |

### Return type

[**models::DeleteProductFieldsResponse**](DeleteProductFieldsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_field

> models::GetProductFieldResponse get_product_field(id)
Get one product field

Returns data about a specific product field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product field | [required] |

### Return type

[**models::GetProductFieldResponse**](GetProductFieldResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_fields

> models::GetProductFieldsResponse get_product_fields(start, limit)
Get all product fields

Returns data about all product fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**models::GetProductFieldsResponse**](GetProductFieldsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_product_field

> models::GetProductFieldResponse update_product_field(id, update_product_field_response)
Update a product field

Updates a product field. For more information, see the tutorial for <a href=\" https://pipedrive.readme.io/docs/updating-custom-field-value \" target=\"_blank\" rel=\"noopener noreferrer\">updating custom fields' values</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product field | [required] |
**update_product_field_response** | Option<[**UpdateProductFieldResponse**](UpdateProductFieldResponse.md)> |  |  |

### Return type

[**models::GetProductFieldResponse**](GetProductFieldResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

