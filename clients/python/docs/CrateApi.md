# openapi_client.CrateApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle_add_email_channel**](CrateApi.md#handle_add_email_channel) | **POST** /add_channel/email | Add email channel
[**handle_add_telegram_channel**](CrateApi.md#handle_add_telegram_channel) | **POST** /add_channel/telegram | Add telegram channel
[**handle_send_notification**](CrateApi.md#handle_send_notification) | **POST** /notify | Send notification


# **handle_add_email_channel**
> handle_add_email_channel(add_email_channel_body)

Add email channel

Add email channel  Add email notification channel for user 

### Example


```python
import time
import openapi_client
from openapi_client.api import crate_api
from openapi_client.model.add_email_channel_body import AddEmailChannelBody
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with openapi_client.ApiClient() as api_client:
    # Create an instance of the API class
    api_instance = crate_api.CrateApi(api_client)
    add_email_channel_body = AddEmailChannelBody(
        email="email_example",
        user_id="user_id_example",
    ) # AddEmailChannelBody | 

    # example passing only required values which don't have defaults set
    try:
        # Add email channel
        api_instance.handle_add_email_channel(add_email_channel_body)
    except openapi_client.ApiException as e:
        print("Exception when calling CrateApi->handle_add_email_channel: %s\n" % e)
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **add_email_channel_body** | [**AddEmailChannelBody**](AddEmailChannelBody.md)|  |

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined


### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Channel added successfully |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **handle_add_telegram_channel**
> handle_add_telegram_channel(add_telegram_channel_body)

Add telegram channel

Add telegram channel  Add telegram notification channel for user 

### Example


```python
import time
import openapi_client
from openapi_client.api import crate_api
from openapi_client.model.add_telegram_channel_body import AddTelegramChannelBody
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with openapi_client.ApiClient() as api_client:
    # Create an instance of the API class
    api_instance = crate_api.CrateApi(api_client)
    add_telegram_channel_body = AddTelegramChannelBody(
        telegram_username="telegram_username_example",
        user_id="user_id_example",
    ) # AddTelegramChannelBody | 

    # example passing only required values which don't have defaults set
    try:
        # Add telegram channel
        api_instance.handle_add_telegram_channel(add_telegram_channel_body)
    except openapi_client.ApiException as e:
        print("Exception when calling CrateApi->handle_add_telegram_channel: %s\n" % e)
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **add_telegram_channel_body** | [**AddTelegramChannelBody**](AddTelegramChannelBody.md)|  |

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined


### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Channel added successfully |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **handle_send_notification**
> handle_send_notification(send_notification_body)

Send notification

Send notification  send notification to user with given id on all channels registered for that user 

### Example


```python
import time
import openapi_client
from openapi_client.api import crate_api
from openapi_client.model.send_notification_body import SendNotificationBody
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with openapi_client.ApiClient() as api_client:
    # Create an instance of the API class
    api_instance = crate_api.CrateApi(api_client)
    send_notification_body = SendNotificationBody(
        message="message_example",
        user_id="user_id_example",
        subject="subject_example",
    ) # SendNotificationBody | 

    # example passing only required values which don't have defaults set
    try:
        # Send notification
        api_instance.handle_send_notification(send_notification_body)
    except openapi_client.ApiException as e:
        print("Exception when calling CrateApi->handle_send_notification: %s\n" % e)
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **send_notification_body** | [**SendNotificationBody**](SendNotificationBody.md)|  |

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined


### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Notification sent successfully |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

