# Lead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The unique ID of the lead in the UUID format | [optional]
**title** | Option<**String**> | The title of the lead | [optional]
**owner_id** | Option<**i32**> | The ID of the user who owns the lead | [optional]
**creator_id** | Option<**i32**> | The ID of the user who created the lead | [optional]
**label_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | The IDs of the lead labels which are associated with the lead | [optional]
**person_id** | Option<**i32**> | The ID of a person which this lead is linked to | [optional]
**organization_id** | Option<**i32**> | The ID of an organization which this lead is linked to | [optional]
**source_name** | Option<**String**> | Defines where the lead comes from. Will be `API` if the lead was created through the Public API and will be `Manually created` if the lead was created manually through the UI.  | [optional]
**origin** | Option<**String**> | The way this Lead was created. `origin` field is set by Pipedrive when Lead is created and cannot be changed. | [optional]
**origin_id** | Option<**String**> | The optional ID to further distinguish the origin of the lead - e.g. Which API integration created this Lead. | [optional]
**channel** | Option<**i32**> | The ID of your Marketing channel this Lead was created from. Recognized Marketing channels can be configured in your <a href=\"https://app.pipedrive.com/settings/fields\" target=\"_blank\" rel=\"noopener noreferrer\">Company settings</a>. | [optional]
**channel_id** | Option<**String**> | The optional ID to further distinguish the Marketing channel. | [optional]
**is_archived** | Option<**bool**> | A flag indicating whether the lead is archived or not | [optional]
**was_seen** | Option<**bool**> | A flag indicating whether the lead was seen by someone in the Pipedrive UI | [optional]
**value** | Option<[**models::LeadValue**](Lead_value.md)> |  | [optional]
**expected_close_date** | Option<[**String**](string.md)> | The date of when the deal which will be created from the lead is expected to be closed. In ISO 8601 format: YYYY-MM-DD. | [optional]
**next_activity_id** | Option<**i32**> | The ID of the next activity associated with the lead | [optional]
**add_time** | Option<**String**> | The date and time of when the lead was created. In ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ. | [optional]
**update_time** | Option<**String**> | The date and time of when the lead was last updated. In ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ. | [optional]
**visible_to** | Option<**String**> |  | [optional]
**cc_email** | Option<**String**> | The BCC email of the lead | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


