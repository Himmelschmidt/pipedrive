# GetTokensResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token** | Option<**String**> | You need to use an `access_token` for accessing the user's data via API. You will need to [refresh the access token](https://pipedrive.readme.io/docs/marketplace-oauth-authorization#step-7-refreshing-the-tokens) if the `access_token` becomes invalid. | [optional]
**token_type** | Option<**String**> | The format of the token. Always \"Bearer\". | [optional]
**refresh_token** | Option<**String**> | A refresh token is needed when you refresh the access token. refresh_token will expire if it isn't used in 60 days. Each time refresh_token is used, its expiry date is reset back to 60 days. | [optional]
**scope** | Option<**String**> | List of scopes to which users have agreed to grant access within this `access_token` | [optional]
**expires_in** | Option<**i32**> | The maximum time in seconds until the `access_token` expires | [optional]
**api_domain** | Option<**String**> | The base URL path, including the company_domain, where the requests can be sent to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


