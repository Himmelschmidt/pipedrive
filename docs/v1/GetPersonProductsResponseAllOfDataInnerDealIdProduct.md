# GetPersonProductsResponseAllOfDataInnerDealIdProduct

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the product | [optional]
**company_id** | Option<**i32**> | The ID of the company | [optional]
**name** | Option<**String**> | The name of the product | [optional]
**code** | Option<**String**> | The product code | [optional]
**description** | Option<**String**> | The description of the product | [optional]
**unit** | Option<**String**> | The unit in which this product is sold | [optional]
**tax** | Option<**f64**> | The tax percentage | [optional][default to 0]
**category** | Option<**String**> | The category of the product | [optional]
**active_flag** | Option<**bool**> | Whether this product will be made active or not | [optional][default to true]
**selectable** | Option<**bool**> | Whether this product can be selected in deals or not | [optional][default to true]
**first_char** | Option<**String**> | The first letter of the product name | [optional]
**visible_to** | Option<**String**> |  | [optional]
**owner_id** | Option<**i32**> | The ID of the user who will be marked as the owner of this product. When omitted, the authorized user ID will be used | [optional]
**files_count** | Option<**i32**> | The count of files | [optional]
**add_time** | Option<**String**> | The date and time when the product was added to the deal | [optional]
**update_time** | Option<**String**> | The date and time when the product was updated to the deal | [optional]
**deal_id** | Option<**i32**> | The ID of the deal | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


