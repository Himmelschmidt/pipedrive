# GetOrganizationsCollectionResponseObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**String**> | The full address of the organization | [optional]
**address_subpremise** | Option<**String**> | The sub-premise of the organization location | [optional]
**address_street_number** | Option<**String**> | The street number of the organization location | [optional]
**address_route** | Option<**String**> | The route of the organization location | [optional]
**address_sublocality** | Option<**String**> | The sub-locality of the organization location | [optional]
**address_locality** | Option<**String**> | The locality of the organization location | [optional]
**address_admin_area_level_1** | Option<**String**> | The level 1 admin area of the organization location | [optional]
**address_admin_area_level_2** | Option<**String**> | The level 2 admin area of the organization location | [optional]
**address_country** | Option<**String**> | The country of the organization location | [optional]
**address_postal_code** | Option<**String**> | The postal code of the organization location | [optional]
**address_formatted_address** | Option<**String**> | The formatted organization location | [optional]
**id** | Option<**i32**> | The ID of the organization | [optional]
**active_flag** | Option<**bool**> | Whether the organization is active or not | [optional]
**owner_id** | Option<**i32**> | The ID of the owner | [optional]
**name** | Option<**String**> | The name of the organization | [optional]
**update_time** | Option<**String**> | The last updated date and time of the organization. Format: YYYY-MM-DD HH:MM:SS | [optional]
**delete_time** | Option<**String**> | The date and time this organization was deleted. Format: YYYY-MM-DD HH:MM:SS | [optional]
**add_time** | Option<**String**> | The date and time when the organization was added/created. Format: YYYY-MM-DD HH:MM:SS | [optional]
**visible_to** | Option<**String**> | The visibility group ID of who can see the organization | [optional]
**label** | Option<**i32**> | The label assigned to the organization. When the `label` field is updated, the `label_ids` field value will be overwritten by the `label` field value. | [optional]
**label_ids** | Option<**Vec<i32>**> | The IDs of labels assigned to the organization. When the `label_ids` field is updated, the `label` field value will be set to the first value of the `label_ids` field. | [optional]
**cc_email** | Option<**String**> | The BCC email associated with the organization | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


