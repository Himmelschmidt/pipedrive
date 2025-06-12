# ProductListItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the deal-product (the ID of the product attached to the deal) | [optional]
**deal_id** | Option<**i32**> | The ID of the deal | [optional]
**order_nr** | Option<**i32**> | The order number of the product | [optional]
**product_id** | Option<**i32**> | The ID of the product | [optional]
**product_variation_id** | Option<**i32**> | The ID of the product variation | [optional]
**item_price** | Option<**f64**> | The price value of the product | [optional]
**discount** | Option<**f64**> | The value of the discount. The `discount_type` field can be used to specify whether the value is an amount or a percentage | [optional][default to 0]
**discount_type** | Option<**String**> | The type of the discount's value | [optional][default to Percentage]
**sum** | Option<**f64**> | The sum of all the products attached to the deal | [optional]
**currency** | Option<**String**> | The currency associated with the deal product | [optional]
**enabled_flag** | Option<**bool**> | Whether the product is enabled or not | [optional]
**add_time** | Option<**String**> | The date and time when the product was added to the deal | [optional]
**last_edit** | Option<**String**> | The date and time when the deal product was last edited | [optional]
**comments** | Option<**String**> | The comments of the product | [optional]
**active_flag** | Option<**bool**> | Whether the product is active or not | [optional]
**tax** | Option<**f64**> | The product tax | [optional]
**tax_method** | Option<**String**> | The tax option to be applied to the products. When using `inclusive`, the tax percentage will already be included in the price. When using `exclusive`, the tax will not be included in the price. When using `none`, no tax will be added. Use the `tax` field for defining the tax percentage amount. By default, the user setting value for tax options will be used. Changing this in one product affects the rest of the products attached to the deal | [optional]
**name** | Option<**String**> | The product name | [optional]
**sum_formatted** | Option<**String**> | The formatted sum of the product | [optional]
**quantity_formatted** | Option<**String**> | The formatted quantity of the product | [optional]
**quantity** | Option<**i32**> | The quantity of the product | [optional]
**product** | Option<[**models::ProductListItemProduct**](ProductListItem_product.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


