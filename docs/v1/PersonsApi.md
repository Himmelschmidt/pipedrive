# \PersonsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_person**](PersonsApi.md#add_person) | **POST** /persons | Add a person
[**add_person_follower**](PersonsApi.md#add_person_follower) | **POST** /persons/{id}/followers | Add a follower to a person
[**add_person_picture**](PersonsApi.md#add_person_picture) | **POST** /persons/{id}/picture | Add person picture
[**delete_person**](PersonsApi.md#delete_person) | **DELETE** /persons/{id} | Delete a person
[**delete_person_follower**](PersonsApi.md#delete_person_follower) | **DELETE** /persons/{id}/followers/{follower_id} | Delete a follower from a person
[**delete_person_picture**](PersonsApi.md#delete_person_picture) | **DELETE** /persons/{id}/picture | Delete person picture
[**delete_persons**](PersonsApi.md#delete_persons) | **DELETE** /persons | Delete multiple persons in bulk
[**get_person**](PersonsApi.md#get_person) | **GET** /persons/{id} | Get details of a person
[**get_person_activities**](PersonsApi.md#get_person_activities) | **GET** /persons/{id}/activities | List activities associated with a person
[**get_person_changelog**](PersonsApi.md#get_person_changelog) | **GET** /persons/{id}/changelog | List updates about person field values
[**get_person_deals**](PersonsApi.md#get_person_deals) | **GET** /persons/{id}/deals | List deals associated with a person
[**get_person_files**](PersonsApi.md#get_person_files) | **GET** /persons/{id}/files | List files attached to a person
[**get_person_followers**](PersonsApi.md#get_person_followers) | **GET** /persons/{id}/followers | List followers of a person
[**get_person_mail_messages**](PersonsApi.md#get_person_mail_messages) | **GET** /persons/{id}/mailMessages | List mail messages associated with a person
[**get_person_products**](PersonsApi.md#get_person_products) | **GET** /persons/{id}/products | List products associated with a person
[**get_person_updates**](PersonsApi.md#get_person_updates) | **GET** /persons/{id}/flow | List updates about a person
[**get_person_users**](PersonsApi.md#get_person_users) | **GET** /persons/{id}/permittedUsers | List permitted users
[**get_persons**](PersonsApi.md#get_persons) | **GET** /persons | Get all persons
[**get_persons_collection**](PersonsApi.md#get_persons_collection) | **GET** /persons/collection | Get all persons collection
[**merge_persons**](PersonsApi.md#merge_persons) | **PUT** /persons/{id}/merge | Merge two persons
[**search_persons**](PersonsApi.md#search_persons) | **GET** /persons/search | Search persons
[**update_person**](PersonsApi.md#update_person) | **PUT** /persons/{id} | Update a person



## add_person

> models::AddPersonResponse add_person(add_person_request)
Add a person

Adds a new person. Note that you can supply additional custom fields along with the request that are not described here. These custom fields are different for each Pipedrive account and can be recognized by long hashes as keys. To determine which custom fields exists, fetch the personFields and look for `key` values.<br>If a company uses the [Campaigns product](https://pipedrive.readme.io/docs/campaigns-in-pipedrive-api), then this endpoint will also accept and return the `data.marketing_status` field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_person_request** | Option<[**AddPersonRequest**](AddPersonRequest.md)> |  |  |

### Return type

[**models::AddPersonResponse**](AddPersonResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_person_follower

> models::AddPersonFollowerResponse add_person_follower(id, add_person_follower_request)
Add a follower to a person

Adds a follower to a person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
**add_person_follower_request** | Option<[**AddPersonFollowerRequest**](AddPersonFollowerRequest.md)> |  |  |

### Return type

[**models::AddPersonFollowerResponse**](AddPersonFollowerResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_person_picture

> models::AddPersonPictureResponse add_person_picture(id, file, crop_x, crop_y, crop_width, crop_height)
Add person picture

Adds a picture to a person. If a picture is already set, the old picture will be replaced. Added image (or the cropping parameters supplied with the request) should have an equal width and height and should be at least 128 pixels. GIF, JPG and PNG are accepted. All added images will be resized to 128 and 512 pixel wide squares.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
**file** | **std::path::PathBuf** | One image supplied in the multipart/form-data encoding | [required] |
**crop_x** | Option<**i32**> | X coordinate to where start cropping form (in pixels) |  |
**crop_y** | Option<**i32**> | Y coordinate to where start cropping form (in pixels) |  |
**crop_width** | Option<**i32**> | The width of the cropping area (in pixels) |  |
**crop_height** | Option<**i32**> | The height of the cropping area (in pixels) |  |

### Return type

[**models::AddPersonPictureResponse**](AddPersonPictureResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
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

> models::DeletePersonResponse delete_person_follower(id, follower_id)
Delete a follower from a person

Deletes a follower from a person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
**follower_id** | **i32** | The ID of the relationship between the follower and the person | [required] |

### Return type

[**models::DeletePersonResponse**](DeletePersonResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_person_picture

> models::DeletePersonResponse delete_person_picture(id)
Delete person picture

Deletes a person’s picture.

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


## delete_persons

> models::DeletePersonsResponse delete_persons(ids)
Delete multiple persons in bulk

Marks multiple persons as deleted. After 30 days, the persons will be permanently deleted. <br>This endpoint has been deprecated. Please use <a href=\"https://developers.pipedrive.com/docs/api/v1/Persons#deletePerson\" target=\"_blank\" rel=\"noopener noreferrer\">DELETE /api/v2/persons/{id}</a> instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | The comma-separated IDs that will be deleted | [required] |

### Return type

[**models::DeletePersonsResponse**](DeletePersonsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person

> models::GetPersonDetailsResponse get_person(id)
Get details of a person

Returns the details of a person. Note that this also returns some additional fields which are not present when asking for all persons. Also note that custom fields appear as long hashes in the resulting data. These hashes can be mapped against the `key` value of personFields.<br>If a company uses the [Campaigns product](https://pipedrive.readme.io/docs/campaigns-in-pipedrive-api), then this endpoint will also return the `data.marketing_status` field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |

### Return type

[**models::GetPersonDetailsResponse**](GetPersonDetailsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_activities

> models::GetAssociatedActivitiesResponse get_person_activities(id, start, limit, done, exclude)
List activities associated with a person

Lists activities associated with a person. <br>This endpoint has been deprecated. Please use <a href=\"https://developers.pipedrive.com/docs/api/v1/Activities#getActivities\" target=\"_blank\" rel=\"noopener noreferrer\">GET /api/v2/activities?person_id={id}</a> instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**done** | Option<**f64**> | Whether the activity is done or not. 0 = Not done, 1 = Done. If omitted, returns both Done and Not done activities. |  |
**exclude** | Option<**String**> | A comma-separated string of activity IDs to exclude from result |  |

### Return type

[**models::GetAssociatedActivitiesResponse**](GetAssociatedActivitiesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_changelog

> models::GetChangelogResponse get_person_changelog(id, cursor, limit)
List updates about person field values

Lists updates about field values of a person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**models::GetChangelogResponse**](GetChangelogResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_deals

> models::GetAssociatedDealsResponse get_person_deals(id, start, limit, status, sort)
List deals associated with a person

Lists deals associated with a person. <br>This endpoint has been deprecated. Please use <a href=\"https://developers.pipedrive.com/docs/api/v1/Deals#getDeals\" target=\"_blank\" rel=\"noopener noreferrer\">GET /api/v2/deals?person_id={id}</a> instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**status** | Option<**String**> | Only fetch deals with a specific status. If omitted, all not deleted deals are returned. If set to deleted, deals that have been deleted up to 30 days ago will be included. |  |[default to all_not_deleted]
**sort** | Option<**String**> | The field names and sorting mode separated by a comma (`field_name_1 ASC`, `field_name_2 DESC`). Only first-level field keys are supported (no nested keys). |  |

### Return type

[**models::GetAssociatedDealsResponse**](GetAssociatedDealsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_files

> models::GetAssociatedFilesResponse get_person_files(id, start, limit, sort)
List files attached to a person

Lists files associated with a person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page. Please note that a maximum value of 100 is allowed. |  |
**sort** | Option<**String**> | Supported fields: `id`, `update_time` |  |

### Return type

[**models::GetAssociatedFilesResponse**](GetAssociatedFilesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_followers

> models::GetListFollowersResponse get_person_followers(id)
List followers of a person

Lists the followers of a person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |

### Return type

[**models::GetListFollowersResponse**](GetListFollowersResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_mail_messages

> models::GetAssociatedMailMessagesResponse get_person_mail_messages(id, start, limit)
List mail messages associated with a person

Lists mail messages associated with a person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**models::GetAssociatedMailMessagesResponse**](GetAssociatedMailMessagesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_products

> models::GetPersonProductsResponse get_person_products(id, start, limit)
List products associated with a person

Lists products associated with a person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**models::GetPersonProductsResponse**](GetPersonProductsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_updates

> models::GetAssociatedPersonUpdatesResponse get_person_updates(id, start, limit, all_changes, items)
List updates about a person

Lists updates about a person.<br>If a company uses the [Campaigns product](https://pipedrive.readme.io/docs/campaigns-in-pipedrive-api), then this endpoint's response will also include updates for the `marketing_status` field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**all_changes** | Option<**String**> | Whether to show custom field updates or not. 1 = Include custom field changes. If omitted returns changes without custom field updates. |  |
**items** | Option<**String**> | A comma-separated string for filtering out item specific updates. (Possible values - call, activity, plannedActivity, change, note, deal, file, dealChange, personChange, organizationChange, follower, dealFollower, personFollower, organizationFollower, participant, comment, mailMessage, mailMessageWithAttachment, invoice, document, marketing_campaign_stat, marketing_status_change). |  |

### Return type

[**models::GetAssociatedPersonUpdatesResponse**](GetAssociatedPersonUpdatesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_users

> models::GetPermittedUsersResponse1 get_person_users(id)
List permitted users

List users permitted to access a person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |

### Return type

[**models::GetPermittedUsersResponse1**](GetPermittedUsersResponse_1.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_persons

> models::GetPersonsResponse1 get_persons(user_id, filter_id, first_char, start, limit, sort)
Get all persons

Returns all persons.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**i32**> | If supplied, only persons owned by the given user will be returned. However, `filter_id` takes precedence over `user_id` when both are supplied. |  |
**filter_id** | Option<**i32**> | The ID of the filter to use |  |
**first_char** | Option<**String**> | If supplied, only persons whose name starts with the specified letter will be returned (case-insensitive) |  |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**sort** | Option<**String**> | The field names and sorting mode separated by a comma (`field_name_1 ASC`, `field_name_2 DESC`). Only first-level field keys are supported (no nested keys). |  |

### Return type

[**models::GetPersonsResponse1**](GetPersonsResponse_1.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_persons_collection

> models::GetPersonsCollection200Response get_persons_collection(cursor, limit, since, until, owner_id, first_char)
Get all persons collection

Returns all persons. Please note that only global admins (those with global permissions) can access this endpoint. Users with regular permissions will receive a 403 response. Read more about global permissions <a href=\"https://support.pipedrive.com/en/article/global-user-management\" target=\"_blank\" rel=\"noopener noreferrer\">here</a>. <br>This endpoint has been deprecated. Please use <a href=\"https://developers.pipedrive.com/docs/api/v1/Persons#getPersons\" target=\"_blank\" rel=\"noopener noreferrer\">GET /api/v2/persons</a> instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**since** | Option<**String**> | The time boundary that points to the start of the range of data. Datetime in ISO 8601 format. E.g. 2022-11-01 08:55:59. Operates on the `update_time` field. |  |
**until** | Option<**String**> | The time boundary that points to the end of the range of data. Datetime in ISO 8601 format. E.g. 2022-11-01 08:55:59. Operates on the `update_time` field. |  |
**owner_id** | Option<**i32**> | If supplied, only persons owned by the given user will be returned |  |
**first_char** | Option<**String**> | If supplied, only persons whose name starts with the specified letter will be returned (case-insensitive) |  |

### Return type

[**models::GetPersonsCollection200Response**](getPersonsCollection_200_response.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_persons

> models::MergePersonsResponse merge_persons(id, merge_persons_request)
Merge two persons

Merges a person with another person. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/merging-two-persons\" target=\"_blank\" rel=\"noopener noreferrer\">merging two persons</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
**merge_persons_request** | Option<[**MergePersonsRequest**](MergePersonsRequest.md)> |  |  |

### Return type

[**models::MergePersonsResponse**](MergePersonsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_persons

> models::GetPersonSearchResponse search_persons(term, fields, exact_match, organization_id, include_fields, start, limit)
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
**start** | Option<**i32**> | Pagination start. Note that the pagination is based on main results and does not include related items when using `search_for_related_items` parameter. |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**models::GetPersonSearchResponse**](GetPersonSearchResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_person

> models::UpdatePersonResponse update_person(id, update_person_request)
Update a person

Updates the properties of a person. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/updating-a-person\" target=\"_blank\" rel=\"noopener noreferrer\">updating a person</a>.<br>If a company uses the [Campaigns product](https://pipedrive.readme.io/docs/campaigns-in-pipedrive-api), then this endpoint will also accept and return the `data.marketing_status` field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the person | [required] |
**update_person_request** | Option<[**UpdatePersonRequest**](UpdatePersonRequest.md)> |  |  |

### Return type

[**models::UpdatePersonResponse**](UpdatePersonResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

