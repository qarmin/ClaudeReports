# Correction (2026-10-25, session 4)

The original RESULT.md treated "install command ignored when a softdep exists" as a defect and noted only that the 2012 design might be intentional. modprobe.d(5) (man/modprobe.d.5.scd, softdep section) states it explicitly: "Note: if there are *install* or *remove* commands with the same _modulename_ argument, *softdep* takes precedence." The observed behavior (the install command is skipped, plain insmod is used) is therefore the documented one. The reproduction itself still works (`repro/` shows the install command not running, 5/5), and upstream issue #446 / PR #461 argue for changing the documented behavior, but it is a design change, not a deviation from the documentation.

Status changed from Confirmed to Inconclusive (behavior reproduces, defect status disputed by the documentation). The row was removed from SUMMARY-SHORT.md.
