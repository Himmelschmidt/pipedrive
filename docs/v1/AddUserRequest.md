# AddUserRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **String** | The email of the user | 
**access** | Option<[**Vec<models::UserAccess1>**](UserAccess_1.md)> | The access given to the user. Each item in the array represents access to a specific app. Optionally may include either admin flag or permission set ID to specify which access to give within the app. If both are omitted, the default access for the corresponding app will be used. It requires structure as follows: `[{ app: 'sales', permission_set_id: '62cc4d7f-4038-4352-abf3-a8c1c822b631' }, { app: 'global', admin: true }, { app: 'account_settings' }]`  | [optional][default to [{"app":"sales"}]]
**active_flag** | Option<**bool**> | Whether the user is active or not. `false` = Not activated, `true` = Activated | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


