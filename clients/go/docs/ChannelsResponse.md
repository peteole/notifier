# ChannelsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Channels** | [**[]ChannelResponse**](ChannelResponse.md) |  | 
**UserId** | **string** |  | 

## Methods

### NewChannelsResponse

`func NewChannelsResponse(channels []ChannelResponse, userId string, ) *ChannelsResponse`

NewChannelsResponse instantiates a new ChannelsResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewChannelsResponseWithDefaults

`func NewChannelsResponseWithDefaults() *ChannelsResponse`

NewChannelsResponseWithDefaults instantiates a new ChannelsResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetChannels

`func (o *ChannelsResponse) GetChannels() []ChannelResponse`

GetChannels returns the Channels field if non-nil, zero value otherwise.

### GetChannelsOk

`func (o *ChannelsResponse) GetChannelsOk() (*[]ChannelResponse, bool)`

GetChannelsOk returns a tuple with the Channels field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChannels

`func (o *ChannelsResponse) SetChannels(v []ChannelResponse)`

SetChannels sets Channels field to given value.


### GetUserId

`func (o *ChannelsResponse) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *ChannelsResponse) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *ChannelsResponse) SetUserId(v string)`

SetUserId sets UserId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


