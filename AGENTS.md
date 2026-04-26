# Rust Portfolio - Agent Guide

## Project Overview

This is a Rust + WebAssembly portfolio project using Leptos (serverless component library). The portfolio features animations (GSAP), achievements system, and multiple interactive components.

## Stack

- **Frontend Framework**: Leptos 0.8 (WebAssembly/CSSR)
- **Build Tool**: Trunk
- **Animations**: GSAP (via wasm_bindgen inline_js)
- **Styling**: CSS
- **Backend**: WASM via Rust
- **Deployment**: Static site (GitHub Pages)

## File Structure

```
rust-portfolio/
├── Cargo.toml                 # Rust dependencies
├── index.html                  # Main HTML entry point
├── Trunk.toml                  # Trunk build configuration
├── agent-orchestrator.yaml    # AO session metadata
├── src/
│   ├── main.rs                 # Entry point
│   ├── content.rs             # Content data (JSON imported via wasm-bindgen)
│   ├── easter_eggs.rs          # Achievement system & hidden features
│   └── components/            # Component modules (.rs)
│       ├── mod.rs
│       ├── hero.rs
│       ├── skills.rs
│       ├── recognition.rs
│       ├── insights.rs
│       ├── achievements_panel.rs
│       ├── splash.rs
│       ├── contact.rs
│       ├── experience.rs
│       ├── philosophy.rs
│       ├── projects.rs
│       └── ticker.rs
├── styles/
│   ├── portfolio.css          # Main styles
│   └── input.css              # Input styles
├── public/
│   ├── sitemap.xml            # SEO
│   └── robots.txt             # SEO
└── content.json                # Dynamic content data
```

## Rules & Guidelines

### Git Workflow
- All agents MUST work in git worktrees: `~/.worktrees/rust-portfolio/<session_id>`
- Base branch: `branches/dev`
- Feature branches: `branches/features/<issue_id>`
- Never work directly on `trunk` or `branches/dev`
- Only `branches/dev` is merged into `trunk` (for new releases)
- Every feature must have a PR to `branches/dev`

### Commits
- Agents use identity: `mxlkhbot@duck.com`
- Commits are automatically GPG-signed (configured globally)
- Commit messages: clear, concise, describe the change

### Code Style
- NO comments on functions, classes, objects, or fields
- ONLY necessary comments for complex logic
- Clear, self-documenting code
- Maintain existing code style

### File Permissions
- ONLY modify: `.rs`, `.html`, `.css`, `.scss`, `.xml`, `.json`, `.js`, `.ts`, `.md`, `.ps1`, `.sh`
- Do NOT modify any other file types

## Git Commands Reference

### Session Setup
```bash
# Check current worktree
cd ~/.worktrees/rust-portfolio/<session_id>

# See branches
git branch -a

# Create feature branch from branches/dev
git checkout -b branches/features/<issue_id>

# Check status
git status

# Commit changes
git add .
git commit -m "message"

# Push branch
git push -u origin branches/features/<issue_id>

# Create PR
gh pr create --title "Issue <id>: feature description" --body "Description of changes"
```

### Workflow
```bash
# Start your work
cd ~/.worktrees/rust-portfolio/<your_session_id>
git checkout -b branches/features/<issue_id>

# ... make changes ...

# Commit frequently with clear messages
git add .
git commit -m "Add/fix something"

# Push and create PR
git push -u origin branches/features/<issue_id>
gh pr create --title "Issue <id>: description" --body "What was changed"

# Review the PR, merge into branches/dev
```

## Issue & PR Management
- Agents can create/update/delete issues and PRs (except `branches/dev` and `trunk`)
- Only `nakiradu77` manages `branches/dev` and `trunk`
- Always PR features to `branches/dev` for review and merging

## Environment
- GitHub SSH key configured
- GPG signing configured globally
- Worktrees created automatically on spawn

## Agent Orchestrator (ao) Session

You are running inside an Agent Orchestrator managed workspace.
Session metadata is updated automatically via shell wrappers.

If automatic updates fail, you can manually update metadata:
```bash
~/.ao/bin/ao-metadata-helper.sh  # sourced automatically
# Then call: update_ao_metadata <key> <value>
```
