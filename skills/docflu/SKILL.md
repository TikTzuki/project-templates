---
name: docflu
description: "Sync Docusaurus markdown docs to Confluence, Google Docs, or Notion using docflu CLI. Use when syncing documentation, setting up docflu config, converting diagrams (Mermaid/PlantUML/D2/Graphviz) for Confluence, or managing doc sync workflows. Triggers on 'sync docs', 'push to confluence', 'docflu', 'confluence sync', or when .env contains CONFLUENCE_BASE_URL/DOCFLU_ variables."
metadata:
  {
    "openclaw": {
      "emoji": "📄",
      "requires": { "bins": [ "docflu", "node" ] },
      "install": [
        { "command": "git clone https://github.com/tuanpmt/docflu.git && cd docflu && npm install && npm link", "name": "source" }
      ]
    }
  }
homepage: https://github.com/tuanpmt/docflu
---

# Docflu - Docusaurus to Confluence/GDocs/Notion Sync

## Quick Reference

```bash
# Interactive setup (creates .env)
docflu init

# Sync all docs to Confluence (default)
docflu sync --docs

# Sync to Google Docs or Notion
docflu sync --docs --gdocs
docflu sync --docs --notion

# Sync a single file
docflu sync --file path/to/doc.md

# Sync to a specific Confluence page
docflu sync --file path/to/doc.md --target <pageId>

# Preview changes without applying
docflu sync --dry-run
```

## Environment Configuration

Docflu uses a `.env` file for all configuration. Run `docflu init` or create manually:

### Confluence (Primary)

```env
CONFLUENCE_BASE_URL=https://your-domain.atlassian.net/
CONFLUENCE_USERNAME=user@example.com
CONFLUENCE_API_TOKEN=your-api-token
CONFLUENCE_SPACE_KEY=DOC
CONFLUENCE_ROOT_PAGE_TITLE=Documentation

# Mermaid plugin support
CONFLUENCE_USE_MERMAID_PLUGIN=true
CONFLUENCE_MERMAID_PLUGIN_NAME=mermaid
```

### Google Docs

```env
GOOGLE_CLIENT_ID=your-client-id
GOOGLE_CLIENT_SECRET=your-client-secret
GOOGLE_DOCUMENT_TITLE=target-document
```

### Notion

```env
NOTION_API_TOKEN=your-notion-token
NOTION_ROOT_PAGE_ID=root-page-id
```

### Docflu Options

```env
DOCFLU_EXCLUDE_PATTERNS=*.draft.md,private/**
DOCFLU_CONCURRENT_UPLOADS=5
DOCFLU_RETRY_COUNT=3
```

## Frontmatter Targeting

Sync a doc to a specific Confluence page via frontmatter:

```yaml
---
title: My Document
confluence_target: 123456
---
```

## Diagram Support

Docflu auto-converts diagram code blocks to SVG before upload:

- **Mermaid** - flowcharts, sequence, class, state, ER, gantt
- **PlantUML** - UML, architecture, sequence
- **Graphviz/DOT** - directed graphs, networks
- **D2** - modern declarative diagrams

Auto-installs required CLI tools (e.g. `mmdc`) if missing. Falls back to code blocks on failure.

## Key Behaviors

- **Hierarchy**: Mirrors Docusaurus folder structure on target platform
- **Internal links**: Converts relative markdown links to platform-specific URLs
- **Images**: Uploads local images as attachments
- **Incremental sync**: Tracks state in `.docusaurus/sync-state.json`, only syncs changed files
- **Requires**: Node.js >= 16.0.0

## Workflow

1. Ensure `.env` is configured (use `docflu init` or create manually)
2. Add `.env` to `.gitignore`
3. Run `docflu sync --dry-run` to preview
4. Run `docflu sync --docs` to push
5. Check Confluence/GDocs/Notion for synced content
