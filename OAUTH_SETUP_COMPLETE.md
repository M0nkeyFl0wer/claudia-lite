# OAuth Setup Complete!

## What Just Happened

Little Helper now automatically uses your **Claude Max subscription** via OAuth!

## How It Works

1. **Claude Code manages OAuth** - When you sign in with `claude`, it stores credentials at `~/.claude/.credentials.json`
2. **Little Helper reuses those credentials** - No separate API key needed!
3. **Automatic fallback** - If Claude Code isn't signed in, falls back to `ANTHROPIC_API_KEY`

## Your Current Setup

```bash
Subscription: max
Rate Limit: default_claude_max_5x (5x higher than standard!)
```

## Authentication Priority

Little Helper tries authentication in this order:

1. **Claude Code OAuth** (PREFERRED) → Uses your Max subscription
2. **ANTHROPIC_API_KEY** → Falls back to API key if OAuth unavailable
3. **Ollama** → Local AI if no Claude authentication
4. **Gemini** → Google AI as last resort

## Usage

### Option 1: Just Run It! (Recommended)

```bash
# No setup needed - OAuth credentials already exist!
/home/flower/Downloads/claudia-lite/target/release/app
```

You should see:
```
✓ Using Claude Code authentication: OAuth (max)
```

### Option 2: Quick Test

```bash
# Run test script
/home/flower/Downloads/claudia-lite/test-oauth.sh

# Or run directly
./target/release/app
```

## What Changed from Before

### Old Method (API Key):
- Required `ANTHROPIC_API_KEY` environment variable
- Uses API billing
- Separate from Claude Max subscription

### New Method (OAuth):
- Reuses Claude Code authentication
- Uses your Claude Max subscription
- 5x higher rate limits
- No environment variables needed

## Token Expiration Handling

OAuth tokens expire, but Claude Code automatically refreshes them when you use the CLI.

**If you see "OAuth token expired":**

```bash
# Just run any claude command to refresh
claude

# Or explicitly
claude setup
```

This updates the credentials file and Little Helper will pick up the new tokens.

## Benefits

✓ **Single Sign-On** - One login for Claude Code AND Little Helper
✓ **Claude Max Rates** - 5x higher limits (`default_claude_max_5x`)
✓ **No API Keys** - No need to manage separate credentials
✓ **Automatic Refresh** - Claude Code keeps tokens fresh
✓ **Team Ready** - Same setup works for everyone with Claude Code

## For Team Deployment

When rolling out to MacAllister Polling team:

1. **Install Claude Code** on each machine
2. **Sign in once** with `claude`
3. **Run Little Helper** - OAuth works automatically!

No API key distribution needed.

## Troubleshooting

### "No authentication found"

```bash
# Sign in with Claude Code
claude

# Verify credentials exist
ls -la ~/.claude/.credentials.json

# Check subscription
jq -r '.claudeAiOauth.subscriptionType' ~/.claude/.credentials.json
```

### "OAuth token expired"

```bash
# Refresh by running any claude command
claude

# Little Helper will pick up new tokens automatically
```

### Still want to use API key?

```bash
# OAuth takes priority, but you can disable it by moving credentials
mv ~/.claude/.credentials.json ~/.claude/.credentials.json.backup

# Now set API key
export ANTHROPIC_API_KEY="your-key-here"

# Run Little Helper - will use API key
./target/release/app
```

## Code Changes Summary

Modified `/home/flower/Downloads/claudia-lite/crates/providers/src/claude.rs`:

- Added OAuth credential structures
- `load_oauth_credentials()` - Reads `~/.claude/.credentials.json`
- Token expiration checking (5 minute buffer)
- Authentication priority: OAuth → API Key → Error
- Debug messages show which auth method is active

## Next Steps

1. **Test it!** - Run the app and verify OAuth authentication
2. **Try chatting** - Make sure Claude Max responses work
3. **Share with team** - OAuth setup is much simpler than API keys

---

**Last Updated:** 2025-12-12
**Status:** ✅ OAuth Integration Complete
**Subscription:** Claude Max (5x rate limits)
