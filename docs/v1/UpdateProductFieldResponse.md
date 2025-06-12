# UpdateProductFieldResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the field | [optional]
**options** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | When `field_type` is either set or enum, possible options on update must be supplied as an array of objects each containing id and label, for example: [{\"id\":1, \"label\":\"red\"},{\"id\":2, \"label\":\"blue\"},{\"id\":3, \"label\":\"lilac\"}] | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


