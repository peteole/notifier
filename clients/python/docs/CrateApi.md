# openapi_client.CrateApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle_add_channel**](CrateApi.md#handle_add_channel) | **POST** /add_channel | Add channel
[**handle_get_channels**](CrateApi.md#handle_get_channels) | **GET** /get_channels/{user_id} | Get notification channels for user
[**handle_get_telegram_chat_id**](CrateApi.md#handle_get_telegram_chat_id) | **GET** /get_telegram_chat_id/{username} | Get the chat ID of a telegram username
[**handle_notify**](CrateApi.md#handle_notify) | **POST** /notify | Send notification
[**handle_remove_channel**](CrateApi.md#handle_remove_channel) | **POST** /remove_channel | Remove channel


# **handle_add_channel**
> handle_add_channel(add_channel_body)

Add channel

Add channel  Add notification channel for user 

### Example


```python
import time
import openapi_client
from openapi_client.api import crate_api
from openapi_client.model.add_channel_body import AddChannelBody
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
    add_channel_body = AddChannelBody(
        service_id="service_id_example",
        service_username="service_username_example",
        user_id="user_id_example",
    ) # AddChannelBody | 

    # example passing only required values which don't have defaults set
    try:
        # Add channel
        api_instance.handle_add_channel(add_channel_body)
    except openapi_client.ApiException as e:
        print("Exception when calling CrateApi->handle_add_channel: %s\n" % e)
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **add_channel_body** | [**AddChannelBody**](AddChannelBody.md)|  |

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

# **handle_get_channels**
> ChannelsResponse handle_get_channels(user_id)

Get notification channels for user

Get notification channels for user  get all channels registered for user with given id 

### Example


```python
import time
import openapi_client
from openapi_client.api import crate_api
from openapi_client.model.channels_response import ChannelsResponse
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
    user_id = "user_id_example" # str | User id to get notification channels for

    # example passing only required values which don't have defaults set
    try:
        # Get notification channels for user
        api_response = api_instance.handle_get_channels(user_id)
        pprint(api_response)
    except openapi_client.ApiException as e:
        print("Exception when calling CrateApi->handle_get_channels: %s\n" % e)
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user_id** | **str**| User id to get notification channels for |

### Return type

[**ChannelsResponse**](ChannelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Notification channels retrieved successfully |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **handle_get_telegram_chat_id**
> str handle_get_telegram_chat_id(username)

Get the chat ID of a telegram username

Get the chat ID of a telegram username  First call this endpoint, then ask the user to send a message to the bot, then the chat id will be returned 

### Example


```python
import time
import openapi_client
from openapi_client.api import crate_api
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
    username = "username_example" # str | User id to get notification channels for

    # example passing only required values which don't have defaults set
    try:
        # Get the chat ID of a telegram username
        api_response = api_instance.handle_get_telegram_chat_id(username)
        pprint(api_response)
    except openapi_client.ApiException as e:
        print("Exception when calling CrateApi->handle_get_telegram_chat_id: %s\n" % e)
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **username** | **str**| User id to get notification channels for |

### Return type

**str**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain


### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Returns the chat ID |  -  |
**500** | Could not look up username |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **handle_notify**
> handle_notify(notify_body)

Send notification

Send notification  send notification to user with given id on all channels registered for that user 

### Example


```python
import time
import openapi_client
from openapi_client.api import crate_api
from openapi_client.model.notify_body import NotifyBody
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
    notify_body = NotifyBody(
        message="message_example",
        user_id="user_id_example",
        subject="subject_example",
    ) # NotifyBody | 

    # example passing only required values which don't have defaults set
    try:
        # Send notification
        api_instance.handle_notify(notify_body)
    except openapi_client.ApiException as e:
        print("Exception when calling CrateApi->handle_notify: %s\n" % e)
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **notify_body** | [**NotifyBody**](NotifyBody.md)|  |

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

# **handle_remove_channel**
> handle_remove_channel(remove_channel_body)

Remove channel

Remove channel  Remove notification channel for user 

### Example


```python
import time
import openapi_client
from openapi_client.api import crate_api
from openapi_client.model.remove_channel_body import RemoveChannelBody
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
    remove_channel_body = RemoveChannelBody(
        user_id="user_id_example",
        service_id="service_id_example",
    ) # RemoveChannelBody | 

    # example passing only required values which don't have defaults set
    try:
        # Remove channel
        api_instance.handle_remove_channel(remove_channel_body)
    except openapi_client.ApiException as e:
        print("Exception when calling CrateApi->handle_remove_channel: %s\n" % e)
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **remove_channel_body** | [**RemoveChannelBody**](RemoveChannelBody.md)|  |

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
**200** | Channel removed successfully |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

