# vtb-ltk-tools

[![Build Status](https://github.com/gsmeven/vtb-ltk-tools/actions/workflows/update-tools.yml/badge.svg)](https://github.com/gsmeven/vtb-ltk-tools/actions/workflows/update-tools.yml)
[![Release](https://img.shields.io/github/v/release/gsmeven/vtb-ltk-tools?color=blue)](https://github.com/gsmeven/vtb-ltk-tools/releases/tag/tools-latest)

Automated CI/CD pipeline that builds and serves bleeding-edge Windows binaries of League Toolkit (LTK) tools for [Vanilla True Builder](https://github.com/gsmeven/vanilla-true-builder).

## Overview

This repository acts as an autonomous binary supply chain for Vanilla True Builder. It monitors upstream LeagueToolkit source repositories, builds native 64-bit Windows executables (`wadtools.exe`, `ritobin.exe`, and `league-mod.exe`) via GitHub Actions, and distributes them through a rolling release package.

## Pipeline Workflow

The workflow (`update-tools.yml`) executes every 3 hours via cron schedule, on relevant repository pushes, or through manual dispatch (`workflow_dispatch`):

1. **Upstream Commit Inspection:** Queries GitHub APIs for the latest commits across:
   - `LeagueToolkit/wadtools`
   - `LeagueToolkit/ritobin-tools`
   - `LeagueToolkit/league-mod` (`crates/ltk_mod_project` and `crates/ltk_modpkg`)
   - Local `./vtb-packer` source changes
2. **State Comparison:** Compares commit hashes against the existing release body. If no relevant changes are detected on scheduled runs, compilation is skipped.
3. **Compilation:** Builds `wadtools` and `ritobin-tools` (`cargo install --git ... --locked`) alongside the local `./vtb-packer` crate (`cargo build --release`) on a clean `windows-latest` runner using the stable Rust toolchain, with strict exit-code checks.
4. **Binary Normalization & Verification:** Standardizes executable names to `wadtools.exe`, `ritobin.exe`, and `league-mod.exe` inside a dedicated `tools/` container, verifies binary integrity, and runs `--help` smoke tests prior to archiving.
5. **Release Distribution:** Packages the `tools/` folder into `tools.zip` and updates the rolling `tools-latest` release tag with commit provenance.

## Included Binaries

| Binary | Source | Role in Vanilla True Builder |
| :--- | :--- | :--- |
| `wadtools.exe` | [LeagueToolkit/wadtools](https://github.com/LeagueToolkit/wadtools) | Decompression and selective extraction of `.wad.client` archives. |
| `ritobin.exe` | [LeagueToolkit/ritobin-tools](https://github.com/LeagueToolkit/ritobin-tools) | Decompilation and serialization of Riot property trees (`PROP` / `.bin`). |
| `league-mod.exe` | `./vtb-packer` via [LeagueToolkit/league-mod](https://github.com/LeagueToolkit/league-mod) (`ltk_mod_project` & `ltk_modpkg`) | Packaging and compilation of native multi-layered LTK mod projects (`.modpkg`). |

> **Note:** Since the upstream `league-mod` CLI was deprecated by LeagueToolkit, `league-mod.exe` in this distribution is compiled from `./vtb-packer` directly against the actively maintained `ltk_mod_project` (`modpkg` feature) and `ltk_modpkg` crates while keeping CLI flag compatibility (`pack -c -f -o --format modpkg`).

## Distribution & Consumption

- **Direct Download:** [`tools.zip`](https://github.com/gsmeven/vtb-ltk-tools/releases/download/tools-latest/tools.zip)
- **Release Channel:** [`tools-latest`](https://github.com/gsmeven/vtb-ltk-tools/releases/tag/tools-latest)

### Archive Structure
The distributed `tools.zip` archive embeds a root `tools/` folder for immediate, drop-in extraction into Vanilla True Builder workspace directories:

```text
tools.zip
└── tools/
    ├── wadtools.exe
    ├── ritobin.exe
    └── league-mod.exe
