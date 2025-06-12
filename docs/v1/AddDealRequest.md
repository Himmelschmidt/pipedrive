# AddDealRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** | The title of the deal | 
**value** | Option<**String**> | The value of the deal. If omitted, value will be set to 0. | [optional]
**label** | Option<**Vec<i32>**> | The array of the labels IDs. | [optional]
**currency** | Option<**String**> | The currency of the deal. Accepts a 3-character currency code. If omitted, currency will be set to the default currency of the authorized user. | [optional]
**user_id** | Option<**i32**> | The ID of the user which will be the owner of the created deal. If not provided, the user making the request will be used. | [optional]
**person_id** | Option<**i32**> | The ID of a person which this deal will be linked to. If the person does not exist yet, it needs to be created first. This property is required unless `org_id` is specified. | [optional]
**org_id** | Option<**i32**> | The ID of an organization which this deal will be linked to. If the organization does not exist yet, it needs to be created first. This property is required unless `person_id` is specified. | [optional]
**pipeline_id** | Option<**i32**> | The ID of the pipeline this deal will be added to. By default, the deal will be added to the first stage of the specified pipeline. Please note that `pipeline_id` and `stage_id` should not be used together as `pipeline_id` will be ignored. | [optional]
**stage_id** | Option<**i32**> | The ID of the stage this deal will be added to. Please note that a pipeline will be assigned automatically based on the `stage_id`. If omitted, the deal will be placed in the first stage of the default pipeline. | [optional]
**is_archived** | Option<**bool**> | Whether the deal is archived or not. If omitted, is_archived will be set to false. | [optional]
**archive_time** | Option<**String**> | The optional date and time of archiving the deal in UTC. Format: YYYY-MM-DD HH:MM:SS. If omitted and `is_archived` is true, it will be set to the current date and time. | [optional]
**status** | Option<**String**> | open = Open, won = Won, lost = Lost, deleted = Deleted. If omitted, status will be set to open. | [optional]
**origin_id** | Option<**String**> | The optional ID to further distinguish the origin of the deal - e.g. Which API integration created this deal. If omitted, `origin_id` will be set to null. | [optional]
**channel** | Option<**i32**> | The ID of Marketing channel this deal was created from. Provided value must be one of the channels configured for your company. You can fetch allowed values with <a href=\"https://developers.pipedrive.com/docs/api/v1/DealFields#getDealField\" target=\"_blank\" rel=\"noopener noreferrer\">GET /v1/dealFields</a>. If omitted, channel will be set to null. | [optional]
**channel_id** | Option<**String**> | The optional ID to further distinguish the Marketing channel. If omitted, `channel_id` will be set to null. | [optional]
**add_time** | Option<**String**> | The optional creation date & time of the deal in UTC. Format: YYYY-MM-DD HH:MM:SS | [optional]
**won_time** | Option<**String**> | The optional date and time of changing the deal status as won in UTC. Format: YYYY-MM-DD HH:MM:SS. Can be set only when deal `status` is already Won. Can not be used together with `lost_time`. | [optional]
**lost_time** | Option<**String**> | The optional date and time of changing the deal status as lost in UTC. Format: YYYY-MM-DD HH:MM:SS. Can be set only when deal `status` is already Lost. Can not be used together with `won_time`. | [optional]
**close_time** | Option<**String**> | The optional date and time of closing the deal in UTC. Format: YYYY-MM-DD HH:MM:SS. | [optional]
**expected_close_date** | Option<[**String**](string.md)> | The expected close date of the deal. In ISO 8601 format: YYYY-MM-DD. | [optional]
**probability** | Option<**f64**> | The success probability percentage of the deal. Used/shown only when `deal_probability` for the pipeline of the deal is enabled. | [optional]
**lost_reason** | Option<**String**> | The optional message about why the deal was lost (to be used when status = lost) | [optional]
**visible_to** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


