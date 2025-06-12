# \OauthApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**authorize**](OauthApi.md#authorize) | **GET** /oauth/authorize | Requesting authorization
[**get_tokens**](OauthApi.md#get_tokens) | **POST** /oauth/token | Getting the tokens
[**refresh_tokens**](OauthApi.md#refresh_tokens) | **POST** /oauth/token/ | Refreshing the tokens



## authorize

> authorize(client_id, redirect_uri, state)
Requesting authorization

Authorize a user by redirecting them to the Pipedrive OAuth authorization page and request their permissions to act on their behalf. This step is necessary to implement only when you allow app installation outside of the Marketplace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client ID provided to you by the Pipedrive Marketplace when you register your app | [required] |
**redirect_uri** | **String** | The callback URL you provided when you registered your app. Authorization code will be sent to that URL (if it matches with the value you entered in the registration form) if a user approves the app install. Or, if a customer declines, the corresponding error will also be sent to this URL. | [required] |
**state** | Option<**String**> | You may pass any random string as the state parameter and the same string will be returned to your app after a user authorizes access. It may be used to store the user's session ID from your app or distinguish different responses. Using state may increase security; see RFC-6749.  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tokens

> models::GetTokensResponse get_tokens(authorization, grant_type, code, redirect_uri)
Getting the tokens

After the customer has confirmed the app installation, you will need to exchange the `authorization_code` to a pair of access and refresh tokens. Using an access token, you can access the user's data through the API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Base 64 encoded string containing the `client_id` and `client_secret` values. The header value should be `Basic <base64(client_id:client_secret)>`. | [required] |
**grant_type** | Option<**String**> | Since you are trying to exchange an authorization code for a pair of tokens, you must use the value \\\"authorization_code\\\" |  |[default to authorization_code]
**code** | Option<**String**> | The authorization code that you received after the user confirmed app installation |  |
**redirect_uri** | Option<**String**> | The callback URL you provided when you registered your app |  |

### Return type

[**models::GetTokensResponse**](GetTokensResponse.md)

### Authorization

[basic_authentication](../README.md#basic_authentication)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_tokens

> models::GetTokensResponse refresh_tokens(authorization, grant_type, refresh_token)
Refreshing the tokens

The `access_token` has a lifetime. After a period of time, which was returned to you in `expires_in` JSON property, the `access_token` will be invalid, and you can no longer use it to get data from our API. To refresh the `access_token`, you must use the `refresh_token`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Base 64 encoded string containing the `client_id` and `client_secret` values. The header value should be `Basic <base64(client_id:client_secret)>`. | [required] |
**grant_type** | Option<**String**> | Since you are to refresh your access_token, you must use the value \\\"refresh_token\\\" |  |[default to refresh_token]
**refresh_token** | Option<**String**> | The refresh token that you received after you exchanged the authorization code |  |

### Return type

[**models::GetTokensResponse**](GetTokensResponse.md)

### Authorization

[basic_authentication](../README.md#basic_authentication)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

