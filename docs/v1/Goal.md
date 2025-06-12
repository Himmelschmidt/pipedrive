# Goal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the goal | [optional]
**owner_id** | Option<**i32**> | The ID of the creator of the goal | [optional]
**title** | Option<**String**> | The title of the goal | [optional]
**r#type** | Option<[**models::GoalType**](Goal_type.md)> |  | [optional]
**assignee** | Option<[**models::GoalAssignee**](Goal_assignee.md)> |  | [optional]
**interval** | Option<**String**> | The interval of the goal | [optional]
**duration** | Option<[**models::GoalDuration**](Goal_duration.md)> |  | [optional]
**expected_outcome** | Option<[**models::GoalExpectedOutcome**](Goal_expected_outcome.md)> |  | [optional]
**is_active** | Option<**bool**> | Whether the goal is currently active or not | [optional]
**report_ids** | Option<**Vec<String>**> | The IDs of the reports that belong to the goal | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


