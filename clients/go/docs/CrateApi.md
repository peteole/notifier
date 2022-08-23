# \CrateApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**HandleAddChannel**](CrateApi.md#HandleAddChannel) | **Post** /add_channel | Add channel
[**HandleGetTelegramChatId**](CrateApi.md#HandleGetTelegramChatId) | **Post** /get_telegram_chat_id | Get the chat ID of a telegram username
[**HandleNotify**](CrateApi.md#HandleNotify) | **Post** /notify | Send notification
[**HandleRemoveChannel**](CrateApi.md#HandleRemoveChannel) | **Post** /remove_channel | Remove channel



## HandleAddChannel

> HandleAddChannel(ctx).AddChannelBody(addChannelBody).Execute()

Add channel



### Example

```go
package main

import (
    "context"
    "fmt"
    "os"
    openapiclient "./openapi"
)

func main() {
    addChannelBody := *openapiclient.NewAddChannelBody("UserId_example", "ServiceUsername_example", "ServiceId_example") // AddChannelBody | 

    configuration := openapiclient.NewConfiguration()
    apiClient := openapiclient.NewAPIClient(configuration)
    resp, r, err := apiClient.CrateApi.HandleAddChannel(context.Background()).AddChannelBody(addChannelBody).Execute()
    if err != nil {
        fmt.Fprintf(os.Stderr, "Error when calling `CrateApi.HandleAddChannel``: %v\n", err)
        fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
    }
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiHandleAddChannelRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **addChannelBody** | [**AddChannelBody**](AddChannelBody.md) |  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## HandleGetTelegramChatId

> HandleGetTelegramChatId(ctx).GetTelegramChatIdBody(getTelegramChatIdBody).Execute()

Get the chat ID of a telegram username



### Example

```go
package main

import (
    "context"
    "fmt"
    "os"
    openapiclient "./openapi"
)

func main() {
    getTelegramChatIdBody := *openapiclient.NewGetTelegramChatIdBody("TelegramUsername_example", "UserId_example") // GetTelegramChatIdBody | 

    configuration := openapiclient.NewConfiguration()
    apiClient := openapiclient.NewAPIClient(configuration)
    resp, r, err := apiClient.CrateApi.HandleGetTelegramChatId(context.Background()).GetTelegramChatIdBody(getTelegramChatIdBody).Execute()
    if err != nil {
        fmt.Fprintf(os.Stderr, "Error when calling `CrateApi.HandleGetTelegramChatId``: %v\n", err)
        fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
    }
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiHandleGetTelegramChatIdRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **getTelegramChatIdBody** | [**GetTelegramChatIdBody**](GetTelegramChatIdBody.md) |  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## HandleNotify

> HandleNotify(ctx).SendNotificationBody(sendNotificationBody).Execute()

Send notification



### Example

```go
package main

import (
    "context"
    "fmt"
    "os"
    openapiclient "./openapi"
)

func main() {
    sendNotificationBody := *openapiclient.NewSendNotificationBody("Subject_example", "UserId_example", "Message_example") // SendNotificationBody | 

    configuration := openapiclient.NewConfiguration()
    apiClient := openapiclient.NewAPIClient(configuration)
    resp, r, err := apiClient.CrateApi.HandleNotify(context.Background()).SendNotificationBody(sendNotificationBody).Execute()
    if err != nil {
        fmt.Fprintf(os.Stderr, "Error when calling `CrateApi.HandleNotify``: %v\n", err)
        fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
    }
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiHandleNotifyRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sendNotificationBody** | [**SendNotificationBody**](SendNotificationBody.md) |  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## HandleRemoveChannel

> HandleRemoveChannel(ctx).RemoveChannelBody(removeChannelBody).Execute()

Remove channel



### Example

```go
package main

import (
    "context"
    "fmt"
    "os"
    openapiclient "./openapi"
)

func main() {
    removeChannelBody := *openapiclient.NewRemoveChannelBody("UserId_example", "ServiceId_example") // RemoveChannelBody | 

    configuration := openapiclient.NewConfiguration()
    apiClient := openapiclient.NewAPIClient(configuration)
    resp, r, err := apiClient.CrateApi.HandleRemoveChannel(context.Background()).RemoveChannelBody(removeChannelBody).Execute()
    if err != nil {
        fmt.Fprintf(os.Stderr, "Error when calling `CrateApi.HandleRemoveChannel``: %v\n", err)
        fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
    }
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiHandleRemoveChannelRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **removeChannelBody** | [**RemoveChannelBody**](RemoveChannelBody.md) |  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

