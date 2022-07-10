/*
 * notifier
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.2.1
 * Contact: 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveChannelBody {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "service_id")]
    pub service_id: String,
}

impl RemoveChannelBody {
    pub fn new(user_id: String, service_id: String) -> RemoveChannelBody {
        RemoveChannelBody {
            user_id,
            service_id,
        }
    }
}

