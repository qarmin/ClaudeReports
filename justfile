sync:
    python3 scripts/find_reports.py ~/test || true
    python3 scripts/find_details.py ~/test || true
    python3 scripts/find_reports.py ~/Projekty/Obce || true
    python3 scripts/find_details.py ~/Projekty/Obce || true
    python3 scripts/find_targets.py || true

# Build the GitHub Pages site into tmp/site and open it in the browser.
site:
    python3 scripts/build_site.py . tmp/site
    xdg-open tmp/site/index.html || echo "open $(pwd)/tmp/site/index.html"

# Re-check the next batch of reports against the projects' current code.
refresh limit="10":
    python3 .claude/skills/refresh-reports/refresh.py prepare --limit {{limit}}

# Finish projects parked for the difficulty pass (after their batches are answered).
refresh-finish:
    python3 .claude/skills/refresh-reports/refresh.py finish

refresh-status:
    python3 .claude/skills/refresh-reports/refresh.py status

refresh-selftest:
    python3 .claude/skills/refresh-reports/extract_report.py test
    python3 .claude/skills/refresh-reports/verify_report.py test
    python3 .claude/skills/refresh-reports/difficulty_pass.py test
    python3 .claude/skills/refresh-reports/refresh.py test
