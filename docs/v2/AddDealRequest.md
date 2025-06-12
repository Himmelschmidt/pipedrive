# AddDealRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** | The title of the deal | 
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
**archive_time** | Option<**String**> | The optional date and time of archiving the deal in UTC. Format: YYYY-MM-DD HH:MM:SS. If omitted and `is_archived` is true, it will be set to the current date and time. | [optional]
**status** | Option<**String**> | The status of the deal | [optional]
**probability** | Option<**f64**> | The success probability percentage of the deal | [optional]
**lost_reason** | Option<**String**> | The reason for losing the deal. Can only be set if deal status is lost. | [optional]
**visible_to** | Option<**i32**> | The visibility of the deal | [optional]
**close_time** | Option<**String**> | The date and time of closing the deal. Can only be set if deal status is won or lost. | [optional]
**won_time** | Option<**String**> | The date and time of changing the deal status as won. Can only be set if deal status is won. | [optional]
**lost_time** | Option<**String**> | The date and time of changing the deal status as lost. Can only be set if deal status is lost. | [optional]
**expected_close_date** | Option<[**String**](string.md)> | The expected close date of the deal | [optional]
**label_ids** | Option<**Vec<i32>**> | The IDs of labels assigned to the deal | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


