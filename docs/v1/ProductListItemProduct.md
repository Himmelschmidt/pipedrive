# ProductListItemProduct

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**f64**> | The ID of the product | [optional]
**name** | Option<**String**> | The name of the product | [optional]
**code** | Option<**String**> | The product code | [optional]
**unit** | Option<**String**> | The unit in which this product is sold | [optional]
**tax** | Option<**f64**> | The tax percentage | [optional][default to 0]
**active_flag** | Option<**bool**> | Whether this product is active or not | [optional][default to true]
**selectable** | Option<**bool**> | Whether this product is selected in deals or not | [optional][default to true]
**visible_to** | Option<**String**> |  | [optional]
**owner_id** | Option<[**serde_json::Value**](.md)> | Information about the Pipedrive user who owns the product | [optional]
**billing_frequency** | Option<**String**> | Only available in Advanced and above plans  How often a customer is billed for access to a service or product  | [optional][default to OneTime]
**billing_frequency_cycles** | Option<**i32**> | Only available in Advanced and above plans  The number of times the billing frequency repeats for a product in a deal  When `billing_frequency` is set to `one-time`, this field is always `null`  When `billing_frequency` is set to `weekly`, this field cannot be `null`  For all the other values of `billing_frequency`, `null` represents a product billed indefinitely  Must be a positive integer less or equal to 208  | [optional]
**prices** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Array of objects, each containing: currency (string), price (number), cost (number, optional), overhead_cost (number, optional) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


