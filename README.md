# ⚙️ VTB LTK Tools

> Automated CI/CD build bot serving bleeding-edge Windows binaries of **League Toolkit (LTK)** for **Vanilla True Builder**.

[![Build and Release Tools](https://github.com/gsmeven/vtb-ltk-tools/actions/workflows/update-tools.yml/badge.svg)](https://github.com/gsmeven/vtb-ltk-tools/actions)
[![Release: tools-latest](https://img.shields.io/badge/release-tools--latest-blue?logo=github)](https://github.com/gsmeven/vtb-ltk-tools/releases/tag/tools-latest)
[![Check Cadence](https://img.shields.io/badge/cron-every%203%20hours-brightgreen)](#-how-it-works)

---

## 🤖 Autonomous Bot Disclaimer

> [!NOTE]
> **Headless Infrastructure / AI-Generated Bot**  
> This repository, its automated GitHub Actions workflows, and integration pipelines were scaffolded and generated almost entirely with AI assistance. It operates strictly as an autonomous compilation bot designed to track official upstream [League Toolkit](https://github.com/LeagueToolkit) repositories, build native Windows binaries directly from source in the cloud, and distribute them with zero human maintenance.

---

## 📖 Overview

**Vanilla True Builder** is designed to be a completely self-sufficient and frictionless restoration suite. To guarantee stability without forcing end-users to install Rust, Cargo, Git, or complex compilation toolchains locally, this repository automates the upstream build process.

Every 3 hours, a scheduled GitHub Actions workflow monitors the upstream **League Toolkit** repositories for new commits. Whenever fresh updates are detected, it compiles clean standalone Windows `.exe` binaries in an isolated cloud runner and bundles them into **`tools.zip`** - the core binary dependency consumed by Vanilla True Builder.

---

## 🛠️ Included Tools

| Binary | Source Repository | Purpose |
| :--- | :--- | :--- |
| **`wadtools.exe`** | [LeagueToolkit/wadtools](https://github.com/LeagueToolkit/wadtools) | Extracts, unpacks, and repacks `.wad.client` game archives. |
| **`ritobin.exe`** | [LeagueToolkit/ritobin-tools](https://github.com/LeagueToolkit/ritobin-tools) | Serializes and deserializes League `.bin` property trees to clean text. |

---

## ⚙️ How It Works

1. **Smart Commit-State Diffing:**  
   The runner queries the GitHub API for the latest commit hashes across upstream locations:
   * `LeagueToolkit/wadtools` (`main`)
   * `LeagueToolkit/ritobin-tools` (`main`)
   * `LeagueToolkit/league-toolkit` (`crates/ltk_ritobin`)
   * `LeagueToolkit/league-toolkit` (`crates/ltk_wad`)

2. **Zero Resource Waste:**  
   If the upstream commit hashes match the current release description in `tools-latest`, the workflow aborts immediately to conserve GitHub Action minutes.

3. **Cloud Native Compilation:**  
   Uses the official Rust stable toolchain on a clean `windows-latest` runner to compile standalone x64 binaries via `cargo install`.

4. **Deterministic Packaging:**  
   Normalizes binary names (`wadtools.exe` and `ritobin.exe`), archives them into `tools.zip`, and updates the permanent release tag: **`tools-latest`**.

---

## 📦 Direct Download

For Vanilla True Builder scripts, automated workflows, or standalone usage, the latest binary bundle can always be fetched directly from:

🔗 **[Download tools.zip (Latest Release)](https://github.com/gsmeven/vtb-ltk-tools/releases/download/tools-latest/tools.zip)**

---

## 🤝 Credits & Acknowledgments

* **[League Toolkit](https://github.com/LeagueToolkit):** Enormous thanks to Crauzer, moonshadow565, and the LTK contributors for pioneering and maintaining the foundational open-source League of Legends modding infrastructure.
* **Legal Disclaimer:** This project is an independent automated utility and is not affiliated with, endorsed by, or sponsored by Riot Games. League of Legends is a registered trademark of Riot Games, Inc.
