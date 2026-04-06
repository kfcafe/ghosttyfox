import { build } from "esbuild";
import { copyFile, mkdir } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const projectDir = path.resolve(scriptDir, "..");
const extensionDir = path.join(projectDir, "extension");
const distDir = path.join(extensionDir, "dist");
const wasmSource = path.join(projectDir, "node_modules", "ghostty-web", "ghostty-vt.wasm");
const wasmTarget = path.join(distDir, "ghostty-vt.wasm");
const bundleTarget = path.join(distDir, "terminal.bundle.js");

await mkdir(distDir, { recursive: true });

await build({
  entryPoints: [path.join(extensionDir, "terminal.js")],
  outfile: bundleTarget,
  bundle: true,
  format: "esm",
  platform: "browser",
  target: ["firefox128"],
  sourcemap: true
});

await copyFile(wasmSource, wasmTarget);

console.log(`Bundled extension to ${bundleTarget}`);
console.log(`Copied WASM to ${wasmTarget}`);
