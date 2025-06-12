# \CallLogsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_call_log**](CallLogsApi.md#add_call_log) | **POST** /callLogs | Add a call log
[**add_call_log_audio_file**](CallLogsApi.md#add_call_log_audio_file) | **POST** /callLogs/{id}/recordings | Attach an audio file to the call log
[**delete_call_log**](CallLogsApi.md#delete_call_log) | **DELETE** /callLogs/{id} | Delete a call log
[**get_call_log**](CallLogsApi.md#get_call_log) | **GET** /callLogs/{id} | Get details of a call log
[**get_user_call_logs**](CallLogsApi.md#get_user_call_logs) | **GET** /callLogs | Get all call logs assigned to a particular user



## add_call_log

> models::GetCallLogResponse add_call_log(add_call_log_request)
Add a call log

Adds a new call log.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_call_log_request** | Option<[**AddCallLogRequest**](AddCallLogRequest.md)> |  |  |

### Return type

[**models::GetCallLogResponse**](GetCallLogResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_call_log_audio_file

> models::BaseResponse add_call_log_audio_file(id, file)
Attach an audio file to the call log

Adds an audio recording to the call log. That audio can be played by those who have access to the call log object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID received when you create the call log | [required] |
**file** | **std::path::PathBuf** | Audio file supported by the HTML5 specification | [required] |

### Return type

[**models::BaseResponse**](baseResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_call_log

> models::BaseResponse delete_call_log(id)
Delete a call log

Deletes a call log. If there is an audio recording attached to it, it will also be deleted. The related activity will not be removed by this request. If you want to remove the related activities, please use the endpoint which is specific for activities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID received when you create the call log | [required] |

### Return type

[**models::BaseResponse**](baseResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_call_log

> models::GetCallLogResponse get_call_log(id)
Get details of a call log

Returns details of a specific call log.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID received when you create the call log | [required] |

### Return type

[**models::GetCallLogResponse**](GetCallLogResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_call_logs

> models::GetCallLogsResponse get_user_call_logs(start, limit)
Get all call logs assigned to a particular user

Returns all call logs assigned to a particular user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. The upper limit is 50. |  |

### Return type

[**models::GetCallLogsResponse**](GetCallLogsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

