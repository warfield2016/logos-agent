//! LLM backend — pluggable inference trait.
//!
//! Default: `llama.cpp` via `llama-cpp-2` (local, no network).
//! Alt: Ollama HTTP, OpenAI-compatible API, custom.

use async_trait::async_trait;

#[async_trait]
pub trait InferenceBackend: Send + Sync {
    async fn complete(&self, prompt: &str, opts: CompletionOpts) -> anyhow::Result<String>;
    fn name(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct CompletionOpts {
    pub max_tokens: usize,
    pub temperature: f32,
    pub stop: Vec<String>,
}

impl Default for CompletionOpts {
    fn default() -> Self {
        Self {
            max_tokens: 512,
            temperature: 0.7,
            stop: vec![],
        }
    }
}

// Concrete backends added in Week-2: llama_cpp.rs, ollama.rs, api.rs.
