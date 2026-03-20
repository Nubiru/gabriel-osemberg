# Environment — {PROJECT_NAME}

**Version**: 1.0 **Last Updated**: {DATE}

This document is the **complete reproducible setup checklist**. Any developer or AI agent should be able to set up the project from scratch by following these steps in order.

---

## 1. Host Requirements

| Property | Current | Minimum |
|----------|---------|---------|
| OS | {CURRENT_OS} | {MINIMUM_OS} |
| RAM | {CURRENT_RAM} | {MINIMUM_RAM} |
| CPU | {CURRENT_CPU} | {MINIMUM_CPU} |
| Shell | {SHELL} | {SHELL} |

---

## 2. System Dependencies

{TOOL_TABLE}

### Installation

```bash
# {PACKAGE_MANAGER} install command
{INSTALL_COMMAND}
```

### Verification

```bash
{VERIFY_COMMAND_1}
{VERIFY_COMMAND_2}
{VERIFY_COMMAND_3}
```

### What each tool does

| Tool | Purpose |
|------|---------|
| {TOOL_1} | {PURPOSE_1} |
| {TOOL_2} | {PURPOSE_2} |
| {TOOL_3} | {PURPOSE_3} |

---

## 3. Additional SDKs / Toolchains

<!--
Document any additional SDKs, toolchains, or runtimes that need separate installation
(e.g., language runtimes, cross-compilation toolchains, cloud CLIs).

### {SDK_NAME}

```bash
# Installation steps
{INSTALL_STEPS}
```

### Verification

```bash
{SDK_VERIFY_COMMAND}
```
-->

---

## 4. Version Control Setup

```bash
# Configure git
git config --global user.name "{YOUR_NAME}"
git config --global user.email "{YOUR_EMAIL}"
```

| Property | Value |
|----------|-------|
| Hosting | {GIT_HOST} |
| Repo | {REPO_URL} |
| Default Branch | {DEFAULT_BRANCH} |

---

## 5. Project Setup

```bash
# Clone the repo
git clone {REPO_URL}
cd {PROJECT_DIR}

# Install dependencies
{DEPENDENCY_INSTALL_COMMAND}

# Verify build
{BUILD_VERIFY_COMMAND}

# Run tests
{TEST_COMMAND}

# Start development server (if applicable)
{DEV_SERVER_COMMAND}
```

---

## 6. Test Framework

{TEST_FRAMEWORK_DESCRIPTION}

---

## 7. Editor

{EDITOR_RECOMMENDATION}

---

## 8. Build Configuration

| Build Mode | Flags / Settings |
|------------|-----------------|
| Development | {DEV_FLAGS} |
| Debug | {DEBUG_FLAGS} |
| Release | {RELEASE_FLAGS} |

---

## 9. Build Targets

| Target | What it does |
|--------|-------------|
| {TARGET_1} | {DESCRIPTION_1} |
| {TARGET_2} | {DESCRIPTION_2} |
| {TARGET_3} | {DESCRIPTION_3} |

---

## 10. Development Workflow

{WORKFLOW_DESCRIPTION}

---

## 11. Known Issues

<!--
Document any known setup issues, workarounds, or environment quirks.

- {ISSUE_1}: {WORKAROUND_1}
- {ISSUE_2}: {WORKAROUND_2}
-->

---

## 12. Build Status (verified {DATE})

| Target | Status | Notes |
|--------|--------|-------|
| {TARGET_1} | {STATUS} | {NOTES} |
| {TARGET_2} | {STATUS} | {NOTES} |
