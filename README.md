# ltk-tools (.exe) for vtb

> Automated CI/CD pipeline that builds and serves bleeding-edge Windows binaries of **League Toolkit (LTK)** tools for [**Vanilla True Builder**](https://github.com/gsmeven/vanilla-true-builder).

[![Build and Release Tools](https://github.com/gsmeven/vtb-ltk-tools/actions/workflows/update-tools.yml/badge.svg)](https://github.com/gsmeven/vtb-ltk-tools/actions)
[![Release: tools-latest](https://img.shields.io/badge/release-tools--latest-blue?logo=github)](https://github.com/gsmeven/vtb-ltk-tools/releases/tag/tools-latest)
[![Check Cadence](https://img.shields.io/badge/cron-every%203%20hours-brightgreen)](#-how-it-works)

---

## 🤖 Autonomous Bot Disclaimer

This repository, its automated GitHub Actions pipelines, and packaging routines were scaffolded and are maintained by an AI agent acting for [Neve](https://github.com/gsmeven). It operates strictly as an autonomous compilation bot designed to track upstream **League Toolkit** repositories, build native Windows binaries directly from Rust source in the cloud, and provide zero-maintenance dependencies for **Vanilla True Builder**.

---

## 📖 Overview

**Vanilla True Builder** is designed to be a self-sufficient, frictionless League of Legends mod builder. To work smoothly without forcing users to install the Rust toolchain, Cargo, Git, or C++ compilers locally, this repository automates the upstream build pipeline.

Every 3 hours, a GitHub Actions workflow monitors the upstream **League Toolkit** repositories for new commits. Whenever changes are detected, it freshly compiles and packages standalone Windows `.exe` files into a single bundle: **`tools.zip`**, the core dependency required to run the builder.

---

## 🛠️ Included Tools

Both tools are compiled directly from upstream **Rust** source crates:

| Binary | Source Repository / Crate | Language | Description |
| :--- | :--- | :---: | :--- |
| **`wadtools.exe`** | [LeagueToolkit/wadtools](https://github.com/LeagueToolkit/wadtools) (`ltk_wad`) | **Rust** | Extracts and builds `.wad.client` game archives. |
| **`ritobin.exe`** | [LeagueToolkit/ritobin-tools](https://github.com/LeagueToolkit/ritobin-tools) (`ltk_ritobin`) | **Rust** | Serializes/deserializes League `.bin` property files to text. |

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
   Uses the official Rust stable toolchain on `windows-latest` to build standalone 64-bit executables via `cargo install`.

4. **Predictable Packaging:**  
   Normalizes binary filenames (`wadtools.exe` and `ritobin.exe`), bundles them into `tools.zip`, and updates the permanent release tag: `tools-latest`.

---

## 📦 Direct Download

**Vanilla True Builder** automatically handles syncing and updating these binaries in the background.

For manual usage, custom automation, or standalone scripts, the latest precompiled bundle can always be fetched directly from:

🔗 **[Download tools.zip (Latest Release)](https://github.com/gsmeven/vtb-ltk-tools/releases/download/tools-latest/tools.zip)**

---

## 🤝 Credits & Acknowledgments

* **[Neve](https://github.com/gsmeven):** Creator and maintainer of **Vanilla True Builder**.
* **[League Toolkit](https://github.com/LeagueToolkit):** Enormous thanks to Crauzer, moonshadow565, and the LTK contributors for pioneering and maintaining the core open-source League of Legends modding infrastructure.
* **Disclaimer:** This project is an independent tool and is not affiliated with, endorsed by, or sponsored by Riot Games. League of Legends is a registered trademark of Riot Games, Inc.
