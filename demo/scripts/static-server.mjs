import fs from "node:fs/promises";
import path from "node:path";
import { createServer } from "node:http";
export async function serveStatic(directory, prefix = "/") {
  const root = path.resolve(directory);
  if (!prefix.startsWith("/") || !prefix.endsWith("/"))
    throw Error("Prefix must begin and end with /");
  const server = createServer(async (req, res) => {
    try {
      const pathname = decodeURIComponent(
        new URL(req.url, "http://localhost").pathname,
      );
      if (!pathname.startsWith(prefix)) {
        res.writeHead(404).end();
        return;
      }
      const relative = pathname.slice(prefix.length) || "index.html";
      const file = path.resolve(root, relative);
      if (!file.startsWith(root + path.sep)) {
        res.writeHead(403).end();
        return;
      }
      const mime =
        {
          ".html": "text/html",
          ".js": "text/javascript",
          ".wasm": "application/wasm",
          ".css": "text/css",
          ".json": "application/json",
          ".woff2": "font/woff2",
          ".woff": "font/woff",
          ".ttf": "font/ttf",
        }[path.extname(file)] || "application/octet-stream";
      const data = await fs.readFile(file);
      res.writeHead(200, { "Content-Type": mime });
      res.end(data);
    } catch {
      res.writeHead(404).end();
    }
  });
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  return { server, url: `http://127.0.0.1:${server.address().port}${prefix}` };
}
