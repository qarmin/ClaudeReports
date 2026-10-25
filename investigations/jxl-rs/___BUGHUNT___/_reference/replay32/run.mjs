import { WASI } from 'node:wasi';
import fs from 'node:fs';
const files = process.argv.slice(2);
const wasi = new WASI({ version: 'preview1', args: ['replay32', ...files], preopens: { '/': '/' }, returnOnExit: true });
const wasm = await WebAssembly.compile(fs.readFileSync(process.env.WASM));
const inst = await WebAssembly.instantiate(wasm, wasi.getImportObject());
const rc = wasi.start(inst);
process.exitCode = rc;
