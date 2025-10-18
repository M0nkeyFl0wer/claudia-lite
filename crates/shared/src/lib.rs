pub mod settings {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ModelProvider {
        pub local_model: String,      // e.g., "llama3.1:8b-instruct-q4"
        pub enable_gemini: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AppSettings {
        pub allowed_dirs: Vec<String>,
        pub model: ModelProvider,
        pub enable_internet_research: bool,
        pub max_results: usize,
    }

    impl Default for AppSettings {
        fn default() -> Self {
            Self {
                allowed_dirs: vec![],
                model: ModelProvider { local_model: "llama3.2:3b".into(), enable_gemini: false },
                enable_internet_research: false,
                max_results: 200,
            }
        }
    }
}

pub mod agent_api {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ChatMessage {
        pub role: String,   // "system" | "user" | "assistant"
        pub content: String,
    }
}

pub mod search_types {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SearchQuery {
        pub text: String,
        pub extensions: Option<Vec<String>>, // e.g., ["pdf","md"]
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SearchResult {
        pub path: String,
        pub file_name: String,
        pub size_bytes: u64,
        pub modified: Option<i64>, // unix timestamp
        pub score: f32,
    }
}
