# \CrateApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**HandleAddEmailChannel**](CrateApi.md#HandleAddEmailChannel) | **Post** /add_channel/email | Add email channel
[**HandleAddTelegramChannel**](CrateApi.md#HandleAddTelegramChannel) | **Post** /add_channel/telegram | Add telegram channel
[**HandleSendNotification**](CrateApi.md#HandleSendNotification) | **Post** /notify | Send notification



## HandleAddEmailChannel

> HandleAddEmailChannel(ctx).AddEmailChannelBody(addEmailChannelBody).Execute()

Add email channel



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
    addEmailChannelBody := *openapiclient.NewAddEmailChannelBody("Email_example", "UserId_example") // AddEmailChannelBody | 

    configuration := openapiclient.NewConfiguration()
    apiClient := openapiclient.NewAPIClient(configuration)
    resp, r, err := apiClient.CrateApi.HandleAddEmailChannel(context.Background()).AddEmailChannelBody(addEmailChannelBody).Execute()
    if err != nil {
        fmt.Fprintf(os.Stderr, "Error when calling `CrateApi.HandleAddEmailChannel``: %v\n", err)
        fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
    }
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiHandleAddEmailChannelRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **addEmailChannelBody** | [**AddEmailChannelBody**](AddEmailChannelBody.md) |  | 

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


## HandleAddTelegramChannel

> HandleAddTelegramChannel(ctx).AddTelegramChannelBody(addTelegramChannelBody).Execute()

Add telegram channel



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
    addTelegramChannelBody := *openapiclient.NewAddTelegramChannelBody("UserId_example", "TelegramUsername_example") // AddTelegramChannelBody | 

    configuration := openapiclient.NewConfiguration()
    apiClient := openapiclient.NewAPIClient(configuration)
    resp, r, err := apiClient.CrateApi.HandleAddTelegramChannel(context.Background()).AddTelegramChannelBody(addTelegramChannelBody).Execute()
    if err != nil {
        fmt.Fprintf(os.Stderr, "Error when calling `CrateApi.HandleAddTelegramChannel``: %v\n", err)
        fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
    }
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiHandleAddTelegramChannelRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **addTelegramChannelBody** | [**AddTelegramChannelBody**](AddTelegramChannelBody.md) |  | 

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


## HandleSendNotification

> HandleSendNotification(ctx).SendNotificationBody(sendNotificationBody).Execute()

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
    resp, r, err := apiClient.CrateApi.HandleSendNotification(context.Background()).SendNotificationBody(sendNotificationBody).Execute()
    if err != nil {
        fmt.Fprintf(os.Stderr, "Error when calling `CrateApi.HandleSendNotification``: %v\n", err)
        fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
    }
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiHandleSendNotificationRequest struct via the builder pattern


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

