# Claudia Lite (local-first terminal agent GUI)

Goals
- Simple, privacy-first GUI for file finding, organizing, and support.
- Local models via Ollama by default; optional Gemini fallback (opt-in).

Build
- Prereqs: Rust toolchain; optional: Ollama running locally.
- Run: cargo run -p app

Env (optional)
- OLLAMA_BASE_URL (default: http://127.0.0.1:11434)
- GEMINI_API_KEY (optional, enables fallback)

Notes
- This is a scaffold with basic search UI; organizer/support are stubs to be filled next.
