#!/usr/bin/env python3
# Builds 5 variants of the project's own tests/formula.issue.xlsx (dimension originally A1:J14,
# 3 scattered formula cells) with different/wrong <dimension> headers, to exercise
# Range::from_sparse_dense_hint()'s fallback-to-sparse path for the formula-string case.
#
# Usage: python3 gen_dimtest.py <path-to-formula.issue.xlsx> <output-dir>
import re
import sys
import zipfile


def make_variant(src, new_dim, outpath):
    z = zipfile.ZipFile(src)
    sheet = z.read('xl/worksheets/sheet1.xml').decode()
    if new_dim is None:
        sheet2 = re.sub(r'<dimension ref="[^"]*"/>', '', sheet)
    else:
        sheet2 = re.sub(r'<dimension ref="[^"]*"/>', f'<dimension ref="{new_dim}"/>', sheet)
    with zipfile.ZipFile(outpath, 'w') as zout:
        for item in z.infolist():
            data = z.read(item.filename)
            if item.filename == 'xl/worksheets/sheet1.xml':
                data = sheet2.encode()
            zout.writestr(item, data)
    print("wrote", outpath, new_dim)


def main():
    src = sys.argv[1]
    outdir = sys.argv[2]
    make_variant(src, 'A1:J14', f'{outdir}/exact.xlsx')       # matches actual used range exactly
    make_variant(src, 'A1:A1', f'{outdir}/small.xlsx')        # declared box far smaller -> mid-stream bail
    make_variant(src, 'A1:Z50', f'{outdir}/large.xlsx')       # declared box larger -> crop path
    make_variant(src, None, f'{outdir}/nodim.xlsx')           # no <dimension> tag at all
    make_variant(src, 'J14:A1', f'{outdir}/reversed.xlsx')    # end < start (degenerate)


if __name__ == "__main__":
    main()
