# \CrateApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle_add_email_channel**](CrateApi.md#handle_add_email_channel) | **POST** /add_channel/email | Add email channel
[**handle_add_telegram_channel**](CrateApi.md#handle_add_telegram_channel) | **POST** /add_channel/telegram | Add telegram channel
[**handle_send_notification**](CrateApi.md#handle_send_notification) | **POST** /notify | Send notification



## handle_add_email_channel

> handle_add_email_channel(addemailchannelbody)
Add email channel

Add email channel  Add email notification channel for user 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addemailchannelbody** | [**AddEmailChannelBody**](AddEmailChannelBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## handle_add_telegram_channel

> handle_add_telegram_channel(addtelegramchannelbody)
Add telegram channel

Add telegram channel  Add telegram notification channel for user 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addtelegramchannelbody** | [**AddTelegramChannelBody**](AddTelegramChannelBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## handle_send_notification

> handle_send_notification(sendnotificationbody)
Send notification

Send notification  send notification to user with given id on all channels registered for that user 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sendnotificationbody** | [**SendNotificationBody**](SendNotificationBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

