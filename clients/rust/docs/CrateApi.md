# \CrateApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle_add_channel**](CrateApi.md#handle_add_channel) | **POST** /add_channel | Add channel
[**handle_get_telegram_chat_id**](CrateApi.md#handle_get_telegram_chat_id) | **POST** /get_telegram_chat_id | Get the chat ID of a telegram username
[**handle_notify**](CrateApi.md#handle_notify) | **POST** /notify | Send notification
[**handle_remove_channel**](CrateApi.md#handle_remove_channel) | **POST** /remove_channel | Remove channel



## handle_add_channel

> handle_add_channel(add_channel_body)
Add channel

Add channel  Remove notification channel for user 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_channel_body** | [**AddChannelBody**](AddChannelBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## handle_get_telegram_chat_id

> handle_get_telegram_chat_id(get_telegram_chat_id_body)
Get the chat ID of a telegram username

Get the chat ID of a telegram username  First call this endpoint, then ask the user to send a message to the bot, then the chat id will be returned 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_telegram_chat_id_body** | [**GetTelegramChatIdBody**](GetTelegramChatIdBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## handle_notify

> handle_notify(send_notification_body)
Send notification

Send notification  send notification to user with given id on all channels registered for that user 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**send_notification_body** | [**SendNotificationBody**](SendNotificationBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## handle_remove_channel

> handle_remove_channel(remove_channel_body)
Remove channel

Remove channel  Remove notification channel for user 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_channel_body** | [**RemoveChannelBody**](RemoveChannelBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

