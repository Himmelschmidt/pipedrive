# \GoalsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_goal**](GoalsApi.md#add_goal) | **POST** /goals | Add a new goal
[**delete_goal**](GoalsApi.md#delete_goal) | **DELETE** /goals/{id} | Delete existing goal
[**get_goal_result**](GoalsApi.md#get_goal_result) | **GET** /goals/{id}/results | Get result of a goal
[**get_goals**](GoalsApi.md#get_goals) | **GET** /goals/find | Find goals
[**update_goal**](GoalsApi.md#update_goal) | **PUT** /goals/{id} | Update existing goal



## add_goal

> models::UpsertGoalResponse add_goal(add_goal_request)
Add a new goal

Adds a new goal. Along with adding a new goal, a report is created to track the progress of your goal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_goal_request** | Option<[**AddGoalRequest**](AddGoalRequest.md)> |  |  |

### Return type

[**models::UpsertGoalResponse**](UpsertGoalResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_goal

> models::DeleteGoalResponse delete_goal(id)
Delete existing goal

Marks a goal as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the goal | [required] |

### Return type

[**models::DeleteGoalResponse**](DeleteGoalResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_goal_result

> models::GetGoalResultResponse get_goal_result(id, period_period_start, period_period_end)
Get result of a goal

Gets the progress of a goal for the specified period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the goal that the results are looked for | [required] |
**period_period_start** | **String** | The start date of the period for which to find the goal's progress. Format: YYYY-MM-DD. This date must be the same or after the goal duration start date.  | [required] |
**period_period_end** | **String** | The end date of the period for which to find the goal's progress. Format: YYYY-MM-DD. This date must be the same or before the goal duration end date.  | [required] |

### Return type

[**models::GetGoalResultResponse**](GetGoalResultResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_goals

> models::GetGoalsResponse get_goals(type_period_name, title, is_active, assignee_period_id, assignee_period_type, expected_outcome_period_target, expected_outcome_period_tracking_metric, expected_outcome_period_currency_id, type_period_params_period_pipeline_id, type_period_params_period_stage_id, type_period_params_period_activity_type_id, period_period_start, period_period_end)
Find goals

Returns data about goals based on criteria. For searching, append `{searchField}={searchValue}` to the URL, where `searchField` can be any one of the lowest-level fields in dot-notation (e.g. `type.params.pipeline_id`; `title`). `searchValue` should be the value you are looking for on that field. Additionally, `is_active=<true|false>` can be provided to search for only active/inactive goals. When providing `period.start`, `period.end` must also be provided and vice versa.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_period_name** | Option<**String**> | The type of the goal. If provided, everyone's goals will be returned. |  |
**title** | Option<**String**> | The title of the goal |  |
**is_active** | Option<**bool**> | Whether the goal is active or not |  |[default to true]
**assignee_period_id** | Option<**i32**> | The ID of the user who's goal to fetch. When omitted, only your goals will be returned. |  |
**assignee_period_type** | Option<**String**> | The type of the goal's assignee. If provided, everyone's goals will be returned. |  |
**expected_outcome_period_target** | Option<**f64**> | The numeric value of the outcome. If provided, everyone's goals will be returned. |  |
**expected_outcome_period_tracking_metric** | Option<**String**> | The tracking metric of the expected outcome of the goal. If provided, everyone's goals will be returned. |  |
**expected_outcome_period_currency_id** | Option<**i32**> | The numeric ID of the goal's currency. Only applicable to goals with `expected_outcome.tracking_metric` with value `sum`. If provided, everyone's goals will be returned. |  |
**type_period_params_period_pipeline_id** | Option<[**Vec<i32>**](i32.md)> | An array of pipeline IDs or `null` for all pipelines. If provided, everyone's goals will be returned. |  |
**type_period_params_period_stage_id** | Option<**i32**> | The ID of the stage. Applicable to only `deals_progressed` type of goals. If provided, everyone's goals will be returned. |  |
**type_period_params_period_activity_type_id** | Option<[**Vec<i32>**](i32.md)> | An array of IDs or `null` for all activity types. Only applicable for `activities_completed` and/or `activities_added` types of goals. If provided, everyone's goals will be returned. |  |
**period_period_start** | Option<**String**> | The start date of the period for which to find goals. Date in format of YYYY-MM-DD. When `period.start` is provided, `period.end` must be provided too. |  |
**period_period_end** | Option<**String**> | The end date of the period for which to find goals. Date in format of YYYY-MM-DD. |  |

### Return type

[**models::GetGoalsResponse**](GetGoalsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_goal

> models::UpsertGoalResponse update_goal(id, basic_goal_request)
Update existing goal

Updates an existing goal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the goal | [required] |
**basic_goal_request** | Option<[**BasicGoalRequest**](BasicGoalRequest.md)> |  |  |

### Return type

[**models::UpsertGoalResponse**](UpsertGoalResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

