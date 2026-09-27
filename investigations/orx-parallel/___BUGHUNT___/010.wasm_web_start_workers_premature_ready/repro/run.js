const { chromium } = require("playwright");
const { spawn } = require("node:child_process");
const path = require("node:path");

async function main() {
    const port = 8712;
    const serverProc = spawn("node", [path.join(__dirname, "server.js"), String(port)], {
        stdio: ["ignore", "pipe", "pipe"],
    });
    await new Promise((resolve, reject) => {
        serverProc.stdout.on("data", (d) => {
            if (d.toString().includes("listening")) resolve();
        });
        serverProc.stderr.on("data", (d) => process.stderr.write(d));
        serverProc.on("exit", (code) => reject(new Error("server exited early: " + code)));
        setTimeout(() => reject(new Error("server did not start in time")), 5000);
    });

    const browser = await chromium.launch();
    try {
        const page = await browser.newPage();
        page.on("console", (msg) => console.log("[page] " + msg.text()));
        page.on("pageerror", (err) => console.log("[pageerror] " + err.message));

        await page.goto(`http://127.0.0.1:${port}/index.html`);
        await page.waitForFunction(() => window.__done !== undefined, { timeout: 10000 });
        const result = await page.evaluate(() => window.__done);
        console.log("RESULT_JSON: " + JSON.stringify(result));

        const bugConfirmed = result.resolved === true && result.gotReady === true &&
            result.errorMsg !== null && /wasm worker entrypoint is missing/.test(result.errorMsg);
        // Exit code intentionally always 0: this script is evidence-printing, not a pass/fail
        // gate - "before" is expected to print BUG_CONFIRMED=true and "after" (patched) is
        // expected to print BUG_CONFIRMED=false; both are successful runs of the script itself.
        console.log("BUG_CONFIRMED=" + bugConfirmed);
    } finally {
        await browser.close();
        serverProc.kill();
    }
}

main().catch((err) => {
    console.error(err);
    process.exit(2);
});
