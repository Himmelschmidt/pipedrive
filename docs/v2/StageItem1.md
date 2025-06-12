# StageItem1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the stage | [optional]
**order_nr** | Option<**i32**> | Defines the order of the stage | [optional]
**name** | Option<**String**> | The name of the stage | [optional]
**is_deleted** | Option<**bool**> | Whether the stage is marked as deleted or not | [optional]
**deal_probability** | Option<**i32**> | The success probability percentage of the deal. Used/shown when the deal weighted values are used. | [optional]
**pipeline_id** | Option<**i32**> | The ID of the pipeline to add the stage to | [optional]
**is_deal_rot_enabled** | Option<**bool**> | Whether deals in this stage can become rotten | [optional]
**days_to_rotten** | Option<**i32**> | The number of days the deals not updated in this stage would become rotten. Applies only if the `is_deal_rot_enabled` is set. | [optional]
**add_time** | Option<**String**> | The stage creation time | [optional]
**update_time** | Option<**String**> | The stage update time | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


