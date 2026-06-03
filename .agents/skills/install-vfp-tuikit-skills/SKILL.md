---
name: install-vfp-tuikit-skills
description: Use when installing the repo's .agents/skills files from GitHub or a raw URL into ~/.agents/skills/.
---

# Install vfp_tuikit Skills

Use this skill when you need to copy the repo's agent skills into another machine or checkout.

## Install

```bash
mkdir -p ~/.agents/skills/vfp-tuikit
curl -fsSL https://raw.githubusercontent.com/victoryforphil/tui_kit/master/.agents/skills/vfp-tuikit/SKILL.md \
  -o ~/.agents/skills/vfp-tuikit/SKILL.md

mkdir -p ~/.agents/skills/install-vfp-tuikit-skills
curl -fsSL https://raw.githubusercontent.com/victoryforphil/tui_kit/master/.agents/skills/install-vfp-tuikit-skills/SKILL.md \
  -o ~/.agents/skills/install-vfp-tuikit-skills/SKILL.md
```

GitHub blob paths for browsing:

```text
https://github.com/victoryforphil/tui_kit/blob/master/.agents/skills/vfp-tuikit/SKILL.md
https://github.com/victoryforphil/tui_kit/blob/master/.agents/skills/install-vfp-tuikit-skills/SKILL.md
```

## Notes

- The repo uses `master` in the examples above.
- Skills are picked up from `~/.agents/skills/` by OpenCode-compatible setups.
