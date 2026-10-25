# vtb-ltk-tools

[![Build Status](https://github.com/gsmeven/vtb-ltk-tools/actions/workflows/update-tools.yml/badge.svg)](https://github.com/gsmeven/vtb-ltk-tools/actions/workflows/update-tools.yml)
[![Release](https://img.shields.io/github/v/release/gsmeven/vtb-ltk-tools?color=blue)](https://github.com/gsmeven/vtb-ltk-tools/releases/tag/tools-latest)

Automated CI/CD pipeline that builds and serves bleeding-edge Windows binaries of League Toolkit (LTK) tools for [Vanilla True Builder](https://github.com/gsmeven/vanilla-true-builder).

## Overview

This repository acts as an autonomous binary supply chain for Vanilla True Builder. It monitors upstream LeagueToolkit source repositories, builds native 64-bit Windows executables (`wadtools.exe` and `ritobin.exe`) via GitHub Actions, and distributes them through a rolling release package.

## Pipeline Workflow

The workflow (`update-tools.yml`) executes every 3 hours via cron schedule or through manual dispatch (`workflow_dispatch`):

1. **Upstream Commit Inspection:** Queries GitHub APIs for the latest commits across:
   - `LeagueToolkit/wadtools`
   - `LeagueToolkit/ritobin-tools`
   - `LeagueToolkit/league-toolkit` (`crates/ltk_ritobin` and `crates/ltk_wad`)
2. **State Comparison:** Compares commit hashes against the existing release body. If no changes are detected, compilation is skipped.
3. **Compilation:** Builds target crates on a clean `windows-latest` runner using the stable Rust toolchain (`cargo install --git`).
4. **Binary Normalization:** Standardizes executable names to `wadtools.exe` and `ritobin.exe`.
5. **Release Distribution:** Packages binaries into `tools.zip` and updates the rolling `tools-latest` release tag with commit provenance.

## Included Binaries

| Binary | Upstream Repository | Role in Vanilla True Builder |
| :--- | :--- | :--- |
| `wadtools.exe` | [LeagueToolkit/wadtools](https://github.com/LeagueToolkit/wadtools) | Decompression and selective extraction of `.wad.client` archives. |
| `ritobin.exe` | [LeagueToolkit/ritobin-tools](https://github.com/LeagueToolkit/ritobin-tools) | Decompilation and serialization of Riot property trees (`PROP` / `.bin`). |

## Distribution & Consumption

- **Direct Download:** [`tools.zip`](https://github.com/gsmeven/vtb-ltk-tools/releases/download/tools-latest/tools.zip)
- **Release Channel:** [`tools-latest`](https://github.com/gsmeven/vtb-ltk-tools/releases/tag/tools-latest)

Vanilla True Builder queries this release conditionally at runtime using HTTP `If-None-Match` (ETag) headers, downloading updates only when upstream changes occur.

## Autonomous Pipeline Notice

- **Zero Human Maintenance:** This repository is an entirely autonomous, machine-operated infrastructure pipeline. It is not actively maintained by any human developer and does not accept issues or pull requests.
- **AI-Architected:** The automation logic, workflow orchestration, and documentation in this repository were generated with artificial intelligence assistance.
- **Upstream Mirror:** The pipeline serves strictly as an automated compiler and distribution bridge, mirroring official LeagueToolkit source repositories without human intervention.

## Upstream & Acknowledgements

Underlying tools are created and maintained by the [LeagueToolkit](https://github.com/LeagueToolkit) project:

- [Crauzer](https://github.com/Crauzer) — Creator of LeagueToolkit and archive utilities.
- [moonshadow565](https://github.com/moonshadow565) — Creator of `ritobin` and original property format specifications.

## License

Workflow automation scripts in this repository are licensed under the [GNU General Public License v3.0](LICENSE). Compiled utilities retain their respective upstream licenses (GPL-3.0 / MIT).
