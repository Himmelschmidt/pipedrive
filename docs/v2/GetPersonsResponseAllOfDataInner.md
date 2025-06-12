# GetPersonsResponseAllOfDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the person | [optional]
**name** | Option<**String**> | The name of the person | [optional]
**first_name** | Option<**String**> | The first name of the person | [optional]
**last_name** | Option<**String**> | The last name of the person | [optional]
**owner_id** | Option<**i32**> | The ID of the user who owns the person | [optional]
**org_id** | Option<**i32**> | The ID of the organization linked to the person | [optional]
**add_time** | Option<**String**> | The creation date and time of the person | [optional]
**update_time** | Option<**String**> | The last updated date and time of the person | [optional]
**emails** | Option<[**Vec<models::GetPersonsResponseAllOfDataInnerEmailsInner>**](GetPersonsResponse_allOf_data_inner_emails_inner.md)> | The emails of the person | [optional]
**phones** | Option<[**Vec<models::GetPersonsResponseAllOfDataInnerPhonesInner>**](GetPersonsResponse_allOf_data_inner_phones_inner.md)> | The phones of the person | [optional]
**is_deleted** | Option<**bool**> | Whether the person is deleted or not | [optional]
**visible_to** | Option<**i32**> | The visibility of the person | [optional]
**label_ids** | Option<**Vec<i32>**> | The IDs of labels assigned to the person | [optional]
**picture_id** | Option<**i32**> | The ID of the picture associated with the person | [optional]
**postal_address** | Option<[**models::GetPersonsResponseAllOfDataInnerPostalAddress**](GetPersonsResponse_allOf_data_inner_postal_address.md)> |  | [optional]
**notes** | Option<**String**> | Contact sync notes of the person, maximum 10 000 characters, included if contact sync is enabled for the company | [optional]
**im** | Option<[**Vec<models::GetPersonsResponseAllOfDataInnerImInner>**](GetPersonsResponse_allOf_data_inner_im_inner.md)> | The instant messaging accounts of the person, included if contact sync is enabled for the company | [optional]
**birthday** | Option<**String**> | The birthday of the person, included if contact sync is enabled for the company | [optional]
**job_title** | Option<**String**> | The job title of the person, included if contact sync is enabled for the company | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


