# Ghosttyfox

A real terminal in a Firefox tab, powered by [Ghostty](https://ghostty.org)'s WASM terminal engine.

Click a button → get a terminal tab running your actual shell. No remote servers. No web terminals. Your local shell, rendered with Ghostty's battle-tested VT100 parser.

## Why

Ghosttyfox is for workflows where the browser is already the main workspace:

- keep a terminal alongside docs, dashboards, and web apps
- pairs well with Tree Style Tabs and tab-heavy setups
- get a real PTY-backed shell, not a fake in-browser terminal
- run local coding agent TUIs next to browser tools

The terminal is fully local. Firefox talks to a small Python native host over native messaging, and that host runs your shell in a PTY on your machine.

## Architecture

```text
┌──────────────────────────────┐      ┌──────────────────────────────┐
│  Firefox Tab                 │      │  Native Host (Python)        │
│                              │      │                              │
│  ghostty-web (WASM)          │◄────►│  PTY management              │
│  Canvas renderer             │ JSON │  Shell spawning ($SHELL)     │
│  Input handling              │      │  Resize support              │
│                              │      │                              │
│  Native Messaging bridge     │      │  stdio protocol              │
└──────────────────────────────┘      └──────────────────────────────┘
```

## Requirements

- Firefox (any recent version)
- Python 3 (ships with macOS and most Linux distros)
- Node.js 18+ and npm (for bundling the extension)
- macOS or Linux

## Install

```bash
npm install
bash scripts/install.sh
```

The install script:
- bundles the extension JS and WASM into `extension/dist/`
- writes the Firefox native messaging manifest (auto-detects macOS vs Linux)

Then load the extension in Firefox:

1. Open `about:debugging#/runtime/this-firefox`
2. Click **Load Temporary Add-on...**
3. Select `extension/manifest.json`

Click the toolbar button to open a terminal tab.

## Development

```bash
bash scripts/dev.sh
```

Or just rebuild the extension bundle:

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
│   └── dist/              (built)
├── native-host/
│   └── ghosttyfox-host    (Python script)
├── native-manifest/
│   └── ghosttyfox.json    (template)
├── scripts/
│   ├── bundle.js
│   ├── dev.sh
│   └── install.sh
└── README.md
```

## Troubleshooting

**Native host not found:** Re-run `bash scripts/install.sh` and check the manifest path in the output points to the right place.

**No terminal output:** Open the browser console on the extension page and look for native messaging errors. Try running the host directly: `echo '{}' | python3 native-host/ghosttyfox-host`

**WASM won't load:** Re-run `npm run build` and confirm `extension/dist/ghostty-vt.wasm` exists.

## License

MIT
