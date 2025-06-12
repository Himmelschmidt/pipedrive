# GetFilesResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the file | [optional]
**user_id** | Option<**i32**> | The ID of the user to associate the file with | [optional]
**deal_id** | Option<**i32**> | The ID of the deal to associate the file with | [optional]
**person_id** | Option<**i32**> | The ID of the person to associate the file with | [optional]
**org_id** | Option<**i32**> | The ID of the organization to associate the file with | [optional]
**product_id** | Option<**i32**> | The ID of the product to associate the file with | [optional]
**activity_id** | Option<**i32**> | The ID of the activity to associate the file with | [optional]
**lead_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The ID of the lead to associate the file with | [optional]
**add_time** | Option<**String**> | The date and time when the file was added/created. Format: YYYY-MM-DD HH:MM:SS | [optional]
**update_time** | Option<**String**> | The last updated date and time of the file. Format: YYYY-MM-DD HH:MM:SS | [optional]
**file_name** | Option<**String**> | The original name of the file | [optional]
**file_size** | Option<**i32**> | The size of the file | [optional]
**active_flag** | Option<**bool**> | Whether the user is active or not. false = Not activated, true = Activated | [optional]
**inline_flag** | Option<**bool**> | Whether the file was uploaded as inline or not | [optional]
**remote_location** | Option<**String**> | The location type to send the file to. Only googledrive is supported at the moment. | [optional]
**remote_id** | Option<**String**> | The ID of the remote item | [optional]
**cid** | Option<**String**> | The ID of the inline attachment | [optional]
**s3_bucket** | Option<**String**> | The location of the cloud storage | [optional]
**mail_message_id** | Option<**String**> | The ID of the mail message to associate the file with | [optional]
**mail_template_id** | Option<**String**> | The ID of the mail template to associate the file with | [optional]
**deal_name** | Option<**String**> | The name of the deal associated with the file | [optional]
**person_name** | Option<**String**> | The name of the person associated with the file | [optional]
**org_name** | Option<**String**> | The name of the organization associated with the file | [optional]
**product_name** | Option<**String**> | The name of the product associated with the file | [optional]
**lead_name** | Option<**String**> | The name of the lead associated with the file | [optional]
**url** | Option<**String**> | The URL of the download file | [optional]
**name** | Option<**String**> | The visible name of the file | [optional]
**description** | Option<**String**> | The description of the file | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


