import { serveStatic } from "./static-server.mjs";
import { fileURLToPath } from "node:url";
const { url } = await serveStatic(
  fileURLToPath(new URL("../dist", import.meta.url)),
  process.env.PUBLIC_PATH || "/katex-wasm/",
);
console.log(url);
