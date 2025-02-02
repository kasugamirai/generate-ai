mod client;
mod error;
mod model;
mod request;
mod response;
mod role;
mod utils;
pub use client::embed;
pub use client::stream_completion;
pub use client::OpenAiEmbedding;
pub use client::OpenAiEmbeddingModel;
pub use error::OpenAiClientError;
pub use model::Model;
pub use request::Request;
pub use request::RequestMessage;
pub use response::ResponseStreamEvent;
pub use role::Role;
pub use utils::is_none_or_empty;
