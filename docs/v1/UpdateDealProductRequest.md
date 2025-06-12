# UpdateDealProductRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product_id** | Option<**i32**> | The ID of the product to use | [optional]
**item_price** | Option<**f64**> | The price at which this product will be added to the deal | [optional]
**quantity** | Option<**i32**> | How many items of this product will be added to the deal | [optional]
**discount** | Option<**f64**> | The value of the discount. The `discount_type` field can be used to specify whether the value is an amount or a percentage | [optional][default to 0]
**discount_type** | Option<**String**> | The type of the discount's value | [optional][default to Percentage]
**product_variation_id** | Option<**i32**> | The ID of the product variation to use. When omitted, no variation will be used | [optional]
**comments** | Option<**String**> | A textual comment associated with this product-deal attachment | [optional]
**tax** | Option<**f64**> | The tax percentage | [optional][default to 0]
**tax_method** | Option<**String**> | The tax option to be applied to the products. When using `inclusive`, the tax percentage will already be included in the price. When using `exclusive`, the tax will not be included in the price. When using `none`, no tax will be added. Use the `tax` field for defining the tax percentage amount | [optional]
**enabled_flag** | Option<**bool**> | Whether the product is enabled for a deal or not  This makes it possible to add products to a deal with a specific price and discount criteria, but keep them disabled, which refrains them from being included in the deal value calculation. When omitted, the product will be marked as enabled by default  Not possible to disable the product if the deal has installments associated and the product is the last one enabled  Not possible to enable the product if the deal has installments associated and the product is recurring  | [optional][default to true]
**billing_frequency** | Option<**String**> | Only available in Advanced and above plans  How often a customer is billed for access to a service or product  To set `billing_frequency` different than `one-time`, the deal must not have installments associated  A deal can have up to 20 products attached with `billing_frequency` different than `one-time`  | [optional]
**billing_frequency_cycles** | Option<**i32**> | Only available in Advanced and above plans  The number of times the billing frequency repeats for a product in a deal  When `billing_frequency` is set to `one-time`, this field must be `null`  When `billing_frequency` is set to `weekly`, this field cannot be `null`  For all the other values of `billing_frequency`, `null` represents a product billed indefinitely  Must be a positive integer less or equal to 208  | [optional]
**billing_start_date** | Option<**String**> | Only available in Advanced and above plans  The billing start date. Must be between 10 years in the past and 10 years in the future  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


