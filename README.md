# 🌸 Little Helper - Tarah's Personal AI Assistant

A beautiful, privacy-first AI assistant that runs completely on your Mac. Chat with your personal AI to find files, get tech support, and research topics - all while keeping everything private and local!

## ✨ Features
- **🔍 Find Mode**: Ask your AI to find any file on your Mac, Google Drive, or connected drives
- **🔧 Fix Mode**: Get personalized tech support and troubleshooting help
- **🔌 Research Mode**: Have conversations about any topic with your AI
- **🎨 Beautiful Design**: Modern, rounded interface with soft colors
- **🔒 Complete Privacy**: Everything runs locally - no data sent to the internet
- **🤖 Smart Conversations**: Real back-and-forth chat with context memory

## 🚀 One-Click Installation (macOS)

For Tarah's Mac, just run this single command in Terminal:

```bash
curl -fsSL https://raw.githubusercontent.com/M0nkeyFl0wer/claudia-lite/main/install-mac.sh | bash
```

This automatically installs:
- ✅ Little Helper app in Applications
- ✅ Local AI engine (Ollama) 
- ✅ Smart AI model (3GB, optimized for conversations)
- ✅ Auto-startup so it's always ready

## 🎯 How to Use

1. **Open Little Helper** from Applications
2. **Choose your mode**: Find, Fix, or Research  
3. **Start chatting!** Ask questions naturally like:
   - "Find my tax documents from 2023"
   - "My WiFi is acting up, can you help?"
   - "Tell me about sustainable gardening"

Your AI remembers the conversation context and asks follow-up questions to better help you!

## 🛠️ Developer Setup

If you want to build from source:
```bash
# Prerequisites: Rust toolchain + Ollama running locally
cargo run -p app
```
