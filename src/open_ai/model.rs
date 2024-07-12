use super::OpenAiClientError;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum Model {
    #[serde(rename = "gpt-3.5-turbo", alias = "gpt-3.5-turbo-0613")]
    ThreePointFiveTurbo,
    #[serde(rename = "gpt-4", alias = "gpt-4-0613")]
    Four,
    #[serde(rename = "gpt-4-turbo-preview", alias = "gpt-4-1106-preview")]
    FourTurbo,
    #[serde(rename = "gpt-4o", alias = "gpt-4o-2024-05-13")]
    #[default]
    FourOmni,
    #[serde(rename = "custom")]
    Custom { name: String, max_tokens: usize },
}

impl Model {
    pub fn from_id(id: &str) -> Result<Self, OpenAiClientError> {
        match id {
            "gpt-3.5-turbo" => Ok(Self::ThreePointFiveTurbo),
            "gpt-4" => Ok(Self::Four),
            "gpt-4-turbo-preview" => Ok(Self::FourTurbo),
            "gpt-4o" => Ok(Self::FourOmni),
            _ => Err(OpenAiClientError::InvalidModelId),
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            Self::ThreePointFiveTurbo => "gpt-3.5-turbo",
            Self::Four => "gpt-4",
            Self::FourTurbo => "gpt-4-turbo-preview",
            Self::FourOmni => "gpt-4o",
            Self::Custom { .. } => "custom",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::ThreePointFiveTurbo => "gpt-3.5-turbo",
            Self::Four => "gpt-4",
            Self::FourTurbo => "gpt-4-turbo",
            Self::FourOmni => "gpt-4o",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Model::ThreePointFiveTurbo => 4096,
            Model::Four => 8192,
            Model::FourTurbo => 128000,
            Model::FourOmni => 128000,
            Model::Custom { max_tokens, .. } => *max_tokens,
        }
    }
}
