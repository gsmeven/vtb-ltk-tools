# ltk-tools (.exe) for vtb

> Autonomous cloud compilation pipeline providing bleeding-edge Windows binaries of **League Toolkit (LTK)** for [Vanilla True Builder](https://github.com/gsmeven/vanilla-true-builder).

[![Build Pipeline](https://img.shields.io/github/actions/workflow/status/gsmeven/vtb-ltk-tools/update-tools.yml?style=flat-square&logo=githubactions&logoColor=white&label=CI%2FCD)](https://github.com/gsmeven/vtb-ltk-tools/actions)
[![Release: tools-latest](https://img.shields.io/badge/release-tools--latest-blue?style=flat-square&logo=github)](https://github.com/gsmeven/vtb-ltk-tools/releases/tag/tools-latest)
[![Maintained By Agent](https://img.shields.io/badge/Maintained%20By-Autonomous%20Agent-blueviolet?style=flat-square&logo=openai&logoColor=white)](#-autonomous-agent-manifest)
[![Toolchain](https://img.shields.io/badge/Compiled%20With-Rust%20Stable-dea584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Cadence](https://img.shields.io/badge/Cron-Every%203%20Hours-brightgreen?style=flat-square)](#-autonomous-execution-loop)

---

## 🤖 Autonomous Agent Manifest

> **System Operational Notice**  
> This repository, its automated GitHub Actions compilation pipelines, commit-diffing heuristics, and documentation were entirely engineered and synthesized by an **autonomous AI agent** executing directives for [Neve](https://github.com/gsmeven).
> 
> The agent's dedicated mission is to eliminate human maintenance from the toolchain delivery layer: monitoring official upstream [League Toolkit](https://github.com/LeagueToolkit) codebases, compiling native Windows binaries directly from Rust source trees in cloud runners, and serving a verified, zero-dependency payload to Vanilla True Builder.

---

## ⚡ Purpose & Architecture

[Vanilla True Builder](https://github.com/gsmeven/vanilla-true-builder) requires underlying reverse-engineering tools to decompile and reconstruct League of Legends game archives.

These upstream utilities are maintained by the League Toolkit team as native **Rust** source crates (`ltk_wad` and `ltk_ritobin`). Forcing end users or builders to compile them locally requires:
* Installing the full Rust toolchain (`rustup`, `cargo`)
* Installing Git
* Multi-gigabyte Microsoft Visual C++ (MSVC) build dependencies

This autonomous worker resolves that friction entirely. Every **3 hours**, the agent polls upstream source repositories, triggers automated cloud compilation for detected revisions, verifies binary outputs, and maintains an atomic rolling release: **`tools.zip`**.

---

## 🛠️ Binary Specification Matrix

All executables are compiled natively from source on `windows-latest` runners:

| Binary | Upstream Crate / Source | Toolchain | Primary Role |
| :--- | :--- | :---: | :--- |
| **`wadtools.exe`** | [LeagueToolkit/wadtools](https://github.com/LeagueToolkit/wadtools) (`ltk_wad`) | **Rust** | Unpacks, inspects, extracts, and repacks `.wad.client` archive archives. |
| **`ritobin.exe`** | [LeagueToolkit/ritobin-tools](https://github.com/LeagueToolkit/ritobin-tools) (`ltk_ritobin`) | **Rust** | Two-way serializer: translates binary `.bin` property trees to editable text and compiles edited trees back to binary format. |

---

## ⚙️ Autonomous Execution Loop

The agent follows a deterministic four-phase lifecycle with zero manual intervention:

```
Upstream Repos ──► Commit Hash Diff ──► [Unchanged?] ──► True  ──► Immediate Abort (0s wasted)
                                              │
                                              └──► False ──► Ephemeral Windows Runner
                                                                      │
                                                                      ▼
                                                              cargo install --git
                                                                      │
                                                                      ▼
                                                              Package tools.zip
                                                                      │
                                                                      ▼
                                                              Sync tools-latest
```

1. **Upstream Commit Diffing:**  
   The pipeline queries the GitHub REST API across official upstream branches and crate subtrees (`ltk_wad`, `ltk_ritobin`).
2. **Zero-Waste Early Abort:**  
   If upstream commit hashes match the manifest of the current `tools-latest` release, the workflow exits in seconds to conserve action quotas.
3. **Cloud Native Compilation:**  
   If new commits exist, an isolated Windows runner builds fresh x64 binaries via `cargo install` against the stable Rust toolchain.
4. **Atomic Delivery:**  
   Binaries are normalized (`wadtools.exe`, `ritobin.exe`), compressed into `tools.zip`, and deployed directly over the rolling release tag.

---

## 📥 Artifact Distribution & Client Sync

Vanilla True Builder natively integrates with this endpoint. During initial setup or patch cycles, the builder queries this repository and silently synchronizes binaries in the background without user intervention.

For manual development, custom tooling, or standalone usage, the latest compiled bundle is always available:

⤓ **[Download tools.zip (Latest Stable Build)](https://github.com/gsmeven/vtb-ltk-tools/releases/download/tools-latest/tools.zip)**

---

## 👥 Ecosystem & Credits

* **[Neve](https://github.com/gsmeven)** — Creator and maintainer of [Vanilla True Builder](https://github.com/gsmeven/vanilla-true-builder).
* **[League Toolkit](https://github.com/LeagueToolkit)** — Foundational reverse-engineering infrastructure by Crauzer, moonshadow565, and community contributors.
* **Agent Operations:** Build pipeline synthesis, maintenance loops, and repository scaffolding autonomously operated via AI.
* **Legal Disclaimer:** This project is an independent automated utility and is not affiliated with, endorsed by, or sponsored by Riot Games. League of Legends is a registered trademark of Riot Games, Inc.
