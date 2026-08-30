---
name: openclaw-agent
description: Configure and work with OpenClaw AI agents. Use when setting up OpenClaw agents, creating bootstrap files (AGENTS.md, SOUL.md, TOOLS.md, IDENTITY.md, USER.md, HEARTBEAT.md, BOOTSTRAP.md, MEMORY.md), understanding OpenClaw's system prompt architecture, configuring prompt modes (full/minimal/none), or working with OpenClaw's workspace and safety features. Triggers on 'OpenClaw', 'openclaw', bootstrap file names, or agent configuration tasks.
---

# OpenClaw Agent

Comprehensive support for configuring and working with OpenClaw AI agents, including system prompt architecture,
bootstrap file management, and agent behavior customization.

## Quick Start

When working with OpenClaw agents, common tasks include:

1. **Create bootstrap files**: Use templates from `assets/templates/` to set up the 8 bootstrap files
2. **Understand system prompt**: See `references/system-prompt.md` for architecture details
3. **Configure agent behavior**: Edit SOUL.md for personality, IDENTITY.md for role definition
4. **Set up tools**: Define available tools and usage patterns in TOOLS.md
5. **Manage memory**: Use MEMORY.md for persistent context across sessions

## Bootstrap Files Setup

OpenClaw auto-injects 8 bootstrap files into the agent context. Create these files in your workspace:

### Required Bootstrap Files

1. **AGENTS.md** - Agent and sub-agent definitions
    - Template: `assets/templates/AGENTS.md`
    - Use: Define main agent and sub-agent configurations
    - Sub-agents also receive this file

2. **SOUL.md** - Agent personality and behavioral guidelines
    - Template: `assets/templates/SOUL.md`
    - Use: Define communication style, tone, values, and core behaviors

3. **TOOLS.md** - Tool definitions and usage patterns
    - Template: `assets/templates/TOOLS.md`
    - Use: Describe available tools and when/how to use them
    - Sub-agents also receive this file

4. **IDENTITY.md** - Agent identity and role definition
    - Template: `assets/templates/IDENTITY.md`
    - Use: Define agent's name, role, expertise, and responsibilities

5. **USER.md** - User preferences and context
    - Template: `assets/templates/USER.md`
    - Use: Store user-specific preferences and workflow patterns

6. **HEARTBEAT.md** - Heartbeat configuration
    - Template: `assets/templates/HEARTBEAT.md`
    - Use: Configure status updates for long-running tasks

7. **BOOTSTRAP.md** - General bootstrap instructions
    - Template: `assets/templates/BOOTSTRAP.md`
    - Use: Initialization guidance and startup configuration

8. **MEMORY.md** - Persistent memory
    - Template: `assets/templates/MEMORY.md`
    - Use: Store learned patterns and context across sessions
    - Case-insensitive: `memory.md` also works

### Bootstrap File Limits

- **Individual file limit**: 20,000 characters max
- **Total bootstrap content**: 150,000 characters (default)
- **Sub-agent injection**: Only AGENTS.md and TOOLS.md

### Creating Bootstrap Files

To set up bootstrap files, copy templates from `assets/templates/`:

```bash
# Copy all templates to current directory
cp assets/templates/*.md .

# Or copy individual files as needed
cp assets/templates/AGENTS.md .
cp assets/templates/SOUL.md .
```

Then customize each file for your specific agent needs.

## Prompt Modes

OpenClaw supports three prompt modes via `promptMode` configuration:

### 1. full (default)

Complete system prompt with all sections. Use for standard agents.

Includes: Tooling, Safety, Skills, Self-Update, Workspace, Documentation, Workspace Files, Sandbox, Date & Time, Reply
Tags, Heartbeats, Runtime, Reasoning

### 2. minimal

Streamlined prompt for sub-agents, omitting Skills, Self-Update, Reply Tags, Heartbeats, and other sections.

Use when spawning sub-agents for specific tasks.

### 3. none

Identity line only, no system prompt sections.

Use for maximum flexibility when you want to provide a completely custom prompt.

## System Prompt Architecture

For detailed understanding of OpenClaw's system prompt components, see `references/system-prompt.md`.

Key concepts:

- **Compact structure**: Fixed sections keep prompts concise
- **Auto-injection**: Bootstrap files automatically loaded into context
- **Progressive disclosure**: Information loaded as needed
- **Safety model**: Advisory guardrails + runtime enforcement

## Safety Considerations

**Important**: System prompt safety guardrails are advisory, not enforcement mechanisms.

For security, rely on:

- **Tool policies**: Define what tools can/cannot do
- **Execution approvals**: Require user approval for sensitive operations
- **Sandboxing**: Isolate agent execution environments
- **Allowlists**: Restrict access to specific resources

The system prompt guides behavior, but runtime controls enforce security.

## Common Workflows

### Setting up a new OpenClaw agent

1. Copy bootstrap templates to workspace
2. Customize IDENTITY.md with agent name and role
3. Define personality in SOUL.md
4. Configure available tools in TOOLS.md
5. Set user preferences in USER.md
6. Optionally configure sub-agents in AGENTS.md

### Configuring sub-agents

1. Edit AGENTS.md to define sub-agent configurations
2. Set `promptMode: minimal` for sub-agents
3. Define specific tools available to each sub-agent
4. Note: Sub-agents only receive AGENTS.md and TOOLS.md

### Managing agent memory

1. Edit MEMORY.md to store learned patterns
2. Document important decisions and rationale
3. Track user preferences and project context
4. Memory persists across sessions

## References

- **System Prompt Architecture**: See `references/system-prompt.md` for complete details on OpenClaw's prompt structure,
  components, and behavior
