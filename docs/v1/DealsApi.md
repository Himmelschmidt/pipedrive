# \DealsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_deal**](DealsApi.md#add_deal) | **POST** /deals | Add a deal
[**add_deal_follower**](DealsApi.md#add_deal_follower) | **POST** /deals/{id}/followers | Add a follower to a deal
[**add_deal_participant**](DealsApi.md#add_deal_participant) | **POST** /deals/{id}/participants | Add a participant to a deal
[**add_deal_product**](DealsApi.md#add_deal_product) | **POST** /deals/{id}/products | Add a product to a deal
[**delete_deal**](DealsApi.md#delete_deal) | **DELETE** /deals/{id} | Delete a deal
[**delete_deal_follower**](DealsApi.md#delete_deal_follower) | **DELETE** /deals/{id}/followers/{follower_id} | Delete a follower from a deal
[**delete_deal_participant**](DealsApi.md#delete_deal_participant) | **DELETE** /deals/{id}/participants/{deal_participant_id} | Delete a participant from a deal
[**delete_deal_product**](DealsApi.md#delete_deal_product) | **DELETE** /deals/{id}/products/{product_attachment_id} | Delete an attached product from a deal
[**delete_deals**](DealsApi.md#delete_deals) | **DELETE** /deals | Delete multiple deals in bulk
[**duplicate_deal**](DealsApi.md#duplicate_deal) | **POST** /deals/{id}/duplicate | Duplicate deal
[**get_archived_deals**](DealsApi.md#get_archived_deals) | **GET** /deals/archived | Get all archived deals
[**get_archived_deals_summary**](DealsApi.md#get_archived_deals_summary) | **GET** /deals/summary/archived | Get archived deals summary
[**get_archived_deals_timeline**](DealsApi.md#get_archived_deals_timeline) | **GET** /deals/timeline/archived | Get archived deals timeline
[**get_deal**](DealsApi.md#get_deal) | **GET** /deals/{id} | Get details of a deal
[**get_deal_activities**](DealsApi.md#get_deal_activities) | **GET** /deals/{id}/activities | List activities associated with a deal
[**get_deal_changelog**](DealsApi.md#get_deal_changelog) | **GET** /deals/{id}/changelog | List updates about deal field values
[**get_deal_files**](DealsApi.md#get_deal_files) | **GET** /deals/{id}/files | List files attached to a deal
[**get_deal_followers**](DealsApi.md#get_deal_followers) | **GET** /deals/{id}/followers | List followers of a deal
[**get_deal_mail_messages**](DealsApi.md#get_deal_mail_messages) | **GET** /deals/{id}/mailMessages | List mail messages associated with a deal
[**get_deal_participants**](DealsApi.md#get_deal_participants) | **GET** /deals/{id}/participants | List participants of a deal
[**get_deal_participants_changelog**](DealsApi.md#get_deal_participants_changelog) | **GET** /deals/{id}/participantsChangelog | List updates about participants of a deal
[**get_deal_persons**](DealsApi.md#get_deal_persons) | **GET** /deals/{id}/persons | List all persons associated with a deal
[**get_deal_products**](DealsApi.md#get_deal_products) | **GET** /deals/{id}/products | List products attached to a deal
[**get_deal_updates**](DealsApi.md#get_deal_updates) | **GET** /deals/{id}/flow | List updates about a deal
[**get_deal_users**](DealsApi.md#get_deal_users) | **GET** /deals/{id}/permittedUsers | List permitted users
[**get_deals**](DealsApi.md#get_deals) | **GET** /deals | Get all deals
[**get_deals_collection**](DealsApi.md#get_deals_collection) | **GET** /deals/collection | Get all deals collection
[**get_deals_summary**](DealsApi.md#get_deals_summary) | **GET** /deals/summary | Get deals summary
[**get_deals_timeline**](DealsApi.md#get_deals_timeline) | **GET** /deals/timeline | Get deals timeline
[**merge_deals**](DealsApi.md#merge_deals) | **PUT** /deals/{id}/merge | Merge two deals
[**search_deals**](DealsApi.md#search_deals) | **GET** /deals/search | Search deals
[**update_deal**](DealsApi.md#update_deal) | **PUT** /deals/{id} | Update a deal
[**update_deal_product**](DealsApi.md#update_deal_product) | **PUT** /deals/{id}/products/{product_attachment_id} | Update the product attached to a deal



## add_deal

> models::GetAddedDealResponse add_deal(add_deal_request)
Add a deal

Adds a new deal. All deals created through the Pipedrive API will have a `origin` set to `API`. Note that you can supply additional custom fields along with the request that are not described here. These custom fields are different for each Pipedrive account and can be recognized by long hashes as keys. To determine which custom fields exists, fetch the dealFields and look for `key` values. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/creating-a-deal\" target=\"_blank\" rel=\"noopener noreferrer\">adding a deal</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_deal_request** | Option<[**AddDealRequest**](AddDealRequest.md)> |  |  |

### Return type

[**models::GetAddedDealResponse**](GetAddedDealResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_deal_follower

> models::AddDealFollowerResponse add_deal_follower(id, add_deal_follower_request)
Add a follower to a deal

Adds a follower to a deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**add_deal_follower_request** | Option<[**AddDealFollowerRequest**](AddDealFollowerRequest.md)> |  |  |

### Return type

[**models::AddDealFollowerResponse**](AddDealFollowerResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_deal_participant

> models::AddParticipantsResponse add_deal_participant(id, add_deal_participant_request)
Add a participant to a deal

Adds a participant to a deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**add_deal_participant_request** | Option<[**AddDealParticipantRequest**](AddDealParticipantRequest.md)> |  |  |

### Return type

[**models::AddParticipantsResponse**](AddParticipantsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_deal_product

> models::GetAddProductAttachmentDetailsResponse add_deal_product(id, add_deal_product_request)
Add a product to a deal

Adds a product to a deal, creating a new item called a deal-product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**add_deal_product_request** | Option<[**AddDealProductRequest**](AddDealProductRequest.md)> |  |  |

### Return type

[**models::GetAddProductAttachmentDetailsResponse**](GetAddProductAttachmentDetailsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
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

> models::DeleteDealFollowerResponse delete_deal_follower(id, follower_id)
Delete a follower from a deal

Deletes a follower from a deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**follower_id** | **i32** | The ID of the relationship between the follower and the deal | [required] |

### Return type

[**models::DeleteDealFollowerResponse**](DeleteDealFollowerResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_deal_participant

> models::DeleteDealParticipantResponse delete_deal_participant(id, deal_participant_id)
Delete a participant from a deal

Deletes a participant from a deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**deal_participant_id** | **i32** | The ID of the participant of the deal | [required] |

### Return type

[**models::DeleteDealParticipantResponse**](DeleteDealParticipantResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_deal_product

> models::DeleteDealProductResponse delete_deal_product(id, product_attachment_id)
Delete an attached product from a deal

Deletes a product attachment from a deal, using the `product_attachment_id`  Not possible to delete the attached product if the deal has installments associated and the product is the last one enabled 

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


## delete_deals

> models::DeleteDealsResponse delete_deals(ids)
Delete multiple deals in bulk

Marks multiple deals as deleted. After 30 days, the deals will be permanently deleted. <br>This endpoint has been deprecated. Please use <a href=\"https://developers.pipedrive.com/docs/api/v1/Deals#deleteDeal\" target=\"_blank\" rel=\"noopener noreferrer\">DELETE /api/v2/deals/{id}</a> instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | The comma-separated IDs that will be deleted | [required] |

### Return type

[**models::DeleteDealsResponse**](DeleteDealsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## duplicate_deal

> models::GetDuplicatedDealResponse duplicate_deal(id)
Duplicate deal

Duplicates a deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |

### Return type

[**models::GetDuplicatedDealResponse**](GetDuplicatedDealResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_archived_deals

> models::GetDealsResponse get_archived_deals(user_id, filter_id, person_id, org_id, product_id, pipeline_id, stage_id, status, start, limit, sort, owned_by_you)
Get all archived deals

Returns all archived deals.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**i32**> | If supplied, only deals matching the given user will be returned. However, `filter_id` and `owned_by_you` takes precedence over `user_id` when supplied. |  |
**filter_id** | Option<**i32**> | The ID of the filter to use |  |
**person_id** | Option<**i32**> | If supplied, only deals linked to the specified person are returned. If filter_id is provided, this is ignored. |  |
**org_id** | Option<**i32**> | If supplied, only deals linked to the specified organization are returned. If filter_id is provided, this is ignored. |  |
**product_id** | Option<**i32**> | If supplied, only deals linked to the specified product are returned. If filter_id is provided, this is ignored. |  |
**pipeline_id** | Option<**i32**> | If supplied, only deals in the specified pipeline are returned. If filter_id is provided, this is ignored. |  |
**stage_id** | Option<**i32**> | If supplied, only deals in the specified stage are returned. If filter_id is provided, this is ignored. |  |
**status** | Option<**String**> | Only fetch deals with a specific status. If omitted, all not deleted deals are returned. If set to deleted, deals that have been deleted up to 30 days ago will be included. |  |[default to all_not_deleted]
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**sort** | Option<**String**> | The field names and sorting mode separated by a comma (`field_name_1 ASC`, `field_name_2 DESC`). Only first-level field keys are supported (no nested keys). |  |
**owned_by_you** | Option<**f64**> | When supplied, only deals owned by you are returned. However, `filter_id` takes precedence over `owned_by_you` when both are supplied. |  |

### Return type

[**models::GetDealsResponse**](GetDealsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_archived_deals_summary

> models::GetDealsSummaryResponse get_archived_deals_summary(status, filter_id, user_id, pipeline_id, stage_id)
Get archived deals summary

Returns a summary of all archived deals.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | Only fetch deals with a specific status. open = Open, won = Won, lost = Lost. |  |
**filter_id** | Option<**i32**> | <code>user_id</code> will not be considered. Only deals matching the given filter will be returned. |  |
**user_id** | Option<**i32**> | Only deals matching the given user will be returned. `user_id` will not be considered if you use `filter_id`. |  |
**pipeline_id** | Option<**i32**> | Only deals within the given pipeline will be returned |  |
**stage_id** | Option<**i32**> | Only deals within the given stage will be returned |  |

### Return type

[**models::GetDealsSummaryResponse**](GetDealsSummaryResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_archived_deals_timeline

> models::GetDealsTimelineResponse get_archived_deals_timeline(start_date, interval, amount, field_key, user_id, pipeline_id, filter_id, exclude_deals, totals_convert_currency)
Get archived deals timeline

Returns archived open and won deals, grouped by a defined interval of time set in a date-type dealField (`field_key`) — e.g. when month is the chosen interval, and 3 months are asked starting from January 1st, 2012, deals are returned grouped into 3 groups — January, February and March — based on the value of the given `field_key`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | **String** | The date when the first interval starts. Format: YYYY-MM-DD. | [required] |
**interval** | **String** | The type of the interval<table><tr><th>Value</th><th>Description</th></tr><tr><td>`day`</td><td>Day</td></tr><tr><td>`week`</td><td>A full week (7 days) starting from `start_date`</td></tr><tr><td>`month`</td><td>A full month (depending on the number of days in given month) starting from `start_date`</td></tr><tr><td>`quarter`</td><td>A full quarter (3 months) starting from `start_date`</td></tr></table> | [required] |
**amount** | **i32** | The number of given intervals, starting from `start_date`, to fetch. E.g. 3 (months). | [required] |
**field_key** | **String** | The date field key which deals will be retrieved from | [required] |
**user_id** | Option<**i32**> | If supplied, only deals matching the given user will be returned |  |
**pipeline_id** | Option<**i32**> | If supplied, only deals matching the given pipeline will be returned |  |
**filter_id** | Option<**i32**> | If supplied, only deals matching the given filter will be returned |  |
**exclude_deals** | Option<**f64**> | Whether to exclude deals list (1) or not (0). Note that when deals are excluded, the timeline summary (counts and values) is still returned. |  |
**totals_convert_currency** | Option<**String**> | The 3-letter currency code of any of the supported currencies. When supplied, `totals_converted` is returned per each interval which contains the currency-converted total amounts in the given currency. You may also set this parameter to `default_currency` in which case the user's default currency is used. |  |

### Return type

[**models::GetDealsTimelineResponse**](GetDealsTimelineResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal

> models::GetDealResponse get_deal(id)
Get details of a deal

Returns the details of a specific deal. Note that this also returns some additional fields which are not present when asking for all deals – such as deal age and stay in pipeline stages. Also note that custom fields appear as long hashes in the resulting data. These hashes can be mapped against the `key` value of dealFields. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/getting-details-of-a-deal\" target=\"_blank\" rel=\"noopener noreferrer\">getting details of a deal</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |

### Return type

[**models::GetDealResponse**](GetDealResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal_activities

> models::GetDealActivitiesResponse get_deal_activities(id, start, limit, done, exclude)
List activities associated with a deal

Lists activities associated with a deal. <br>This endpoint has been deprecated. Please use <a href=\"https://developers.pipedrive.com/docs/api/v1/Activities#getActivities\" target=\"_blank\" rel=\"noopener noreferrer\">GET /api/v2/activities?deal_id={id}</a> instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**done** | Option<**f64**> | Whether the activity is done or not. 0 = Not done, 1 = Done. If omitted, returns both Done and Not done activities. |  |
**exclude** | Option<**String**> | A comma-separated string of activity IDs to exclude from result |  |

### Return type

[**models::GetDealActivitiesResponse**](GetDealActivitiesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal_changelog

> models::GetChangelogResponse get_deal_changelog(id, cursor, limit)
List updates about deal field values

Lists updates about field values of a deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
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


## get_deal_files

> models::GetAssociatedFilesResponse get_deal_files(id, start, limit, sort)
List files attached to a deal

Lists files associated with a deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
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


## get_deal_followers

> models::GetListFollowersResponse get_deal_followers(id)
List followers of a deal

Lists the followers of a deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |

### Return type

[**models::GetListFollowersResponse**](GetListFollowersResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal_mail_messages

> models::GetAssociatedMailMessagesResponse get_deal_mail_messages(id, start, limit)
List mail messages associated with a deal

Lists mail messages associated with a deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
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


## get_deal_participants

> models::GetDealParticipantsResponse get_deal_participants(id, start, limit)
List participants of a deal

Lists the participants associated with a deal.<br>If a company uses the [Campaigns product](https://pipedrive.readme.io/docs/campaigns-in-pipedrive-api), then this endpoint will also return the `data.marketing_status` field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**models::GetDealParticipantsResponse**](GetDealParticipantsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal_participants_changelog

> models::GetParticipantsChangelogResponse get_deal_participants_changelog(id, limit, cursor)
List updates about participants of a deal

List updates about participants of a deal. This is a cursor-paginated endpoint. For more information, please refer to our documentation on <a href=\"https://pipedrive.readme.io/docs/core-api-concepts-pagination\" target=\"_blank\" rel=\"noopener noreferrer\">pagination</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**limit** | Option<**i32**> | Items shown per page |  |
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |

### Return type

[**models::GetParticipantsChangelogResponse**](GetParticipantsChangelogResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal_persons

> models::GetPersonsResponse get_deal_persons(id, start, limit)
List all persons associated with a deal

Lists all persons associated with a deal, regardless of whether the person is the primary contact of the deal, or added as a participant.<br>If a company uses the [Campaigns product](https://pipedrive.readme.io/docs/campaigns-in-pipedrive-api), then this endpoint will also return the `data.marketing_status` field. <br>This endpoint has been deprecated. Please use <a href=\"https://developers.pipedrive.com/docs/api/v1/Persons#getPersons\" target=\"_blank\" rel=\"noopener noreferrer\">GET /api/v2/persons?deal_id={id}</a> instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**models::GetPersonsResponse**](GetPersonsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal_products

> models::GetProductsResponse get_deal_products(id, start, limit, include_product_data)
List products attached to a deal

Lists products attached to a deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**include_product_data** | Option<**f64**> | Whether to fetch product data along with each attached product (1) or not (0, default) |  |

### Return type

[**models::GetProductsResponse**](GetProductsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal_updates

> models::GetDealUpdatesResponse get_deal_updates(id, start, limit, all_changes, items)
List updates about a deal

Lists updates about a deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**all_changes** | Option<**String**> | Whether to show custom field updates or not. 1 = Include custom field changes. If omitted returns changes without custom field updates. |  |
**items** | Option<**String**> | A comma-separated string for filtering out item specific updates. (Possible values - call, activity, plannedActivity, change, note, deal, file, dealChange, personChange, organizationChange, follower, dealFollower, personFollower, organizationFollower, participant, comment, mailMessage, mailMessageWithAttachment, invoice, document, marketing_campaign_stat, marketing_status_change). |  |

### Return type

[**models::GetDealUpdatesResponse**](GetDealUpdatesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal_users

> models::GetPermittedUsersResponse get_deal_users(id)
List permitted users

Lists the users permitted to access a deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |

### Return type

[**models::GetPermittedUsersResponse**](GetPermittedUsersResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deals

> models::GetDealsResponse get_deals(user_id, filter_id, stage_id, status, start, limit, sort, owned_by_you)
Get all deals

Returns all not archived deals. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/getting-all-deals\" target=\"_blank\" rel=\"noopener noreferrer\">getting all deals</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**i32**> | If supplied, only deals matching the given user will be returned. However, `filter_id` and `owned_by_you` takes precedence over `user_id` when supplied. |  |
**filter_id** | Option<**i32**> | The ID of the filter to use |  |
**stage_id** | Option<**i32**> | If supplied, only deals within the given stage will be returned |  |
**status** | Option<**String**> | Only fetch deals with a specific status. If omitted, all not deleted deals are returned. If set to deleted, deals that have been deleted up to 30 days ago will be included. |  |[default to all_not_deleted]
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**sort** | Option<**String**> | The field names and sorting mode separated by a comma (`field_name_1 ASC`, `field_name_2 DESC`). Only first-level field keys are supported (no nested keys). |  |
**owned_by_you** | Option<**f64**> | When supplied, only deals owned by you are returned. However, `filter_id` takes precedence over `owned_by_you` when both are supplied. |  |

### Return type

[**models::GetDealsResponse**](GetDealsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deals_collection

> models::GetDealsCollectionResponse get_deals_collection(cursor, limit, since, until, user_id, stage_id, status)
Get all deals collection

Returns all deals. Please note that only global admins (those with global permissions) can access this endpoint. Users with regular permissions will receive a 403 response. Read more about global permissions <a href=\"https://support.pipedrive.com/en/article/global-user-management\" target=\"_blank\" rel=\"noopener noreferrer\">here</a>. <br>This endpoint has been deprecated. Please use <a href=\"https://developers.pipedrive.com/docs/api/v1/Deals#getDeals\" target=\"_blank\" rel=\"noopener noreferrer\">GET /api/v2/deals</a> instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**since** | Option<**String**> | The time boundary that points to the start of the range of data. Datetime in ISO 8601 format. E.g. 2022-11-01 08:55:59. Operates on the `update_time` field. |  |
**until** | Option<**String**> | The time boundary that points to the end of the range of data. Datetime in ISO 8601 format. E.g. 2022-11-01 08:55:59. Operates on the `update_time` field. |  |
**user_id** | Option<**i32**> | If supplied, only deals matching the given user will be returned |  |
**stage_id** | Option<**i32**> | If supplied, only deals within the given stage will be returned |  |
**status** | Option<**String**> | Only fetch deals with a specific status. If omitted, all not deleted deals are returned. If set to deleted, deals that have been deleted up to 30 days ago will be included. |  |

### Return type

[**models::GetDealsCollectionResponse**](GetDealsCollectionResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deals_summary

> models::GetDealsSummaryResponse get_deals_summary(status, filter_id, user_id, pipeline_id, stage_id)
Get deals summary

Returns a summary of all not archived deals.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | Only fetch deals with a specific status. open = Open, won = Won, lost = Lost. |  |
**filter_id** | Option<**i32**> | <code>user_id</code> will not be considered. Only deals matching the given filter will be returned. |  |
**user_id** | Option<**i32**> | Only deals matching the given user will be returned. `user_id` will not be considered if you use `filter_id`. |  |
**pipeline_id** | Option<**i32**> | Only deals within the given pipeline will be returned |  |
**stage_id** | Option<**i32**> | Only deals within the given stage will be returned |  |

### Return type

[**models::GetDealsSummaryResponse**](GetDealsSummaryResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deals_timeline

> models::GetDealsTimelineResponse get_deals_timeline(start_date, interval, amount, field_key, user_id, pipeline_id, filter_id, exclude_deals, totals_convert_currency)
Get deals timeline

Returns not archived open and won deals, grouped by a defined interval of time set in a date-type dealField (`field_key`) — e.g. when month is the chosen interval, and 3 months are asked starting from January 1st, 2012, deals are returned grouped into 3 groups — January, February and March — based on the value of the given `field_key`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | **String** | The date when the first interval starts. Format: YYYY-MM-DD. | [required] |
**interval** | **String** | The type of the interval<table><tr><th>Value</th><th>Description</th></tr><tr><td>`day`</td><td>Day</td></tr><tr><td>`week`</td><td>A full week (7 days) starting from `start_date`</td></tr><tr><td>`month`</td><td>A full month (depending on the number of days in given month) starting from `start_date`</td></tr><tr><td>`quarter`</td><td>A full quarter (3 months) starting from `start_date`</td></tr></table> | [required] |
**amount** | **i32** | The number of given intervals, starting from `start_date`, to fetch. E.g. 3 (months). | [required] |
**field_key** | **String** | The date field key which deals will be retrieved from | [required] |
**user_id** | Option<**i32**> | If supplied, only deals matching the given user will be returned |  |
**pipeline_id** | Option<**i32**> | If supplied, only deals matching the given pipeline will be returned |  |
**filter_id** | Option<**i32**> | If supplied, only deals matching the given filter will be returned |  |
**exclude_deals** | Option<**f64**> | Whether to exclude deals list (1) or not (0). Note that when deals are excluded, the timeline summary (counts and values) is still returned. |  |
**totals_convert_currency** | Option<**String**> | The 3-letter currency code of any of the supported currencies. When supplied, `totals_converted` is returned per each interval which contains the currency-converted total amounts in the given currency. You may also set this parameter to `default_currency` in which case the user's default currency is used. |  |

### Return type

[**models::GetDealsTimelineResponse**](GetDealsTimelineResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_deals

> models::GetMergedDealResponse merge_deals(id, merge_deals_request)
Merge two deals

Merges a deal with another deal. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/merging-two-deals\" target=\"_blank\" rel=\"noopener noreferrer\">merging two deals</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**merge_deals_request** | Option<[**MergeDealsRequest**](MergeDealsRequest.md)> |  |  |

### Return type

[**models::GetMergedDealResponse**](GetMergedDealResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_deals

> models::GetDealSearchResponse search_deals(term, fields, exact_match, person_id, organization_id, status, include_fields, start, limit)
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
**start** | Option<**i32**> | Pagination start. Note that the pagination is based on main results and does not include related items when using `search_for_related_items` parameter. |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**models::GetDealSearchResponse**](GetDealSearchResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_deal

> models::GetAddedDealResponse update_deal(id, update_deal_request)
Update a deal

Updates the properties of a deal. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/updating-a-deal\" target=\"_blank\" rel=\"noopener noreferrer\">updating a deal</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**update_deal_request** | Option<[**UpdateDealRequest**](UpdateDealRequest.md)> |  |  |

### Return type

[**models::GetAddedDealResponse**](GetAddedDealResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_deal_product

> models::GetProductAttachmentDetailsResponse update_deal_product(id, product_attachment_id, update_deal_product_request)
Update the product attached to a deal

Updates the details of the product that has been attached to a deal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the deal | [required] |
**product_attachment_id** | **i32** | The ID of the deal-product (the ID of the product attached to the deal) | [required] |
**update_deal_product_request** | Option<[**UpdateDealProductRequest**](UpdateDealProductRequest.md)> |  |  |

### Return type

[**models::GetProductAttachmentDetailsResponse**](GetProductAttachmentDetailsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

