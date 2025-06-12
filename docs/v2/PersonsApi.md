# \PersonsApi

All URIs are relative to *https://api.pipedrive.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_person**](PersonsApi.md#add_person) | **POST** /persons | Add a new person
[**add_person_follower**](PersonsApi.md#add_person_follower) | **POST** /persons/{id}/followers | Add a follower to a person
[**delete_person**](PersonsApi.md#delete_person) | **DELETE** /persons/{id} | Delete a person
[**delete_person_follower**](PersonsApi.md#delete_person_follower) | **DELETE** /persons/{id}/followers/{follower_id} | Delete a follower from a person
[**get_person**](PersonsApi.md#get_person) | **GET** /persons/{id} | Get details of a person
[**get_person_followers**](PersonsApi.md#get_person_followers) | **GET** /persons/{id}/followers | List followers of a person
[**get_person_followers_changelog**](PersonsApi.md#get_person_followers_changelog) | **GET** /persons/{id}/followers/changelog | List followers changelog of a person
[**get_persons**](PersonsApi.md#get_persons) | **GET** /persons | Get all persons
[**search_persons**](PersonsApi.md#search_persons) | **GET** /persons/search | Search persons
[**update_person**](PersonsApi.md#update_person) | **PATCH** /persons/{id} | Update a person



## add_person

> models::UpsertPersonResponse add_person(add_person_request)
Add a new person

Adds a new person. If the company uses the [Campaigns product](https://pipedrive.readme.io/docs/campaigns-in-pipedrive-api), then this endpoint will also accept and return the `marketing_status` field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_person_request** | Option<[**AddPersonRequest**](AddPersonRequest.md)> |  |  |

### Return type

[**models::UpsertPersonResponse**](UpsertPersonResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_person_follower

> models::AddFollowerResponse add_person_follower(id, add_deal_follower_request)
Add a follower to a person

Adds a user as a follower to the person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
**add_deal_follower_request** | Option<[**AddDealFollowerRequest**](AddDealFollowerRequest.md)> |  |  |

### Return type

[**models::AddFollowerResponse**](AddFollowerResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_person

> models::DeletePersonResponse delete_person(id)
Delete a person

Marks a person as deleted. After 30 days, the person will be permanently deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |

### Return type

[**models::DeletePersonResponse**](DeletePersonResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_person_follower

> models::DeleteFollowerResponse delete_person_follower(id, follower_id)
Delete a follower from a person

Deletes a user follower from the person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
**follower_id** | **i32** | The ID of the following user | [required] |

### Return type

[**models::DeleteFollowerResponse**](DeleteFollowerResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person

> models::UpsertPersonResponse get_person(id, include_fields, custom_fields)
Get details of a person

Returns the details of a specific person. Fields `ims`, `postal_address`, `notes`, `birthday`, and `job_title` are only included if contact sync is enabled for the company.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
**include_fields** | Option<**String**> | Optional comma separated string array of additional fields to include. `marketing_status` and `doi_status` can only be included if the company has marketing app enabled. |  |
**custom_fields** | Option<**String**> | Optional comma separated string array of custom fields keys to include. If you are only interested in a particular set of custom fields, please use this parameter for faster results and smaller response.<br/>A maximum of 15 keys is allowed. |  |

### Return type

[**models::UpsertPersonResponse**](UpsertPersonResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_followers

> models::GetFollowersResponse get_person_followers(id, limit, cursor)
List followers of a person

Lists users who are following the person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
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


## get_person_followers_changelog

> models::GetFollowerChangelogsResponse get_person_followers_changelog(id, limit, cursor)
List followers changelog of a person

Lists changelogs about users have followed the person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
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


## get_persons

> models::GetPersonsResponse get_persons(filter_id, ids, owner_id, org_id, updated_since, updated_until, sort_by, sort_direction, include_fields, custom_fields, limit, cursor)
Get all persons

Returns data about all persons. Fields `ims`, `postal_address`, `notes`, `birthday`, and `job_title` are only included if contact sync is enabled for the company.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_id** | Option<**i32**> | If supplied, only persons matching the specified filter are returned |  |
**ids** | Option<**String**> | Optional comma separated string array of up to 100 entity ids to fetch. If filter_id is provided, this is ignored. If any of the requested entities do not exist or are not visible, they are not included in the response. |  |
**owner_id** | Option<**i32**> | If supplied, only persons owned by the specified user are returned. If filter_id is provided, this is ignored. |  |
**org_id** | Option<**i32**> | If supplied, only persons linked to the specified organization are returned. If filter_id is provided, this is ignored. |  |
**updated_since** | Option<**String**> | If set, only persons with an `update_time` later than or equal to this time are returned. In RFC3339 format, e.g. 2025-01-01T10:20:00Z. |  |
**updated_until** | Option<**String**> | If set, only persons with an `update_time` earlier than this time are returned. In RFC3339 format, e.g. 2025-01-01T10:20:00Z. |  |
**sort_by** | Option<**String**> | The field to sort by. Supported fields: `id`, `update_time`, `add_time`. |  |[default to id]
**sort_direction** | Option<**String**> | The sorting direction. Supported values: `asc`, `desc`. |  |[default to asc]
**include_fields** | Option<**String**> | Optional comma separated string array of additional fields to include. `marketing_status` and `doi_status` can only be included if the company has marketing app enabled. |  |
**custom_fields** | Option<**String**> | Optional comma separated string array of custom fields keys to include. If you are only interested in a particular set of custom fields, please use this parameter for faster results and smaller response.<br/>A maximum of 15 keys is allowed. |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |

### Return type

[**models::GetPersonsResponse**](GetPersonsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_persons

> models::GetPersonSearchResponse search_persons(term, fields, exact_match, organization_id, include_fields, limit, cursor)
Search persons

Searches all persons by name, email, phone, notes and/or custom fields. This endpoint is a wrapper of <a href=\"https://developers.pipedrive.com/docs/api/v1/ItemSearch#searchItem\">/v1/itemSearch</a> with a narrower OAuth scope. Found persons can be filtered by organization ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | **String** | The search term to look for. Minimum 2 characters (or 1 if using `exact_match`). Please note that the search term has to be URL encoded. | [required] |
**fields** | Option<**String**> | A comma-separated string array. The fields to perform the search from. Defaults to all of them. Only the following custom field types are searchable: `address`, `varchar`, `text`, `varchar_auto`, `double`, `monetary` and `phone`. Read more about searching by custom fields <a href=\"https://support.pipedrive.com/en/article/search-finding-what-you-need#searching-by-custom-fields\" target=\"_blank\" rel=\"noopener noreferrer\">here</a>. |  |
**exact_match** | Option<**bool**> | When enabled, only full exact matches against the given term are returned. It is <b>not</b> case sensitive. |  |
**organization_id** | Option<**i32**> | Will filter persons by the provided organization ID. The upper limit of found persons associated with the organization is 2000. |  |
**include_fields** | Option<**String**> | Supports including optional fields in the results which are not provided by default |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |

### Return type

[**models::GetPersonSearchResponse**](GetPersonSearchResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_person

> models::UpsertPersonResponse update_person(id, update_person_request)
Update a person

Updates the properties of a person. <br>If the company uses the [Campaigns product](https://pipedrive.readme.io/docs/campaigns-in-pipedrive-api), then this endpoint will also accept and return the `marketing_status` field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
**update_person_request** | Option<[**UpdatePersonRequest**](UpdatePersonRequest.md)> |  |  |

### Return type

[**models::UpsertPersonResponse**](UpsertPersonResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

