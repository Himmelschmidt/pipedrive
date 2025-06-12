# SubscriptionItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the subscription | [optional]
**user_id** | Option<**i32**> | The ID of the user who created the subscription | [optional]
**deal_id** | Option<**i32**> | The ID of the deal this subscription is associated with | [optional]
**description** | Option<**String**> | The description of the recurring subscription | [optional]
**is_active** | Option<**bool**> | The subscription status | [optional]
**cycles_count** | Option<**i32**> | Shows how many payments a recurring subscription has | [optional]
**cycle_amount** | Option<**i32**> | The amount of each payment | [optional]
**infinite** | Option<**bool**> | Indicates that the recurring subscription will last until it is manually canceled or deleted | [optional]
**currency** | Option<**String**> | The currency of the subscription | [optional]
**cadence_type** | Option<**String**> | The interval between payments | [optional]
**start_date** | Option<[**String**](string.md)> | The start date of the recurring subscription | [optional]
**end_date** | Option<[**String**](string.md)> | The end date of the subscription | [optional]
**lifetime_value** | Option<**f64**> | The total value of all payments | [optional]
**final_status** | Option<**String**> | The final status of the subscription | [optional]
**add_time** | Option<**String**> | The creation time of the subscription | [optional]
**update_time** | Option<**String**> | The update time of the subscription | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


