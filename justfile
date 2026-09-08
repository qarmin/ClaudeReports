sync:
    python3 find_reports.py ~/test || true
    python3 find_details.py ~/test || true
    python3 find_reports.py ~/Projekty/Obce || true
    python3 find_details.py ~/Projekty/Obce || true

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
