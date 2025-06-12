# \ItemSearchApi

All URIs are relative to *https://api.pipedrive.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_item**](ItemSearchApi.md#search_item) | **GET** /itemSearch | Perform a search from multiple item types
[**search_item_by_field**](ItemSearchApi.md#search_item_by_field) | **GET** /itemSearch/field | Perform a search using a specific field from an item type



## search_item

> models::GetItemSearchResponse search_item(term, item_types, fields, search_for_related_items, exact_match, include_fields, limit, cursor)
Perform a search from multiple item types

Performs a search from your choice of item types and fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | **String** | The search term to look for. Minimum 2 characters (or 1 if using `exact_match`). Please note that the search term has to be URL encoded. | [required] |
**item_types** | Option<**String**> | A comma-separated string array. The type of items to perform the search from. Defaults to all. |  |
**fields** | Option<**String**> | A comma-separated string array. The fields to perform the search from. Defaults to all. Relevant for each item type are:<br> <table> <tr><th><b>Item type</b></th><th><b>Field</b></th></tr> <tr><td>Deal</td><td>`custom_fields`, `notes`, `title`</td></tr> <tr><td>Person</td><td>`custom_fields`, `email`, `name`, `notes`, `phone`</td></tr> <tr><td>Organization</td><td>`address`, `custom_fields`, `name`, `notes`</td></tr> <tr><td>Product</td><td>`code`, `custom_fields`, `name`</td></tr> <tr><td>Lead</td><td>`custom_fields`, `notes`, `email`, `organization_name`, `person_name`, `phone`, `title`</td></tr> <tr><td>File</td><td>`name`</td></tr> <tr><td>Mail attachment</td><td>`name`</td></tr> <tr><td>Project</td><td> `custom_fields`, `notes`, `title`, `description` </td></tr> </table> <br> Only the following custom field types are searchable: `address`, `varchar`, `text`, `varchar_auto`, `double`, `monetary` and `phone`. Read more about searching by custom fields <a href=\"https://support.pipedrive.com/en/article/search-finding-what-you-need#searching-by-custom-fields\" target=\"_blank\" rel=\"noopener noreferrer\">here</a>.<br/> When searching for leads, the email, organization_name, person_name, and phone fields will return results only for leads not linked to contacts. For searching leads by person or organization values, please use `search_for_related_items`. |  |
**search_for_related_items** | Option<**bool**> | When enabled, the response will include up to 100 newest related leads and 100 newest related deals for each found person and organization and up to 100 newest related persons for each found organization |  |
**exact_match** | Option<**bool**> | When enabled, only full exact matches against the given term are returned. It is <b>not</b> case sensitive. |  |
**include_fields** | Option<**String**> | A comma-separated string array. Supports including optional fields in the results which are not provided by default. |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |

### Return type

[**models::GetItemSearchResponse**](GetItemSearchResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_item_by_field

> models::GetItemSearchFieldResponse search_item_by_field(term, entity_type, field, r#match, limit, cursor)
Perform a search using a specific field from an item type

Performs a search from the values of a specific field. Results can either be the distinct values of the field (useful for searching autocomplete field values), or the IDs of actual items (deals, leads, persons, organizations or products).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | **String** | The search term to look for. Minimum 2 characters (or 1 if `match` is `exact`). Please note that the search term has to be URL encoded. | [required] |
**entity_type** | **String** | The type of the field to perform the search from | [required] |
**field** | **String** | The key of the field to search from. The field key can be obtained by fetching the list of the fields using any of the fields' API GET methods (dealFields, personFields, etc.). Only the following custom field types are searchable: `address`, `varchar`, `text`, `varchar_auto`, `double`, `monetary` and `phone`. Read more about searching by custom fields <a href=\"https://support.pipedrive.com/en/article/search-finding-what-you-need#searching-by-custom-fields\" target=\"_blank\" rel=\"noopener noreferrer\">here</a>. | [required] |
**r#match** | Option<**String**> | The type of match used against the term. The search <b>is</b> case sensitive.<br/><br/> E.g. in case of searching for a value `monkey`, <ul> <li>with `exact` match, you will only find it if term is `monkey`</li> <li>with `beginning` match, you will only find it if the term matches the beginning or the whole string, e.g. `monk` and `monkey`</li> <li>with `middle` match, you will find the it if the term matches any substring of the value, e.g. `onk` and `ke`</li> </ul>. |  |[default to exact]
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |

### Return type

[**models::GetItemSearchFieldResponse**](GetItemSearchFieldResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

