// TEMPLATE - adapt before passing to the Workflow tool, do not run verbatim.
//
// Used only for the AUTONOMOUS mode described in SKILL.md: discovery-from-scratch and/or a
// budget bigger than one candidate. For a single, user-specified check, don't use this - do the
// candidate loop directly in the conversation (see SKILL.md "Direct mode").
//
// IMPORTANT: this script never calls Date.now()/new Date() itself (Workflow scripts can't), but
// it still honors a real wall-clock budget by asking a tiny agent() to run `date +%s` and return
// the number. That's just data flowing through an ordinary agent() call, resume-safe like any
// other result - not a direct clock read. This is the exact mechanism perf-optimize's own
// templates/workflow_autonomous.js uses (built after a guessed-candidate-count translation
// overran a stated budget badly in practice) - see that skill's SKILL.md "Budget translation" for
// the full rationale if this needs re-deriving later.
//
// NOTE: this is the Workflow-tool mechanism. If an environment has no Workflow tool at all, see
// SKILL.md "Autonomous mode: running the Workflow" for the fork-based replacement - a fork
// replicates this same script's logic by hand (including the maxParallel batching below), it just
// isn't literally this JS file.
//
// args shape expected by this script:
//   {
//     projectPath: string,           // absolute path to the project root being checked
//     optDir: string,                // absolute path to <projectPath>/___BUGHUNT___
//     budgetMinutes: number|null,    // wall-clock budget in minutes, or null for a pure count budget
//     targetCandidates: number|null, // candidate-count cap, or null for a pure time budget
//     focusHint: string,             // optional - a specific area/subsystem the user pointed at, else ""
//     diskSpaceNote: string,         // optional - a disk-space constraint to enforce this session, else ""
//     maxParallel: number,           // optional - candidates investigated at once per batch, default 1
//                                     // (fully sequential, today's behavior). Clamped to [1, 6] - see
//                                     // SKILL.md "Parallel batches". No lock needed here (unlike
//                                     // perf-optimize) - a bug either reproduces or doesn't, regardless
//                                     // of CPU contention from a sibling candidate.
//   }
// At least one of budgetMinutes/targetCandidates must be set. If both are set, whichever is hit
// first stops the loop.

export const meta = {
  name: 'bug-hunt-autonomous',
  description: 'Consume the IDEAS.md backlog, discover new suspects when it is empty, dig into one or more bug candidates at once (see maxParallel) within a real wall-clock and/or count budget, never pause for confirmation',
  phases: [
    { title: 'Discovery' },
    { title: 'Candidate' },
  ],
}

const { projectPath, optDir, budgetMinutes, targetCandidates, focusHint, diskSpaceNote } = args
const maxParallel = Math.max(1, Math.min(6, args.maxParallel || 1))
const skillDir = '~/.claude/skills/bug-hunt'
// A fixed "list 1-3 suspects" ask per finder starved batches once maxParallel > ~3: 3-4 finders
// each capped at 1-3, then deduplicated, rarely cleared a handful of candidates - so a requested
// maxParallel of 6 mostly ran batches of 2-4 in practice (observed on a real 3h/maxParallel=6
// vello session, 2026-09-01: 7 of 8 batches undershot 6, and ~30% of the session's wall-clock went
// into repeated undersized Discovery rounds instead of candidate work). Scale the ask with
// maxParallel so the combined pre-dedup pool comfortably exceeds it.
const suspectsPerFinderCap = Math.max(3, maxParallel * 2)
const diskNote = diskSpaceNote ? ` Disk space constraint this session: ${diskSpaceNote} - check ` +
  `available space (df -h) before building; see SKILL.md "Watching disk space during a long ` +
  `session" if it gets tight.` : ''

// The only way this script sees real time - see the top-of-file note. Cheap: just a shell call.
async function nowEpoch(label) {
  const r = await agent(
    'Run `date +%s` in bash and return exactly its numeric stdout as epochSeconds - nothing else.',
    { phase: 'Discovery', label: label || 'clock-check',
      schema: { type: 'object', properties: { epochSeconds: { type: 'number' } }, required: ['epochSeconds'] } }
  )
  return r.epochSeconds
}

// No history yet this session - guess conservatively (high) rather than optimistically. Bug
// candidates vary even more than perf ones: a dead-end suspect can resolve in a few minutes, a
// confirmed UB finding needing a careful fix and re-verification can take much longer. When
// maxParallel > 1 this estimates a whole BATCH's duration (its slowest candidate), not one
// candidate's - see "Parallel batches" in SKILL.md.
const DEFAULT_FIRST_BATCH_ESTIMATE_MIN = 35
const SAFETY_FACTOR = 0.75

let startEpoch = null
let deadlineEpoch = null
if (budgetMinutes) {
  startEpoch = await nowEpoch('session-start-clock')
  deadlineEpoch = startEpoch + budgetMinutes * 60
  log(`time budget: ${budgetMinutes} min`)
}
if (maxParallel > 1) log(`parallel batches enabled: up to ${maxParallel} candidates at once`)

let done = 0
let dryRounds = 0
let consecutiveFailures = 0
let issuesSyncedThisSession = false
const batchDurationsMin = []

while (true) {
  if (targetCandidates && done >= targetCandidates) {
    log(`stopping: reached the ${targetCandidates}-candidate count budget`)
    break
  }
  if (dryRounds >= 2) {
    log('stopping: backlog and discovery both came up dry twice in a row')
    break
  }
  if (deadlineEpoch) {
    const t = await nowEpoch('deadline-check')
    if (t >= deadlineEpoch) {
      log(`stopping: time budget elapsed (~${Math.round((t - startEpoch) / 60)} min of ` +
          `${budgetMinutes} planned) - not starting a new discovery round`)
      break
    }
  }

  phase('Discovery')

  const wantedThisBatch = targetCandidates ? Math.min(maxParallel, targetCandidates - done) : maxParallel

  // Claiming (picking backlog entries AND assigning their NNN numbers AND removing them from
  // IDEAS.md) happens in this one sequential step, before anything is dispatched in parallel below -
  // the only place backlog/numbering state is mutated, so two candidates can never claim the same
  // idea or collide on a folder number. See SKILL.md "Parallel batches" step 1.
  const state = await agent(
    `Read ${optDir}/IDEAS.md (create it from ${skillDir}/templates/IDEAS_template.md if it ` +
    `does not exist yet). Claim up to ${wantedThisBatch} of the most promising unchecked backlog ` +
    `entries, if any exist: for each one claimed, assign it the next sequential 3-digit candidate ` +
    `number (continuing whatever numbering ${optDir}/SUMMARY.md and existing NNN.* folders already ` +
    `established) and REMOVE it from IDEAS.md's backlog table immediately, in this same step. ` +
    `Return the claimed batch (empty array if the backlog was empty).`,
    { phase: 'Discovery', schema: { type: 'object', properties: {
        claimed: { type: 'array', items: { type: 'object', properties: {
          number: { type: 'string' }, candidateName: { type: 'string' }, candidateDescription: { type: 'string' },
        }, required: ['number', 'candidateName', 'candidateDescription'] } },
      }, required: ['claimed'] } }
  )

  let batch = state.claimed

  if (batch.length === 0) {
    const focus = focusHint ? ` Focus area: ${focusHint}.` : ''
    const finders = [
      `Read through ${projectPath}'s source looking for the patterns in ` +
      `${skillDir}/HEURISTICS.md.${focus} List up to ${suspectsPerFinderCap} concrete suspects ` +
      `(file:line, which pattern, why it looks suspicious) - don't stop at 1-3 just because that ` +
      `would technically satisfy the ask, this batch needs up to ${maxParallel} candidates and ` +
      `this is one of several finders feeding the same pool.`,
      `Run whatever static analysis is idiomatic for this project's language (e.g. ` +
      `\`cargo clippy\` with pedantic lints for Rust) against ${projectPath}${focus ? ' focused ' +
      'on ' + focusHint : ''} and list up to ${suspectsPerFinderCap} real, correctness-relevant ` +
      `hits (not style-only ones).`,
      `Check ${skillDir}/projects/ for a file matching this project's name; if one exists, read ` +
      `it and list any "next suspect" leads it records that are not already listed in ` +
      `${optDir}/SUMMARY.md (already investigated) or ${optDir}/IDEAS.md's current backlog ` +
      `(already queued). Also check whether ${projectPath}/___ISSUES___/bugs/ exists (read-only - ` +
      `do not run any other skill) and list any issue there already confirmed live in the code ` +
      `but with no working reproduction yet.`,
    ]
    // GitHub issue/PR inspiration - at most once per session (see SKILL.md), so only added to the
    // finder list the first time Discovery actually runs the parallel fan-out, never again after.
    if (!issuesSyncedThisSession) {
      issuesSyncedThisSession = true
      finders.push(
        `Read the watermark line at the top of ${optDir}/IDEAS.md ("Last GitHub issue/PR checked ` +
        `for inspiration: #N (repo: R)", "#0 (repo: none yet)" if missing/never done). Then run ` +
        `(global flags before the subcommand): \`python3 ~/.claude/skills/gh-issues-check/` +
        `fetch_issues.py --dir ${optDir}/__issues_raw raw-sync --since <N>\` - unless R is "none ` +
        `yet", first confirm R still matches this project's own repo before trusting <N>; if it ` +
        `doesn't (remote changed), use --since 0 instead, since <N> was recorded against a ` +
        `different repo's own numbering. This calls gh-issues-check's own fetch script directly ` +
        `as a utility, NOT the gh-issues-check skill itself, and writes only raw open-issues/` +
        `open-PRs JSON, no closed/merged history (no index, no categories, effectively no limit - ` +
        `a 2000/endpoint safety cap only, meant to fetch everything currently open). Best-effort: ` +
        `if it fails (no network/gh auth), report zero ` +
        `candidates and move on, that's not an error. Otherwise read through whatever it just ` +
        `wrote under __issues_raw/issues/ and __issues_raw/pulls/ once, keep only entries relevant to ` +
        `bug-hunting on this project (a still-open crash/correctness/UB report, or an open, ` +
        `not-yet-merged PR attempting to fix one - worth checking whether a sibling code path has ` +
        `the same bug it doesn't touch)${focus} - skip anything else without forcing a ` +
        `connection. Separately (not as a "candidate"), update ${optDir}/IDEAS.md's watermark ` +
        `line to "#<MAX_NUMBER_SEEN> (repo: <REPO>)" using both values the command printed, even ` +
        `if nothing relevant was found.`
      )
    }
    const findingsSchema = { type: 'object', properties: {
      candidates: { type: 'array', items: { type: 'object', properties: {
        name: { type: 'string' }, description: { type: 'string' }, source: { type: 'string' },
      }, required: ['name', 'description'] } },
    }, required: ['candidates'] }

    const findings = (await parallel(finders.map(f => () =>
      agent(f, { phase: 'Discovery', schema: findingsSchema })
    ))).filter(Boolean).flatMap(r => r.candidates)

    if (findings.length === 0) {
      dryRounds++
      log(`discovery round dry (${dryRounds}/2)`)
      continue
    }

    const picked = await agent(
      `Append these suspect ideas to ${optDir}/IDEAS.md's backlog table (create the file from ` +
      `${skillDir}/templates/IDEAS_template.md if missing), deduplicating against what's already ` +
      `there: ${JSON.stringify(findings)}. Then claim up to ${wantedThisBatch} of the most ` +
      `promising ones as this batch's candidates: for each, assign it the next sequential 3-digit ` +
      `candidate number and REMOVE it from the backlog table immediately, same as above. Return ` +
      `the claimed batch.`,
      { phase: 'Discovery', schema: { type: 'object', properties: {
          claimed: { type: 'array', items: { type: 'object', properties: {
            number: { type: 'string' }, candidateName: { type: 'string' }, candidateDescription: { type: 'string' },
          }, required: ['number', 'candidateName', 'candidateDescription'] } },
        }, required: ['claimed'] } }
    )
    batch = picked.claimed
  }

  if (batch.length === 0) continue
  dryRounds = 0

  // Safety-buffer gate: don't commit to a batch likely to blow well past the deadline. Uses this
  // session's OWN observed BATCH durations once there's history (a batch's cost is its slowest
  // candidate, not their sum - see the estimate below). Unlike the pre-parallel version, a batch
  // that's claimed but not started here MUST be written back to IDEAS.md before stopping, since
  // claiming already removed it from the backlog (the sequential version could just leave an
  // unclaimed idea untouched).
  if (deadlineEpoch) {
    const t = await nowEpoch('pre-batch-clock')
    const remainingMin = (deadlineEpoch - t) / 60
    const estimate = batchDurationsMin.length
      ? batchDurationsMin.reduce((a, b) => a + b, 0) / batchDurationsMin.length
      : DEFAULT_FIRST_BATCH_ESTIMATE_MIN
    if (remainingMin < estimate * SAFETY_FACTOR) {
      log(`stopping: ~${Math.round(remainingMin)} min left, this session's batches have averaged ` +
          `~${Math.round(estimate)} min - not enough headroom to safely start a new batch of ` +
          `${batch.length}.`)
      await agent(
        `Write these claimed-but-not-started candidates back into ${optDir}/IDEAS.md's backlog ` +
        `table, each with its already-assigned number noted so it isn't reassigned: ` +
        `${JSON.stringify(batch)}`,
        { phase: 'Discovery' }
      )
      break
    }
  }

  phase('Candidate')
  const elapsedTag = deadlineEpoch
    ? ` (elapsed ~${Math.round(((await nowEpoch('progress-clock')) - startEpoch) / 60)}/${budgetMinutes} min)`
    : ''
  log(`batch of ${batch.length}${targetCandidates ? ` (done ${done}/${targetCandidates})` : ''}` +
      `${elapsedTag}: ${batch.map(c => c.number + '.' + c.candidateName).join(', ')}`)

  const batchStart = deadlineEpoch ? await nowEpoch('batch-start-clock') : null

  // Each candidate in the batch runs as its own agent() call, dispatched together via parallel() -
  // each works inside its OWN git worktree (isolation: "worktree"; never the main project's shared
  // tree - see SKILL.md "Parallel batches") so concurrent candidates can never collide on
  // apply/build/revert. A candidate never touches SUMMARY.md/IDEAS.md itself; it only returns a
  // structured result, transcribed sequentially once the whole batch resolves (below) - the one
  // place this avoids the exact race that made per-candidate direct writes unsafe under
  // parallelism. With maxParallel === 1 this degenerates to a single-element batch - same code
  // path, no special-casing needed.
  const batchResults = await parallel(batch.map(c => () => agent(
    `Follow ${skillDir}/SKILL.md's "Candidate loop" steps 1-9 (NOT step 10 - do not touch ` +
    `SUMMARY.md/SUMMARY-SHORT.md/IDEAS.md yourself; this candidate is part of a batch and a later, ` +
    `separate step transcribes results after the whole batch finishes), for this candidate, in ` +
    `project ${projectPath} (workspace ${optDir}). Work inside your own isolated git worktree ` +
    `(isolation: "worktree") - never the main project's own working tree directly.\n` +
    `Candidate ${c.number}: ${c.candidateName} - ${c.candidateDescription}\n` +
    `Full cycle: dig with up to 3 escalating tools/approaches per HEURISTICS.md's "Tool ` +
    `selection" guidance (debug-assertions build, miri, ASAN/UBSAN, a short one-off fuzz harness, ` +
    `or manual boundary-value construction, whichever fits the suspected bug class), keep digging ` +
    `past the soft cap only if a real signal appeared. If nothing reproduces, mark Not ` +
    `reproducible with exactly what was tried. If it reproduces, build and actually run a ` +
    `standalone repro/ (own Cargo.toml, bare [workspace], public API where possible) that exercises ` +
    `the REAL end-to-end trigger path per CONFIRMATION.md section 11 - if the finding is about how ` +
    `one participant (a binary, a caller) reacts to input actually produced by another (a peer, an ` +
    `upstream caller, the wire), the repro must run the real affected entry point driven by a real ` +
    `second participant genuinely producing that input, NOT hand-authored bytes/state standing in ` +
    `for what that participant would send - a repro that fakes a second participant does not get ` +
    `written up as Confirmed, except CONFIRMATION.md section 11's rare, explicitly justified ` +
    `exception. Apply CONFIRMATION.md's repeatability bar before writing Confirmed, root-cause it, ` +
    `best-effort identify the introducing commit (git log -S / git blame first, git bisect only if ` +
    `those don't pin it down and a cheap go/no-go check exists; plain text not backticks, or "not ` +
    `determined" with the actual reason). Attempt a bounded fix directly on this worktree's own ` +
    `copy of the source, verify by re-running repro/ against the patched tree AND the project's ` +
    `existing test suite before and after (mandatory, state actual pass/fail counts or explicit ` +
    `N/A). Write the numbered candidate folder at ${optDir}/${c.number}.<NAME>/ exactly as the ` +
    `candidate loop's step 9 describes (RESULT.md - facts only, no severity label; patch.diff only ` +
    `if a verified fix was found; repro/ mandatory whenever Confirmed; output.txt with real ` +
    `captured output; a self-contained justfile from the template with repo_url/base_commit filled ` +
    `in). Then end your final report with a fenced block, one field per line, EXACTLY these keys ` +
    `so it can be transcribed verbatim without re-deriving anything: Number, CandidateFolderName, ` +
    `Status (Confirmed|Not reproducible|Inconclusive), Class (if Confirmed), FullyReproducible ` +
    `(Yes|No+justification, if Confirmed), IntroducedBy (short hash + subject, or "not determined", ` +
    `if Confirmed), HasPatch (yes|no), FixLines (+A/-D or n/a), RootCauseOneLiner (if Confirmed), ` +
    `TestsBeforeAfter (pass/fail counts or N/A), NewSuspectsNoticed (semicolon-separated, or none).` +
    `${diskNote}`,
    { phase: 'Candidate', effort: 'high' }
  )))

  // Transcribe sequentially - the only step in this batch that writes to SUMMARY.md/
  // SUMMARY-SHORT.md/IDEAS.md, so concurrent candidate writes never race each other.
  const succeeded = []
  const failedInBatch = []
  batchResults.forEach((r, i) => {
    if (r === null) failedInBatch.push(batch[i]); else succeeded.push({ claim: batch[i], report: r })
  })

  if (succeeded.length > 0) {
    await agent(
      `For each of these completed candidates, append one row to ${optDir}/SUMMARY.md (and, if ` +
      `Status is Confirmed, a condensed row to ${optDir}/SUMMARY-SHORT.md) using the fields it ` +
      `reported in its final report's fenced block, following SUMMARY_template.md / ` +
      `SUMMARY_SHORT_template.md's own table hygiene rules (one row per physical line, no blank ` +
      `line before/between rows, escape a literal | as \\|). Do NOT touch SUMMARY-SHORT-CRASHES.md/ ` +
      `SUMMARY-SHORT-LOGIC.md/SUMMARY-CRASHES.md/SUMMARY-LOGIC.md - those are generated by ` +
      `\`just gen-summaries\`/\`just pack*\` from the two files above, never hand-written. Also ` +
      `remove each candidate's already-claimed entry from IDEAS.md if any trace of it remains there ` +
      `(it shouldn't, claiming already removed it, but confirm). Results: ${JSON.stringify(succeeded)}`,
      { phase: 'Candidate' }
    )
  }
  if (failedInBatch.length > 0) {
    log(`${failedInBatch.length} candidate(s) in this batch did not complete: ` +
        `${failedInBatch.map(c => c.number + '.' + c.candidateName).join(', ')} - returning them to IDEAS.md`)
    await agent(
      `These candidates were claimed (already removed from the backlog, with numbers already ` +
      `assigned) but their own Agent call did not complete - write them back into ` +
      `${optDir}/IDEAS.md's backlog with a resume note, keeping their already-assigned numbers so ` +
      `they aren't reassigned later; if a worktree survives for one of them, inspect it once for a ` +
      `usable partial repro before it gets discarded. Candidates: ${JSON.stringify(failedInBatch)}`,
      { phase: 'Candidate' }
    )
    consecutiveFailures++
    if (consecutiveFailures >= 2) {
      log('stopping: 2 batches in a row had a failing candidate - likely an environment problem ' +
          '(build/tooling broken) rather than bad luck; needs a human look before continuing.')
      break
    }
  } else {
    consecutiveFailures = 0
  }

  // Clean up this batch's worktrees now that everything needed has been pulled into each
  // candidate's own NNN.NAME/ folder - fresh worktrees per batch, no persistent pool (see SKILL.md
  // "Parallel batches" step 4).
  if (maxParallel > 1) {
    await agent(
      `Remove every git worktree created for this batch's candidates (git worktree remove) now ` +
      `that their patch.diff/repro/output.txt (where applicable) have been saved into ` +
      `${optDir}/NNN.NAME/ folders. Candidate numbers: ${JSON.stringify(batch.map(c => c.number))}`,
      { phase: 'Candidate' }
    )
  }

  if (deadlineEpoch) {
    const batchEnd = await nowEpoch('batch-end-clock')
    batchDurationsMin.push((batchEnd - batchStart) / 60)
  }

  done += succeeded.length
}

let reason
if (targetCandidates && done >= targetCandidates) reason = 'candidate-count budget reached'
else if (consecutiveFailures >= 2) reason = 'two batches in a row had a failing candidate - needs a human look (check for dirty leftover worktrees / broken build)'
else if (dryRounds >= 2) reason = 'backlog and discovery both came up dry twice in a row'
else if (deadlineEpoch) reason = 'time budget elapsed or too little headroom left to safely start another batch'
else reason = 'stopped'

const totalElapsedMin = deadlineEpoch ? Math.round(((await nowEpoch('final-clock')) - startEpoch) / 60) : null
log(`stopped after ${done} candidate(s) - ${reason}` +
    (totalElapsedMin !== null ? ` (~${totalElapsedMin} min elapsed of ${budgetMinutes} planned)` : '') +
    (maxParallel > 1 ? ` [maxParallel=${maxParallel}]` : ''))
return {
  candidatesCompleted: done,
  stoppedReason: reason,
  elapsedMinutes: totalElapsedMin,
  budgetMinutes: budgetMinutes || null,
  maxParallel,
}
