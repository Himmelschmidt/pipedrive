# \PermissionSetsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_permission_set**](PermissionSetsApi.md#get_permission_set) | **GET** /permissionSets/{id} | Get one permission set
[**get_permission_set_assignments**](PermissionSetsApi.md#get_permission_set_assignments) | **GET** /permissionSets/{id}/assignments | List permission set assignments
[**get_permission_sets**](PermissionSetsApi.md#get_permission_sets) | **GET** /permissionSets | Get all permission sets



## get_permission_set

> models::GetPermissionSetResponse get_permission_set(id)
Get one permission set

Returns data about a specific permission set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the permission set | [required] |

### Return type

[**models::GetPermissionSetResponse**](GetPermissionSetResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_permission_set_assignments

> models::GetUserAssignmentsToPermissionSetResponse get_permission_set_assignments(id, start, limit)
List permission set assignments

Returns the list of assignments for a permission set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the permission set | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**models::GetUserAssignmentsToPermissionSetResponse**](GetUserAssignmentsToPermissionSetResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_permission_sets

> models::GetPermissionSetsResponse get_permission_sets(app)
Get all permission sets

Returns data about all permission sets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app** | Option<**String**> | The app to filter the permission sets by |  |

### Return type

[**models::GetPermissionSetsResponse**](GetPermissionSetsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

