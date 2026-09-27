// Same trigger question as probe_context_loss.js, for a wider set of genuine WebGL-level
// failure conditions (undersized buffer, invalid dimensions, out-of-bounds sub-rectangle,
// deleted texture object) instead of just context loss.
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
      '--enable-unsafe-swiftshader',
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
  page.on('pageerror', (err) => console.log('PAGE ERROR:', err.message));

  const result = await page.evaluate(() => {
    const canvas = document.createElement('canvas');
    canvas.width = 64;
    canvas.height = 64;
    const gl = canvas.getContext('webgl2');
    if (!gl) return { error: 'no webgl2 context' };

    const out = {};
    const tex = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, tex);
    gl.texStorage2D(gl.TEXTURE_2D, 1, gl.RGBA32UI, 8, 8);

    // Case A: buffer too small for requested width/height/format (a genuine data.len() mismatch,
    // like what would happen if the debug_assert! in vello_hybrid were wrong/stripped in release).
    const tooSmall = new Uint32Array(2); // needs 4*1*4=16 u32s for width=4,rows=1
    try {
      gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, 4, 1, gl.RGBA_INTEGER, gl.UNSIGNED_INT, tooSmall);
      out.tooSmallThrew = false;
    } catch (e) {
      out.tooSmallThrew = true;
      out.tooSmallError = String(e);
    }
    out.glErrorAfterTooSmall = gl.getError();

    // Case B: negative/out-of-range width via a huge u32 that wraps to negative i32 (simulating
    // a hypothetical arithmetic bug upstream feeding this function a bogus width).
    const dataOk = new Uint32Array(4 * 1 * 4);
    try {
      gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, -1, 1, gl.RGBA_INTEGER, gl.UNSIGNED_INT, dataOk);
      out.negWidthThrew = false;
    } catch (e) {
      out.negWidthThrew = true;
      out.negWidthError = String(e);
    }
    out.glErrorAfterNegWidth = gl.getError();

    // Case C: out-of-bounds sub-rectangle (xoffset+width exceeds texture storage size).
    try {
      gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, 999, 999, gl.RGBA_INTEGER, gl.UNSIGNED_INT, new Uint32Array(999*999*4));
      out.oobThrew = false;
    } catch (e) {
      out.oobThrew = true;
      out.oobError = String(e);
    }
    out.glErrorAfterOob = gl.getError();

    // Case D: deleted texture bound at call time.
    const tex2 = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, tex2);
    gl.texStorage2D(gl.TEXTURE_2D, 1, gl.RGBA32UI, 4, 4);
    gl.deleteTexture(tex2);
    try {
      gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, 4, 1, gl.RGBA_INTEGER, gl.UNSIGNED_INT, dataOk);
      out.deletedTexThrew = false;
    } catch (e) {
      out.deletedTexThrew = true;
      out.deletedTexError = String(e);
    }
    out.glErrorAfterDeletedTex = gl.getError();

    return out;
  });

  console.log(JSON.stringify(result, null, 2));
  await browser.close();
})();
