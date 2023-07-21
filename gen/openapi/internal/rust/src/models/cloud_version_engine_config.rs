/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudVersionEngineConfig {
    #[serde(rename = "godot", skip_serializing_if = "Option::is_none")]
    pub godot: Option<serde_json::Value>,
    #[serde(rename = "html5", skip_serializing_if = "Option::is_none")]
    pub html5: Option<serde_json::Value>,
    #[serde(rename = "unity", skip_serializing_if = "Option::is_none")]
    pub unity: Option<serde_json::Value>,
    #[serde(rename = "unreal", skip_serializing_if = "Option::is_none")]
    pub unreal: Option<Box<crate::models::CloudVersionEngineUnrealConfig>>,
}

impl CloudVersionEngineConfig {
    pub fn new() -> CloudVersionEngineConfig {
        CloudVersionEngineConfig {
            godot: None,
            html5: None,
            unity: None,
            unreal: None,
        }
    }
}


