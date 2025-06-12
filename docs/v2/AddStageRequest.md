# AddStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the stage | 
**pipeline_id** | **i32** | The ID of the pipeline to add stage to | 
**deal_probability** | Option<**i32**> | The success probability percentage of the deal. Used/shown when deal weighted values are used. | [optional]
**is_deal_rot_enabled** | Option<**bool**> | Whether deals in this stage can become rotten | [optional]
**days_to_rotten** | Option<**i32**> | The number of days the deals not updated in this stage would become rotten. Applies only if the `is_deal_rot_enabled` is set. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


