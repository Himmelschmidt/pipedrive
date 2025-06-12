# \CurrenciesApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_currencies**](CurrenciesApi.md#get_currencies) | **GET** /currencies | Get all supported currencies



## get_currencies

> models::GetCurrenciesResponse get_currencies(term)
Get all supported currencies

Returns all supported currencies in given account which should be used when saving monetary values with other objects. The `code` parameter of the returning objects is the currency code according to ISO 4217 for all non-custom currencies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | Option<**String**> | Optional search term that is searched for from currency's name and/or code |  |

### Return type

[**models::GetCurrenciesResponse**](GetCurrenciesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

