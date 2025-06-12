# ReceiveMessageRequestAttachmentsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the attachment | 
**r#type** | **String** | The mime-type of the attachment | 
**name** | Option<**String**> | The name of the attachment | [optional]
**size** | Option<**f64**> | The size of the attachment | [optional]
**url** | **String** | A URL to the file | 
**preview_url** | Option<**String**> | A URL to a preview picture of the file | [optional]
**link_expires** | Option<**bool**> | If true, it will use the getMessageById endpoint for fetching updated attachment's urls. Find out more [here](https://pipedrive.readme.io/docs/implementing-messaging-app-extension) | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


