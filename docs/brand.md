# The brand layer

Atelier Seal is PdfCraft with a different name on the door. We keep the change small on purpose:
upstream lands many changes a day, and every file we touch is a file that can conflict when we pull
them in. So:

**Renamed (what a person sees):** the window title, menus, dialogs and About (with the Atelier mark
and wordmark), the home screen's Nation card, the top-bar "Nation" button, Help-menu links (the
Nation, Atelier, this repository), messages in the UI crate and the apps, the update check (this
repository's releases), Windows file properties, installer and package names, vendor, links and
the "Create PDF with…" context menu, the browser page, the app icon, and where autosaves are kept
for recovery (`~/Library/Application Support/Atelier Seal/Recovery`, `%LOCALAPPDATA%\Atelier Seal\Recovery`).

**Not renamed (machine names):** crate names (`pdfcraft-*`), the binaries (`pdfcraft`,
`pdfcraft-cli`), command ids (`help.discord` opens the Nation, `help.website` the Atelier suite) and
automation ids, the `ai.storyteller.pdfcraft` app id and icon file names in packaging, the settings
folder (eframe storage under "PdfCraft"), the single-instance socket folder, portable-mode markers
(`PdfCraft.portable`, `PdfCraftData`), Windows registry keys and ProgIDs (`PdfCraft.Document`),
environment variables (`PDFCRAFT_*`), the HTTP user agent, the widget function names
(`artcraft_logo`, `artcraft_mark`, now drawing Atelier's marks), and the docs under `docs/`.

## Where the name lives

| Place | File |
|---|---|
| The constants and the translation rewrite | `crates/ui-egui/src/identity.rs` (`PRODUCT`, `UPSTREAM`, `rewrite`, `retired`) |
| Our own strings, with translations | `crates/ui-egui/src/i18n/brand.tsv` (added to each catalog as it loads; the upstream `.tsv` catalogs stay untouched) |
| Where the rewrite runs | `crates/ui-egui/src/i18n/catalog.rs` (`parse_entries`, `with_plain`) and `i18n/mod.rs` (`LangInfo::catalog`) |
| English UI strings | literal `"Atelier Seal"` in `crates/ui-egui/src`, `apps/` and the Help command labels in `crates/engine/src/commands.rs` |
| Links | `crates/engine/src/links.rs` |
| Atelier mark and wordmark | `assets/brand/*.svg`, drawn by `crates/ui-egui/src/widgets.rs` |
| Update check | `apps/pdfcraft/src/updates.rs` (`LATEST`), `crates/ui-egui/src/updates.rs` (`RELEASES_PAGE`) |
| Recovery folder | `crates/ui-egui/src/recovery.rs` |
| Windows file properties | `apps/pdfcraft/build.rs`, `apps/pdfcraft-cli/build.rs` |
| macOS bundle | `packaging/macos/Info.plist.in` (display name, copyright; identifier unchanged) |
| Windows installer | `packaging/windows/pdfcraft.wxs` (names, manufacturer, links, install folder, context menu; registry keys and upgrade code unchanged) |
| Linux desktop entry, AppStream, nfpm | `packaging/linux/ai.storyteller.pdfcraft.*`, `nfpm.yaml` |
| Browser build | `apps/pdfcraft-web/index.html` |
| README and NOTICE | README: our header above upstream's feature sections; NOTICE: ours on top of theirs, kept on merge by `.gitattributes` |
| App icon | `assets/app-icon/` (ours; rows in `ATTRIBUTION.toml`) |

The update check asks this repository for its latest release. Until Atelier Seal publishes
releases (and while the repository is private), it reports that it couldn't reach them; it never
offers PdfCraft's downloads.

Still upstream's name, for later passes: messages from the engine and other library crates, and
the `docs/` manuals. For the packaging pass (real installers): the macOS `.app` bundle and DMG
names (`packaging/macos/package.sh`), the DMG background, and the package scripts' file names.
