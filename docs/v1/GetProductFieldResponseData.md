# GetProductFieldResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the field | 
**options** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | When `field_type` is either `set` or `enum`, possible options must be supplied as a JSON-encoded sequential array, for example:</br>`[{\"label\":\"red\"}, {\"label\":\"blue\"}, {\"label\":\"lilac\"}]` | [optional]
**field_type** | **String** | The type of the field<table><tr><th>Value</th><th>Description</th></tr><tr><td>`varchar`</td><td>Text (up to 255 characters)</td><tr><td>`varchar_auto`</td><td>Autocomplete text (up to 255 characters)</td><tr><td>`text`</td><td>Long text (up to 65k characters)</td><tr><td>`double`</td><td>Numeric value</td><tr><td>`monetary`</td><td>Monetary field (has a numeric value and a currency value)</td><tr><td>`date`</td><td>Date (format YYYY-MM-DD)</td><tr><td>`set`</td><td>Options field with a possibility of having multiple chosen options</td><tr><td>`enum`</td><td>Options field with a single possible chosen option</td><tr><td>`user`</td><td>User field (contains a user ID of another Pipedrive user)</td><tr><td>`org`</td><td>Organization field (contains an organization ID which is stored on the same account)</td><tr><td>`people`</td><td>Person field (contains a product ID which is stored on the same account)</td><tr><td>`phone`</td><td>Phone field (up to 255 numbers and/or characters)</td><tr><td>`time`</td><td>Time field (format HH:MM:SS)</td><tr><td>`timerange`</td><td>Time-range field (has a start time and end time value, both HH:MM:SS)</td><tr><td>`daterange`</td><td>Date-range field (has a start date and end date value, both YYYY-MM-DD)</td><tr><td>`address`</td><td>Address field</dd></table> | 
**id** | Option<**i32**> | The ID of the product field | [optional]
**key** | Option<**String**> | The key of the product field | [optional]
**order_nr** | Option<**i32**> | The position (index) of the product field in the detail view | [optional]
**add_time** | Option<**String**> | The product field creation time. Format: YYYY-MM-DD HH:MM:SS | [optional]
**update_time** | Option<**String**> | The product field last update time. Format: YYYY-MM-DD HH:MM:SS | [optional]
**last_updated_by_user_id** | Option<**i32**> | The ID of the last user to update the product field | [optional]
**created_by_user_id** | Option<**i32**> | The ID of the user who created the product field | [optional]
**active_flag** | Option<**bool**> | Whether or not the product field is currently active | [optional]
**edit_flag** | Option<**bool**> | Whether or not the product field name and metadata is editable | [optional]
**add_visible_flag** | Option<**bool**> | Whether or not the product field is visible in the Add Product Modal | [optional]
**important_flag** | Option<**bool**> | Whether or not the product field is marked as important | [optional]
**bulk_edit_allowed** | Option<**bool**> | Whether or not the product field data can be edited | [optional]
**searchable_flag** | Option<**bool**> | Whether or not the product field is searchable | [optional]
**filtering_allowed** | Option<**bool**> | Whether or not the product field value can be used when filtering searches | [optional]
**sortable_flag** | Option<**bool**> | Whether or not the product field is sortable | [optional]
**mandatory_flag** | Option<**bool**> | Whether or not the product field is mandatory when creating products | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


