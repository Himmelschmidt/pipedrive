# UpdateDealRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | The title of the deal | [optional]
**value** | Option<**String**> | The value of the deal. | [optional]
**label** | Option<**Vec<i32>**> | Array of the deal labels IDs. | [optional]
**currency** | Option<**String**> | The currency of the deal. Accepts a 3-character currency code. | [optional]
**user_id** | Option<**i32**> | The ID of the user which will be the new owner of the deal. | [optional]
**person_id** | Option<**i32**> | The ID of a person which this deal will be linked to. If the person does not exist yet, it needs to be created first. | [optional]
**org_id** | Option<**i32**> | The ID of an organization which this deal will be linked to. If the organization does not exist yet, it needs to be created first. | [optional]
**pipeline_id** | Option<**i32**> | The ID of the pipeline this deal will be added to. By default, the deal will be added to the first stage of the specified pipeline. Please note that `pipeline_id` and `stage_id` should not be used together as `pipeline_id` will be ignored. | [optional]
**stage_id** | Option<**i32**> | The ID of the stage this deal will be added to. Please note that a pipeline will be assigned automatically based on the `stage_id`. | [optional]
**is_archived** | Option<**bool**> | Whether the deal is archived or not | [optional]
**status** | Option<**String**> | open = Open, won = Won, lost = Lost, deleted = Deleted. | [optional]
**channel** | Option<**i32**> | The ID of Marketing channel this deal was created from. Provided value must be one of the channels configured for your company which you can fetch with <a href=\"https://developers.pipedrive.com/docs/api/v1/DealFields#getDealField\" target=\"_blank\" rel=\"noopener noreferrer\">GET /v1/dealFields</a>. | [optional]
**channel_id** | Option<**String**> | The optional ID to further distinguish the Marketing channel. | [optional]
**won_time** | Option<**String**> | The optional date and time of changing the deal status as won in UTC. Format: YYYY-MM-DD HH:MM:SS. Can be set only when deal `status` is already Won. Can not be used together with `lost_time`. | [optional]
**lost_time** | Option<**String**> | The optional date and time of changing the deal status as lost in UTC. Format: YYYY-MM-DD HH:MM:SS. Can be set only when deal `status` is already Lost. Can not be used together with `won_time`. | [optional]
**close_time** | Option<**String**> | The optional date and time of closing the deal in UTC. Format: YYYY-MM-DD HH:MM:SS. | [optional]
**expected_close_date** | Option<[**String**](string.md)> | The expected close date of the deal. In ISO 8601 format: YYYY-MM-DD. | [optional]
**probability** | Option<**f64**> | The success probability percentage of the deal. Used/shown only when `deal_probability` for the pipeline of the deal is enabled. | [optional]
**lost_reason** | Option<**String**> | The optional message about why the deal was lost (to be used when status = lost) | [optional]
**visible_to** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


