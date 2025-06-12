# GetProductResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**f64**> | The ID of the product | [optional]
**name** | Option<**String**> | The name of the product | [optional]
**code** | Option<**String**> | The product code | [optional]
**unit** | Option<**String**> | The unit in which this product is sold | [optional]
**tax** | Option<**f64**> | The tax percentage | [optional][default to 0]
**is_deleted** | Option<**bool**> | Whether this product will be made marked as deleted or not | [optional][default to false]
**is_linkable** | Option<**bool**> | Whether this product can be added to a deal or not | [optional][default to true]
**visible_to** | Option<**f64**> |  | [optional]
**owner_id** | Option<**i32**> | Information about the Pipedrive user who owns the product | [optional]
**custom_fields** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | An object where each key represents a custom field. All custom fields are referenced as randomly generated 40-character hashes | [optional]
**billing_frequency** | Option<**String**> | Only available in Advanced and above plans  How often a customer is billed for access to a service or product  | [optional][default to OneTime]
**billing_frequency_cycles** | Option<**i32**> | Only available in Advanced and above plans  The number of times the billing frequency repeats for a product in a deal  When `billing_frequency` is set to `one-time`, this field must be `null`  When `billing_frequency` is set to `weekly`, this field cannot be `null`  For all the other values of `billing_frequency`, `null` represents a product billed indefinitely  Must be a positive integer less or equal to 208  | [optional]
**prices** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Array of objects, each containing: product_id (number), currency (string), price (number), cost (number), direct_cost (number | null), notes (string) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


