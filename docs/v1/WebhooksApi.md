# \WebhooksApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_webhook**](WebhooksApi.md#add_webhook) | **POST** /webhooks | Create a new Webhook
[**delete_webhook**](WebhooksApi.md#delete_webhook) | **DELETE** /webhooks/{id} | Delete existing Webhook
[**get_webhooks**](WebhooksApi.md#get_webhooks) | **GET** /webhooks | Get all Webhooks



## add_webhook

> models::GetWebhookResponse add_webhook(add_webhook_request)
Create a new Webhook

Creates a new Webhook and returns its details. Note that specifying an event which triggers the Webhook combines 2 parameters - `event_action` and `event_object`. E.g., use `*.*` for getting notifications about all events, `create.deal` for any newly added deals, `delete.persons` for any deleted persons, etc. See <a href=\"https://pipedrive.readme.io/docs/guide-for-webhooks-v2?ref=api_reference\" target=\"_blank\" rel=\"noopener noreferrer\">the guide for Webhooks</a> for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_webhook_request** | Option<[**AddWebhookRequest**](AddWebhookRequest.md)> |  |  |

### Return type

[**models::GetWebhookResponse**](GetWebhookResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook

> models::BaseResponse delete_webhook(id)
Delete existing Webhook

Deletes the specified Webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the Webhook to delete | [required] |

### Return type

[**models::BaseResponse**](BaseResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhooks

> models::GetWebhooksResponse get_webhooks()
Get all Webhooks

Returns data about all the Webhooks of a company.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetWebhooksResponse**](GetWebhooksResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

