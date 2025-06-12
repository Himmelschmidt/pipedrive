# \LeadsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_lead**](LeadsApi.md#add_lead) | **POST** /leads | Add a lead
[**delete_lead**](LeadsApi.md#delete_lead) | **DELETE** /leads/{id} | Delete a lead
[**get_archived_leads**](LeadsApi.md#get_archived_leads) | **GET** /leads/archived | Get all archived leads
[**get_lead**](LeadsApi.md#get_lead) | **GET** /leads/{id} | Get one lead
[**get_lead_users**](LeadsApi.md#get_lead_users) | **GET** /leads/{id}/permittedUsers | List permitted users
[**get_leads**](LeadsApi.md#get_leads) | **GET** /leads | Get all leads
[**search_leads**](LeadsApi.md#search_leads) | **GET** /leads/search | Search leads
[**update_lead**](LeadsApi.md#update_lead) | **PATCH** /leads/{id} | Update a lead



## add_lead

> models::GetLeadResponse add_lead(add_lead_request)
Add a lead

Creates a lead. A lead always has to be linked to a person or an organization or both. All leads created through the Pipedrive API will have a lead source and origin set to `API`. Here's the tutorial for <a href=\"https://pipedrive.readme.io/docs/adding-a-lead\" target=\"_blank\" rel=\"noopener noreferrer\">adding a lead</a>. If a lead contains custom fields, the fields' values will be included in the response in the same format as with the `Deals` endpoints. If a custom field's value hasn't been set for the lead, it won't appear in the response. Please note that leads do not have a separate set of custom fields, instead they inherit the custom fields' structure from deals. See an example given in the <a href=\"https://pipedrive.readme.io/docs/updating-custom-field-value\" target=\"_blank\" rel=\"noopener noreferrer\">updating custom fields' values tutorial</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_lead_request** | Option<[**AddLeadRequest**](AddLeadRequest.md)> |  |  |

### Return type

[**models::GetLeadResponse**](GetLeadResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lead

> models::GetLeadIdResponse delete_lead(id)
Delete a lead

Deletes a specific lead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The ID of the lead | [required] |

### Return type

[**models::GetLeadIdResponse**](GetLeadIdResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_archived_leads

> models::GetLeadsResponse get_archived_leads(limit, start, owner_id, person_id, organization_id, filter_id, sort)
Get all archived leads

Returns multiple archived leads. Leads are sorted by the time they were created, from oldest to newest. Pagination can be controlled using `limit` and `start` query parameters. If a lead contains custom fields, the fields' values will be included in the response in the same format as with the `Deals` endpoints. If a custom field's value hasn't been set for the lead, it won't appear in the response. Please note that leads do not have a separate set of custom fields, instead they inherit the custom fields' structure from deals. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. |  |
**start** | Option<**i32**> | For pagination, the position that represents the first result for the page |  |
**owner_id** | Option<**i32**> | If supplied, only leads matching the given user will be returned. However, `filter_id` takes precedence over `owner_id` when supplied. |  |
**person_id** | Option<**i32**> | If supplied, only leads matching the given person will be returned. However, `filter_id` takes precedence over `person_id` when supplied. |  |
**organization_id** | Option<**i32**> | If supplied, only leads matching the given organization will be returned. However, `filter_id` takes precedence over `organization_id` when supplied. |  |
**filter_id** | Option<**i32**> | The ID of the filter to use |  |
**sort** | Option<**String**> | The field names and sorting mode separated by a comma (`field_name_1 ASC`, `field_name_2 DESC`). Only first-level field keys are supported (no nested keys). |  |

### Return type

[**models::GetLeadsResponse**](GetLeadsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lead

> models::GetLeadResponse get_lead(id)
Get one lead

Returns details of a specific lead. If a lead contains custom fields, the fields' values will be included in the response in the same format as with the `Deals` endpoints. If a custom field's value hasn't been set for the lead, it won't appear in the response. Please note that leads do not have a separate set of custom fields, instead they inherit the custom fields’ structure from deals.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The ID of the lead | [required] |

### Return type

[**models::GetLeadResponse**](GetLeadResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lead_users

> models::UserIds get_lead_users(id)
List permitted users

Lists the users permitted to access a lead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the lead | [required] |

### Return type

[**models::UserIds**](userIds.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_leads

> models::GetLeadsResponse get_leads(limit, start, owner_id, person_id, organization_id, filter_id, sort)
Get all leads

Returns multiple not archived leads. Leads are sorted by the time they were created, from oldest to newest. Pagination can be controlled using `limit` and `start` query parameters. If a lead contains custom fields, the fields' values will be included in the response in the same format as with the `Deals` endpoints. If a custom field's value hasn't been set for the lead, it won't appear in the response. Please note that leads do not have a separate set of custom fields, instead they inherit the custom fields' structure from deals. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. |  |
**start** | Option<**i32**> | For pagination, the position that represents the first result for the page |  |
**owner_id** | Option<**i32**> | If supplied, only leads matching the given user will be returned. However, `filter_id` takes precedence over `owner_id` when supplied. |  |
**person_id** | Option<**i32**> | If supplied, only leads matching the given person will be returned. However, `filter_id` takes precedence over `person_id` when supplied. |  |
**organization_id** | Option<**i32**> | If supplied, only leads matching the given organization will be returned. However, `filter_id` takes precedence over `organization_id` when supplied. |  |
**filter_id** | Option<**i32**> | The ID of the filter to use |  |
**sort** | Option<**String**> | The field names and sorting mode separated by a comma (`field_name_1 ASC`, `field_name_2 DESC`). Only first-level field keys are supported (no nested keys). |  |

### Return type

[**models::GetLeadsResponse**](GetLeadsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_leads

> models::GetLeadSearchResponse search_leads(term, fields, exact_match, person_id, organization_id, include_fields, start, limit)
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
**start** | Option<**i32**> | Pagination start. Note that the pagination is based on main results and does not include related items when using `search_for_related_items` parameter. |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**models::GetLeadSearchResponse**](GetLeadSearchResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_lead

> models::GetLeadResponse update_lead(id, update_lead_request)
Update a lead

Updates one or more properties of a lead. Only properties included in the request will be updated. Send `null` to unset a property (applicable for example for `value`, `person_id` or `organization_id`). If a lead contains custom fields, the fields' values will be included in the response in the same format as with the `Deals` endpoints. If a custom field's value hasn't been set for the lead, it won't appear in the response. Please note that leads do not have a separate set of custom fields, instead they inherit the custom fields’ structure from deals. See an example given in the <a href=\"https://pipedrive.readme.io/docs/updating-custom-field-value\" target=\"_blank\" rel=\"noopener noreferrer\">updating custom fields’ values tutorial</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The ID of the lead | [required] |
**update_lead_request** | Option<[**UpdateLeadRequest**](UpdateLeadRequest.md)> |  |  |

### Return type

[**models::GetLeadResponse**](GetLeadResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

