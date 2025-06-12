# \LeadSourcesApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lead_sources**](LeadSourcesApi.md#get_lead_sources) | **GET** /leadSources | Get all lead sources



## get_lead_sources

> models::GetLeadsSourceResponse get_lead_sources()
Get all lead sources

Returns all lead sources. Please note that the list of lead sources is fixed, it cannot be modified. All leads created through the Pipedrive API will have a lead source `API` assigned. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetLeadsSourceResponse**](GetLeadsSourceResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

