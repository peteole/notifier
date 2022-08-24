# \CrateApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**HandleAddChannel**](CrateApi.md#HandleAddChannel) | **Post** /add_channel | Add channel
[**HandleGetChannels**](CrateApi.md#HandleGetChannels) | **Get** /get_channels/{user_id} | Get notification channels for user
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
    addChannelBody := *openapiclient.NewAddChannelBody("ServiceUsername_example", "UserId_example", "ServiceId_example") // AddChannelBody | 

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


## HandleGetChannels

> ChannelsResponse HandleGetChannels(ctx, userId).Execute()

Get notification channels for user



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
    userId := "userId_example" // string | User id to get notification channels for

    configuration := openapiclient.NewConfiguration()
    apiClient := openapiclient.NewAPIClient(configuration)
    resp, r, err := apiClient.CrateApi.HandleGetChannels(context.Background(), userId).Execute()
    if err != nil {
        fmt.Fprintf(os.Stderr, "Error when calling `CrateApi.HandleGetChannels``: %v\n", err)
        fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
    }
    // response from `HandleGetChannels`: ChannelsResponse
    fmt.Fprintf(os.Stdout, "Response from `CrateApi.HandleGetChannels`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**userId** | **string** | User id to get notification channels for | 

### Other Parameters

Other parameters are passed through a pointer to a apiHandleGetChannelsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ChannelsResponse**](ChannelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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
    getTelegramChatIdBody := *openapiclient.NewGetTelegramChatIdBody("TelegramUsername_example") // GetTelegramChatIdBody | 

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

> HandleNotify(ctx).NotifyBody(notifyBody).Execute()

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
    notifyBody := *openapiclient.NewNotifyBody("Subject_example", "Message_example", "UserId_example") // NotifyBody | 

    configuration := openapiclient.NewConfiguration()
    apiClient := openapiclient.NewAPIClient(configuration)
    resp, r, err := apiClient.CrateApi.HandleNotify(context.Background()).NotifyBody(notifyBody).Execute()
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
 **notifyBody** | [**NotifyBody**](NotifyBody.md) |  | 

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
    removeChannelBody := *openapiclient.NewRemoveChannelBody("ServiceId_example", "UserId_example") // RemoveChannelBody | 

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

