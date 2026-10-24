Essa caixa de `[!TIP]` com lâmpada realmente costuma poluir e quebrar o ritmo da leitura técnica.

Aqui está a seção redesenhada: **sem caixa de alerta**, com texto direto em 2 frases objetivas que mantêm 100% das informações (auto-update do VTB + link para quem precisa avulso) e um visual limpo de documentação profissional:

---

### Opção 1: Texto Corrido & Direto (Recomendada — Mais elegante)

```markdown
## 📥 Direct Download

[**Vanilla True Builder**](https://github.com/gsmeven/vanilla-true-builder) automatically queries and hot-swaps these binaries in the background, so manual installation is not required for normal usage. 

For standalone scripts, custom automation, or manual troubleshooting, grab the latest precompiled bundle here:

⤓ **[Download tools.zip (Latest Release)](https://github.com/gsmeven/vtb-ltk-tools/releases/download/tools-latest/tools.zip)**
```

---

### Opção 2: Estilo Tabela / Especificação Técnica

Se preferir no formato tabular (muito comum em repositórios de binários de CI/CD):

```markdown
## 📥 Direct Download

[**Vanilla True Builder**](https://github.com/gsmeven/vanilla-true-builder) handles syncing and auto-updating these binaries natively. Manual downloads are only needed for custom pipelines or troubleshooting:

| Package | Contents | Platform | Direct Link |
| :--- | :--- | :---: | :--- |
| **`tools.zip`** | `wadtools.exe` + `ritobin.exe` (Rust) | Windows x64 | **[Download Latest Release](https://github.com/gsmeven/vtb-ltk-tools/releases/download/tools-latest/tools.zip)** |
```

---

### O README Completo Atualizado (com a Opção 1 e limpando o `* ` solto no final):

```markdown
# ltk-tools (.exe) for vtb

> Automated CI/CD build bot serving bleeding-edge Windows binaries of **League Toolkit (LTK)** reverse-engineering tools for [**Vanilla True Builder**](https://github.com/gsmeven/vanilla-true-builder).

[![Build and Release Tools](https://github.com/gsmeven/vtb-ltk-tools/actions/workflows/update-tools.yml/badge.svg)](https://github.com/gsmeven/vtb-ltk-tools/actions)
[![Release: tools-latest](https://img.shields.io/badge/release-tools--latest-blue?logo=github)](https://github.com/gsmeven/vtb-ltk-tools/releases/tag/tools-latest)
[![Vanilla True Builder](https://img.shields.io/badge/Companion%20Repo-Vanilla%20True%20Builder-orange?logo=github)](https://github.com/gsmeven/vanilla-true-builder)
[![Check Cadence](https://img.shields.io/badge/cron-every%203%20hours-brightgreen)](#-how-it-works)

---

## 🤖 Autonomous Bot Disclaimer

> [!NOTE]
> **Headless Infrastructure / AI-Generated Bot**  
> This repository, its automated GitHub Actions workflows, and integration pipelines were scaffolded and generated almost entirely with AI assistance. It operates strictly as an autonomous compilation bot designed to track official upstream [League Toolkit](https://github.com/LeagueToolkit) repositories, synthesize and compile the latest reverse-engineering tools directly from Rust source code in the cloud, and serve them with zero human maintenance for [**Vanilla True Builder**](https://github.com/gsmeven/vanilla-true-builder).

---

## 📖 Overview

[**Vanilla True Builder**](https://github.com/gsmeven/vanilla-true-builder) is designed to be a completely self-sufficient and frictionless restoration suite. 

The core League reverse-engineering tools maintained by the **League Toolkit (LTK)** team are developed natively in **Rust** (distributed as source crates like `ltk_wad` and `ltk_ritobin`). Normally, building them requires installing the full Rust toolchain (`rustup`, `cargo`), Git, and MSVC build tools.

To eliminate this barrier for end-users, this repository serves as a headless compilation pipeline. Every **3 hours**, a scheduled GitHub Actions workflow checks upstream LTK commits, compiles clean standalone Windows x64 `.exe` binaries in an isolated cloud runner, and packages them into **`tools.zip`** — ready for immediate consumption.

---

## 🛠️ Included Tools & Specifications

Both tools are part of the modern **Rust**-based League Toolkit ecosystem:

| Binary | Source Repository / Crate | Language | Purpose in Reverse Engineering |
| :--- | :--- | :---: | :--- |
| **`wadtools.exe`** | [LeagueToolkit/wadtools](https://github.com/LeagueToolkit/wadtools) (`ltk_wad`) | **Rust** | Extracts, unpacks, inspects, and repacks League of Legends `.wad.client` archive packages. |
| **`ritobin.exe`** | [LeagueToolkit/ritobin-tools](https://github.com/LeagueToolkit/ritobin-tools) (`ltk_ritobin`) | **Rust** | Serializes binary property tree files (`.bin`) into readable plaintext format and compiles edited text back into binary format. |

---

## ⚙️ How It Works

1. **Smart Commit-State Diffing:**  
   The runner queries the GitHub API for the latest commit hashes across upstream Rust repositories:
   * `LeagueToolkit/wadtools` (`main`)
   * `LeagueToolkit/ritobin-tools` (`main`)
   * `LeagueToolkit/league-toolkit` (`crates/ltk_ritobin`)
   * `LeagueToolkit/league-toolkit` (`crates/ltk_wad`)

2. **Zero Resource Waste:**  
   If the upstream commit hashes match the current release description in [`tools-latest`](https://github.com/gsmeven/vtb-ltk-tools/releases/tag/tools-latest), the workflow aborts immediately to conserve GitHub Action minutes.

3. **Cloud-Native Rust Compilation:**  
   Uses the official Rust stable toolchain on a clean `windows-latest` runner to compile standalone x64 binaries via `cargo install`.

4. **Deterministic Packaging:**  
   Normalizes binary filenames (`wadtools.exe` and `ritobin.exe`), archives them into `tools.zip`, and updates the permanent release tag: **`tools-latest`**.

---

## 📥 Direct Download

[**Vanilla True Builder**](https://github.com/gsmeven/vanilla-true-builder) automatically queries and hot-swaps these binaries in the background, so manual installation is not required for normal usage. 

For standalone scripts, custom automation, or manual troubleshooting, grab the latest precompiled bundle here:

⤓ **[Download tools.zip (Latest Release)](https://github.com/gsmeven/vtb-ltk-tools/releases/download/tools-latest/tools.zip)**

---

## 🤝 Credits & Acknowledgments

* **[League Toolkit](https://github.com/LeagueToolkit):** Enormous thanks to Crauzer, moonshadow565, and the LTK contributors for pioneering and maintaining the foundational open-source League of Legends modding and reverse-engineering infrastructure.
* **[Vanilla True Builder](https://github.com/gsmeven/vanilla-true-builder):** Main project repository maintained by [gsmeven](https://github.com/gsmeven).
* **Legal Disclaimer:** This project is an independent automated utility and is not affiliated with, endorsed by, or sponsored by Riot Games. League of Legends is a registered trademark of Riot Games, Inc.
```
