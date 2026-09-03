const XLSX = require('xlsx');

const ROWS = 300000;
const COLS = 8;
// Small pool of unique strings, heavily reused, similar shape to big_300k_rows.xlsx
const pool = ['North', 'South', 'East', 'West', 'Active', 'Inactive', 'Pending', 'Closed', 'Alpha', 'Beta'];

const ws = {};
let cellCount = 0;
for (let r = 0; r < ROWS; r++) {
  for (let c = 0; c < COLS; c++) {
    const addr = XLSX.utils.encode_cell({ r, c });
    if (c % 2 === 0) {
      ws[addr] = { t: 'n', v: r * COLS + c };
    } else {
      ws[addr] = { t: 's', v: pool[(r + c) % pool.length] };
    }
    cellCount++;
  }
}
ws['!ref'] = XLSX.utils.encode_range({ s: { r: 0, c: 0 }, e: { r: ROWS - 1, c: COLS - 1 } });

const wb = { SheetNames: ['Sheet1'], Sheets: { Sheet1: ws } };
XLSX.writeFile(wb, '/tmp/xlsb_build/big_300k_rows.xlsb', { bookType: 'xlsb', bookSST: true });
console.log('wrote', cellCount, 'cells');
