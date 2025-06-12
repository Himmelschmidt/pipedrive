# \NotesApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_note**](NotesApi.md#add_note) | **POST** /notes | Add a note
[**add_note_comment**](NotesApi.md#add_note_comment) | **POST** /notes/{id}/comments | Add a comment to a note
[**delete_comment**](NotesApi.md#delete_comment) | **DELETE** /notes/{id}/comments/{commentId} | Delete a comment related to a note
[**delete_note**](NotesApi.md#delete_note) | **DELETE** /notes/{id} | Delete a note
[**get_comment**](NotesApi.md#get_comment) | **GET** /notes/{id}/comments/{commentId} | Get one comment
[**get_note**](NotesApi.md#get_note) | **GET** /notes/{id} | Get one note
[**get_note_comments**](NotesApi.md#get_note_comments) | **GET** /notes/{id}/comments | Get all comments for a note
[**get_notes**](NotesApi.md#get_notes) | **GET** /notes | Get all notes
[**update_comment_for_note**](NotesApi.md#update_comment_for_note) | **PUT** /notes/{id}/comments/{commentId} | Update a comment related to a note
[**update_note**](NotesApi.md#update_note) | **PUT** /notes/{id} | Update a note



## add_note

> models::UpsertNoteResponse add_note(add_note_request)
Add a note

Adds a new note.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_note_request** | Option<[**AddNoteRequest**](AddNoteRequest.md)> |  |  |

### Return type

[**models::UpsertNoteResponse**](UpsertNoteResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_note_comment

> models::UpsertCommentResponse add_note_comment(id, comment_post_put_object)
Add a comment to a note

Adds a new comment to a note.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the note | [required] |
**comment_post_put_object** | Option<[**CommentPostPutObject**](CommentPostPutObject.md)> |  |  |

### Return type

[**models::UpsertCommentResponse**](UpsertCommentResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_comment

> models::DeleteCommentResponse delete_comment(id, comment_id)
Delete a comment related to a note

Deletes a comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the note | [required] |
**comment_id** | **uuid::Uuid** | The ID of the comment | [required] |

### Return type

[**models::DeleteCommentResponse**](DeleteCommentResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_note

> models::DeleteNoteResponse delete_note(id)
Delete a note

Deletes a specific note.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the note | [required] |

### Return type

[**models::DeleteNoteResponse**](DeleteNoteResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comment

> models::UpsertCommentResponse get_comment(id, comment_id)
Get one comment

Returns the details of a comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the note | [required] |
**comment_id** | **uuid::Uuid** | The ID of the comment | [required] |

### Return type

[**models::UpsertCommentResponse**](UpsertCommentResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_note

> models::UpsertNoteResponse get_note(id)
Get one note

Returns details about a specific note.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the note | [required] |

### Return type

[**models::UpsertNoteResponse**](UpsertNoteResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_note_comments

> models::GetCommentsResponse get_note_comments(id, start, limit)
Get all comments for a note

Returns all comments associated with a note.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the note | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**models::GetCommentsResponse**](GetCommentsResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notes

> models::GetNotesResponse get_notes(user_id, lead_id, deal_id, person_id, org_id, project_id, start, limit, sort, start_date, end_date, pinned_to_lead_flag, pinned_to_deal_flag, pinned_to_organization_flag, pinned_to_person_flag, pinned_to_project_flag)
Get all notes

Returns all notes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**i32**> | The ID of the user whose notes to fetch. If omitted, notes by all users will be returned. |  |
**lead_id** | Option<**uuid::Uuid**> | The ID of the lead which notes to fetch. If omitted, notes about all leads will be returned. |  |
**deal_id** | Option<**i32**> | The ID of the deal which notes to fetch. If omitted, notes about all deals will be returned. |  |
**person_id** | Option<**i32**> | The ID of the person whose notes to fetch. If omitted, notes about all persons will be returned. |  |
**org_id** | Option<**i32**> | The ID of the organization which notes to fetch. If omitted, notes about all organizations will be returned. |  |
**project_id** | Option<**i32**> | The ID of the project which notes to fetch. If omitted, notes about all projects will be returned. |  |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**sort** | Option<**String**> | The field names and sorting mode separated by a comma (`field_name_1 ASC`, `field_name_2 DESC`). Only first-level field keys are supported (no nested keys). Supported fields: `id`, `user_id`, `deal_id`, `person_id`, `org_id`, `content`, `add_time`, `update_time`. |  |
**start_date** | Option<**String**> | The date in format of YYYY-MM-DD from which notes to fetch |  |
**end_date** | Option<**String**> | The date in format of YYYY-MM-DD until which notes to fetch to |  |
**pinned_to_lead_flag** | Option<**f64**> | If set, the results are filtered by note to lead pinning state |  |
**pinned_to_deal_flag** | Option<**f64**> | If set, the results are filtered by note to deal pinning state |  |
**pinned_to_organization_flag** | Option<**f64**> | If set, the results are filtered by note to organization pinning state |  |
**pinned_to_person_flag** | Option<**f64**> | If set, the results are filtered by note to person pinning state |  |
**pinned_to_project_flag** | Option<**f64**> | If set, the results are filtered by note to project pinning state |  |

### Return type

[**models::GetNotesResponse**](GetNotesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_comment_for_note

> models::UpsertCommentResponse update_comment_for_note(id, comment_id, comment_post_put_object)
Update a comment related to a note

Updates a comment related to a note.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the note | [required] |
**comment_id** | **uuid::Uuid** | The ID of the comment | [required] |
**comment_post_put_object** | Option<[**CommentPostPutObject**](CommentPostPutObject.md)> |  |  |

### Return type

[**models::UpsertCommentResponse**](UpsertCommentResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_note

> models::UpsertNoteResponse update_note(id, note_request)
Update a note

Updates a note.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the note | [required] |
**note_request** | Option<[**NoteRequest**](NoteRequest.md)> |  |  |

### Return type

[**models::UpsertNoteResponse**](UpsertNoteResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

