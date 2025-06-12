# BaseWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the Webhook | [optional]
**company_id** | Option<**i32**> | The ID of the company related to the Webhook | [optional]
**owner_id** | Option<**i32**> | The ID of the user who owns the Webhook | [optional]
**user_id** | Option<**i32**> | The ID of the user related to the Webhook | [optional]
**event_action** | Option<**String**> | The Webhook action | [optional]
**event_object** | Option<**String**> | The Webhook object | [optional]
**subscription_url** | Option<**String**> | The subscription URL of the Webhook | [optional]
**is_active** | Option<**f64**> |  | [optional][default to Variant1]
**add_time** | Option<**String**> | The date when the Webhook was added | [optional]
**remove_time** | Option<**String**> | The date when the Webhook was removed (if removed) | [optional]
**r#type** | Option<**String**> | The type of the Webhook | [optional]
**http_auth_user** | Option<**String**> | The username of the `subscription_url` of the Webhook | [optional]
**http_auth_password** | Option<**String**> | The password of the `subscription_url` of the Webhook | [optional]
**additional_data** | Option<[**serde_json::Value**](.md)> | Any additional data related to the Webhook | [optional]
**remove_reason** | Option<**String**> | The removal reason of the Webhook (if removed) | [optional]
**last_delivery_time** | Option<**String**> | The last delivery time of the Webhook | [optional]
**last_http_status** | Option<**i32**> | The last delivery HTTP status of the Webhook | [optional]
**admin_id** | Option<**i32**> | The ID of the admin of the Webhook | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


