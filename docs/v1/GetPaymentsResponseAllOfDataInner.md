# GetPaymentsResponseAllOfDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the payment | [optional]
**subscription_id** | Option<**i32**> | The ID of the subscription this payment is associated with | [optional]
**deal_id** | Option<**i32**> | The ID of the deal this payment is associated with | [optional]
**is_active** | Option<**bool**> | The payment status | [optional]
**amount** | Option<**f64**> | The payment amount | [optional]
**currency** | Option<**String**> | The currency of the payment | [optional]
**change_amount** | Option<**f64**> | The difference between the amount of the current payment and the previous payment. The value can be either positive or negative. | [optional]
**due_at** | Option<[**String**](string.md)> | The date when payment occurs | [optional]
**revenue_movement_type** | Option<**String**> | Represents the movement of revenue in comparison with the previous payment. Possible values are: `New` - first payment of the subscription. `Recurring` - no movement. `Expansion` - current payment amount > previous payment amount. `Contraction` - current payment amount < previous payment amount. `Churn` - last payment of the subscription. | [optional]
**payment_type** | Option<**String**> | The type of the payment. Possible values are: `Recurring` - payments occur over fixed intervals of time, `Additional` - extra payment not the recurring payment of the recurring subscription, `Installment` - payment of the installment subscription. | [optional]
**description** | Option<**String**> | The description of the payment | [optional]
**add_time** | Option<**String**> | The creation time of the payment | [optional]
**update_time** | Option<**String**> | The update time of the payment | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


