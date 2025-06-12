# BaseUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The user ID | [optional]
**name** | Option<**String**> | The user name | [optional]
**default_currency** | Option<**String**> | The user default currency | [optional]
**locale** | Option<**String**> | The user locale | [optional]
**lang** | Option<**i32**> | The user language ID | [optional]
**email** | Option<**String**> | The user email | [optional]
**phone** | Option<**String**> | The user phone | [optional]
**activated** | Option<**bool**> | Boolean that indicates whether the user is activated | [optional]
**last_login** | Option<**String**> | The last login date and time of the user. Format: YYYY-MM-DD HH:MM:SS | [optional]
**created** | Option<**String**> | The creation date and time of the user. Format: YYYY-MM-DD HH:MM:SS | [optional]
**modified** | Option<**String**> | The last modification date and time of the user. Format: YYYY-MM-DD HH:MM:SS | [optional]
**has_created_company** | Option<**bool**> | Boolean that indicates whether the user has created a company | [optional]
**access** | Option<[**Vec<models::UserAccess>**](UserAccess.md)> |  | [optional]
**active_flag** | Option<**bool**> | Boolean that indicates whether the user is activated | [optional]
**timezone_name** | Option<**String**> | The user timezone name | [optional]
**timezone_offset** | Option<**String**> | The user timezone offset | [optional]
**role_id** | Option<**i32**> | The ID of the user role | [optional]
**icon_url** | Option<**String**> | The user icon URL | [optional]
**is_you** | Option<**bool**> | Boolean that indicates if the requested user is the same which is logged in (in this case, always true) | [optional]
**is_deleted** | Option<**bool**> | Boolean that indicates whether the user is deleted from the company | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


