# \FilesApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_file**](FilesApi.md#add_file) | **POST** /files | Add file
[**add_file_and_link_it**](FilesApi.md#add_file_and_link_it) | **POST** /files/remote | Create a remote file and link it to an item
[**delete_file**](FilesApi.md#delete_file) | **DELETE** /files/{id} | Delete a file
[**download_file**](FilesApi.md#download_file) | **GET** /files/{id}/download | Download one file
[**get_file**](FilesApi.md#get_file) | **GET** /files/{id} | Get one file
[**get_files**](FilesApi.md#get_files) | **GET** /files | Get all files
[**link_file_to_item**](FilesApi.md#link_file_to_item) | **POST** /files/remoteLink | Link a remote file to an item
[**update_file**](FilesApi.md#update_file) | **PUT** /files/{id} | Update file details



## add_file

> models::AddFileResponse add_file(file, deal_id, person_id, org_id, product_id, activity_id, lead_id)
Add file

Lets you upload a file and associate it with a deal, person, organization, activity, product or lead. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/adding-a-file\" target=\"_blank\" rel=\"noopener noreferrer\">adding a file</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | A single file, supplied in the multipart/form-data encoding and contained within the given boundaries | [required] |
**deal_id** | Option<**i32**> | The ID of the deal to associate file(s) with |  |
**person_id** | Option<**i32**> | The ID of the person to associate file(s) with |  |
**org_id** | Option<**i32**> | The ID of the organization to associate file(s) with |  |
**product_id** | Option<**i32**> | The ID of the product to associate file(s) with |  |
**activity_id** | Option<**i32**> | The ID of the activity to associate file(s) with |  |
**lead_id** | Option<**uuid::Uuid**> | The ID of the lead to associate file(s) with |  |

### Return type

[**models::AddFileResponse**](AddFileResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_file_and_link_it

> models::AddRemoteFileAndLinkItToItemResponse add_file_and_link_it(file_type, title, item_type, item_id, remote_location)
Create a remote file and link it to an item

Creates a new empty file in the remote location (`googledrive`) that will be linked to the item you supply. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/adding-a-remote-file\" target=\"_blank\" rel=\"noopener noreferrer\">adding a remote file</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_type** | **String** | The file type | [required] |
**title** | **String** | The title of the file | [required] |
**item_type** | **String** | The item type | [required] |
**item_id** | **i32** | The ID of the item to associate the file with | [required] |
**remote_location** | **String** | The location type to send the file to. Only `googledrive` is supported at the moment. | [required] |

### Return type

[**models::AddRemoteFileAndLinkItToItemResponse**](AddRemoteFileAndLinkItToItemResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_file

> models::DeleteFileResponse delete_file(id)
Delete a file

Marks a file as deleted. After 30 days, the file will be permanently deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the file | [required] |

### Return type

[**models::DeleteFileResponse**](DeleteFileResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_file

> String download_file(id)
Download one file

Initializes a file download.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the file | [required] |

### Return type

**String**

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file

> models::GetFileResponse get_file(id)
Get one file

Returns data about a specific file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the file | [required] |

### Return type

[**models::GetFileResponse**](GetFileResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files

> models::GetFilesResponse get_files(start, limit, sort)
Get all files

Returns data about all files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page. Please note that a maximum value of 100 is allowed. |  |
**sort** | Option<**String**> | Supported fields: `id`, `update_time` |  |

### Return type

[**models::GetFilesResponse**](GetFilesResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_file_to_item

> models::GetLinkRemoteFileToItemResponse link_file_to_item(item_type, item_id, remote_id, remote_location)
Link a remote file to an item

Links an existing remote file (`googledrive`) to the item you supply. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/adding-a-remote-file\" target=\"_blank\" rel=\"noopener noreferrer\">adding a remote file</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_type** | **String** | The item type | [required] |
**item_id** | **i32** | The ID of the item to associate the file with | [required] |
**remote_id** | **String** | The remote item ID | [required] |
**remote_location** | **String** | The location type to send the file to. Only `googledrive` is supported at the moment. | [required] |

### Return type

[**models::GetLinkRemoteFileToItemResponse**](GetLinkRemoteFileToItemResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_file

> models::UpdateFileResponse update_file(id, name, description)
Update file details

Updates the properties of a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the file | [required] |
**name** | Option<**String**> | The visible name of the file |  |
**description** | Option<**String**> | The description of the file |  |

### Return type

[**models::UpdateFileResponse**](UpdateFileResponse.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

