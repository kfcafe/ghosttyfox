import { Ghostty, Terminal, FitAddon } from "ghostty-web";

const container = document.getElementById("terminal");
const statusLine = "\r\n\x1b[90m[ghosttyfox]\x1b[0m ";

function printStatus(term, message) {
  term.writeln(`${statusLine}${message}`);
}

function decodeBase64(data) {
  return Uint8Array.from(atob(data), (char) => char.charCodeAt(0));
}

function clampSize(value, fallback) {
  const number = Number(value);

  if (!Number.isFinite(number) || number <= 0) {
    return fallback;
  }

  return Math.min(1000, Math.floor(number));
}

async function main() {
  if (!container) {
    throw new Error("Missing #terminal container");
  }

  const ghostty = await Ghostty.load(browser.runtime.getURL("dist/ghostty-vt.wasm"));
  const term = new Terminal({
    ghostty,
    cols: 80,
    rows: 24,
    cursorBlink: true,
    fontSize: 14,
    scrollback: 10000
  });
  const fit = new FitAddon();
  const port = browser.runtime.connectNative("ghosttyfox");

  let disconnected = false;

  term.loadAddon(fit);
  term.open(container);
  fit.fit();
  fit.observeResize();
  term.focus();

  const sendResize = (cols = term.cols, rows = term.rows) => {
    port.postMessage({
      type: "resize",
      cols: clampSize(cols, 80),
      rows: clampSize(rows, 24)
    });
  };

  term.onData((data) => {
    port.postMessage({ type: "input", data });
  });

  term.onResize(({ cols, rows }) => {
    sendResize(cols, rows);
  });

  port.onMessage.addListener((message) => {
    switch (message?.type) {
      case "output": {
        if (typeof message.data === "string") {
          term.write(decodeBase64(message.data));
        }
        break;
      }
      case "exit": {
        const code = Number.isFinite(message.code) ? message.code : 0;
        printStatus(term, `shell exited with code ${code}`);
        break;
      }
      case "error": {
        printStatus(term, `host error: ${message.message ?? "unknown error"}`);
        break;
      }
      default: {
        printStatus(term, `unexpected host message: ${JSON.stringify(message)}`);
      }
    }
  });

  port.onDisconnect.addListener(() => {
    if (disconnected) {
      return;
    }

    disconnected = true;
    const error = browser.runtime.lastError;

    if (error?.message) {
      printStatus(term, `native host disconnected: ${error.message}`);
      return;
    }

    printStatus(term, "native host disconnected");
  });

  sendResize();
  printStatus(term, "connected to native host");
}

main().catch((error) => {
  console.error(error);

  if (container) {
    container.textContent = `Ghosttyfox failed to start: ${error.message}`;
  }
});
