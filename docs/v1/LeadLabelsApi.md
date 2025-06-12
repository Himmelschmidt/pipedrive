# \LeadLabelsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_lead_label**](LeadLabelsApi.md#add_lead_label) | **POST** /leadLabels | Add a lead label
[**delete_lead_label**](LeadLabelsApi.md#delete_lead_label) | **DELETE** /leadLabels/{id} | Delete a lead label
[**get_lead_labels**](LeadLabelsApi.md#get_lead_labels) | **GET** /leadLabels | Get all lead labels
[**update_lead_label**](LeadLabelsApi.md#update_lead_label) | **PATCH** /leadLabels/{id} | Update a lead label



## add_lead_label

> models::UpsertLeadLabelResponse add_lead_label(add_lead_label_request)
Add a lead label

Creates a lead label.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_lead_label_request** | Option<[**AddLeadLabelRequest**](AddLeadLabelRequest.md)> |  |  |

### Return type

[**models::UpsertLeadLabelResponse**](UpsertLeadLabelResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lead_label

> models::DeleteLeadIdResponse delete_lead_label(id)
Delete a lead label

Deletes a specific lead label.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The ID of the lead label | [required] |

### Return type

[**models::DeleteLeadIdResponse**](DeleteLeadIdResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lead_labels

> models::GetLeadLabelsResponse get_lead_labels()
Get all lead labels

Returns details of all lead labels. This endpoint does not support pagination and all labels are always returned.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetLeadLabelsResponse**](GetLeadLabelsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_lead_label

> models::UpsertLeadLabelResponse update_lead_label(id, update_lead_label_request)
Update a lead label

Updates one or more properties of a lead label. Only properties included in the request will be updated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The ID of the lead label | [required] |
**update_lead_label_request** | Option<[**UpdateLeadLabelRequest**](UpdateLeadLabelRequest.md)> |  |  |

### Return type

[**models::UpsertLeadLabelResponse**](UpsertLeadLabelResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

