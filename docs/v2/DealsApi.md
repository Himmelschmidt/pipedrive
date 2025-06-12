# \DealsApi

All URIs are relative to *https://api.pipedrive.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_deal**](DealsApi.md#add_deal) | **POST** /deals | Add a new deal
[**add_deal_follower**](DealsApi.md#add_deal_follower) | **POST** /deals/{id}/followers | Add a follower to a deal
[**add_deal_product**](DealsApi.md#add_deal_product) | **POST** /deals/{id}/products | Add a product to a deal
[**convert_deal_to_lead**](DealsApi.md#convert_deal_to_lead) | **POST** /deals/{id}/convert/lead | Convert a deal to a lead (BETA)
[**delete_additional_discount**](DealsApi.md#delete_additional_discount) | **DELETE** /deals/{id}/discounts/{discount_id} | Delete a discount from a deal
[**delete_deal**](DealsApi.md#delete_deal) | **DELETE** /deals/{id} | Delete a deal
[**delete_deal_follower**](DealsApi.md#delete_deal_follower) | **DELETE** /deals/{id}/followers/{follower_id} | Delete a follower from a deal
[**delete_deal_product**](DealsApi.md#delete_deal_product) | **DELETE** /deals/{id}/products/{product_attachment_id} | Delete an attached product from a deal
[**delete_installment**](DealsApi.md#delete_installment) | **DELETE** /deals/{id}/installments/{installment_id} | Delete an installment from a deal
[**get_additional_discounts**](DealsApi.md#get_additional_discounts) | **GET** /deals/{id}/discounts | List discounts added to a deal
[**get_archived_deals**](DealsApi.md#get_archived_deals) | **GET** /deals/archived | Get all archived deals
[**get_deal**](DealsApi.md#get_deal) | **GET** /deals/{id} | Get details of a deal
[**get_deal_conversion_status**](DealsApi.md#get_deal_conversion_status) | **GET** /deals/{id}/convert/status/{conversion_id} | Get Deal conversion status (BETA)
[**get_deal_followers**](DealsApi.md#get_deal_followers) | **GET** /deals/{id}/followers | List followers of a deal
[**get_deal_followers_changelog**](DealsApi.md#get_deal_followers_changelog) | **GET** /deals/{id}/followers/changelog | List followers changelog of a deal
[**get_deal_products**](DealsApi.md#get_deal_products) | **GET** /deals/{id}/products | List products attached to a deal
[**get_deals**](DealsApi.md#get_deals) | **GET** /deals | Get all deals
[**get_deals_products**](DealsApi.md#get_deals_products) | **GET** /deals/products | Get deal products of several deals
[**get_installments**](DealsApi.md#get_installments) | **GET** /deals/installments | List installments added to a list of deals
[**post_additional_discount**](DealsApi.md#post_additional_discount) | **POST** /deals/{id}/discounts | Add a discount to a deal
[**post_installment**](DealsApi.md#post_installment) | **POST** /deals/{id}/installments | Add an installment to a deal
[**search_deals**](DealsApi.md#search_deals) | **GET** /deals/search | Search deals
[**update_additional_discount**](DealsApi.md#update_additional_discount) | **PATCH** /deals/{id}/discounts/{discount_id} | Update a discount added to a deal
[**update_deal**](DealsApi.md#update_deal) | **PATCH** /deals/{id} | Update a deal
[**update_deal_product**](DealsApi.md#update_deal_product) | **PATCH** /deals/{id}/products/{product_attachment_id} | Update the product attached to a deal
[**update_installment**](DealsApi.md#update_installment) | **PATCH** /deals/{id}/installments/{installment_id} | Update an installment added to a deal



## add_deal

> models::UpsertDealResponse add_deal(add_deal_request)
Add a new deal

Adds a new deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_deal_request** | Option<[**AddDealRequest**](AddDealRequest.md)> |  |  |

### Return type

[**models::UpsertDealResponse**](UpsertDealResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_deal_follower

> models::AddFollowerResponse add_deal_follower(id, add_deal_follower_request)
Add a follower to a deal

Adds a user as a follower to the deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**add_deal_follower_request** | Option<[**AddDealFollowerRequest**](AddDealFollowerRequest.md)> |  |  |

### Return type

[**models::AddFollowerResponse**](AddFollowerResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_deal_product

> models::AddDealProductResponse add_deal_product(id, add_deal_product_request)
Add a product to a deal

Adds a product to a deal, creating a new item called a deal-product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**add_deal_product_request** | Option<[**AddDealProductRequest**](AddDealProductRequest.md)> |  |  |

### Return type

[**models::AddDealProductResponse**](AddDealProductResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## delete_additional_discount

> models::DeleteAdditionalDiscountResponse delete_additional_discount(id, discount_id)
Delete a discount from a deal

Removes a discount from a deal, changing the deal value if the deal has one-time products attached.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**discount_id** | **i32** | The ID of the discount | [required] |

### Return type

[**models::DeleteAdditionalDiscountResponse**](DeleteAdditionalDiscountResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_deal

> models::DeleteDealResponse delete_deal(id)
Delete a deal

Marks a deal as deleted. After 30 days, the deal will be permanently deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |

### Return type

[**models::DeleteDealResponse**](DeleteDealResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_deal_follower

> models::DeleteFollowerResponse delete_deal_follower(id, follower_id)
Delete a follower from a deal

Deletes a user follower from the deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**follower_id** | **i32** | The ID of the following user | [required] |

### Return type

[**models::DeleteFollowerResponse**](DeleteFollowerResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_deal_product

> models::DeleteDealProductResponse delete_deal_product(id, product_attachment_id)
Delete an attached product from a deal

Deletes a product attachment from a deal, using the `product_attachment_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**product_attachment_id** | **i32** | The product attachment ID | [required] |

### Return type

[**models::DeleteDealProductResponse**](DeleteDealProductResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
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


## get_additional_discounts

> models::GetAdditionalDiscountsResponse get_additional_discounts(id)
List discounts added to a deal

Lists discounts attached to a deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |

### Return type

[**models::GetAdditionalDiscountsResponse**](GetAdditionalDiscountsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_archived_deals

> models::GetDealsResponse get_archived_deals(filter_id, ids, owner_id, person_id, org_id, pipeline_id, stage_id, status, updated_since, updated_until, sort_by, sort_direction, include_fields, custom_fields, limit, cursor)
Get all archived deals

Returns data about all archived deals.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_id** | Option<**i32**> | If supplied, only deals matching the specified filter are returned |  |
**ids** | Option<**String**> | Optional comma separated string array of up to 100 entity ids to fetch. If filter_id is provided, this is ignored. If any of the requested entities do not exist or are not visible, they are not included in the response. |  |
**owner_id** | Option<**i32**> | If supplied, only deals owned by the specified user are returned. If filter_id is provided, this is ignored. |  |
**person_id** | Option<**i32**> | If supplied, only deals linked to the specified person are returned. If filter_id is provided, this is ignored. |  |
**org_id** | Option<**i32**> | If supplied, only deals linked to the specified organization are returned. If filter_id is provided, this is ignored. |  |
**pipeline_id** | Option<**i32**> | If supplied, only deals in the specified pipeline are returned. If filter_id is provided, this is ignored. |  |
**stage_id** | Option<**i32**> | If supplied, only deals in the specified stage are returned. If filter_id is provided, this is ignored. |  |
**status** | Option<**String**> | Only fetch deals with a specific status. If omitted, all not deleted deals are returned. If set to deleted, deals that have been deleted up to 30 days ago will be included. Multiple statuses can be included as a comma separated array. If filter_id is provided, this is ignored. |  |
**updated_since** | Option<**String**> | If set, only deals with an `update_time` later than or equal to this time are returned. In RFC3339 format, e.g. 2025-01-01T10:20:00Z. |  |
**updated_until** | Option<**String**> | If set, only deals with an `update_time` earlier than this time are returned. In RFC3339 format, e.g. 2025-01-01T10:20:00Z. |  |
**sort_by** | Option<**String**> | The field to sort by. Supported fields: `id`, `update_time`, `add_time`. |  |[default to id]
**sort_direction** | Option<**String**> | The sorting direction. Supported values: `asc`, `desc`. |  |[default to asc]
**include_fields** | Option<**String**> | Optional comma separated string array of additional fields to include |  |
**custom_fields** | Option<**String**> | Optional comma separated string array of custom fields keys to include. If you are only interested in a particular set of custom fields, please use this parameter for faster results and smaller response.<br/>A maximum of 15 keys is allowed. |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |

### Return type

[**models::GetDealsResponse**](GetDealsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal

> models::UpsertDealResponse get_deal(id, include_fields, custom_fields)
Get details of a deal

Returns the details of a specific deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**include_fields** | Option<**String**> | Optional comma separated string array of additional fields to include |  |
**custom_fields** | Option<**String**> | Optional comma separated string array of custom fields keys to include. If you are only interested in a particular set of custom fields, please use this parameter for faster results and smaller response.<br/>A maximum of 15 keys is allowed. |  |

### Return type

[**models::UpsertDealResponse**](UpsertDealResponse.md)

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


## get_deal_followers

> models::GetFollowersResponse get_deal_followers(id, limit, cursor)
List followers of a deal

Lists users who are following the deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |

### Return type

[**models::GetFollowersResponse**](GetFollowersResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal_followers_changelog

> models::GetFollowerChangelogsResponse get_deal_followers_changelog(id, limit, cursor)
List followers changelog of a deal

Lists changelogs about users have followed the deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |

### Return type

[**models::GetFollowerChangelogsResponse**](GetFollowerChangelogsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal_products

> models::GetDealsProductsResponse get_deal_products(id, cursor, limit, sort_by, sort_direction)
List products attached to a deal

Lists products attached to a deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**sort_by** | Option<**String**> | The field to sort by. Supported fields: `id`, `add_time`, `update_time`. |  |[default to id]
**sort_direction** | Option<**String**> | The sorting direction. Supported values: `asc`, `desc`. |  |[default to asc]

### Return type

[**models::GetDealsProductsResponse**](GetDealsProductsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deals

> models::GetDealsResponse get_deals(filter_id, ids, owner_id, person_id, org_id, pipeline_id, stage_id, status, updated_since, updated_until, sort_by, sort_direction, include_fields, custom_fields, limit, cursor)
Get all deals

Returns data about all not archived deals.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_id** | Option<**i32**> | If supplied, only deals matching the specified filter are returned |  |
**ids** | Option<**String**> | Optional comma separated string array of up to 100 entity ids to fetch. If filter_id is provided, this is ignored. If any of the requested entities do not exist or are not visible, they are not included in the response. |  |
**owner_id** | Option<**i32**> | If supplied, only deals owned by the specified user are returned. If filter_id is provided, this is ignored. |  |
**person_id** | Option<**i32**> | If supplied, only deals linked to the specified person are returned. If filter_id is provided, this is ignored. |  |
**org_id** | Option<**i32**> | If supplied, only deals linked to the specified organization are returned. If filter_id is provided, this is ignored. |  |
**pipeline_id** | Option<**i32**> | If supplied, only deals in the specified pipeline are returned. If filter_id is provided, this is ignored. |  |
**stage_id** | Option<**i32**> | If supplied, only deals in the specified stage are returned. If filter_id is provided, this is ignored. |  |
**status** | Option<**String**> | Only fetch deals with a specific status. If omitted, all not deleted deals are returned. If set to deleted, deals that have been deleted up to 30 days ago will be included. Multiple statuses can be included as a comma separated array. If filter_id is provided, this is ignored. |  |
**updated_since** | Option<**String**> | If set, only deals with an `update_time` later than or equal to this time are returned. In RFC3339 format, e.g. 2025-01-01T10:20:00Z. |  |
**updated_until** | Option<**String**> | If set, only deals with an `update_time` earlier than this time are returned. In RFC3339 format, e.g. 2025-01-01T10:20:00Z. |  |
**sort_by** | Option<**String**> | The field to sort by. Supported fields: `id`, `update_time`, `add_time`. |  |[default to id]
**sort_direction** | Option<**String**> | The sorting direction. Supported values: `asc`, `desc`. |  |[default to asc]
**include_fields** | Option<**String**> | Optional comma separated string array of additional fields to include |  |
**custom_fields** | Option<**String**> | Optional comma separated string array of custom fields keys to include. If you are only interested in a particular set of custom fields, please use this parameter for faster results and smaller response.<br/>A maximum of 15 keys is allowed. |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |

### Return type

[**models::GetDealsResponse**](GetDealsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deals_products

> models::GetDealsProductsResponse get_deals_products(deal_ids, cursor, limit, sort_by, sort_direction)
Get deal products of several deals

Returns data about products attached to deals

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deal_ids** | [**Vec<i32>**](i32.md) | An array of integers with the IDs of the deals for which the attached products will be returned. A maximum of 100 deal IDs can be provided. | [required] |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**sort_by** | Option<**String**> | The field to sort by. Supported fields: `id`, `deal_id`, `add_time`, `update_time`. |  |[default to id]
**sort_direction** | Option<**String**> | The sorting direction. Supported values: `asc`, `desc`. |  |[default to asc]

### Return type

[**models::GetDealsProductsResponse**](GetDealsProductsResponse.md)

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


## post_additional_discount

> models::AddAdditionalDiscountResponse post_additional_discount(id, add_additional_discount_request_body)
Add a discount to a deal

Adds a discount to a deal changing, the deal value if the deal has one-time products attached.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**add_additional_discount_request_body** | Option<[**AddAdditionalDiscountRequestBody**](AddAdditionalDiscountRequestBody.md)> |  |  |

### Return type

[**models::AddAdditionalDiscountResponse**](AddAdditionalDiscountResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
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


## search_deals

> models::GetDealSearchResponse search_deals(term, fields, exact_match, person_id, organization_id, status, include_fields, limit, cursor)
Search deals

Searches all deals by title, notes and/or custom fields. This endpoint is a wrapper of <a href=\"https://developers.pipedrive.com/docs/api/v1/ItemSearch#searchItem\">/v1/itemSearch</a> with a narrower OAuth scope. Found deals can be filtered by the person ID and the organization ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | **String** | The search term to look for. Minimum 2 characters (or 1 if using `exact_match`). Please note that the search term has to be URL encoded. | [required] |
**fields** | Option<**String**> | A comma-separated string array. The fields to perform the search from. Defaults to all of them. Only the following custom field types are searchable: `address`, `varchar`, `text`, `varchar_auto`, `double`, `monetary` and `phone`. Read more about searching by custom fields <a href=\"https://support.pipedrive.com/en/article/search-finding-what-you-need#searching-by-custom-fields\" target=\"_blank\" rel=\"noopener noreferrer\">here</a>. |  |
**exact_match** | Option<**bool**> | When enabled, only full exact matches against the given term are returned. It is <b>not</b> case sensitive. |  |
**person_id** | Option<**i32**> | Will filter deals by the provided person ID. The upper limit of found deals associated with the person is 2000. |  |
**organization_id** | Option<**i32**> | Will filter deals by the provided organization ID. The upper limit of found deals associated with the organization is 2000. |  |
**status** | Option<**String**> | Will filter deals by the provided specific status. open = Open, won = Won, lost = Lost. The upper limit of found deals associated with the status is 2000. |  |
**include_fields** | Option<**String**> | Supports including optional fields in the results which are not provided by default |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |

### Return type

[**models::GetDealSearchResponse**](GetDealSearchResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_additional_discount

> models::UpdateAdditionalDiscountResponse update_additional_discount(id, discount_id, update_additional_discount_request_body)
Update a discount added to a deal

Edits a discount added to a deal, changing the deal value if the deal has one-time products attached.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**discount_id** | **i32** | The ID of the discount | [required] |
**update_additional_discount_request_body** | Option<[**UpdateAdditionalDiscountRequestBody**](UpdateAdditionalDiscountRequestBody.md)> |  |  |

### Return type

[**models::UpdateAdditionalDiscountResponse**](UpdateAdditionalDiscountResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_deal

> models::UpsertDealResponse update_deal(id, update_deal_request)
Update a deal

Updates the properties of a deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**update_deal_request** | Option<[**UpdateDealRequest**](UpdateDealRequest.md)> |  |  |

### Return type

[**models::UpsertDealResponse**](UpsertDealResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_deal_product

> models::AddDealProductResponse update_deal_product(id, product_attachment_id, update_deal_product_request)
Update the product attached to a deal

Updates the details of the product that has been attached to a deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**product_attachment_id** | **i32** | The ID of the deal-product (the ID of the product attached to the deal) | [required] |
**update_deal_product_request** | Option<[**UpdateDealProductRequest**](UpdateDealProductRequest.md)> |  |  |

### Return type

[**models::AddDealProductResponse**](AddDealProductResponse.md)

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

