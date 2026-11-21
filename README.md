# vtb-ltk-tools

Automated Windows x64 CI/CD toolchain builder for **[Vanilla True Builder](https://github.com/gsmeven/vanilla-true-builder)**.

Every 3 hours (or on workflow dispatch), GitHub Actions checks upstream commits and publishes verified release binaries inside `tools.zip` (`tools-latest`):

| Binary in `tools.zip` | Source | Purpose |
| :--- | :--- | :--- |
| `wadtools.exe` | [`LeagueToolkit/wadtools`](https://github.com/LeagueToolkit/wadtools) | Selective `.wad.client` archive extraction |
| `ritobin.exe` | [`LeagueToolkit/ritobin-tools`](https://github.com/LeagueToolkit/ritobin-tools) | Lossless `.bin` ↔ `.bin.txt` conversion |
| `league-mod.exe` | `./vtb-packer` (powered by [`ltk_mod_project`](https://github.com/LeagueToolkit/league-mod/tree/main/crates/ltk_mod_project) & `ltk_modpkg`) | Modular `.modpkg` project packer |

> **Note:** Since the upstream `league-mod` CLI was deprecated by LeagueToolkit, `league-mod.exe` in this distribution is compiled directly from `vtb-packer` against the actively maintained `ltk_mod_project` (`modpkg` feature) crate while keeping CLI flag compatibility (`pack -c -f -o --format modpkg`).
