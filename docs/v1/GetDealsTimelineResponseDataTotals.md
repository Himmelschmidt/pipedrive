# GetDealsTimelineResponseDataTotals

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | Option<**i32**> | The number of deals for the given period | [optional]
**values** | Option<[**serde_json::Value**](.md)> | The total values of deals grouped by deal currency | [optional]
**weighted_values** | Option<[**serde_json::Value**](.md)> | The total weighted values of deals for the given period grouped by deal currency. The weighted value of a deal is calculated as probability times deal value. | [optional]
**open_count** | Option<**i32**> | The number of open deals for the given period | [optional]
**open_values** | Option<[**serde_json::Value**](.md)> | The total values of open deals for the given period grouped by deal currency | [optional]
**weighted_open_values** | Option<[**serde_json::Value**](.md)> | The total weighted values of open deals for the given period grouped by deal currency. The weighted value of a deal is calculated as probability times deal value. | [optional]
**won_count** | Option<**i32**> | The number of won deals for the given period | [optional]
**won_values** | Option<[**serde_json::Value**](.md)> | The total values of won deals for the given period grouped by deal currency | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


