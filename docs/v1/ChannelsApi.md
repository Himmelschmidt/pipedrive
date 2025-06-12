# \ChannelsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_channel**](ChannelsApi.md#add_channel) | **POST** /channels | Add a channel
[**delete_channel**](ChannelsApi.md#delete_channel) | **DELETE** /channels/{id} | Delete a channel
[**delete_conversation**](ChannelsApi.md#delete_conversation) | **DELETE** /channels/{channel_id}/conversations/{conversation_id} | Delete a conversation
[**receive_message**](ChannelsApi.md#receive_message) | **POST** /channels/messages/receive | Receives an incoming message



## add_channel

> models::AddChannelResponse add_channel(add_channel_request)
Add a channel

Adds a new messaging channel, only admins are able to register new channels. It will use the getConversations endpoint to fetch conversations, participants and messages afterward. To use the endpoint, you need to have **Messengers integration** OAuth scope enabled and the Messaging manifest ready for the [Messaging app extension](https://pipedrive.readme.io/docs/messaging-app-extension).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_channel_request** | Option<[**AddChannelRequest**](AddChannelRequest.md)> |  |  |

### Return type

[**models::AddChannelResponse**](AddChannelResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_channel

> models::DeleteChannel200Response delete_channel(id)
Delete a channel

Deletes an existing messenger’s channel and all related entities (conversations and messages). To use the endpoint, you need to have **Messengers integration** OAuth scope enabled and the Messaging manifest ready for the [Messaging app extension](https://pipedrive.readme.io/docs/messaging-app-extension).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the channel provided by the integration | [required] |

### Return type

[**models::DeleteChannel200Response**](deleteChannel_200_response.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_conversation

> models::DeleteChannel200Response delete_conversation(channel_id, conversation_id)
Delete a conversation

Deletes an existing conversation. To use the endpoint, you need to have **Messengers integration** OAuth scope enabled and the Messaging manifest ready for the [Messaging app extension](https://pipedrive.readme.io/docs/messaging-app-extension).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel provided by the integration | [required] |
**conversation_id** | **String** | The ID of the conversation provided by the integration | [required] |

### Return type

[**models::DeleteChannel200Response**](deleteChannel_200_response.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## receive_message

> models::GetReceiveMessageSuccessResponse receive_message(receive_message_request)
Receives an incoming message

Adds a message to a conversation. To use the endpoint, you need to have **Messengers integration** OAuth scope enabled and the Messaging manifest ready for the [Messaging app extension](https://pipedrive.readme.io/docs/messaging-app-extension).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**receive_message_request** | Option<[**ReceiveMessageRequest**](ReceiveMessageRequest.md)> |  |  |

### Return type

[**models::GetReceiveMessageSuccessResponse**](GetReceiveMessageSuccessResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

