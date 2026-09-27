// Real end-to-end probe of the actual browser/WebGL behavior that
// `upload_rgba32ui_rows`'s `gl.tex_sub_image_2d_with_...().unwrap()`
// (sparse_strips/vello_hybrid/src/render/webgl/mod.rs) depends on: does a real browser's
// texSubImage2D ever throw a JS exception (which would become an `Err` that `.unwrap()` turns
// into a panic) for a genuine WebGL-level failure, or does it only ever set a GL error
// (retrievable via getError(), which the web-sys binding never surfaces as `Err`)?
//
// Set CHROME_PATH to an existing Chrome/Chromium-for-Testing binary if `puppeteer`'s own
// bundled download (triggered by `npm install`) isn't available in your environment.
const puppeteer = require('puppeteer');

(async () => {
  const launchOpts = {
    headless: true,
    args: [
      '--no-sandbox',
      '--use-angle=swiftshader',
      '--use-gl=angle',
      '--enable-webgl',
      '--enable-webgl2',
      '--ignore-gpu-blocklist',
      '--disable-gpu-sandbox',
    ],
  };
  if (process.env.CHROME_PATH) {
    launchOpts.executablePath = process.env.CHROME_PATH;
  }
  const browser = await puppeteer.launch(launchOpts);
  const page = await browser.newPage();
  page.on('console', (msg) => console.log('PAGE LOG:', msg.text()));
  page.on('pageerror', (err) => console.log('PAGE ERROR:', err.message));

  const result = await page.evaluate(() => {
    const canvas = document.createElement('canvas');
    canvas.width = 64;
    canvas.height = 64;
    const gl = canvas.getContext('webgl2');
    if (!gl) return { error: 'no webgl2 context' };

    const out = {};
    out.isContextLostBefore = gl.isContextLost();

    // Set up a real RGBA32UI texture, matching vello_hybrid's usage pattern.
    const tex = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, tex);
    gl.texStorage2D(gl.TEXTURE_2D, 1, gl.RGBA32UI, 4, 4);

    const dataOk = new Uint32Array(4 * 1 * 4); // width=4, rows=1
    // Baseline call before context loss: should succeed.
    try {
      gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, 4, 1, gl.RGBA_INTEGER, gl.UNSIGNED_INT, dataOk);
      out.baselineCallThrew = false;
    } catch (e) {
      out.baselineCallThrew = true;
      out.baselineError = String(e);
    }
    out.glErrorAfterBaseline = gl.getError();

    // Force context loss via the WEBGL_lose_context extension.
    const ext = gl.getExtension('WEBGL_lose_context');
    out.hasLoseContextExt = !!ext;
    if (ext) {
      ext.loseContext();
    }
    out.isContextLostAfterLose = gl.isContextLost();

    // Now call the exact same operation again with the context lost.
    try {
      gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, 4, 1, gl.RGBA_INTEGER, gl.UNSIGNED_INT, dataOk);
      out.lostContextCallThrew = false;
    } catch (e) {
      out.lostContextCallThrew = true;
      out.lostContextError = String(e);
    }
    out.glErrorAfterLostCall = gl.getError();

    // Also try a mismatched-length buffer (too short) while context is still alive-like state,
    // to see if a genuine length-mismatch produces a JS exception instead of a GL error.
    return out;
  });

  console.log(JSON.stringify(result, null, 2));
  await browser.close();
})();
