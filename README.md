# Excalidraw Desktop

An unofficial desktop version of [Excalidraw](https://excalidraw.com), built with [Tauri v2](https://v2.tauri.app). This project wraps the full Excalidraw web application in a native desktop shell, giving you the same drawing experience without needing a browser.

## About

[Excalidraw](https://github.com/excalidraw/excalidraw) is an open-source virtual whiteboard with a hand-drawn feel. This desktop port tracks the upstream repository closely and adds a minimal Tauri v2 shell on top, keeping changes to the original codebase as small as possible.

**This project is not affiliated with the Excalidraw team.** It is an independent effort to bring Excalidraw to the desktop.

## Download

Head to the [Releases](https://github.com/imgajeed/excalidraw_desktop/releases) page and download the latest build for your platform:

| Platform              | Format                      |
| --------------------- | --------------------------- |
| Windows               | `.msi`, `.exe`              |
| macOS (Intel)         | `.dmg`                      |
| macOS (Apple Silicon) | `.dmg`                      |
| Linux                 | `.deb`, `.rpm`, `.AppImage` |

## Building from source

### Prerequisites

- [Node.js](https://nodejs.org) (LTS)
- [Yarn](https://yarnpkg.com) (`corepack enable`)
- [Rust](https://rustup.rs) (stable)
- System dependencies for your platform (see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/))

### Development

```bash
# Install dependencies
yarn install

# Run in development mode (hot-reload)
yarn tauri dev
```

### Production build

```bash
yarn tauri build
```

Build artifacts will be in `src-tauri/target/release/bundle/`.

## Updating from upstream

This repo tracks [excalidraw/excalidraw](https://github.com/excalidraw/excalidraw) via a git remote:

```bash
git fetch upstream
git merge upstream/master
```

The only modified upstream file is `excalidraw-app/vite.config.mts` (minor Tauri integration tweaks). Everything else (`src-tauri/`, `.gitignore` additions, `package.json` devDependency) is additive and won't conflict.

## Project structure

```
.
├── excalidraw-app/       # Excalidraw web app (from upstream)
├── packages/             # Excalidraw core packages (from upstream)
├── src-tauri/            # Tauri v2 desktop shell (our addition)
│   ├── tauri.conf.json   # Tauri configuration
│   ├── Cargo.toml        # Rust dependencies
│   ├── src/              # Rust entry points
│   ├── icons/            # App icons (generated from Excalidraw logo)
│   └── capabilities/     # Tauri v2 permissions
└── ...                   # Other upstream excalidraw files
```

## Acknowledgments

- [Excalidraw](https://github.com/excalidraw/excalidraw) - the original open-source whiteboard
- [Tauri](https://tauri.app) - the framework powering the desktop shell

Desktop port by [Oliver Seifert](https://oseifert.ch)

## License

This project is open-source and available under the same license as the original Excalidraw project (MIT).
