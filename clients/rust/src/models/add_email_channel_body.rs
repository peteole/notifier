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
pub struct AddEmailChannelBody {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
}

impl AddEmailChannelBody {
    pub fn new(email: String, user_id: String) -> AddEmailChannelBody {
        AddEmailChannelBody {
            email,
            user_id,
        }
    }
}


