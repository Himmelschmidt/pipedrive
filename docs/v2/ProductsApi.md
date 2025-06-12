# \ProductsApi

All URIs are relative to *https://api.pipedrive.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_product**](ProductsApi.md#add_product) | **POST** /products | Add a product
[**add_product_follower**](ProductsApi.md#add_product_follower) | **POST** /products/{id}/followers | Add a follower to a product
[**add_product_variation**](ProductsApi.md#add_product_variation) | **POST** /products/{id}/variations | Add a product variation
[**delete_product**](ProductsApi.md#delete_product) | **DELETE** /products/{id} | Delete a product
[**delete_product_follower**](ProductsApi.md#delete_product_follower) | **DELETE** /products/{id}/followers/{follower_id} | Delete a follower from a product
[**delete_product_variation**](ProductsApi.md#delete_product_variation) | **DELETE** /products/{id}/variations/{product_variation_id} | Delete a product variation
[**get_product**](ProductsApi.md#get_product) | **GET** /products/{id} | Get one product
[**get_product_followers**](ProductsApi.md#get_product_followers) | **GET** /products/{id}/followers | List followers of a product
[**get_product_followers_changelog**](ProductsApi.md#get_product_followers_changelog) | **GET** /products/{id}/followers/changelog | List followers changelog of a product
[**get_product_variations**](ProductsApi.md#get_product_variations) | **GET** /products/{id}/variations | Get all product variations
[**get_products**](ProductsApi.md#get_products) | **GET** /products | Get all products
[**search_products**](ProductsApi.md#search_products) | **GET** /products/search | Search products
[**update_product**](ProductsApi.md#update_product) | **PATCH** /products/{id} | Update a product
[**update_product_variation**](ProductsApi.md#update_product_variation) | **PATCH** /products/{id}/variations/{product_variation_id} | Update a product variation



## add_product

> models::GetProductResponse add_product(add_product_request)
Add a product

Adds a new product to the Products inventory. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/adding-a-product\" target=\"_blank\" rel=\"noopener noreferrer\">adding a product</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_product_request** | Option<[**AddProductRequest**](AddProductRequest.md)> |  |  |

### Return type

[**models::GetProductResponse**](GetProductResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_product_follower

> models::AddFollowerResponse add_product_follower(id, add_deal_follower_request)
Add a follower to a product

Adds a user as a follower to the product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |
**add_deal_follower_request** | Option<[**AddDealFollowerRequest**](AddDealFollowerRequest.md)> |  |  |

### Return type

[**models::AddFollowerResponse**](AddFollowerResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_product_variation

> models::GetProductVariationResponse add_product_variation(id, add_product_variation_request)
Add a product variation

Adds a new product variation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |
**add_product_variation_request** | Option<[**AddProductVariationRequest**](AddProductVariationRequest.md)> |  |  |

### Return type

[**models::GetProductVariationResponse**](GetProductVariationResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_product

> models::DeleteProductResponse delete_product(id)
Delete a product

Marks a product as deleted. After 30 days, the product will be permanently deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |

### Return type

[**models::DeleteProductResponse**](DeleteProductResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_product_follower

> models::DeleteFollowerResponse delete_product_follower(id, follower_id)
Delete a follower from a product

Deletes a user follower from the product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |
**follower_id** | **i32** | The ID of the following user | [required] |

### Return type

[**models::DeleteFollowerResponse**](DeleteFollowerResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_product_variation

> models::DeleteProductVariationResponse delete_product_variation(id, product_variation_id)
Delete a product variation

Deletes a product variation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |
**product_variation_id** | **i32** | The ID of the product variation | [required] |

### Return type

[**models::DeleteProductVariationResponse**](DeleteProductVariationResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product

> models::GetProductResponse get_product(id)
Get one product

Returns data about a specific product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |

### Return type

[**models::GetProductResponse**](GetProductResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_followers

> models::GetFollowersResponse get_product_followers(id, limit, cursor)
List followers of a product

Lists users who are following the product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |
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


## get_product_followers_changelog

> models::GetFollowerChangelogsResponse get_product_followers_changelog(id, limit, cursor)
List followers changelog of a product

Lists changelogs about users have followed the product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |
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


## get_product_variations

> models::GetProductVariationsResponse get_product_variations(id, cursor, limit)
Get all product variations

Returns data about all product variations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |

### Return type

[**models::GetProductVariationsResponse**](GetProductVariationsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_products

> models::GetProductsResponse get_products(owner_id, ids, filter_id, cursor, limit, sort_by, sort_direction, custom_fields)
Get all products

Returns data about all products.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_id** | Option<**i32**> | If supplied, only products owned by the given user will be returned |  |
**ids** | Option<**String**> | Optional comma separated string array of up to 100 entity ids to fetch. If filter_id is provided, this is ignored. If any of the requested entities do not exist or are not visible, they are not included in the response. |  |
**filter_id** | Option<**i32**> | The ID of the filter to use |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**sort_by** | Option<**String**> | The field to sort by. Supported fields: `id`, `name`, `add_time`, `update_time`. |  |[default to id]
**sort_direction** | Option<**String**> | The sorting direction. Supported values: `asc`, `desc`. |  |[default to asc]
**custom_fields** | Option<**String**> | Comma separated string array of custom fields keys to include. If you are only interested in a particular set of custom fields, please use this parameter for a smaller response.<br/>A maximum of 15 keys is allowed. |  |

### Return type

[**models::GetProductsResponse**](GetProductsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_products

> models::GetProductSearchResponse search_products(term, fields, exact_match, include_fields, limit, cursor)
Search products

Searches all products by name, code and/or custom fields. This endpoint is a wrapper of <a href=\"https://developers.pipedrive.com/docs/api/v1/ItemSearch#searchItem\">/v1/itemSearch</a> with a narrower OAuth scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | **String** | The search term to look for. Minimum 2 characters (or 1 if using `exact_match`). Please note that the search term has to be URL encoded. | [required] |
**fields** | Option<**String**> | A comma-separated string array. The fields to perform the search from. Defaults to all of them. Only the following custom field types are searchable: `address`, `varchar`, `text`, `varchar_auto`, `double`, `monetary` and `phone`. Read more about searching by custom fields <a href=\"https://support.pipedrive.com/en/article/search-finding-what-you-need#searching-by-custom-fields\" target=\"_blank\" rel=\"noopener noreferrer\">here</a>. |  |
**exact_match** | Option<**bool**> | When enabled, only full exact matches against the given term are returned. It is <b>not</b> case sensitive. |  |
**include_fields** | Option<**String**> | Supports including optional fields in the results which are not provided by default |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |

### Return type

[**models::GetProductSearchResponse**](GetProductSearchResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_product

> models::UpdateProductResponse update_product(id, update_product_request)
Update a product

Updates product data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |
**update_product_request** | Option<[**UpdateProductRequest**](UpdateProductRequest.md)> |  |  |

### Return type

[**models::UpdateProductResponse**](UpdateProductResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_product_variation

> models::GetProductVariationResponse update_product_variation(id, product_variation_id, update_product_variation_request)
Update a product variation

Updates product variation data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |
**product_variation_id** | **i32** | The ID of the product variation | [required] |
**update_product_variation_request** | Option<[**UpdateProductVariationRequest**](UpdateProductVariationRequest.md)> |  |  |

### Return type

[**models::GetProductVariationResponse**](GetProductVariationResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

