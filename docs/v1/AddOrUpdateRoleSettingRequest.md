# AddOrUpdateRoleSettingRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**setting_key** | **String** |  | 
**value** | **i32** | Possible values for the `default_visibility` setting depending on the subscription plan:<br> <table class='role-setting'> <caption><b>Essential / Advanced plan</b></caption> <tr><th><b>Value</b></th><th><b>Description</b></th></tr> <tr><td>`1`</td><td>Owner & Followers</td></tr> <tr><td>`3`</td><td>Entire company</td></tr> </table> <br> <table class='role-setting'> <caption><b>Professional / Enterprise plan</b></caption> <tr><th><b>Value</b></th><th><b>Description</b></th></tr> <tr><td>`1`</td><td>Owner only</td></tr> <tr><td>`3`</td><td>Owner&#39;s visibility group</td></tr> <tr><td>`5`</td><td>Owner&#39;s visibility group and sub-groups</td></tr> <tr><td>`7`</td><td>Entire company</td></tr> </table> <br> Read more about visibility groups <a href='https://support.pipedrive.com/en/article/visibility-groups'>here</a>. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


