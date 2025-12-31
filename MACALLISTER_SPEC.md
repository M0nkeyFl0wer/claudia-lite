# Little Helper for MacAllister Polling - Specification

## Project Overview
Transform claudia-lite into a team IDE for MacAllister Polling & Research - providing terminal-level AI assistance for non-technical users.

## Core Vision
**"Training wheels for the command line"**
- Terminal agents in the background (invisible to users)
- Clean editor + preview interface
- Pre-loaded templates and skills
- Shared team workspace via Google Drive
- Slack integration for commands

---

## Users
- MacAllister Polling team (all on Mac)
- Same Google Workspace account
- Same Slack workspace
- Non-technical users who need AI assistance

---

## Core Features

### 1. Tiling Window Layout
```
┌──────────────────────────────────────┐
│  Little Helper    [Find|Fix|Research]│
├──────────────┬──────────────┬────────┤
│   Editor     │   Preview    │  Chat  │
│              │              │        │
│  Edit files  │  View render │  AI    │
│  (.md, .txt) │  (markdown)  │  Help  │
└──────────────┴──────────────┴────────┘
```

### 2. File Type Support

**Phase 1 (Simple App):**
- ✅ Markdown (.md) - edit + preview
- ✅ Plain text (.txt) - edit only
- ⏸️ Other formats - open in system viewer

**Phase 2 (VS Codium Fork):**
- ✅ All file types via extensions
- ✅ PDF viewer
- ✅ Word/Excel preview
- ✅ Images, videos
- ✅ Tableau files

### 3. Terminal Agent (Background)
- Executes commands invisibly
- No terminal UI shown to users
- Output appears in chat as "System" messages
- AI decides what commands to run
- Safe mode: whitelist allowed commands

### 4. Shared Workspace
- **Google Drive access** via Desktop app
  - Domain: mcallister-research.com
  - Path: `~/Google Drive/` or `~/Google Drive/Shared drives/[folder]/`
  - Real-time sync (already works)
- **Templates pre-loaded:**
  1. Poll/survey templates
  2. Topline report templates
  3. Tableau templates
  4. Verbatim report templates
  5. Targeted analysis report templates

### 5. Slack Integration
- Slack bot receives commands
- Example: `@LittleHelper find Q4 polling template`
- Bot executes via terminal agent
- Posts results back to Slack
- Can work with files in Google Drive

### 6. Skills System (Future)
Pre-configured workflows for common tasks:
- "Create polling question"
- "Generate topline report"
- "Analyze survey results"
- "Draft social media content"
- "Monitor campaign analytics"

### 7. MCP/API Integration (Future)
Connect to external tools:
- Canva (design automation)
- 11 Labs (voice/audio)
- FFmpeg (video editing)
- Chrome automation
- Social media APIs
- Analytics platforms

---

## Two-Track Development

### Track 1: Simple App (claudia-lite enhanced)
**Timeline:** 2-4 weeks
**Approach:** Modify existing Rust/egui app
**Outcome:** Quick working prototype

**Features:**
- Tiling layout (editor | preview | chat)
- Basic file operations
- Markdown support
- Google Drive folder access
- Template loading
- Slack bot

### Track 2: VS Codium Fork
**Timeline:** 4-8 weeks
**Approach:** Customize VS Codium
**Outcome:** Full-featured IDE

**Features:**
- All file type support
- Extension ecosystem
- Polished UX
- Integrated AI chat
- Hidden complexity
- Professional look

---

## Non-Goals (Out of Scope)

❌ Multi-user authentication (all team uses same Google/Slack)
❌ Version control UI (use git via terminal agent if needed)
❌ Database integration (files only for now)
❌ Mobile app (Mac desktop only initially)
❌ Real-time collaboration (Google Drive handles sync)

---

## Success Criteria

### MVP (Minimum Viable Product)
- [ ] Can edit markdown files
- [ ] Can preview rendered markdown
- [ ] Can access Google Drive team folder
- [ ] Can load and use templates
- [ ] Chat with AI for help
- [ ] Works on Mac
- [ ] Team can install via .app file

### V1.0 (First Release)
- [ ] All MVP features
- [ ] Slack bot integration
- [ ] Terminal agent executes commands
- [ ] Pre-loaded 5 template types
- [ ] Clean UX (no scary features)
- [ ] Distributed to full team

### V2.0 (Enhanced)
- [ ] VS Codium fork ready
- [ ] PDF/Office preview
- [ ] MCP integration
- [ ] Skills system
- [ ] Windows/Linux builds

---

## Technical Stack

### Simple App (Track 1)
- **Language:** Rust
- **UI:** egui (immediate-mode GUI)
- **AI:** Ollama (local) + Gemini (fallback)
- **Backend:** Existing claudia-lite services
- **Build:** Cargo, Mac .app bundle

### VS Codium Fork (Track 2)
- **Base:** VS Codium (Electron)
- **Frontend:** Monaco editor + custom UI
- **Backend:** Rust (reuse claudia-lite)
- **IPC:** Electron ↔ Rust communication
- **Extensions:** Pre-bundled for file types

---

## Dependencies

### User Must Provide:
1. Google Drive folder structure/path
2. Template files (5 types)
3. Slack workspace name/channel
4. API keys (if using external tools)

### Software Requirements:
- macOS 11+
- Google Drive Desktop app
- Ollama installed (or auto-install)
- Slack workspace access

---

## Distribution

### Installation
1. Download .app file
2. Move to /Applications/
3. First run: Grant permissions (Files, Network)
4. Auto-configure Google Drive path
5. Load templates
6. Ready to use

### Updates
- Manual download initially
- Auto-update mechanism (future)
- Notify team via Slack when new version ready

---

## Open Source Plan

### Timeline
- Use internally: 3-6 months
- Gather feedback, refine
- Remove company-specific configs
- Document customization
- Release on GitHub
- MIT License

### What's Open Sourced
- Core app code
- Template system (generic)
- Slack integration (generic)
- Build scripts

### What Stays Private
- MacAllister-specific templates
- API keys
- Client data
- Company configurations

---

## Questions Still Needed

1. ✅ **Google Drive domain:** mcallister-research.com
2. ✅ **Slack workspace:** mcallister-research.slack.com
3. ⏳ **Google Drive path:** Exact folder structure? (Shared drive name? Folder path?)
4. ⏳ **Templates:** Can you share the 5 template files?
5. ⏳ **Slack channel:** Which channel for bot?
6. ⏳ **File formats:** Are templates .md, .docx, or mixed?
7. ⏳ **Priorities:** Which track to prioritize if one gets delayed?

---

## Next Steps

1. ✅ Agents working on both tracks
2. ⏳ Gather info from questions above
3. ⏳ Test simple app prototype (Agent 1)
4. ⏳ Review VS Codium plan (Agent 2)
5. ⏳ Choose primary track or continue both
6. ⏳ Build MVP
7. ⏳ Test with team
8. ⏳ Iterate based on feedback

---

**Last Updated:** 2025-12-12
**Status:** In Development
**Primary Contact:** Ben (user)
