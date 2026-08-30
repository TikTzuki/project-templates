#!/usr/bin/env python3
"""Validate that the marketplace manifest and every plugin manifest agree."""

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
errors: list[str] = []


def load(path: Path):
    try:
        return json.loads(path.read_text())
    except (OSError, json.JSONDecodeError) as exc:
        errors.append(f"{path.relative_to(ROOT)}: {exc}")
        return None


marketplace = load(ROOT / ".claude-plugin" / "marketplace.json")
if marketplace is None:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)

listed = {}
for entry in marketplace.get("plugins", []):
    name, source = entry.get("name"), entry.get("source")
    if not name or not source:
        errors.append(f"marketplace entry missing name or source: {entry}")
        continue

    listed[name] = source
    plugin_dir = ROOT / source
    manifest_path = plugin_dir / ".claude-plugin" / "plugin.json"

    if not manifest_path.is_file():
        errors.append(f"{name}: missing {manifest_path.relative_to(ROOT)}")
        continue

    manifest = load(manifest_path)
    if manifest is None:
        continue

    if manifest.get("name") != name:
        errors.append(
            f"{name}: plugin.json name is {manifest.get('name')!r}, "
            f"marketplace says {name!r}"
        )

    skills = sorted(p for p in (plugin_dir / "skills").glob("*/SKILL.md"))
    if not skills:
        errors.append(f"{name}: no skills/*/SKILL.md found under {source}")

    for skill in skills:
        if not skill.read_text().startswith("---"):
            errors.append(f"{name}: {skill.relative_to(ROOT)} has no YAML frontmatter")

# Every plugin directory on disk must be listed in the marketplace.
for plugin_dir in sorted((ROOT / "plugins").iterdir()):
    if plugin_dir.is_dir() and plugin_dir.name not in listed:
        errors.append(f"plugins/{plugin_dir.name} is not listed in marketplace.json")

if errors:
    print("\n".join(f"  - {e}" for e in errors), file=sys.stderr)
    sys.exit(1)

print(f"OK: {len(listed)} plugins validated")
