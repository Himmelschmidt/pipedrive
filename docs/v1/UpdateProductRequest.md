# UpdateProductRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the product | [optional]
**code** | Option<**String**> | The product code | [optional]
**description** | Option<**String**> | The product description | [optional]
**unit** | Option<**String**> | The unit in which this product is sold | [optional]
**tax** | Option<**f64**> | The tax percentage | [optional][default to 0]
**active_flag** | Option<**bool**> | Whether this product will be made active or not | [optional][default to true]
**selectable** | Option<**bool**> | Whether this product can be selected in deals or not | [optional][default to true]
**visible_to** | Option<**String**> |  | [optional]
**owner_id** | Option<**i32**> | The ID of the user who will be marked as the owner of this product. When omitted, the authorized user ID will be used | [optional]
**prices** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | An array of objects, each containing: `currency` (string), `price` (number), `cost` (number, optional), `overhead_cost` (number, optional), `notes` (string, optional). Note that there can only be one price per product per currency. When `prices` is omitted altogether, a default price of 0 and a default currency based on the company's currency will be assigned. | [optional]
**billing_frequency** | Option<**String**> | Only available in Advanced and above plans  How often a customer is billed for access to a service or product  | [optional]
**billing_frequency_cycles** | Option<**i32**> | Only available in Advanced and above plans  The number of times the billing frequency repeats for a product in a deal  When `billing_frequency` is set to `one-time`, this field must be `null`  For all the other values of `billing_frequency`, `null` represents a product billed indefinitely  Must be a positive integer less or equal to 208  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


