# \RolesApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_or_update_role_setting**](RolesApi.md#add_or_update_role_setting) | **POST** /roles/{id}/settings | Add or update role setting
[**add_role**](RolesApi.md#add_role) | **POST** /roles | Add a role
[**add_role_assignment**](RolesApi.md#add_role_assignment) | **POST** /roles/{id}/assignments | Add role assignment
[**delete_role**](RolesApi.md#delete_role) | **DELETE** /roles/{id} | Delete a role
[**delete_role_assignment**](RolesApi.md#delete_role_assignment) | **DELETE** /roles/{id}/assignments | Delete a role assignment
[**get_role**](RolesApi.md#get_role) | **GET** /roles/{id} | Get one role
[**get_role_assignments**](RolesApi.md#get_role_assignments) | **GET** /roles/{id}/assignments | List role assignments
[**get_role_pipelines**](RolesApi.md#get_role_pipelines) | **GET** /roles/{id}/pipelines | List pipeline visibility for a role
[**get_role_settings**](RolesApi.md#get_role_settings) | **GET** /roles/{id}/settings | List role settings
[**get_roles**](RolesApi.md#get_roles) | **GET** /roles | Get all roles
[**update_role**](RolesApi.md#update_role) | **PUT** /roles/{id} | Update role details
[**update_role_pipelines**](RolesApi.md#update_role_pipelines) | **PUT** /roles/{id}/pipelines | Update pipeline visibility for a role



## add_or_update_role_setting

> models::UpsertRoleSettingsResponse add_or_update_role_setting(id, add_or_update_role_setting_request)
Add or update role setting

Adds or updates the visibility setting for a role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the role | [required] |
**add_or_update_role_setting_request** | Option<[**AddOrUpdateRoleSettingRequest**](AddOrUpdateRoleSettingRequest.md)> |  |  |

### Return type

[**models::UpsertRoleSettingsResponse**](UpsertRoleSettingsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_role

> models::AddRolesResponse add_role(add_role_request)
Add a role

Adds a new role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_role_request** | Option<[**AddRoleRequest**](AddRoleRequest.md)> |  |  |

### Return type

[**models::AddRolesResponse**](AddRolesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_role_assignment

> models::AddRoleAssignmentResponse add_role_assignment(id, add_role_assignment_request)
Add role assignment

Assigns a user to a role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the role | [required] |
**add_role_assignment_request** | Option<[**AddRoleAssignmentRequest**](AddRoleAssignmentRequest.md)> |  |  |

### Return type

[**models::AddRoleAssignmentResponse**](AddRoleAssignmentResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_role

> models::DeleteRoleResponse delete_role(id)
Delete a role

Marks a role as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the role | [required] |

### Return type

[**models::DeleteRoleResponse**](DeleteRoleResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_role_assignment

> models::DeleteRoleAssignmentResponse delete_role_assignment(id, delete_role_assignment_request)
Delete a role assignment

Removes the assigned user from a role and adds to the default role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the role | [required] |
**delete_role_assignment_request** | Option<[**DeleteRoleAssignmentRequest**](DeleteRoleAssignmentRequest.md)> |  |  |

### Return type

[**models::DeleteRoleAssignmentResponse**](DeleteRoleAssignmentResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role

> models::GetRoleResponse get_role(id)
Get one role

Returns the details of a specific role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the role | [required] |

### Return type

[**models::GetRoleResponse**](GetRoleResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_assignments

> models::GetRoleAssignmentsResponse get_role_assignments(id, start, limit)
List role assignments

Returns all users assigned to a role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the role | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**models::GetRoleAssignmentsResponse**](GetRoleAssignmentsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_pipelines

> models::GetRolePipelinesResponse get_role_pipelines(id, visible)
List pipeline visibility for a role

Returns the list of either visible or hidden pipeline IDs for a specific role. For more information on pipeline visibility, please refer to the <a href=\"https://support.pipedrive.com/en/article/visibility-groups\" target=\"_blank\" rel=\"noopener noreferrer\">Visibility groups article</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the role | [required] |
**visible** | Option<**bool**> | Whether to return the visible or hidden pipelines for the role |  |[default to true]

### Return type

[**models::GetRolePipelinesResponse**](GetRolePipelinesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_settings

> models::GetRoleSettingsResponse get_role_settings(id)
List role settings

Returns the visibility settings of a specific role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the role | [required] |

### Return type

[**models::GetRoleSettingsResponse**](GetRoleSettingsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_roles

> models::GetRolesResponse get_roles(start, limit)
Get all roles

Returns all the roles within the company.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**models::GetRolesResponse**](GetRolesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role

> models::UpdateRoleResponse update_role(id, base_role_request)
Update role details

Updates the parent role and/or the name of a specific role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the role | [required] |
**base_role_request** | Option<[**BaseRoleRequest**](BaseRoleRequest.md)> |  |  |

### Return type

[**models::UpdateRoleResponse**](UpdateRoleResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role_pipelines

> models::GetRolePipelinesResponse update_role_pipelines(id, put_role_pipelines_body)
Update pipeline visibility for a role

Updates the specified pipelines to be visible and/or hidden for a specific role. For more information on pipeline visibility, please refer to the <a href=\"https://support.pipedrive.com/en/article/visibility-groups\" target=\"_blank\" rel=\"noopener noreferrer\">Visibility groups article</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the role | [required] |
**put_role_pipelines_body** | Option<[**PutRolePipelinesBody**](PutRolePipelinesBody.md)> |  |  |

### Return type

[**models::GetRolePipelinesResponse**](GetRolePipelinesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

