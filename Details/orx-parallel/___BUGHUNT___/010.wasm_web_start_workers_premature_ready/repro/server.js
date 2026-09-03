// Minimal static file server for the repro page. The only special case is
// mapping the directory URL that "../../../../.." resolves to (from deep
// inside the snippets/ tree, mirroring wasm-bindgen's real local-module
// packaging layout) onto public/index.js, exactly like a real deployment's
// bundler/static host would need to serve *some* real resource at that
// resolved URL.
const http = require("node:http");
const fs = require("node:fs");
const path = require("node:path");

const root = path.join(__dirname, "public");

const mime = (p) => {
    if (p.endsWith(".html")) return "text/html";
    if (p.endsWith(".js")) return "text/javascript";
    return "application/octet-stream";
};

const server = http.createServer((req, res) => {
    let urlPath = req.url.split("?")[0];
    if (urlPath === "/" || urlPath === "") urlPath = "/index.js"; // pkg-root resolution
    if (urlPath === "/page") urlPath = "/index.html";
    const filePath = path.join(root, urlPath);
    if (!filePath.startsWith(root)) {
        res.writeHead(403);
        res.end();
        return;
    }
    fs.readFile(filePath, (err, data) => {
        if (err) {
            res.writeHead(404);
            res.end("not found: " + urlPath);
            return;
        }
        res.writeHead(200, { "Content-Type": mime(filePath) });
        res.end(data);
    });
});

const port = process.argv[2] ? Number(process.argv[2]) : 8712;
server.listen(port, "127.0.0.1", () => {
    console.log("listening on " + port);
});
