# GetStageResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the stage | [optional]
**order_nr** | Option<**i32**> | Defines the order of the stage | [optional]
**name** | Option<**String**> | The name of the stage | [optional]
**active_flag** | Option<**bool**> | Whether the stage is active or deleted | [optional]
**deal_probability** | Option<**i32**> | The success probability percentage of the deal. Used/shown when the deal weighted values are used. | [optional]
**pipeline_id** | Option<**i32**> | The ID of the pipeline to add the stage to | [optional]
**rotten_flag** | Option<**bool**> | Whether deals in this stage can become rotten | [optional]
**rotten_days** | Option<**i32**> | The number of days the deals not updated in this stage would become rotten. Applies only if the `rotten_flag` is set. | [optional]
**add_time** | Option<**String**> | The stage creation time. Format: YYYY-MM-DD HH:MM:SS. | [optional]
**update_time** | Option<**String**> | The stage update time. Format: YYYY-MM-DD HH:MM:SS. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


