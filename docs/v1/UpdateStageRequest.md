# UpdateStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the stage | [optional]
**pipeline_id** | Option<**i32**> | The ID of the pipeline to add stage to | [optional]
**deal_probability** | Option<**i32**> | The success probability percentage of the deal. Used/shown when deal weighted values are used. | [optional]
**rotten_flag** | Option<**bool**> | Whether deals in this stage can become rotten | [optional]
**rotten_days** | Option<**i32**> | The number of days the deals not updated in this stage would become rotten. Applies only if the `rotten_flag` is set. | [optional]
**order_nr** | Option<**i32**> | An order number for this stage. Order numbers should be used to order the stages in the pipeline. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


