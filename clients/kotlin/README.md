# org.openapitools.client - Kotlin client library for notifier

## Requires

* Kotlin 1.4.30
* Gradle 6.8.3

## Build

First, create the gradle wrapper script:

```
gradle wrapper
```

Then, run:

```
./gradlew check assemble
```

This runs all tests and packages the library.

## Features/Implementation Notes

* Supports JSON inputs/outputs, File inputs, and Form inputs.
* Supports collection formats for query parameters: csv, tsv, ssv, pipes.
* Some Kotlin and Java types are fully qualified to avoid conflicts with types defined in OpenAPI definitions.
* Implementation of ApiClient is intended to reduce method counts, specifically to benefit Android targets.

<a name="documentation-for-api-endpoints"></a>
## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*CrateApi* | [**handleAddEmailChannel**](docs/CrateApi.md#handleaddemailchannel) | **POST** /add_channel/email | Add email channel
*CrateApi* | [**handleAddTelegramChannel**](docs/CrateApi.md#handleaddtelegramchannel) | **POST** /add_channel/telegram | Add telegram channel
*CrateApi* | [**handleSendNotification**](docs/CrateApi.md#handlesendnotification) | **POST** /notify | Send notification


<a name="documentation-for-models"></a>
## Documentation for Models

 - [org.openapitools.client.models.AddEmailChannelBody](docs/AddEmailChannelBody.md)
 - [org.openapitools.client.models.AddTelegramChannelBody](docs/AddTelegramChannelBody.md)
 - [org.openapitools.client.models.RemoveChannelBody](docs/RemoveChannelBody.md)
 - [org.openapitools.client.models.SendNotificationBody](docs/SendNotificationBody.md)


<a name="documentation-for-authorization"></a>
## Documentation for Authorization

All endpoints do not require authorization.
