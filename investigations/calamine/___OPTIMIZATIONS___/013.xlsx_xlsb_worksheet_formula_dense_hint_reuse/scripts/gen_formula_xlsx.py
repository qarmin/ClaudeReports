#!/usr/bin/env python3
# Generates a synthetic, formula-heavy XLSX with an accurate <dimension> header, for measuring
# peak RSS on worksheet_formula(). No existing public/free-standing large formula-heavy XLSX
# fixture was found in the time budgeted for this candidate (README.md's own testdata_external
# table only has the 74KB formula_stress_test.xlsx, documented there as too small to be a real
# memory-stress fixture) - this script builds an equivalent to big_300k_rows.xlsx (candidate
# 007's own primary fixture) but with a formula in every cell instead of plain values, using only
# Python's stdlib zipfile (no openpyxl/libreoffice available on this box).
#
# Usage: python3 gen_formula_xlsx.py <rows> <cols> <output.xlsx>
# Default (no args): 300000 rows x 8 cols -> 2.4M formula cells, ~18MB, matching big_300k_rows.xlsx's shape.
import sys
import zipfile


def col_letter(c):
    s = ""
    c += 1
    while c > 0:
        c, r = divmod(c - 1, 26)
        s = chr(65 + r) + s
    return s


def main():
    rows = int(sys.argv[1]) if len(sys.argv) > 1 else 300000
    cols = int(sys.argv[2]) if len(sys.argv) > 2 else 8
    outpath = sys.argv[3] if len(sys.argv) > 3 else "big_formula.xlsx"

    content_types = '''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
<Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
</Types>'''

    root_rels = '''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/>
</Relationships>'''

    workbook = '''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
<sheets><sheet name="Sheet1" sheetId="1" r:id="rId1"/></sheets>
</workbook>'''

    workbook_rels = '''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
</Relationships>'''

    last_col = col_letter(cols - 1)
    dim_ref = f"A1:{last_col}{rows}"

    def sheet_xml():
        parts = [
            '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\n',
            '<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">',
            f'<dimension ref="{dim_ref}"/>',
            '<sheetData>',
        ]
        for r in range(1, rows + 1):
            parts.append(f'<row r="{r}">')
            for c in range(cols):
                cl = col_letter(c)
                ref = f"{cl}{r}"
                formula = f"{c + 1}" if r == 1 else f"{cl}{r - 1}+{c + 1}"
                parts.append(f'<c r="{ref}"><f>{formula}</f><v>{r + c}</v></c>')
            parts.append('</row>')
        parts.append('</sheetData></worksheet>')
        return ''.join(parts)

    with zipfile.ZipFile(outpath, 'w', zipfile.ZIP_DEFLATED) as z:
        z.writestr('[Content_Types].xml', content_types)
        z.writestr('_rels/.rels', root_rels)
        z.writestr('xl/workbook.xml', workbook)
        z.writestr('xl/_rels/workbook.xml.rels', workbook_rels)
        z.writestr('xl/worksheets/sheet1.xml', sheet_xml())

    print("wrote", outpath, f"({rows}x{cols} = {rows * cols} formula cells)")


if __name__ == "__main__":
    main()
