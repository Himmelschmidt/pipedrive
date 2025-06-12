# DealItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the deal | [optional]
**title** | Option<**String**> | The title of the deal | [optional]
**owner_id** | Option<**i32**> | The ID of the user who owns the deal | [optional]
**person_id** | Option<**i32**> | The ID of the person linked to the deal | [optional]
**org_id** | Option<**i32**> | The ID of the organization linked to the deal | [optional]
**pipeline_id** | Option<**i32**> | The ID of the pipeline associated with the deal | [optional]
**stage_id** | Option<**i32**> | The ID of the deal stage | [optional]
**value** | Option<**f64**> | The value of the deal | [optional]
**currency** | Option<**String**> | The currency associated with the deal | [optional]
**add_time** | Option<**String**> | The creation date and time of the deal | [optional]
**update_time** | Option<**String**> | The last updated date and time of the deal | [optional]
**stage_change_time** | Option<**String**> | The last updated date and time of the deal stage | [optional]
**is_deleted** | Option<**bool**> | Whether the deal is deleted or not | [optional]
**is_archived** | Option<**bool**> | Whether the deal is archived or not | [optional]
**status** | Option<**String**> | The status of the deal | [optional]
**probability** | Option<**f64**> | The success probability percentage of the deal | [optional]
**lost_reason** | Option<**String**> | The reason for losing the deal | [optional]
**visible_to** | Option<**i32**> | The visibility of the deal | [optional]
**close_time** | Option<**String**> | The date and time of closing the deal | [optional]
**won_time** | Option<**String**> | The date and time of changing the deal status as won | [optional]
**lost_time** | Option<**String**> | The date and time of changing the deal status as lost | [optional]
**expected_close_date** | Option<[**String**](string.md)> | The expected close date of the deal | [optional]
**label_ids** | Option<**Vec<i32>**> | The IDs of labels assigned to the deal | [optional]
**origin** | Option<**String**> | The way this Deal was created. `origin` field is set by Pipedrive when Deal is created and cannot be changed. | [optional]
**origin_id** | Option<**String**> | The optional ID to further distinguish the origin of the deal - e.g. Which API integration created this Deal. | [optional]
**channel** | Option<**i32**> | The ID of your Marketing channel this Deal was created from. Recognized Marketing channels can be configured in your <a href=\"https://app.pipedrive.com/settings/fields\" target=\"_blank\" rel=\"noopener noreferrer\">Company settings</a>. | [optional]
**channel_id** | Option<**String**> | The optional ID to further distinguish the Marketing channel. | [optional]
**arr** | Option<**f64**> | Only available in Advanced and above plans  The Annual Recurring Revenue of the deal  Null if there are no products attached to the deal  | [optional]
**mrr** | Option<**f64**> | Only available in Advanced and above plans  The Monthly Recurring Revenue of the deal  Null if there are no products attached to the deal  | [optional]
**acv** | Option<**f64**> | Only available in Advanced and above plans  The Annual Contract Value of the deal  Null if there are no products attached to the deal  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


