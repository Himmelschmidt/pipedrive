# \OrganizationsApi

All URIs are relative to *https://api.pipedrive.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_organization**](OrganizationsApi.md#add_organization) | **POST** /organizations | Add a new organization
[**add_organization_follower**](OrganizationsApi.md#add_organization_follower) | **POST** /organizations/{id}/followers | Add a follower to an organization
[**delete_organization**](OrganizationsApi.md#delete_organization) | **DELETE** /organizations/{id} | Delete a organization
[**delete_organization_follower**](OrganizationsApi.md#delete_organization_follower) | **DELETE** /organizations/{id}/followers/{follower_id} | Delete a follower from an organization
[**get_organization**](OrganizationsApi.md#get_organization) | **GET** /organizations/{id} | Get details of a organization
[**get_organization_followers**](OrganizationsApi.md#get_organization_followers) | **GET** /organizations/{id}/followers | List followers of an organization
[**get_organization_followers_changelog**](OrganizationsApi.md#get_organization_followers_changelog) | **GET** /organizations/{id}/followers/changelog | List followers changelog of an organization
[**get_organizations**](OrganizationsApi.md#get_organizations) | **GET** /organizations | Get all organizations
[**search_organization**](OrganizationsApi.md#search_organization) | **GET** /organizations/search | Search organizations
[**update_organization**](OrganizationsApi.md#update_organization) | **PATCH** /organizations/{id} | Update a organization



## add_organization

> models::UpsertOrganizationResponse add_organization(add_organization_request)
Add a new organization

Adds a new organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_organization_request** | Option<[**AddOrganizationRequest**](AddOrganizationRequest.md)> |  |  |

### Return type

[**models::UpsertOrganizationResponse**](UpsertOrganizationResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_organization_follower

> models::AddFollowerResponse add_organization_follower(id, add_deal_follower_request)
Add a follower to an organization

Adds a user as a follower to the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |
**add_deal_follower_request** | Option<[**AddDealFollowerRequest**](AddDealFollowerRequest.md)> |  |  |

### Return type

[**models::AddFollowerResponse**](AddFollowerResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization

> models::DeleteOrganizationResponse delete_organization(id)
Delete a organization

Marks a organization as deleted. After 30 days, the organization will be permanently deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |

### Return type

[**models::DeleteOrganizationResponse**](DeleteOrganizationResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_follower

> models::DeleteFollowerResponse delete_organization_follower(id, follower_id)
Delete a follower from an organization

Deletes a user follower from the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |
**follower_id** | **i32** | The ID of the following user | [required] |

### Return type

[**models::DeleteFollowerResponse**](DeleteFollowerResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization

> models::UpsertOrganizationResponse get_organization(id, include_fields, custom_fields)
Get details of a organization

Returns the details of a specific organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |
**include_fields** | Option<**String**> | Optional comma separated string array of additional fields to include |  |
**custom_fields** | Option<**String**> | Optional comma separated string array of custom fields keys to include. If you are only interested in a particular set of custom fields, please use this parameter for faster results and smaller response.<br/>A maximum of 15 keys is allowed. |  |

### Return type

[**models::UpsertOrganizationResponse**](UpsertOrganizationResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_followers

> models::GetFollowersResponse get_organization_followers(id, limit, cursor)
List followers of an organization

Lists users who are following the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |
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


## get_organization_followers_changelog

> models::GetFollowerChangelogsResponse get_organization_followers_changelog(id, limit, cursor)
List followers changelog of an organization

Lists changelogs about users have followed the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |
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


## get_organizations

> models::GetOrganizationsResponse get_organizations(filter_id, ids, owner_id, updated_since, updated_until, sort_by, sort_direction, include_fields, custom_fields, limit, cursor)
Get all organizations

Returns data about all organizations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_id** | Option<**i32**> | If supplied, only organizations matching the specified filter are returned |  |
**ids** | Option<**String**> | Optional comma separated string array of up to 100 entity ids to fetch. If filter_id is provided, this is ignored. If any of the requested entities do not exist or are not visible, they are not included in the response. |  |
**owner_id** | Option<**i32**> | If supplied, only organization owned by the specified user are returned. If filter_id is provided, this is ignored. |  |
**updated_since** | Option<**String**> | If set, only organizations with an `update_time` later than or equal to this time are returned. In RFC3339 format, e.g. 2025-01-01T10:20:00Z. |  |
**updated_until** | Option<**String**> | If set, only organizations with an `update_time` earlier than this time are returned. In RFC3339 format, e.g. 2025-01-01T10:20:00Z. |  |
**sort_by** | Option<**String**> | The field to sort by. Supported fields: `id`, `update_time`, `add_time`. |  |[default to id]
**sort_direction** | Option<**String**> | The sorting direction. Supported values: `asc`, `desc`. |  |[default to asc]
**include_fields** | Option<**String**> | Optional comma separated string array of additional fields to include |  |
**custom_fields** | Option<**String**> | Optional comma separated string array of custom fields keys to include. If you are only interested in a particular set of custom fields, please use this parameter for faster results and smaller response.<br/>A maximum of 15 keys is allowed. |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |

### Return type

[**models::GetOrganizationsResponse**](GetOrganizationsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_organization

> models::GetOrganizationSearchResponse search_organization(term, fields, exact_match, limit, cursor)
Search organizations

Searches all organizations by name, address, notes and/or custom fields. This endpoint is a wrapper of <a href=\"https://developers.pipedrive.com/docs/api/v1/ItemSearch#searchItem\">/v1/itemSearch</a> with a narrower OAuth scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | **String** | The search term to look for. Minimum 2 characters (or 1 if using `exact_match`). Please note that the search term has to be URL encoded. | [required] |
**fields** | Option<**String**> | A comma-separated string array. The fields to perform the search from. Defaults to all of them. Only the following custom field types are searchable: `address`, `varchar`, `text`, `varchar_auto`, `double`, `monetary` and `phone`. Read more about searching by custom fields <a href=\"https://support.pipedrive.com/en/article/search-finding-what-you-need#searching-by-custom-fields\" target=\"_blank\" rel=\"noopener noreferrer\">here</a>. |  |
**exact_match** | Option<**bool**> | When enabled, only full exact matches against the given term are returned. It is <b>not</b> case sensitive. |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |

### Return type

[**models::GetOrganizationSearchResponse**](GetOrganizationSearchResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization

> models::UpsertOrganizationResponse update_organization(id, update_organization_request)
Update a organization

Updates the properties of a organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |
**update_organization_request** | Option<[**UpdateOrganizationRequest**](UpdateOrganizationRequest.md)> |  |  |

### Return type

[**models::UpsertOrganizationResponse**](UpsertOrganizationResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

