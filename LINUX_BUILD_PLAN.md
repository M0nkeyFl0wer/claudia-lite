# 🐧 Little Helper - Linux Build Plan

## Overview

Port "Little Helper" to Linux for power users who want a privacy-first AI assistant for system administration, coding, and research.

## Target Audience

### Primary: Power Users & System Administrators
- Use Linux daily for work/personal computing
- Need AI assistance for troubleshooting, coding, configuration
- Privacy-conscious (don't want cloud AI tools)
- Comfortable with terminal but appreciate good UIs

### Secondary: Privacy-Focused General Users
- Switched to Linux for privacy/control
- Want helpful AI without cloud dependency
- Less technical than primary audience

## Key Differences from macOS Version

### macOS (Current)
- **Target User:** Friend/family members (non-technical)
- **Use Cases:** Find files, basic tech support, casual research
- **UI:** Essential (primary interface)
- **Installation:** One-click automated
- **Model:** Small, fast (3-7B parameters)

### Linux (Planned)
- **Target User:** Power users, developers, sysadmins
- **Use Cases:** System admin, debugging, coding, security auditing
- **UI:** Optional (CLI + GUI options)
- **Installation:** More flexible (script + manual options)
- **Model:** Larger, more capable (14-32B parameters)

## Architecture

### Core Components (Shared with macOS)
```
┌─────────────────────────────────────────┐
│         Little Helper Core              │
│         (Rust - cross-platform)         │
├─────────────────────────────────────────┤
│  - Ollama integration                   │
│  - Conversation management              │
│  - Mode system (Find/Fix/Research)      │
│  - File system operations               │
└─────────────────────────────────────────┘
```

### Linux-Specific Additions
```
┌─────────────────────────────────────────┐
│       Linux-Specific Features           │
├─────────────────────────────────────────┤
│  - SystemD service management           │
│  - Package manager integration          │
│    (apt, dnf, pacman, etc.)             │
│  - Log analysis (journalctl)            │
│  - Network monitoring (ss, netstat)     │
│  - Security mode (red team features)    │
│  - SSH integration                      │
└─────────────────────────────────────────┘
```

### UI Options
```
1. CLI Mode (default)
   └─> Terminal-based chat interface
       (like `aider` but with modes)

2. TUI Mode (terminal UI)
   └─> Full-screen terminal app
       (using `ratatui` or `cursive`)

3. GUI Mode (optional)
   └─> GTK4 or Tauri-based desktop app
       (similar to macOS version)
```

## Feature Set

### Phase 1: Core Functionality (MVP)
**Goal:** Match macOS feature parity

- [ ] **Find Mode:** Locate files across filesystem
  - Search home directory
  - Search external drives
  - Filter by type, date, name
  - Smart search (context-aware)

- [ ] **Fix Mode:** System troubleshooting
  - Package management help
  - Service status/restart
  - Log analysis
  - Configuration debugging
  - Permission issues

- [ ] **Research Mode:** General assistance
  - Programming help
  - Explain concepts
  - Draft documentation
  - General Q&A

- [ ] **CLI Interface:** Terminal-based chat
  - Color output
  - Markdown rendering
  - Command history
  - Context memory

### Phase 2: Linux-Specific Features
**Goal:** Leverage Linux advantages

- [ ] **Admin Mode:** System administration
  - Service management (systemctl)
  - Package updates/installation
  - User/group management
  - Firewall configuration review
  - Cron job management

- [ ] **Security Mode:** Red team features
  - Port scanning (local network)
  - Log analysis for anomalies
  - Permission auditing
  - Package vulnerability checking
  - SSH key management
  - OAuth grant auditing

- [ ] **Dev Mode:** Coding assistance
  - Git integration
  - Code review
  - Dependency management
  - Container/Docker help
  - CI/CD pipeline assistance

- [ ] **Network Mode:** SSH/remote systems
  - SSH connection management
  - Remote command execution
  - SCP/rsync assistance
  - Multi-system orchestration

### Phase 3: Advanced Features
**Goal:** Power user capabilities

- [ ] **Multi-Model Support:**
  - Switch between models for different tasks
  - Small model for quick queries (qwen2.5:7b)
  - Large model for complex tasks (qwen2.5:32b)
  - Specialized models (coding, reasoning, etc.)

- [ ] **Tool Integration:**
  - Web search (DuckDuckGo API)
  - OSINT tools (HaveIBeenPwned, etc.)
  - System monitoring (htop, iotop data)
  - Cloud provider CLIs (aws, gcloud, etc.)

- [ ] **Automation:**
  - Save common workflows
  - Schedule tasks
  - Background monitoring
  - Alert notifications

- [ ] **Collaboration:**
  - Share configurations
  - Export conversation threads
  - Generate documentation from sessions

## Technical Implementation

### Installation Methods

#### 1. One-Line Installer (Recommended)
```bash
curl -fsSL https://raw.githubusercontent.com/M0nkeyFl0wer/claudia-lite/main/install-linux.sh | bash
```

**What it does:**
- Detects distro (Ubuntu/Debian, Fedora, Arch, etc.)
- Installs dependencies (Ollama, Rust if building from source)
- Downloads/builds Little Helper
- Installs to `~/.local/bin/` or `/usr/local/bin/`
- Sets up shell completion (bash, zsh, fish)
- Pulls recommended model (qwen2.5-coder:14b)
- Creates desktop entry (for GUI mode)
- Optionally sets up systemd service

#### 2. Distribution Packages
```bash
# Debian/Ubuntu
sudo dpkg -i little-helper_1.0.0_amd64.deb

# Fedora
sudo dnf install little-helper-1.0.0.x86_64.rpm

# Arch (AUR)
yay -S little-helper

# Snap (universal)
sudo snap install little-helper
```

#### 3. Manual Installation
```bash
# Clone repo
git clone https://github.com/M0nkeyFl0wer/claudia-lite
cd claudia-lite

# Build
cargo build --release --bin little-helper-linux

# Install
sudo cp target/release/little-helper-linux /usr/local/bin/little-helper

# Set up
little-helper --setup
```

### Directory Structure
```
~/.config/little-helper/
├── config.toml           # User configuration
├── conversations/        # Chat history
│   ├── 2025-10-23-fix-ssh.json
│   └── 2025-10-24-debug-systemd.json
├── models/              # Model preferences
│   └── preferred.txt
└── workflows/           # Saved automation scripts

~/.local/share/little-helper/
├── logs/               # Application logs
└── cache/              # Temporary data

/usr/share/little-helper/  (system-wide)
└── examples/           # Example configurations
```

### Configuration File (config.toml)
```toml
[user]
name = "Ben"              # Personalizes to "Ben's Little Helper"
shell = "/bin/bash"

[ollama]
host = "http://localhost:11434"
default_model = "qwen2.5-coder:14b"

[models]
quick = "qwen2.5:7b"      # Fast responses
coding = "qwen2.5-coder:32b"  # Code generation
reasoning = "deepseek-r1:14b"  # Complex problems
security = "qwen2.5:14b"  # Security mode

[features]
web_search = true         # Enable DuckDuckGo integration
security_mode = false     # Enable red team features (opt-in)
background_monitoring = false  # Daily security scans

[ui]
interface = "cli"         # cli, tui, or gui
color_scheme = "auto"     # auto, dark, light
markdown_rendering = true

[security]
allow_sudo = false        # Require confirmation for sudo commands
log_commands = true       # Log all executed commands
max_file_size = "100MB"   # Max file size to analyze
```

### CLI Usage

```bash
# Start interactive session (default mode: research)
little-helper

# Start in specific mode
little-helper --mode find
little-helper --mode fix
little-helper --mode research
little-helper --mode security
little-helper --mode admin

# One-shot question
little-helper "How do I restart nginx?"
little-helper --mode find "Where are my SSH keys?"

# Use specific model
little-helper --model qwen2.5-coder:32b "Write a Rust HTTP server"

# Background security monitoring
little-helper --mode security --background

# View security report
little-helper --mode security --report

# Setup wizard
little-helper --setup
```

### Example Interactions

**Find Mode:**
```
$ little-helper --mode find

Ben's Little Helper (Find Mode)
What are you looking for?

> My bash configuration files

I'll search for bash configuration files in your home directory.
Found 5 files:

1. ~/.bashrc (modified 2 hours ago)
2. ~/.bash_profile (modified 3 days ago)
3. ~/.bash_aliases (modified 1 week ago)
4. ~/.bash_history (modified 1 hour ago)
5. ~/.config/bash/custom.sh (modified 2 weeks ago)

Would you like me to explain what each file does?
```

**Fix Mode:**
```
$ little-helper --mode fix

Ben's Little Helper (Fix Mode)
What's the problem?

> SSH isn't working, getting "connection refused"

Let me help diagnose the SSH issue. I'll check:
1. Is the SSH service running?
2. Is it listening on the correct port?
3. Are there any firewall rules blocking it?

Running diagnostics...

Found the issue:
❌ SSH service is not running

Would you like me to:
1. Start the SSH service now (systemctl start sshd)
2. Enable it to start on boot (systemctl enable sshd)
3. Both of the above
4. Show me more details first

What would you like to do? [1/2/3/4]
```

**Security Mode:**
```
$ little-helper --mode security --report

Ben's Little Helper - Security Report
Generated: 2025-10-24 09:00

🔴 Critical Issues (2)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. SSH exposed on default port 22
   • Detected: netstat shows :22 listening on 0.0.0.0
   • Risk: Common brute force target
   • Fix: Change SSH port in /etc/ssh/sshd_config

2. Credential in bash history
   • File: ~/.bash_history line 342
   • Pattern: export API_KEY=sk_live_xxx
   • Fix: Remove from history, use .env file

🟠 High Priority (5)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. 23 packages with security updates available
2. Firewall (ufw) is not enabled
3. Unused OAuth grant: "Random AI Tool" (unused 6 months)
...

Would you like me to help fix any of these? [y/n]
```

## Security Considerations

### Safe by Design
- **Local Processing:** All AI runs on-device via Ollama
- **No Telemetry:** Zero data sent to external servers
- **Transparent:** All commands shown before execution
- **User Control:** Requires confirmation for privileged operations

### Additional Protections
- **Command Logging:** All executed commands saved to log
- **Sudo Protection:** Never runs sudo without explicit approval
- **File Size Limits:** Won't process huge files (DoS protection)
- **Network Isolation:** Can run in offline mode

### Security Mode Features
- **Read-Only by Default:** Security scans don't modify system
- **API-Based OSINT:** Only uses safe APIs (no web scraping)
- **Privacy-Preserving:** Email hashes for breach checks, not raw emails
- **Local Only:** All scans happen locally, results stay local

## Distribution Strategy

### Phase 1: GitHub Release
- Pre-built binaries for major distros
- One-line installer script
- Documentation and examples
- Community building

### Phase 2: Package Repositories
- Submit to AUR (Arch User Repository)
- Create .deb packages (Ubuntu/Debian)
- Create .rpm packages (Fedora/RHEL)
- Snap package (universal)
- Flatpak (sandboxed)

### Phase 3: Promotion
- Reddit (r/linux, r/selfhosted, r/privacy)
- Hacker News
- Linux YouTube channels
- Blog posts about privacy-first AI

## Development Roadmap

### Milestone 1: Core CLI (2-3 weeks)
- [ ] Port existing Rust code to Linux
- [ ] Implement CLI interface
- [ ] Add Find/Fix/Research modes
- [ ] Create installer script
- [ ] Write documentation

### Milestone 2: Linux Features (2-3 weeks)
- [ ] Admin mode (systemd, packages)
- [ ] Security mode (basic scanning)
- [ ] Network mode (SSH integration)
- [ ] Configuration system
- [ ] Shell completion

### Milestone 3: Polish (1-2 weeks)
- [ ] TUI interface (optional)
- [ ] GUI interface (optional)
- [ ] Distribution packages
- [ ] Testing on multiple distros
- [ ] User documentation

### Milestone 4: Advanced Features (ongoing)
- [ ] Web search integration
- [ ] Multi-model support
- [ ] Background monitoring
- [ ] Workflow automation
- [ ] Tool integrations

## Marketing Angles

### For Privacy Users
> "Your AI assistant that **can't snitch** - because it never talks to the cloud"

### For Power Users
> "The AI sysadmin that lives on **your** machine, speaks **your** shell, and knows **your** system"

### For Security-Conscious
> "An AI that **watches your back** instead of watching you"

### Differentiation from Other Tools
- **vs Claude Code/Cursor:** Fully local, no cloud dependency
- **vs ChatGPT/Claude API:** Privacy-first, no data leaves machine
- **vs GitHub Copilot:** No telemetry, works offline
- **vs Local LLMs:** Easier setup, specialized for Linux sysadmin

## Success Metrics

### Technical
- [ ] Runs on Ubuntu, Fedora, Arch, Debian
- [ ] Installer works on fresh installs
- [ ] <5 minute setup time
- [ ] <2GB RAM usage
- [ ] Works offline after initial setup

### User Experience
- [ ] Clear, helpful responses
- [ ] Fast response time (<2s for simple queries)
- [ ] Non-technical users can install
- [ ] Power users find it genuinely useful

### Community
- [ ] 100+ GitHub stars
- [ ] 10+ contributors
- [ ] Package in major distro repos
- [ ] Positive feedback on Reddit/HN

## Open Questions

1. **Model Size Trade-offs:**
   - Default to 7B (fast, fits most systems) or 14B (better quality)?
   - How to handle users with limited RAM/GPU?

2. **GUI vs CLI Priority:**
   - Focus on CLI first (faster to build, matches Linux users)?
   - Or build GUI to attract less technical users?

3. **Security Mode Scope:**
   - How aggressive should automated security scanning be?
   - Opt-in or opt-out for invasive features?

4. **Tool Integration:**
   - Which external tools to integrate first?
   - How to handle dependencies users might not have?

5. **Remote Execution:**
   - Allow SSH to remote systems?
   - How to prevent accidental damage to production systems?

## Next Steps

1. **Validate Concept:** Test basic CLI with target users
2. **Build MVP:** Core Find/Fix/Research modes in CLI
3. **Gather Feedback:** Share with Linux communities
4. **Iterate:** Add most-requested features
5. **Polish:** Package for easy distribution
6. **Launch:** Promote on Linux forums/communities

## Resources Needed

- **Development Time:** 6-8 weeks for MVP
- **Testing:** Multiple Linux VMs (Ubuntu, Fedora, Arch)
- **Documentation:** User guide, API docs, examples
- **Community:** Beta testers, early adopters, contributors

---

## Getting Started (For Contributors)

```bash
# Clone the repo
git clone https://github.com/M0nkeyFl0wer/claudia-lite
cd claudia-lite

# Checkout Linux branch
git checkout linux-build

# Install dependencies
curl -fsSL https://ollama.com/install.sh | sh
ollama pull qwen2.5-coder:14b

# Build
cargo build --release --bin little-helper-linux

# Run
./target/release/little-helper-linux --setup
```

## Contact

Questions? Ideas? Want to contribute?
- GitHub Issues: https://github.com/M0nkeyFl0wer/claudia-lite/issues
- Discussions: https://github.com/M0nkeyFl0wer/claudia-lite/discussions

Let's build something that respects user privacy while being genuinely helpful! 🐧
