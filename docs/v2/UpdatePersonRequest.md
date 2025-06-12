# UpdatePersonRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the person | [optional]
**owner_id** | Option<**i32**> | The ID of the user who owns the person | [optional]
**org_id** | Option<**i32**> | The ID of the organization linked to the person | [optional]
**add_time** | Option<**String**> | The creation date and time of the person | [optional]
**update_time** | Option<**String**> | The last updated date and time of the person | [optional]
**emails** | Option<[**Vec<models::AddPersonRequestEmailsInner>**](addPerson_request_emails_inner.md)> | The emails of the person | [optional]
**phones** | Option<[**Vec<models::AddPersonRequestPhonesInner>**](addPerson_request_phones_inner.md)> | The phones of the person | [optional]
**visible_to** | Option<**i32**> | The visibility of the person | [optional]
**label_ids** | Option<**Vec<i32>**> | The IDs of labels assigned to the person | [optional]
**marketing_status** | Option<**String**> | If the person does not have a valid email address, then the marketing status is **not set** and `no_consent` is returned for the `marketing_status` value when the new person is created. If the change is forbidden, the status will remain unchanged for every call that tries to modify the marketing status. Please be aware that it is only allowed **once** to change the marketing status from an old status to a new one.<table><tr><th>Value</th><th>Description</th></tr><tr><td>`no_consent`</td><td>The customer has not given consent to receive any marketing communications</td></tr><tr><td>`unsubscribed`</td><td>The customers have unsubscribed from ALL marketing communications</td></tr><tr><td>`subscribed`</td><td>The customers are subscribed and are counted towards marketing caps</td></tr><tr><td>`archived`</td><td>The customers with `subscribed` status can be moved to `archived` to save consent, but they are not paid for</td></tr></table> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


