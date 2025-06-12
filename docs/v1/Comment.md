# Comment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The ID of the note | [optional]
**active_flag** | Option<**bool**> | Whether the note is active or deleted | [optional]
**add_time** | Option<**String**> | The creation date and time of the note | [optional]
**update_time** | Option<**String**> | The creation date and time of the note | [optional]
**content** | Option<**String**> | The content of the note in HTML format. Subject to sanitization on the back-end. | [optional]
**object_id** | Option<**String**> | The ID of the object that the comment is attached to, will be the id of the note | [optional]
**object_type** | Option<**String**> | The type of object that the comment is attached to, will be \"note\" | [optional]
**user_id** | Option<**i32**> | The ID of the user who created the comment | [optional]
**updater_id** | Option<**i32**> | The ID of the user who last updated the comment | [optional]
**company_id** | Option<**i32**> | The ID of the company | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


