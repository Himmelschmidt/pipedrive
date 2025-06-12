# UpdateOrganizationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the organization | [optional]
**owner_id** | Option<**i32**> | The ID of the user who will be marked as the owner of this organization. When omitted, the authorized user ID will be used. | [optional]
**label** | Option<**i32**> | The label assigned to the organization. When the `label` field is updated, the `label_ids` field value will be overwritten by the `label` field value. | [optional]
**label_ids** | Option<**Vec<i32>**> | The IDs of labels assigned to the organization. When the `label_ids` field is updated, the `label` field value will be set to the first value of the `label_ids` field. | [optional]
**visible_to** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


