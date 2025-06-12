# \LegacyTeamsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_team**](LegacyTeamsApi.md#add_team) | **POST** /legacyTeams | Add a new team
[**add_team_user**](LegacyTeamsApi.md#add_team_user) | **POST** /legacyTeams/{id}/users | Add users to a team
[**delete_team_user**](LegacyTeamsApi.md#delete_team_user) | **DELETE** /legacyTeams/{id}/users | Delete users from a team
[**get_team**](LegacyTeamsApi.md#get_team) | **GET** /legacyTeams/{id} | Get a single team
[**get_team_users**](LegacyTeamsApi.md#get_team_users) | **GET** /legacyTeams/{id}/users | Get all users in a team
[**get_teams**](LegacyTeamsApi.md#get_teams) | **GET** /legacyTeams | Get all teams
[**get_user_teams**](LegacyTeamsApi.md#get_user_teams) | **GET** /legacyTeams/user/{id} | Get all teams of a user
[**update_team**](LegacyTeamsApi.md#update_team) | **PUT** /legacyTeams/{id} | Update a team



## add_team

> models::GetTeamResponse add_team(add_team_request)
Add a new team

Adds a new team to the company and returns the created object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_team_request** | Option<[**AddTeamRequest**](AddTeamRequest.md)> |  |  |

### Return type

[**models::GetTeamResponse**](GetTeamResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_team_user

> models::UserIds add_team_user(id, add_team_user_request)
Add users to a team

Adds users to an existing team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the team | [required] |
**add_team_user_request** | Option<[**AddTeamUserRequest**](AddTeamUserRequest.md)> |  |  |

### Return type

[**models::UserIds**](userIds.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_team_user

> models::UserIds delete_team_user(id, delete_team_user_request)
Delete users from a team

Deletes users from an existing team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the team | [required] |
**delete_team_user_request** | Option<[**DeleteTeamUserRequest**](DeleteTeamUserRequest.md)> |  |  |

### Return type

[**models::UserIds**](userIds.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team

> models::GetTeamResponse get_team(id, skip_users)
Get a single team

Returns data about a specific team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the team | [required] |
**skip_users** | Option<**f64**> | When enabled, the teams will not include IDs of member users |  |[default to 0]

### Return type

[**models::GetTeamResponse**](GetTeamResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_users

> models::UserIds get_team_users(id)
Get all users in a team

Returns a list of all user IDs within a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the team | [required] |

### Return type

[**models::UserIds**](userIds.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams

> models::GetTeamsResponse get_teams(order_by, skip_users)
Get all teams

Returns data about teams within the company.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_by** | Option<**String**> | The field name to sort returned teams by |  |[default to id]
**skip_users** | Option<**f64**> | When enabled, the teams will not include IDs of member users |  |[default to 0]

### Return type

[**models::GetTeamsResponse**](GetTeamsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_teams

> models::GetTeamsResponse get_user_teams(id, order_by, skip_users)
Get all teams of a user

Returns data about all teams which have the specified user as a member.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the user | [required] |
**order_by** | Option<**String**> | The field name to sort returned teams by |  |[default to id]
**skip_users** | Option<**f64**> | When enabled, the teams will not include IDs of member users |  |[default to 0]

### Return type

[**models::GetTeamsResponse**](GetTeamsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_team

> models::GetTeamResponse update_team(id, update_team_request)
Update a team

Updates an existing team and returns the updated object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the team | [required] |
**update_team_request** | Option<[**UpdateTeamRequest**](UpdateTeamRequest.md)> |  |  |

### Return type

[**models::GetTeamResponse**](GetTeamResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

