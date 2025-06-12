# AddWebhookRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subscription_url** | **String** | A full, valid, publicly accessible URL which determines where to send the notifications. Please note that you cannot use Pipedrive API endpoints as the `subscription_url` and the chosen URL must not redirect to another link. | 
**event_action** | **String** | The type of action to receive notifications about. Wildcard will match all supported actions. | 
**event_object** | **String** | The type of object to receive notifications about. Wildcard will match all supported objects. | 
**user_id** | Option<**i32**> | The ID of the user that this webhook will be authorized with. You have the option to use a different user's `user_id`. If it is not set, the current user's `user_id` will be used. As each webhook event is checked against a user's permissions, the webhook will only be sent if the user has access to the specified object(s). If you want to receive notifications for all events, please use a top-level admin user’s `user_id`. | [optional]
**http_auth_user** | Option<**String**> | The HTTP basic auth username of the subscription URL endpoint (if required) | [optional]
**http_auth_password** | Option<**String**> | The HTTP basic auth password of the subscription URL endpoint (if required) | [optional]
**version** | Option<**String**> | The webhook's version. NB! Webhooks v2 is the default from March 17th, 2025. See <a href=\"https://developers.pipedrive.com/changelog/post/breaking-change-webhooks-v2-will-become-the-new-default-version\" target=\"_blank\" rel=\"noopener noreferrer\">this Changelog post</a> for more details. | [optional][default to Variant2Period0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


