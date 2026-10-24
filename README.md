# ltk-tools (.exe) for Vanilla True Builder

> Automated CI/CD pipeline that builds and serves bleeding-edge Windows binaries of **League Toolkit (LTK)** tools for [**Vanilla True Builder**](https://github.com/gsmeven/vanilla-true-builder).

[![Build and Release Tools](https://github.com/gsmeven/vtb-ltk-tools/actions/workflows/update-tools.yml/badge.svg)](https://github.com/gsmeven/vtb-ltk-tools/actions)
[![Release: tools-latest](https://img.shields.io/badge/release-tools--latest-blue?logo=github)](https://github.com/gsmeven/vtb-ltk-tools/releases/tag/tools-latest)
[![Vanilla True Builder](https://img.shields.io/badge/Companion%20Repo-Vanilla%20True%20Builder-orange?logo=github)](https://github.com/gsmeven/vanilla-true-builder)
[![Check Cadence](https://img.shields.io/badge/cron-every%203%20hours-brightgreen)](#-how-it-works)

---

## 📖 Overview

[**Vanilla True Builder**](https://github.com/gsmeven/vanilla-true-builder) is designed to be a self-sufficient, frictionless League of Legends mod builder. To work smoothly without forcing users to install Rust, Git, or build toolchains locally, this repository automates the upstream build pipeline.

Every 3 hours, a GitHub Actions workflow monitors the upstream **League Toolkit** repositories for new commits. Whenever changes are detected, it freshly compiles and packages standalone Windows `.exe` files into a single bundle: **`tools.zip`**, the core dependency required to run the builder.

---

## 🛠️ Included Tools

| Binary | Source Repository | Description |
| :--- | :--- | :--- |
| **`wadtools.exe`** | [LeagueToolkit/wadtools](https://github.com/LeagueToolkit/wadtools) | Extracts and builds `.wad.client` game archives. |
| **`ritobin.exe`** | [LeagueToolkit/ritobin-tools](https://github.com/LeagueToolkit/ritobin-tools) | Serializes/deserializes League `.bin` property files to text. |

---

## ⚙️ How It Works

1. **Smart Commit-State Diffing:**  
   The runner queries the GitHub API for the latest commit hashes across upstream locations:
   - `LeagueToolkit/wadtools` (`main`)
   - `LeagueToolkit/ritobin-tools` (`main`)
   - `LeagueToolkit/league-toolkit` (`crates/ltk_ritobin`)
   - `LeagueToolkit/league-toolkit` (`crates/ltk_wad`)

2. **Zero Resource Waste:**  
   If the upstream state matches the current description in the `tools-latest` release, the runner exits immediately to conserve GitHub Action minutes.

3. **Clean Windows Compilation:**  
   Uses standard Rust stable toolchain on `windows-latest` to build standalone executables via `cargo install`.

4. **Predictable Packaging:**  
   Normalizes the binary filenames (`wadtools.exe` and `ritobin.exe`), bundles them into `tools.zip`, and updates the permanent release tag: `tools-latest`.

---

## 📦 Direct Download

For [**Vanilla True Builder**](https://github.com/gsmeven/vanilla-true-builder) scripts or manual usage, the latest bundle can always be fetched directly from:

🔗 **[Download tools.zip (Latest Release)](https://github.com/gsmeven/vtb-ltk-tools/releases/download/tools-latest/tools.zip)**

---

## 🤖 AI Disclaimer & Purpose

> **Notice:** This repository and its automation scripts were **predominantly generated and structured by AI**. Its sole purpose is to act as a lightweight, automated pipeline to synthesize and package bleeding-edge reverse-engineering tools from upstream LTK directly for [**Vanilla True Builder**](https://github.com/gsmeven/vanilla-true-builder).

---

## 🤝 Credits & Acknowledgments

* **[League Toolkit](https://github.com/LeagueToolkit):** Enormous thanks to the LTK team and contributors for maintaining the core open-source League of Legends modding infrastructure.
* **[Vanilla True Builder](https://github.com/gsmeven/vanilla-true-builder):** Main project repository maintained by [gsmeven](https://github.com/gsmeven).
* **Disclaimer:** This project is an independent tool and is not affiliated with, endorsed by, or sponsored by Riot Games. League of Legends is a registered trademark of Riot Games, Inc.
