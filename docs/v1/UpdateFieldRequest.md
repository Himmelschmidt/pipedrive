# UpdateFieldRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the field | [optional]
**options** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | When `field_type` is either set or enum, possible options must be supplied as a JSON-encoded sequential array of objects. All active items must be supplied and already existing items must have their ID supplied. New items only require a label. Example: `[{\"id\":123,\"label\":\"Existing Item\"},{\"label\":\"New Item\"}]` | [optional]
**add_visible_flag** | Option<**bool**> | Whether the field is available in 'add new' modal or not (both in web and mobile app) | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


