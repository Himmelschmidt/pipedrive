# \MeetingsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_user_provider_link**](MeetingsApi.md#delete_user_provider_link) | **DELETE** /meetings/userProviderLinks/{id} | Delete the link between a user and the installed video call integration
[**save_user_provider_link**](MeetingsApi.md#save_user_provider_link) | **POST** /meetings/userProviderLinks | Link a user with the installed video call integration



## delete_user_provider_link

> models::GetUserProviderLinkSuccessResponse delete_user_provider_link(id)
Delete the link between a user and the installed video call integration

A video calling provider must call this endpoint to remove the link between a user and the installed video calling app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Unique identifier linking a user to the installed integration | [required] |

### Return type

[**models::GetUserProviderLinkSuccessResponse**](GetUserProviderLinkSuccessResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_user_provider_link

> models::GetUserProviderLinkSuccessResponse save_user_provider_link(add_user_provider_link_request)
Link a user with the installed video call integration

A video calling provider must call this endpoint after a user has installed the video calling app so that the new user's information is sent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_user_provider_link_request** | Option<[**AddUserProviderLinkRequest**](AddUserProviderLinkRequest.md)> |  |  |

### Return type

[**models::GetUserProviderLinkSuccessResponse**](GetUserProviderLinkSuccessResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

