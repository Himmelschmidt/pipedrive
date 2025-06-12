# \LeadsApi

All URIs are relative to *https://api.pipedrive.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**convert_lead_to_deal**](LeadsApi.md#convert_lead_to_deal) | **POST** /leads/{id}/convert/deal | Convert a lead to a deal (BETA)
[**get_lead_conversion_status**](LeadsApi.md#get_lead_conversion_status) | **GET** /leads/{id}/convert/status/{conversion_id} | Get Lead conversion status (BETA)
[**search_leads**](LeadsApi.md#search_leads) | **GET** /leads/search | Search leads



## convert_lead_to_deal

> models::AddConvertLeadToDealResponse convert_lead_to_deal(id, convert_lead_to_deal_request)
Convert a lead to a deal (BETA)

Initiates a conversion of a lead to a deal. The return value is an ID of a job that was assigned to perform the conversion. Related entities (notes, files, emails, activities, ...) are transferred during the process to the target entity. If the conversion is successful, the lead is marked as deleted. To retrieve the created entity ID and the result of the conversion, call the <a href=\"https://developers.pipedrive.com/docs/api/v1/Leads#getLeadConversionStatus\">/api/v2/leads/{lead_id}/convert/status/{conversion_id}</a> endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The ID of the lead to convert | [required] |
**convert_lead_to_deal_request** | Option<[**ConvertLeadToDealRequest**](ConvertLeadToDealRequest.md)> |  |  |

### Return type

[**models::AddConvertLeadToDealResponse**](AddConvertLeadToDealResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lead_conversion_status

> models::GetConvertResponse1 get_lead_conversion_status(id, conversion_id)
Get Lead conversion status (BETA)

Returns data about the conversion. Status is always present and its value (not_started, running, completed, failed, rejected) represents the current state of the conversion. Deal ID is only present if the conversion was successfully finished. This data is only temporary and removed after a few days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The ID of a lead | [required] |
**conversion_id** | **uuid::Uuid** | The ID of the conversion | [required] |

### Return type

[**models::GetConvertResponse1**](GetConvertResponse_1.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_leads

> models::GetLeadSearchResponse search_leads(term, fields, exact_match, person_id, organization_id, include_fields, limit, cursor)
Search leads

Searches all leads by title, notes and/or custom fields. This endpoint is a wrapper of <a href=\"https://developers.pipedrive.com/docs/api/v1/ItemSearch#searchItem\">/v1/itemSearch</a> with a narrower OAuth scope. Found leads can be filtered by the person ID and the organization ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | **String** | The search term to look for. Minimum 2 characters (or 1 if using `exact_match`). Please note that the search term has to be URL encoded. | [required] |
**fields** | Option<**String**> | A comma-separated string array. The fields to perform the search from. Defaults to all of them. |  |
**exact_match** | Option<**bool**> | When enabled, only full exact matches against the given term are returned. It is <b>not</b> case sensitive. |  |
**person_id** | Option<**i32**> | Will filter leads by the provided person ID. The upper limit of found leads associated with the person is 2000. |  |
**organization_id** | Option<**i32**> | Will filter leads by the provided organization ID. The upper limit of found leads associated with the organization is 2000. |  |
**include_fields** | Option<**String**> | Supports including optional fields in the results which are not provided by default |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |

### Return type

[**models::GetLeadSearchResponse**](GetLeadSearchResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

