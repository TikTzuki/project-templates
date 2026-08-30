# OpenClaw System Prompt Architecture

## Overview

OpenClaw generates a custom system prompt for each agent run that is distinct from default prompts. The system assembles
and injects this prompt into every execution.

## Structural Components

The prompt maintains a deliberate compact structure with fixed sections:

- **Tooling**: Current tool inventory with concise descriptions
- **Safety**: Brief guardrail reminders targeting power-seeking behavior prevention
- **Skills**: Instructions for loading skill documentation on-demand
- **Self-Update**: Guidance on running `config.apply` and `update.run`
- **Workspace**: References working directory configuration
- **Documentation**: Local path to OpenClaw docs and access guidance
- **Workspace Files**: Bootstrap file inclusion indicators
- **Sandbox**: Runtime environment details when enabled
- **Date & Time**: User timezone and temporal context
- **Reply Tags**: Optional syntax for supported providers
- **Heartbeats**: Prompt and acknowledgment behavior
- **Runtime**: Host, OS, Node version, model, and reasoning level
- **Reasoning**: Visibility level with toggle hints

## Prompt Modes

Three rendering options exist via `promptMode`:

1. **full** (default): Complete sections for standard agents
2. **minimal**: Streamlined for sub-agents, omitting Skills, Self-Update, Reply Tags, Heartbeats, and other sections
3. **none**: Identity line only

## Bootstrap File Injection

Eight file types auto-inject into context:

1. **AGENTS.md** - Agent definitions and sub-agent configurations
2. **SOUL.md** - Agent personality and behavioral guidelines
3. **TOOLS.md** - Tool definitions and usage patterns
4. **IDENTITY.md** - Agent identity and role definition
5. **USER.md** - User preferences and context
6. **HEARTBEAT.md** - Heartbeat configuration and behavior
7. **BOOTSTRAP.md** - General bootstrap instructions
8. **MEMORY.md/memory.md** - Persistent memory across sessions

### Bootstrap File Limits

- Individual files: 20,000 characters max
- Total bootstrap content: 150,000 characters by default
- Sub-agents: Receive only AGENTS.md and TOOLS.md

## Safety Model

**Important:** Safety guardrails in the system prompt are advisory. They guide model behavior but do not enforce policy.

Hard enforcement relies on:

- Tool policies
- Execution approvals
- Sandboxing
- Allowlists

The system prompt provides guidance, but security must be implemented through runtime controls.
