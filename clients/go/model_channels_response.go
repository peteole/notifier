/*
notifier

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

API version: 0.3.1
Contact: 
*/

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package openapi

import (
	"encoding/json"
)

// ChannelsResponse struct for ChannelsResponse
type ChannelsResponse struct {
	Channels []ChannelResponse `json:"channels"`
	UserId string `json:"user_id"`
}

// NewChannelsResponse instantiates a new ChannelsResponse object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewChannelsResponse(channels []ChannelResponse, userId string) *ChannelsResponse {
	this := ChannelsResponse{}
	this.Channels = channels
	this.UserId = userId
	return &this
}

// NewChannelsResponseWithDefaults instantiates a new ChannelsResponse object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewChannelsResponseWithDefaults() *ChannelsResponse {
	this := ChannelsResponse{}
	return &this
}

// GetChannels returns the Channels field value
func (o *ChannelsResponse) GetChannels() []ChannelResponse {
	if o == nil {
		var ret []ChannelResponse
		return ret
	}

	return o.Channels
}

// GetChannelsOk returns a tuple with the Channels field value
// and a boolean to check if the value has been set.
func (o *ChannelsResponse) GetChannelsOk() ([]ChannelResponse, bool) {
	if o == nil {
		return nil, false
	}
	return o.Channels, true
}

// SetChannels sets field value
func (o *ChannelsResponse) SetChannels(v []ChannelResponse) {
	o.Channels = v
}

// GetUserId returns the UserId field value
func (o *ChannelsResponse) GetUserId() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.UserId
}

// GetUserIdOk returns a tuple with the UserId field value
// and a boolean to check if the value has been set.
func (o *ChannelsResponse) GetUserIdOk() (*string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.UserId, true
}

// SetUserId sets field value
func (o *ChannelsResponse) SetUserId(v string) {
	o.UserId = v
}

func (o ChannelsResponse) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if true {
		toSerialize["channels"] = o.Channels
	}
	if true {
		toSerialize["user_id"] = o.UserId
	}
	return json.Marshal(toSerialize)
}

type NullableChannelsResponse struct {
	value *ChannelsResponse
	isSet bool
}

func (v NullableChannelsResponse) Get() *ChannelsResponse {
	return v.value
}

func (v *NullableChannelsResponse) Set(val *ChannelsResponse) {
	v.value = val
	v.isSet = true
}

func (v NullableChannelsResponse) IsSet() bool {
	return v.isSet
}

func (v *NullableChannelsResponse) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableChannelsResponse(val *ChannelsResponse) *NullableChannelsResponse {
	return &NullableChannelsResponse{value: val, isSet: true}
}

func (v NullableChannelsResponse) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableChannelsResponse) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


