# ltk-tools (.exe) for vtb

> Automated CI/CD build bot serving bleeding-edge Windows binaries of **League Toolkit (LTK)** reverse-engineering tools for [**Vanilla True Builder**](https://github.com/gsmeven/vanilla-true-builder).

[![Build and Release Tools](https://github.com/gsmeven/vtb-ltk-tools/actions/workflows/update-tools.yml/badge.svg)](https://github.com/gsmeven/vtb-ltk-tools/actions)
[![Release: tools-latest](https://img.shields.io/badge/release-tools--latest-blue?logo=github)](https://github.com/gsmeven/vtb-ltk-tools/releases/tag/tools-latest)
[![Vanilla True Builder](https://img.shields.io/badge/Companion%20Repo-Vanilla%20True%20Builder-orange?logo=github)](https://github.com/gsmeven/vanilla-true-builder)
[![Check Cadence](https://img.shields.io/badge/cron-every%203%20hours-brightgreen)](#-how-it-works)

---

## 🤖 Autonomous Infrastructure

This repository operates strictly as a headless, autonomous compilation bot. Its continuous integration workflows, build matrices, and delivery pipelines were scaffolded and orchestrated with AI assistance to achieve a zero-maintenance build bridge.

The bot continuously tracks upstream [League Toolkit](https://github.com/LeagueToolkit) repositories, compiles raw Rust source code directly in isolated cloud runners, and publishes verified Windows `.exe` binaries tailored for [**Vanilla True Builder**](https://github.com/gsmeven/vanilla-true-builder) without requiring human intervention.

---

## 📖 Overview

[**Vanilla True Builder**](https://github.com/gsmeven/vanilla-true-builder) is an automated restoration engine designed to be entirely self-sufficient. 

The foundational reverse-engineering tools in the League modding ecosystem are maintained by the **League Toolkit (LTK)** team as native **Rust** crates (`ltk_wad` and `ltk_ritobin`). Compiling them manually requires installing the Rust toolchain (`rustup`, `cargo`), Git, and Microsoft Visual C++ Build Tools.

To spare users and developers from local compilation overhead, this bot handles the entire toolchain in the cloud. Every **3 hours**, it inspects upstream repositories for new commits, freshly compiles standalone x64 binaries, and bundles them into **`tools.zip`** — ready for instant consumption.

---

## 🛠️ Included Tools & Specifications

Both utilities are compiled directly from their upstream **Rust** source repositories:

| Binary | Upstream Crate / Source | Language | Role in League Reverse Engineering |
| :--- | :--- | :---: | :--- |
| **`wadtools.exe`** | [LeagueToolkit/wadtools](https://github.com/LeagueToolkit/wadtools) (`ltk_wad`) | **Rust** | Unpacks, inspects, extracts, and repacks `.wad.client` archive packages. |
| **`ritobin.exe`** | [LeagueToolkit/ritobin-tools](https://github.com/LeagueToolkit/ritobin-tools) (`ltk_ritobin`) | **Rust** | Serializes binary property tree files (`.bin`) into editable plaintext and compiles them back to binary format. |

---

## ⚙️ How It Works

1. **Upstream Commit Diffing:**  
   The bot checks the GitHub API across all relevant upstream repositories and crate trees:
   * `LeagueToolkit/wadtools` (`main`)
   * `LeagueToolkit/ritobin-tools` (`main`)
   * `LeagueToolkit/league-toolkit` (`crates/ltk_ritobin`)
   * `LeagueToolkit/league-toolkit` (`crates/ltk_wad`)

2. **Zero Resource Waste:**  
   If the upstream commit hashes match the current release description in [`tools-latest`](https://github.com/gsmeven/vtb-ltk-tools/releases/tag/tools-latest), the workflow exits immediately to conserve runner minutes.

3. **Cloud-Native Rust Compilation:**  
   Spawns a fresh `windows-latest` runner equipped with the official stable Rust toolchain to compile standalone, dependency-free x64 binaries via `cargo install`.

4. **Deterministic Packaging:**  
   Normalizes binary names (`wadtools.exe` and `ritobin.exe`), archives them into `tools.zip`, and updates the persistent **`tools-latest`** release tag.

---

## 📥 Direct Download

[**Vanilla True Builder**](https://github.com/gsmeven/vanilla-true-builder) automatically queries and updates these binaries in the background, so manual installation is not required for standard usage.

For custom automation pipelines, third-party scripts, or manual troubleshooting, grab the latest precompiled bundle here:

⤓ **[Download tools.zip (Latest Release)](https://github.com/gsmeven/vtb-ltk-tools/releases/download/tools-latest/tools.zip)**

---

## 🤝 Credits & Acknowledgments

* **[League Toolkit](https://github.com/LeagueToolkit):** Enormous thanks to Crauzer, moonshadow565, and the LTK contributors for pioneering and maintaining the core open-source League of Legends modding infrastructure.
* **[Vanilla True Builder](https://github.com/gsmeven/vanilla-true-builder):** Canonical restoration engine maintained by [gsmeven](https://github.com/gsmeven).
* **Legal Disclaimer:** This project is an independent automated utility and is not affiliated with, endorsed by, or sponsored by Riot Games. League of Legends is a registered trademark of Riot Games, Inc.
