# Field

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the field. Value is `null` in case of subfields. | [optional]
**key** | Option<**String**> | The key of the field. For custom fields this is generated upon creation. | [optional]
**name** | Option<**String**> | The name of the field | [optional]
**order_nr** | Option<**i32**> | The order number of the field | [optional]
**field_type** | Option<**String**> | The type of the field<table><tr><th>Value</th><th>Description</th></tr><tr><td>`address`</td><td>Address field</td></tr><tr><td>`date`</td><td>Date (format YYYY-MM-DD)</td></tr><tr><td>`daterange`</td><td>Date-range field (has a start date and end date value, both YYYY-MM-DD)</td></tr><tr><td>`double`</td><td>Numeric value</td></tr><tr><td>`enum`</td><td>Options field with a single possible chosen option</td></tr><tr></tr><tr><td>`monetary`</td><td>Monetary field (has a numeric value and a currency value)</td></tr><tr><td>`org`</td><td>Organization field (contains an organization ID which is stored on the same account)</td></tr><tr><td>`people`</td><td>Person field (contains a person ID which is stored on the same account)</td></tr><tr><td>`phone`</td><td>Phone field (up to 255 numbers and/or characters)</td></tr><tr><td>`set`</td><td>Options field with a possibility of having multiple chosen options</td></tr><tr><td>`text`</td><td>Long text (up to 65k characters)</td></tr><tr><td>`time`</td><td>Time field (format HH:MM:SS)</td></tr><tr><td>`timerange`</td><td>Time-range field (has a start time and end time value, both HH:MM:SS)</td></tr><tr><td>`user`</td><td>User field (contains a user ID of another Pipedrive user)</td></tr><tr><td>`varchar`</td><td>Text (up to 255 characters)</td></tr><tr><td>`varchar_auto`</td><td>Autocomplete text (up to 255 characters)</td></tr><tr><td>`visible_to`</td><td>System field that keeps item's visibility setting</td></tr></table> | [optional]
**add_time** | Option<**String**> | The creation time of the field | [optional]
**update_time** | Option<**String**> | The update time of the field | [optional]
**last_updated_by_user_id** | Option<**i32**> | The ID of the user who created or most recently updated the field, only applicable for custom fields | [optional]
**created_by_user_id** | Option<**i32**> | The ID of the user who created the field | [optional]
**active_flag** | Option<**bool**> | The active flag of the field | [optional]
**edit_flag** | Option<**bool**> | The edit flag of the field | [optional]
**index_visible_flag** | Option<**bool**> | Not used | [optional]
**details_visible_flag** | Option<**bool**> | Not used | [optional]
**add_visible_flag** | Option<**bool**> | Not used | [optional]
**important_flag** | Option<**bool**> | Not used | [optional]
**bulk_edit_allowed** | Option<**bool**> | Whether or not the field of an item can be edited in bulk | [optional]
**searchable_flag** | Option<**bool**> | Whether or not items can be searched by this field | [optional]
**filtering_allowed** | Option<**bool**> | Whether or not items can be filtered by this field | [optional]
**sortable_flag** | Option<**bool**> | Whether or not items can be sorted by this field | [optional]
**mandatory_flag** | Option<**bool**> | Whether or not the field is mandatory | [optional]
**options** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | The options of the field. When there are no options, `null` is returned. | [optional]
**options_deleted** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | The deleted options of the field. Only present when there is at least 1 deleted option. | [optional]
**is_subfield** | Option<**bool**> | Whether or not the field is a subfield of another field. Only present if field is subfield. | [optional]
**subfields** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | The subfields of the field. Only present when the field has subfields. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


