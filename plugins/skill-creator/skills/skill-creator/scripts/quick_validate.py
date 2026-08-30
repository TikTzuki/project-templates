#!/usr/bin/env python3
"""
Quick validation script for skills - minimal version
"""

import re
import sys
from pathlib import Path

MAX_SKILL_NAME_LENGTH = 64


class FrontmatterError(ValueError):
    """Raised when the frontmatter block cannot be parsed."""


def parse_frontmatter(text):
    """Parse the small subset of YAML that SKILL.md frontmatter uses.

    Skill frontmatter is a flat block of `key: value` pairs, so a full YAML
    parser is not needed -- avoiding it keeps this script dependency-free on a
    stock Python. Values may span multiple lines (indented continuations, or a
    `{...}` / `[...]` block); every value is returned as a string.
    """
    entries = {}
    key = None
    buffer = []

    def flush():
        if key is None:
            return
        value = " ".join(part.strip() for part in buffer if part.strip())
        if len(value) >= 2 and value[0] == value[-1] and value[0] in "\"'":
            value = value[1:-1]
        entries[key] = value

    for line in text.splitlines():
        if not line.strip():
            buffer.append("")
            continue

        match = re.match(r"^([A-Za-z_][A-Za-z0-9_-]*):(.*)$", line)
        if match and not line[0].isspace():
            flush()
            key = match.group(1)
            rest = match.group(2).strip()
            # `>` and `|` introduce a block scalar; the text follows underneath.
            buffer = [] if rest in (">", "|", ">-", "|-") else [rest]
        elif key is not None:
            buffer.append(line)
        else:
            raise FrontmatterError(f"expected 'key: value', got: {line.strip()!r}")

    flush()

    if not entries:
        raise FrontmatterError("frontmatter is empty")

    return entries


def validate_skill(skill_path):
    """Basic validation of a skill"""
    skill_path = Path(skill_path)

    skill_md = skill_path / "SKILL.md"
    if not skill_md.exists():
        return False, "SKILL.md not found"

    content = skill_md.read_text()
    if not content.startswith("---"):
        return False, "No YAML frontmatter found"

    match = re.match(r"^---\n(.*?)\n---", content, re.DOTALL)
    if not match:
        return False, "Invalid frontmatter format"

    frontmatter_text = match.group(1)

    try:
        frontmatter = parse_frontmatter(frontmatter_text)
    except FrontmatterError as e:
        return False, f"Invalid YAML in frontmatter: {e}"

    allowed_properties = {
        "name",
        "description",
        "license",
        "allowed-tools",
        "argument-hint",
        "disable-model-invocation",
        "homepage",
        "metadata",
        "model",
    }

    unexpected_keys = set(frontmatter.keys()) - allowed_properties
    if unexpected_keys:
        allowed = ", ".join(sorted(allowed_properties))
        unexpected = ", ".join(sorted(unexpected_keys))
        return (
            False,
            f"Unexpected key(s) in SKILL.md frontmatter: {unexpected}. Allowed properties are: {allowed}",
        )

    if "name" not in frontmatter:
        return False, "Missing 'name' in frontmatter"
    if "description" not in frontmatter:
        return False, "Missing 'description' in frontmatter"

    name = frontmatter.get("name", "")
    if not isinstance(name, str):
        return False, f"Name must be a string, got {type(name).__name__}"
    name = name.strip()
    if name:
        if not re.match(r"^[a-z0-9-]+$", name):
            return (
                False,
                f"Name '{name}' should be hyphen-case (lowercase letters, digits, and hyphens only)",
            )
        if name.startswith("-") or name.endswith("-") or "--" in name:
            return (
                False,
                f"Name '{name}' cannot start/end with hyphen or contain consecutive hyphens",
            )
        if len(name) > MAX_SKILL_NAME_LENGTH:
            return (
                False,
                f"Name is too long ({len(name)} characters). "
                f"Maximum is {MAX_SKILL_NAME_LENGTH} characters.",
            )

    description = frontmatter.get("description", "")
    if not isinstance(description, str):
        return False, f"Description must be a string, got {type(description).__name__}"
    description = description.strip()
    if description:
        if "<" in description or ">" in description:
            return False, "Description cannot contain angle brackets (< or >)"
        if len(description) > 1024:
            return (
                False,
                f"Description is too long ({len(description)} characters). Maximum is 1024 characters.",
            )

    return True, "Skill is valid!"


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python quick_validate.py <skill_directory>")
        sys.exit(1)

    valid, message = validate_skill(sys.argv[1])
    print(message)
    sys.exit(0 if valid else 1)
