# LibreVideoDiT

This app imports photos and videos from camera SD cards, classifies them by camera model, and copies them into a destination folder.

## Config

Create `config.json` in the project root (see `config.json.example`) to describe SD card signatures, media roots, and target folders.

Key fields:

- `destination_root`: where imported media will be placed.
- `video_exts` / `photo_exts`: extensions to copy (case-insensitive).
- `cameras`: each camera includes `signature_paths` to detect a card and optional `media_roots` to scan.
- `sd_cards`: optional mapping for a specific SD card mount path to a subfolder.

## Tauri Command

Call the command from the frontend:

```ts
import { invoke } from "@tauri-apps/api/core";

const report = await invoke("import_sd_card", {
  sdCardPath: "/media/user/SDCARD_A",
});
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
