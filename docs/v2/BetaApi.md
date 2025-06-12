# \BetaApi

All URIs are relative to *https://api.pipedrive.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**convert_deal_to_lead**](BetaApi.md#convert_deal_to_lead) | **POST** /deals/{id}/convert/lead | Convert a deal to a lead (BETA)
[**convert_lead_to_deal**](BetaApi.md#convert_lead_to_deal) | **POST** /leads/{id}/convert/deal | Convert a lead to a deal (BETA)
[**delete_installment**](BetaApi.md#delete_installment) | **DELETE** /deals/{id}/installments/{installment_id} | Delete an installment from a deal
[**get_deal_conversion_status**](BetaApi.md#get_deal_conversion_status) | **GET** /deals/{id}/convert/status/{conversion_id} | Get Deal conversion status (BETA)
[**get_installments**](BetaApi.md#get_installments) | **GET** /deals/installments | List installments added to a list of deals
[**get_lead_conversion_status**](BetaApi.md#get_lead_conversion_status) | **GET** /leads/{id}/convert/status/{conversion_id} | Get Lead conversion status (BETA)
[**post_installment**](BetaApi.md#post_installment) | **POST** /deals/{id}/installments | Add an installment to a deal
[**update_installment**](BetaApi.md#update_installment) | **PATCH** /deals/{id}/installments/{installment_id} | Update an installment added to a deal



## convert_deal_to_lead

> models::AddConvertDealToLeadResponse convert_deal_to_lead(id)
Convert a deal to a lead (BETA)

Initiates a conversion of a deal to a lead. The return value is an ID of a job that was assigned to perform the conversion. Related entities (notes, files, emails, activities, ...) are transferred during the process to the target entity. There are exceptions for entities like invoices or history that are not transferred and remain linked to the original deal. If the conversion is successful, the deal is marked as deleted. To retrieve the created entity ID and the result of the conversion, call the <a href=\"https://developers.pipedrive.com/docs/api/v1/Deals#getDealConversionStatus\">/api/v2/deals/{deal_id}/convert/status/{conversion_id}</a> endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal to convert | [required] |

### Return type

[**models::AddConvertDealToLeadResponse**](AddConvertDealToLeadResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## delete_installment

> models::DeleteInstallmentResponse delete_installment(id, installment_id)
Delete an installment from a deal

Removes an installment from a deal.  Only available in Advanced and above plans. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**installment_id** | **i32** | The ID of the installment | [required] |

### Return type

[**models::DeleteInstallmentResponse**](DeleteInstallmentResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal_conversion_status

> models::GetConvertResponse1 get_deal_conversion_status(id, conversion_id)
Get Deal conversion status (BETA)

Returns information about the conversion. Status is always present and its value (not_started, running, completed, failed, rejected) represents the current state of the conversion. Lead ID is only present if the conversion was successfully finished. This data is only temporary and removed after a few days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of a deal | [required] |
**conversion_id** | **uuid::Uuid** | The ID of the conversion | [required] |

### Return type

[**models::GetConvertResponse1**](GetConvertResponse_1.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_installments

> models::GetInstallmentsResponse get_installments(deal_ids, cursor, limit, sort_by, sort_direction)
List installments added to a list of deals

Lists installments attached to a list of deals.  Only available in Advanced and above plans. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deal_ids** | [**Vec<i32>**](i32.md) | An array of integers with the IDs of the deals for which the attached installments will be returned. A maximum of 100 deal IDs can be provided. | [required] |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**sort_by** | Option<**String**> | The field to sort by. Supported fields: `id`, `billing_date`, `deal_id`. |  |[default to id]
**sort_direction** | Option<**String**> | The sorting direction. Supported values: `asc`, `desc`. |  |[default to asc]

### Return type

[**models::GetInstallmentsResponse**](GetInstallmentsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
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


## post_installment

> models::AddAInstallmentResponse post_installment(id, add_installment_request_body)
Add an installment to a deal

Adds an installment to a deal.  An installment can only be added if the deal includes at least one one-time product.  If the deal contains at least one recurring product, adding installments is not allowed.  Only available in Advanced and above plans. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**add_installment_request_body** | Option<[**AddInstallmentRequestBody**](AddInstallmentRequestBody.md)> |  |  |

### Return type

[**models::AddAInstallmentResponse**](AddAInstallmentResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_installment

> models::UpdateInstallmentResponse update_installment(id, installment_id, update_installment_request_body)
Update an installment added to a deal

Edits an installment added to a deal.  Only available in Advanced and above plans. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**installment_id** | **i32** | The ID of the installment | [required] |
**update_installment_request_body** | Option<[**UpdateInstallmentRequestBody**](UpdateInstallmentRequestBody.md)> |  |  |

### Return type

[**models::UpdateInstallmentResponse**](UpdateInstallmentResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

