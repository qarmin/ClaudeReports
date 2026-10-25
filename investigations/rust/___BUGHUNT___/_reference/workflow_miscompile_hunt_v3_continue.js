// Continuation after an abrupt power loss killed the v3 workflow mid-campaign. The Build phase
// (identity-chain generator: templates/identity_chain.py + identity_chain_unsafe.py) already
// completed and was validated before the crash (check_canaries.sh PASS, many rounds of 0
// self-check mismatches through round 21 per ROUNDS.log) - confirmed on disk, so this workflow
// skips straight to Campaign with a FRESH time budget (resuming the old run via its runId would
// replay a stale cached start-of-session clock reading and miscompute the deadline).
//
// args: { projectPath, optDir, fuzzerDir, budgetMinutes, skillDir }

export const meta = {
  name: 'rustc-miscompile-fuzzer-v3-continue',
  description: 'Campaign-only continuation: identity-chain generator already built and validated, just run more rounds with a fresh time budget',
  phases: [
    { title: 'Campaign' },
  ],
}

const { projectPath, optDir, fuzzerDir, budgetMinutes, skillDir } = args

const NEVER_EDIT = `HARD RULES, no exceptions: (1) NEVER edit, create, or delete any file inside ` +
  `${projectPath} except under ${optDir}/ - never run ./x build, ./x test, ./x fmt, or cargo ` +
  `inside the rust-lang/rust checkout itself, and never build rustc from source. (2) NEVER attempt ` +
  `a fix/patch for anything found - "not attempted, out of scope for this session" every time. ` +
  `(3) NEVER write to GitHub - gh issue view/--comments read-only is fine, nothing else. ` +
  `(4) Build/run generated or repro programs only under ${optDir}/ or a /tmp scratch dir.`

async function nowEpoch(label) {
  const r = await agent(
    'Run `date +%s` in bash and return exactly its numeric stdout as epochSeconds - nothing else.',
    { phase: 'Campaign', label: label || 'clock-check',
      schema: { type: 'object', properties: { epochSeconds: { type: 'number' } }, required: ['epochSeconds'] } }
  )
  return r.epochSeconds
}

const startEpoch = await nowEpoch('session-start-clock')
const overallDeadlineEpoch = startEpoch + budgetMinutes * 60
log(`continuation: ${budgetMinutes} min, campaign-only (build already done before the power loss)`)

phase('Campaign')
let roundNum = 0
let newFindingsCount = 0

while (true) {
  const t = await nowEpoch('campaign-clock')
  if (t >= overallDeadlineEpoch) {
    log(`stopping campaign: time budget elapsed (~${Math.round((t - startEpoch) / 60)}/${budgetMinutes} min)`)
    break
  }
  const remainingMin = (overallDeadlineEpoch - t) / 60
  roundNum++
  const roundBudgetMin = Math.max(5, Math.min(20, remainingMin))
  log(`campaign round ${roundNum}: ~${Math.round(roundBudgetMin)} min this round, ` +
      `~${Math.round(remainingMin)} min left overall`)

  const round = await agent(
    `${NEVER_EDIT}\n\n` +
    `Run one campaign round at ${fuzzerDir}/ for roughly ${Math.round(roundBudgetMin)} minutes of ` +
    `real wall-clock work. First, read ${fuzzerDir}/ROUNDS.log's tail (last ~20 lines) to see ` +
    `exactly what's already been tried (categories, --depth/--breadth/--cases-per-file/--noise-level ` +
    `combos, which rustc toolchain) so this round explores something not already covered rather than ` +
    `repeating an identical config - the identity_chain_safe/identity_chain_unsafe categories and the ` +
    `17 pre-existing + 4 static categories are all available (see README.md). Weight toward ` +
    `identity_chain_safe per the user's stated safe-Rust-first priority, but also mix in the ` +
    `pre-existing categories and identity_chain_unsafe. Use run_round.sh. Append a line to ` +
    `ROUNDS.log. Before reporting a divergence, re-verify it (re-run both configs again), re-read ` +
    `the specific generated program for a bug in the identity-chain's own transform library (would ` +
    `show up as a self-check mismatch against .expected, not a cross-config divergence - don't ` +
    `conflate the two) or pre-existing UB - only report ones you're confident are real cross-config ` +
    `divergences. Report cumulative sub-cases tested and whether this round feels dry.`,
    { phase: 'Campaign', label: `campaign-round-${roundNum}`, effort: 'high',
      schema: { type: 'object', properties: {
        divergencesFound: { type: 'array', items: { type: 'object', properties: {
          description: { type: 'string' }, repro_path: { type: 'string' },
          config_a_cmd: { type: 'string' }, config_a_output: { type: 'string' },
          config_b_cmd: { type: 'string' }, config_b_output: { type: 'string' },
        }, required: ['description', 'repro_path', 'config_a_cmd', 'config_a_output', 'config_b_cmd', 'config_b_output'] } },
        cumulativeSubCasesTested: { type: 'number' },
        feelsDry: { type: 'boolean' },
      }, required: ['divergencesFound', 'cumulativeSubCasesTested', 'feelsDry'] } }
  )

  if (round === null) {
    log(`campaign round ${roundNum} failed to complete (possible [cyber] safeguard false positive) - continuing`)
    continue
  }

  if (round.divergencesFound.length === 0) {
    log(`round ${roundNum}: no divergences, ${round.cumulativeSubCasesTested} sub-cases cumulatively`)
  } else {
    log(`round ${roundNum}: ${round.divergencesFound.length} candidate divergence(s) found - triaging`)
    for (const d of round.divergencesFound) {
      const tt = await nowEpoch('triage-clock')
      if (tt >= overallDeadlineEpoch) { log('stopping triage: time budget elapsed mid-batch'); break }

      const claim = await agent(
        `Assign the next sequential 3-digit candidate number (continuing whatever numbering ` +
        `${optDir}/SUMMARY.md and existing NNN.* folders under ${optDir} already established) for ` +
        `a new fuzzer-found candidate. Return it.`,
        { phase: 'Campaign', schema: { type: 'object', properties: { number: { type: 'string' } }, required: ['number'] } }
      )
      if (claim === null) continue

      const triage = await agent(
        `${NEVER_EDIT}\n\n` +
        `The differential-fuzzer harness at ${fuzzerDir}/ reported this possible rustc ` +
        `miscompilation - triage it as bug-hunt candidate ${claim.number} in workspace ${optDir} ` +
        `(ground rules: no patch.diff, no fix, see ${optDir}/README.md). Independently re-run it at ` +
        `least 3x per config. If it came from an identity-chain category, MINIMIZE it by hand first ` +
        `(strip transform-tree nodes one at a time, re-checking the divergence still reproduces ` +
        `after each removal) down to the smallest sub-tree that still diverges - note which specific ` +
        `transform(s) were load-bearing. If confirmed real: write ${optDir}/${claim.number}.<SHORT_NAME>/ ` +
        `(RESULT.md from ${skillDir}/templates/RESULT_template.md - "The fix" says "not attempted, ` +
        `out of scope"; repro/; output.txt; justfile from ${optDir}/candidate_justfile_template), ` +
        `best-effort bisect via cargo-bisect-rustc, append rows to ${optDir}/SUMMARY.md and ` +
        `SUMMARY-SHORT.md yourself (Class "miscompile"). If it does NOT hold up, explain what ` +
        `actually caused it - if it points at a harness bug, fix it, but do NOT create a candidate ` +
        `folder.\n\nDivergence report: ${JSON.stringify(d)}`,
        { phase: 'Campaign', label: `triage-${claim.number}`, effort: 'high' }
      )
      if (triage !== null) newFindingsCount++
    }
  }
}

const finalT = await nowEpoch('final-clock')
const totalElapsedMin = Math.round((finalT - startEpoch) / 60)
log(`continuation done: ${newFindingsCount} new confirmed finding(s) from ${roundNum} campaign ` +
    `round(s), ~${totalElapsedMin}/${budgetMinutes} min elapsed`)

return {
  newFindingsCount,
  campaignRounds: roundNum,
  elapsedMinutes: totalElapsedMin,
  budgetMinutes,
}
