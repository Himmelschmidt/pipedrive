# UpdateSubscriptionInstallmentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payments** | [**Vec<serde_json::Value>**](serde_json::Value.md) | Array of payments. It requires a minimum structure as follows: [{ amount:SUM, description:DESCRIPTION, due_at:PAYMENT_DATE }]. Replace SUM with a payment amount, DESCRIPTION with a explanation string, PAYMENT_DATE with a date (format YYYY-MM-DD). | 
**update_deal_value** | Option<**bool**> | Indicates that the deal value must be set to installment subscription's total value | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


