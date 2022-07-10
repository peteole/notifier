# CrateApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handleAddEmailChannel**](CrateApi.md#handleAddEmailChannel) | **POST** /add_channel/email | Add email channel
[**handleAddTelegramChannel**](CrateApi.md#handleAddTelegramChannel) | **POST** /add_channel/telegram | Add telegram channel
[**handleSendNotification**](CrateApi.md#handleSendNotification) | **POST** /notify | Send notification


<a name="handleAddEmailChannel"></a>
# **handleAddEmailChannel**
> handleAddEmailChannel(addEmailChannelBody)

Add email channel

Add email channel  Add email notification channel for user 

### Example
```kotlin
// Import classes:
//import org.openapitools.client.infrastructure.*
//import org.openapitools.client.models.*

val apiInstance = CrateApi()
val addEmailChannelBody : AddEmailChannelBody =  // AddEmailChannelBody | 
try {
    apiInstance.handleAddEmailChannel(addEmailChannelBody)
} catch (e: ClientException) {
    println("4xx response calling CrateApi#handleAddEmailChannel")
    e.printStackTrace()
} catch (e: ServerException) {
    println("5xx response calling CrateApi#handleAddEmailChannel")
    e.printStackTrace()
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **addEmailChannelBody** | [**AddEmailChannelBody**](AddEmailChannelBody.md)|  |

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined

<a name="handleAddTelegramChannel"></a>
# **handleAddTelegramChannel**
> handleAddTelegramChannel(addTelegramChannelBody)

Add telegram channel

Add telegram channel  Add telegram notification channel for user 

### Example
```kotlin
// Import classes:
//import org.openapitools.client.infrastructure.*
//import org.openapitools.client.models.*

val apiInstance = CrateApi()
val addTelegramChannelBody : AddTelegramChannelBody =  // AddTelegramChannelBody | 
try {
    apiInstance.handleAddTelegramChannel(addTelegramChannelBody)
} catch (e: ClientException) {
    println("4xx response calling CrateApi#handleAddTelegramChannel")
    e.printStackTrace()
} catch (e: ServerException) {
    println("5xx response calling CrateApi#handleAddTelegramChannel")
    e.printStackTrace()
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **addTelegramChannelBody** | [**AddTelegramChannelBody**](AddTelegramChannelBody.md)|  |

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined

<a name="handleSendNotification"></a>
# **handleSendNotification**
> handleSendNotification(sendNotificationBody)

Send notification

Send notification  send notification to user with given id on all channels registered for that user 

### Example
```kotlin
// Import classes:
//import org.openapitools.client.infrastructure.*
//import org.openapitools.client.models.*

val apiInstance = CrateApi()
val sendNotificationBody : SendNotificationBody =  // SendNotificationBody | 
try {
    apiInstance.handleSendNotification(sendNotificationBody)
} catch (e: ClientException) {
    println("4xx response calling CrateApi#handleSendNotification")
    e.printStackTrace()
} catch (e: ServerException) {
    println("5xx response calling CrateApi#handleSendNotification")
    e.printStackTrace()
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sendNotificationBody** | [**SendNotificationBody**](SendNotificationBody.md)|  |

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined

