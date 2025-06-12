# RecentsDataInnerAnyOf9Data

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the product | [optional]
**name** | Option<**String**> | The name of the product | [optional]
**code** | Option<**String**> | The product code | [optional]
**description** | Option<**String**> | The description of the product | [optional]
**unit** | Option<**String**> | The unit in which this product is sold | [optional]
**tax** | Option<**f64**> | The tax percentage | [optional][default to 0]
**category** | Option<**String**> | The category of the product | [optional]
**active_flag** | Option<**bool**> | Whether this product will be made active or not | [optional]
**selectable** | Option<**bool**> | Whether this product can be selected in deals or not | [optional]
**first_char** | Option<**String**> | The first letter of the product name | [optional]
**visible_to** | Option<**i32**> | The visibility of the product. If omitted, the visibility will be set to the default visibility setting of this item type for the authorized user. | [optional]
**owner_id** | Option<**i32**> | The ID of the user who will be marked as the owner of this product. When omitted, authorized user ID will be used. | [optional]
**files_count** | Option<**i32**> | The count of files | [optional]
**add_time** | Option<**String**> | The date and time when the product was added to the deal | [optional]
**update_time** | Option<**String**> | The date and time when the product was updated to the deal | [optional]
**prices** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Array of objects, each containing: `currency` (string), `price` (number), `cost` (number, optional), `overhead_cost` (number, optional). Note that there can only be one price per product per currency. When `prices` is omitted altogether, a default price of 0 and a default currency based on the company's currency will be assigned. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


