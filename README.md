# Ghosttyfox

A real terminal in a Firefox tab, powered by Ghostty's WASM terminal engine and a local native host.

Ghosttyfox is a small Firefox extension plus Rust native-messaging host that opens your real local shell in a browser tab. It is built for people who like browser-centric workflows and want a terminal beside docs, dashboards, chat tools, and local coding agent TUIs without leaving Firefox.

## Why it exists

Ghosttyfox is for workflows where the browser is already the main workspace:

- keeping a terminal in the same window as web docs and apps
- pairing well with Tree Style Tabs and other tab-heavy setups
- running local coding agent TUIs next to browser-based tools
- getting a real PTY-backed shell instead of a fake in-browser terminal

The terminal is local. No remote service is involved. Firefox talks to a native host over native messaging, and that host runs your shell in a PTY on your machine.

## Architecture

```text
Firefox tab
  ├── Ghosttyfox extension page
  ├── ghostty-web terminal renderer
  └── native messaging port
            ↓
Rust native host
  ├── Firefox stdio framing
  ├── PTY management
  ├── shell process
  └── resize + output relay
```

`ghostty-web` provides the Ghostty-based terminal engine in the browser. The Rust host handles shell spawn, PTY I/O, and resize events.

## Requirements

- macOS
- Firefox
- Node.js 18+ and npm
- Rust toolchain via `rustup`

## Install

```bash
npm install
bash scripts/install.sh
```

The install script:

- builds the Rust host in release mode
- bundles the extension into `extension/dist/`
- writes the Firefox native messaging manifest to:
  `~/Library/Application Support/Mozilla/NativeMessagingHosts/ghosttyfox.json`

Then load the extension temporarily in Firefox:

1. Open `about:debugging#/runtime/this-firefox`
2. Click **Load Temporary Add-on...**
3. Select `extension/manifest.json`

After that, click the toolbar button to open a terminal tab.

## Development

```bash
bash scripts/dev.sh
```

The dev script:

- builds the Rust host in debug mode
- bundles the extension
- writes the native messaging manifest for the debug binary
- launches Firefox with `web-ext`

You can also run the build step by itself:

```bash
npm run build
```

## Project layout

```text
ghosttyfox/
├── extension/
│   ├── background.js
│   ├── manifest.json
│   ├── terminal.css
│   ├── terminal.html
│   ├── terminal.js
│   └── dist/
├── native-host/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── protocol.rs
│       └── pty.rs
├── native-manifest/
│   └── ghosttyfox.json
├── scripts/
│   ├── bundle.js
│   ├── dev.sh
│   └── install.sh
└── README.md
```

## Troubleshooting

### Native host not found

Re-run `bash scripts/install.sh` and inspect:

`~/Library/Application Support/Mozilla/NativeMessagingHosts/ghosttyfox.json`

Make sure the `path` points at a real `ghosttyfox-host` binary.

### The extension loads but no terminal output appears

Open the Firefox extension page console and check for native messaging errors. Then verify the host built successfully:

```bash
cargo build --manifest-path native-host/Cargo.toml
```

### WASM asset fails to load

Re-run:

```bash
npm run build
```

and confirm these files exist:

- `extension/dist/terminal.bundle.js`
- `extension/dist/ghostty-vt.wasm`

### Resize feels wrong

Ghosttyfox sends terminal size changes from the browser page to the PTY. If sizing looks stale, reload the extension page or restart the debug session so both the extension bundle and native host are refreshed.

## Notes

- Firefox support currently targets Manifest V2.
- This project is macOS-first for now.
- One native host process is used per terminal tab.

## License

MIT
