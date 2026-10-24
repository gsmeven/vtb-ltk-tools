# ltk-tools (.exe) for vtb

> Autonomous cloud compilation pipeline providing bleeding-edge Windows binaries of **League Toolkit (LTK)** for [Vanilla True Builder](https://github.com/gsmeven/vanilla-true-builder).

[![Build Pipeline](https://img.shields.io/github/actions/workflow/status/gsmeven/vtb-ltk-tools/update-tools.yml?style=flat-square&logo=githubactions&logoColor=white&label=CI%2FCD)](https://github.com/gsmeven/vtb-ltk-tools/actions)
[![Release: tools-latest](https://img.shields.io/badge/release-tools--latest-blue?style=flat-square&logo=github)](https://github.com/gsmeven/vtb-ltk-tools/releases/tag/tools-latest)
[![Toolchain](https://img.shields.io/badge/Compiled%20With-Rust%20Stable-dea584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Cadence](https://img.shields.io/badge/Cron-Every%203%20Hours-brightgreen?style=flat-square)](#-continuous-integration-lifecycle)
[![Target](https://img.shields.io/badge/Target-Windows%20x64-0078d7?style=flat-square&logo=windows&logoColor=white)](https://github.com/gsmeven/vtb-ltk-tools/releases/tag/tools-latest)

---

## ⚡ System Identity & Architecture

This repository is a fully autonomous, AI-scaffolded build worker designed to eliminate manual toolchain maintenance. It acts as an unattended compilation bridge between the upstream [League Toolkit](https://github.com/LeagueToolkit) source repositories and the Vanilla True Builder ecosystem.

### The Problem
The core utilities required to inspect, extract, and serialize League of Legends client archives are authored in **Rust** (distributed as upstream source crates `ltk_wad` and `ltk_ritobin`). Compiling these tools locally requires:
* Full Rust toolchain (`rustup`, `rustc`, `cargo`)
* Git version control
* Multi-gigabyte Microsoft Visual C++ (MSVC) build tools

### The Autonomous Solution
This pipeline shifts 100% of the compilation overhead to GitHub Actions cloud runners. Operating on a scheduled loop, the bot monitors upstream source trees, compiles optimized 64-bit Windows executables directly from git HEAD, verifies binary health, and serves a standardized release payload: **`tools.zip`**.

---

## 🛠️ Binary Matrix

All executables are compiled from source using the latest stable Rust compiler:

| Binary | Upstream Source / Crate | Language | Primary Modding Role |
| :--- | :--- | :---: | :--- |
| **`wadtools.exe`** | [LeagueToolkit/wadtools](https://github.com/LeagueToolkit/wadtools) (`ltk_wad`) | **Rust** | Extracts, inspects, and repacks `.wad.client` game archives. |
| **`ritobin.exe`** | [LeagueToolkit/ritobin-tools](https://github.com/LeagueToolkit/ritobin-tools) (`ltk_ritobin`) | **Rust** | Bidirectional serializer: decompiles binary `.bin` property trees to clean plaintext and compiles edits back to raw bytecode. |

---

## ⚙️ Continuous Integration Lifecycle

The workflow operates under a zero-maintenance, zero-waste contract:

```
Upstream Commits ──► SHA Differential ──► [Hash Match?] ──► True  ──► Abort (0s wasted)
                                              │
                                              └──► False ──► Clean Cargo Build
                                                                    │
                                                                    ▼
                                                            Package tools.zip
                                                                    │
                                                                    ▼
                                                            Sync tools-latest
```

1. **Deterministic Hash Invalidation:**  
   Every 3 hours, the runner polls the GitHub REST API for the latest commit SHAs across:
   * `LeagueToolkit/wadtools` (`main`)
   * `LeagueToolkit/ritobin-tools` (`main`)
   * `LeagueToolkit/league-toolkit` (`crates/ltk_ritobin` & `crates/ltk_wad`)

2. **Early Exit Optimization:**  
   If the aggregate commit state matches the metadata stored in the active `tools-latest` release tag, execution terminates immediately, consuming zero unnecessary action minutes.

3. **Isolated Cloud Compilation:**  
   When an upstream update is detected, an ephemeral `windows-latest` instance compiles both tools via `cargo install --git` using the native MSVC toolchain with optimizations enabled.

4. **Payload Normalization:**  
   Executables are standardized to static filenames (`wadtools.exe`, `ritobin.exe`), compressed into a single portable `tools.zip` archive, and atomically published to overwrite the rolling release.

---

## 📥 Artifact Distribution & Client Sync

Vanilla True Builder natively communicates with this repository. During setup or patch cycles, the builder queries this release tag directly and hot-swaps updated tools in the background without user intervention.

For custom automation pipelines, headless servers, or manual development, precompiled bundles are permanently accessible via:

⤓ **[Download tools.zip (Latest Stable Build)](https://github.com/gsmeven/vtb-ltk-tools/releases/download/tools-latest/tools.zip)**

---

## 👥 Ecosystem & Credits

* **[Neve](https://github.com/gsmeven)** — Creator and maintainer of [Vanilla True Builder](https://github.com/gsmeven/vanilla-true-builder).
* **[League Toolkit](https://github.com/LeagueToolkit)** — Pioneering reverse-engineering work by Crauzer, moonshadow565, and community contributors.
* **Autonomous Pipeline Notice:** Workflows, change-detection logic, and packaging routines in this repository are autonomously maintained with AI orchestration.
* **Disclaimer:** This project is an independent utility and is not affiliated with, endorsed by, or sponsored by Riot Games. League of Legends is a registered trademark of Riot Games, Inc.
