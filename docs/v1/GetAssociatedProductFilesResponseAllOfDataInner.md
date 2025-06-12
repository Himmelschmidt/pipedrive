# GetAssociatedProductFilesResponseAllOfDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the file | [optional]
**product_id** | Option<**i32**> | The ID of the product associated with the file | [optional]
**add_time** | Option<**String**> | The UTC date time when the file was uploaded. Format: YYYY-MM-DD HH:MM:SS | [optional]
**update_time** | Option<**String**> | The UTC date time when the file was last updated. Format: YYYY-MM-DD HH:MM:SS | [optional]
**file_name** | Option<**String**> | The original name of the file | [optional]
**file_size** | Option<**i32**> | The size of the file in bytes | [optional]
**active_flag** | Option<**bool**> | Whether the user is active or not. | [optional]
**inline_flag** | Option<**bool**> | Whether the file was uploaded as inline or not | [optional]
**remote_location** | Option<**String**> | The location type to send the file to. Only googledrive is supported at the moment. | [optional]
**remote_id** | Option<**String**> | The ID of the remote item | [optional]
**s3_bucket** | Option<**String**> | The location of the cloud storage | [optional]
**product_name** | Option<**String**> | The name of the product associated with the file | [optional]
**url** | Option<**String**> | The URL to download the file | [optional]
**name** | Option<**String**> | The visible name of the file | [optional]
**description** | Option<**String**> | The description of the file | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


