mod open_ai;
pub use open_ai::embed;
pub use open_ai::stream_completion;
pub use open_ai::Model;
pub use open_ai::OpenAiEmbedding;
pub use open_ai::OpenAiEmbeddingModel;
pub use open_ai::Request;
pub use open_ai::RequestMessage;

pub const OPEN_AI_API_URL: &str = "https://api.openai.com/v1";
