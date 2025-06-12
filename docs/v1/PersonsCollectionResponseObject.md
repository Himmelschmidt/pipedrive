# PersonsCollectionResponseObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the person | [optional]
**active_flag** | Option<**bool**> | Whether the person is active or not | [optional]
**owner_id** | Option<**i32**> | The ID of the owner related to the person | [optional]
**org_id** | Option<**i32**> | The ID of the organization related to the person | [optional]
**name** | Option<**String**> | The name of the person | [optional]
**email** | Option<[**Vec<models::BasicPersonRequestEmailInner>**](basicPersonRequest_email_inner.md)> | An email address as a string or an array of email objects related to the person. The structure of the array is as follows: `[{ \"value\": \"mail@example.com\", \"primary\": \"true\", \"label\": \"main\" }]`. Please note that only `value` is required. | [optional]
**phone** | Option<[**Vec<models::PersonAllOfPhoneInner>**](Person_allOf_phone_inner.md)> | A phone number supplied as a string or an array of phone objects related to the person. The structure of the array is as follows: `[{ \"value\": \"12345\", \"primary\": \"true\", \"label\": \"mobile\" }]`. Please note that only `value` is required. | [optional]
**update_time** | Option<**String**> | The last updated date and time of the person. Format: YYYY-MM-DD HH:MM:SS | [optional]
**delete_time** | Option<**String**> | The date and time this person was deleted. Format: YYYY-MM-DD HH:MM:SS | [optional]
**add_time** | Option<**String**> | The date and time when the person was added/created. Format: YYYY-MM-DD HH:MM:SS | [optional]
**visible_to** | Option<**String**> | The visibility group ID of who can see the person | [optional]
**picture_id** | Option<**i32**> | The ID of the picture associated with the item | [optional]
**label** | Option<**i32**> | The label assigned to the person. When the `label` field is updated, the `label_ids` field value will be overwritten by the `label` field value. | [optional]
**label_ids** | Option<**Vec<i32>**> | The IDs of labels assigned to the person. When the `label_ids` field is updated, the `label` field value will be set to the first value of the `label_ids` field. | [optional]
**cc_email** | Option<**String**> | The BCC email associated with the person | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


