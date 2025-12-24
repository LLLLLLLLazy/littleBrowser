# 字体放置说明（解决中文显示成方块）

如果界面里中文显示成“方框/口口”，通常是因为默认字体不包含中文字形（glyph）。

本项目会在启动时尝试加载以下路径的字体（优先级从高到低）：

- `assets/fonts/NotoSansSC-Regular.otf`
- `assets/fonts/NotoSansSC-Regular.ttf`
- `assets/fonts/NotoSansCJKsc-Regular.otf`
- `assets/fonts/NotoSansCJKsc-Regular.ttf`

建议做法：下载一个支持中文的开源字体（例如 Noto Sans SC / Noto Sans CJK SC），把单个 `.otf` 或 `.ttf` 文件放到本目录，并命名为上述文件名之一。

注意：macOS 系统自带很多中文字体是 `.ttc`（字体集合），当前 egui 直接加载 `.ttc` 往往会失败；因此建议放置 `.otf/.ttf` 文件到本目录。
