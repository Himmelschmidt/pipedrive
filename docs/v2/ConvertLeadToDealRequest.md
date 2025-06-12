# ConvertLeadToDealRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**stage_id** | Option<**i32**> | The ID of a stage the created deal will be added to. Please note that a pipeline will be assigned automatically based on the `stage_id`. If omitted, the deal will be placed in the first stage of the default pipeline. | [optional]
**pipeline_id** | Option<**i32**> | The ID of a pipeline the created deal will be added to. By default, the deal will be added to the first stage of the specified pipeline. Please note that `pipeline_id` and `stage_id` should not be used together as `pipeline_id` will be ignored. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


