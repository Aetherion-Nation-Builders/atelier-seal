# Atelier Seal app icon

<img src="pdfcraft-small.svg" alt="Atelier Seal app icon: the hidden sun behind an ochre wax seal" width="128">

**The hidden sun behind a wax seal**: an ivory disc rising behind an ochre wax seal, scalloped at the
edge and stamped with the sun mark, on a volcanic black field. Three colours, from the Nation's
palette: volcanic black `#141210`, ivory `#efe6d2`, ochre `#d9a441`.

**Tile:** `viewBox="0 0 512 512"`, a rounded square with `rx=112`, as all the Atelier icons. Windows
and Linux icons use the full tile. macOS icons put it on Apple's grid (an 824 px body centred on a
transparent 1024 px canvas).

**Provenance:** drawn for Atelier Seal by hand as SVG (2026). It replaces PdfCraft's lion icon, which
stays in PdfCraft's own repository. Licence: [LICENSE.txt](LICENSE.txt) (`MIT OR Apache-2.0`). The
file names keep their upstream `pdfcraft` names so packaging stays untouched.

## Files

| File | What it is |
|---|---|
| `pdfcraft.svg` | the master vector; every PNG, `.ico` and `.icns` is rendered from it |
| `pdfcraft-small.svg` | the same artwork (upstream kept a lighter trace here) |
| `pdfcraft-1024.png` | 1024 px on Apple's grid; also the runtime Dock icon on macOS |
| `pdfcraft.icns` | macOS icon (16–1024 px) |
| `pdfcraft.ico` | Windows icon (16–256 px), embedded in `pdfcraft.exe` by `apps/pdfcraft/build.rs` |
| `pdfcraft-document.svg` | Windows PDF file icon master: a Lucide document outline, green badge and outlined PDF lettering; no font dependency |
| `pdfcraft-document.ico` | Windows PDF file icon (16–256 px), embedded as resource 2 in `pdfcraft.exe` |
| `hicolor/<n>x<n>/apps/ai.storyteller.pdfcraft.png` | Linux hicolor theme, 16–512 px; the 256 px one is the runtime icon on Windows and Linux |
| `hicolor/scalable/apps/ai.storyteller.pdfcraft.svg` | Linux scalable icon (copy of the master) |

Where it shows: `apps/pdfcraft/src/main.rs` sets the window icon (Dock, taskbar, Alt-Tab, launcher) and the
Wayland app id `ai.storyteller.pdfcraft`; `packaging/linux/ai.storyteller.pdfcraft.desktop` names the
hicolor icon.

Windows PDF files use resource 2 (`"pdfcraft.exe",-2`) when Atelier Seal is chosen as the default
reader; the app keeps resource 1. The document outline is adapted from Lucide `file-text`
1.49.0 (ISC); its badge and outlined lettering are original (`MIT OR Apache-2.0`), covered by
[LICENSE-document.txt](LICENSE-document.txt). Both icons are embedded, requiring no runtime
sidecar. This provides a file type icon; first-page thumbnails still need a thumbnail provider.

## Regenerate

```sh
packaging/icons.sh        # needs resvg and python3; iconutil (macOS) for the .icns
cargo xtask assets        # then update the sha256 values in ATTRIBUTION.toml and run with --write
```
