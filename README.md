# LibreVideoDiT

一个用于导入相机卡素材的工具：自动从 SD 卡拷贝视频和照片到指定目录，并按相机型号分类。支持通过 SD 卡结构识别相机，且可为每张卡指定目标文件夹。

## 功能

- 读取 SD 卡文件结构识别相机类型（通过 `config.json` 配置）。
- 从文件元数据读取相机型号，按型号建文件夹分类。
- 可为特定 SD 卡设置固定目标子目录。
- 支持视频/照片扩展名过滤与去重拷贝（哈希校验）。

## 配置

在项目根目录创建 `config.json`，参考 `config.json.example`。

字段说明：

- `destination_root`: 导入目标根目录。
- `video_exts` / `photo_exts`: 需要拷贝的扩展名（不区分大小写）。
- `cameras`: 相机配置，包含 SD 卡结构签名与媒体目录。
  - `signature_paths`: 用于识别相机 SD 卡的关键路径集合。
  - `media_roots`: 需要扫描的媒体目录（可为空，默认扫描整卡）。
- `sd_cards`: 可选，绑定特定 SD 卡挂载路径到固定子目录。

示例：

```json
{
  "destination_root": "/home/user/MediaImports",
  "video_exts": ["mp4", "mov", "mxf", "mts", "m2ts"],
  "photo_exts": ["jpg", "jpeg", "tif", "tiff", "dng", "heic"],
  "cameras": [
    {
      "name": "Sony A7SIII",
      "signature_paths": ["DCIM", "PRIVATE/M4ROOT"],
      "media_roots": ["DCIM", "PRIVATE/M4ROOT"]
    }
  ],
  "sd_cards": [
    {
      "root": "/media/user/SDCARD_A",
      "target_subdir": "Project_A",
      "camera_override": null
    }
  ]
}
```

## 使用方式

当前通过 Tauri 命令调用导入流程：

```ts
import { invoke } from "@tauri-apps/api/core";

const report = await invoke("import_sd_card", {
  sdCardPath: "/media/user/SDCARD_A",
});
```

返回的 `report` 会包含拷贝数量、跳过数量、失败数量与错误列表。

## 开发

```bash
npm install
npm run dev
```

运行 Tauri：

```bash
npm run tauri dev
```

## 测试

```bash
cd src-tauri
cargo test
```

## CI 与发布

- CI：GitHub Actions 会在 Linux 和 Windows 上运行前端构建与 Rust 测试。
- 发布：推送 `v*` 标签（例如 `v0.1.0`）会触发多平台构建并创建草稿 Release。

## 已知限制

- 视频相机型号目前仅从同名 `.xmp` 边车文件读取。
- 未配置 `signature_paths` 的相机将不会被识别。

## 推荐 IDE

- VS Code + Vue Official + Tauri + rust-analyzer
