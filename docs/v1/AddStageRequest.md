# AddStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the stage | 
**pipeline_id** | **i32** | The ID of the pipeline to add stage to | 
**deal_probability** | Option<**i32**> | The success probability percentage of the deal. Used/shown when deal weighted values are used. | [optional]
**rotten_flag** | Option<**bool**> | Whether deals in this stage can become rotten | [optional]
**rotten_days** | Option<**i32**> | The number of days the deals not updated in this stage would become rotten. Applies only if the `rotten_flag` is set. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


