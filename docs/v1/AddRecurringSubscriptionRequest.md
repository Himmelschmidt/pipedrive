# AddRecurringSubscriptionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deal_id** | **i32** | The ID of the deal this recurring subscription is associated with | 
**currency** | **String** | The currency of the recurring subscription. Accepts a 3-character currency code. | 
**description** | Option<**String**> | The description of the recurring subscription | [optional]
**cadence_type** | **String** | The interval between payments | 
**cycles_count** | Option<**i32**> | Shows how many payments the subscription has. Note that one field must be set: `cycles_count` or `infinite`. If `cycles_count` is set, then `cycle_amount` and `start_date` are also required. | [optional]
**cycle_amount** | **i32** | The amount of each payment | 
**start_date** | [**String**](string.md) | The start date of the recurring subscription. Format: YYYY-MM-DD | 
**infinite** | Option<**bool**> | This indicates that the recurring subscription will last until it's manually canceled or deleted. Note that only one field must be set: `cycles_count` or `infinite`. | [optional]
**payments** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Array of additional payments. It requires a minimum structure as follows: [{ amount:SUM, description:DESCRIPTION, due_at:PAYMENT_DATE }]. Replace SUM with a payment amount, DESCRIPTION with an explanation string, PAYMENT_DATE with a date (format YYYY-MM-DD). | [optional]
**update_deal_value** | Option<**bool**> | Indicates that the deal value must be set to recurring subscription's MRR value | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


