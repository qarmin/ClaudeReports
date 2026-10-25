// Follow-up workflow, round 2: redesign both generators (random + static) for (a) throughput via
// batching many sub-cases per compiled file, and (b) deliberate "noise injection" - unrelated
// filler code/tautological asserts placed near the real test logic, to perturb size/inlining-
// sensitive LLVM/MIR optimization heuristics the way a real-world miscompile the user recalled
// needed (an unrelated-looking neighbor triggering a different optimization decision). Then run
// an expanded campaign with the upgraded generators. Same hard rules as round 1: never edit/build
// rust-lang/rust itself, never attempt a fix, never write to GitHub.
//
// args: { projectPath, optDir, fuzzerDir, budgetMinutes, buildMinutes, skillDir }

export const meta = {
  name: 'rustc-miscompile-fuzzer-v2',
  description: 'Add batched multi-case files + noise injection to both generators for throughput and heuristic-threshold coverage, then run an expanded campaign',
  phases: [
    { title: 'Build' },
    { title: 'Campaign' },
  ],
}

const { projectPath, optDir, fuzzerDir, budgetMinutes, buildMinutes, skillDir } = args

const NEVER_EDIT = `HARD RULES, no exceptions: (1) NEVER edit, create, or delete any file inside ` +
  `${projectPath} except under ${optDir}/ - never run ./x build, ./x test, ./x fmt, or cargo ` +
  `inside the rust-lang/rust checkout itself, and never build rustc from source. (2) NEVER attempt ` +
  `a fix/patch for anything found - "not attempted, out of scope for this session" every time. ` +
  `(3) NEVER write to GitHub - gh issue view/--comments read-only is fine, nothing else. ` +
  `(4) Build/run generated or repro programs only under ${optDir}/ or a /tmp scratch dir.`

async function nowEpoch(label) {
  const r = await agent(
    'Run `date +%s` in bash and return exactly its numeric stdout as epochSeconds - nothing else.',
    { phase: 'Build', label: label || 'clock-check',
      schema: { type: 'object', properties: { epochSeconds: { type: 'number' } }, required: ['epochSeconds'] } }
  )
  return r.epochSeconds
}

const startEpoch = await nowEpoch('session-start-clock')
const buildDeadlineEpoch = startEpoch + buildMinutes * 60
const overallDeadlineEpoch = startEpoch + budgetMinutes * 60
log(`round 2: ${budgetMinutes} min total, build <=${buildMinutes} min, campaign gets the rest ` +
    `(~${Math.round((overallDeadlineEpoch - buildDeadlineEpoch) / 60)} min planned)`)

// --- Build: batching + noise injection ---------------------------------------------------------
phase('Build')

const buildReport = await agent(
  `${NEVER_EDIT}\n\n` +
  `Upgrade the existing differential-fuzzer tooling at ${fuzzerDir}/ (read its README.md and the ` +
  `current templates/, templates_static/, generator.py, static_generator.py, diff_runner.py first - ` +
  `do not rebuild from scratch, extend what's there). Two concrete improvements, both requested by ` +
  `the user:\n\n` +
  `1. BATCHING FOR THROUGHPUT. Right now each compiled .rs file tests exactly one scenario, so most ` +
  `wall-clock time goes to rustc process-startup overhead rather than actual compilation work. Add ` +
  `a batched mode to BOTH generator.py and static_generator.py: pack many independent sub-cases ` +
  `(drawn the same way as today - same templates, same per-category parameter generation) into ONE ` +
  `.rs file, each as its own function with a unique name, all called in sequence from main(), each ` +
  `printing its own deterministic result line (e.g. "case_<n>:<value>"). The file's .expected ` +
  `sidecar is the concatenation of every sub-case's independently-predicted expected line, in the ` +
  `same order. This must not change what's being tested, only how many scenarios share one rustc ` +
  `invocation - add a --cases-per-file N flag (e.g. default ~50-100, tunable) to both generators, ` +
  `default to today's 1-case-per-file when N=1 so nothing existing breaks. If a mismatch happens in ` +
  `a batched file, diff_runner.py's existing stdout/.expected comparison will still flag the whole ` +
  `file as a divergence - make sure its REPORT.md (or your own quick addition to diff_runner.py) ` +
  `makes it easy to tell WHICH line/sub-case actually differs (a plain line-by-line diff of the two ` +
  `stdout captures is enough, doesn't need to be fancy). Validate the throughput win concretely: ` +
  `time a batch of, say, 5000 total sub-cases at cases-per-file=1 vs cases-per-file=50-100 and ` +
  `report the real wall-clock difference.\n\n` +
  `2. NOISE INJECTION. The user specifically recalled a real rustc miscompile class that only ` +
  `triggered when unrelated-looking code sat near the actual trigger - this matches how several ` +
  `LLVM/MIR optimization passes gate on function size, instruction count, or inlining-budget ` +
  `heuristics, so padding a function's neighborhood with inert filler can flip which optimization ` +
  `path gets taken. Add a shared noise-generation helper (one module reused by both generator.py's ` +
  `templates/ and static_generator.py's templates_static/, e.g. templates/noise.py) that can inject, ` +
  `around/between the real test logic in a generated function:\n` +
  `   - extra unrelated helper functions with plausible-looking arithmetic/control-flow, called but ` +
  `with their result passed through std::hint::black_box and never read by the real test logic\n` +
  `   - extra unrelated struct/enum type definitions\n` +
  `   - tautological assert!()/debug_assert!() calls interspersed with the real logic (conditions ` +
  `that are provably always true by construction, e.g. involving wrapping arithmetic identities - ` +
  `vary their shape, don't reuse one condition everywhere)\n` +
  `   - dead-but-black-boxed computations\n` +
  `   The CRITICAL invariant, stated explicitly in the code and the README: noise must be PROVABLY ` +
  `inert - it must never read or write any variable the real test logic reads, must be fully ` +
  `deterministic (no RNG evaluated at program runtime, only at generation time), and its own result ` +
  `must never be printed/compared (black_box it and discard) - so any divergence found is still ` +
  `attributable to the real tested pattern, never to the noise itself. Add a --noise-level ` +
  `{0,1,2} flag to both generators (0 = none/today's behavior, 1 = light, 2 = heavy - more/larger ` +
  `unrelated functions, deliberately aiming to cross typical inlining-budget thresholds) to both ` +
  `generators, and combine it with batching (noise applies per sub-case within a batched file too).` +
  `\n\n` +
  `3. VALIDATE before moving on: run ${fuzzerDir}/check_canaries.sh first (must still PASS - if ` +
  `your changes to diff_runner.py broke the detector, fix that before anything else). Then generate ` +
  `a moderate baseline batch at a representative --cases-per-file and --noise-level=2 and confirm ` +
  `zero self-check mismatches (generator-oracle bugs introduced by your own changes) - debug and fix ` +
  `until clean, this is not optional.\n\n` +
  `4. Update ${fuzzerDir}/README.md: document both new flags, the noise-inertness invariant, and the ` +
  `measured throughput improvement from step 1.\n\n` +
  `Report: the measured throughput win (cases/sec before vs after batching), noise-level design ` +
  `summary, and the baseline validation result.`,
  { phase: 'Build', label: 'build-v2-batching-noise', effort: 'high' }
)
log(`build v2 done: ${JSON.stringify(buildReport).slice(0, 300)}`)

// --- Campaign: run the upgraded generators -------------------------------------------------
phase('Campaign')
let dryRounds = 0
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

  const isLateRound = remainingMin < roundBudgetMin * 1.5 // near the very end of the budget
  const round = await agent(
    `${NEVER_EDIT}\n\n` +
    `Run one campaign round at ${fuzzerDir}/ for roughly ${Math.round(roundBudgetMin)} minutes of ` +
    `real wall-clock work using the UPGRADED generators (batched + noise-injected - use ` +
    `run_round.sh if it still fits, or invoke generator.py/static_generator.py directly with ` +
    `--cases-per-file and --noise-level if run_round.sh hasn't been extended with those flags yet, ` +
    `in which case extend it so future rounds don't have to). Prefer a high --cases-per-file and ` +
    `--noise-level=2 to maximize both throughput and heuristic-threshold coverage, across as many ` +
    `of the ~17+4 template categories as fit in the time. Append a line to ROUNDS.log same as ` +
    `before.${isLateRound ? ' This is one of the last rounds this session has budget for - if ' +
    'stable-toolchain coverage already feels broad across categories/noise levels, consider using ' +
    '--rustc "rustc +nightly" for this one instead, per the user\'s stated priority (breadth on ' +
    'current examples first, nightly as a later addition once that feels solid).' : ''} Before ` +
    `reporting a divergence, re-verify it (re-run both configs again) and re-read the specific ` +
    `generated program (including any injected noise) for pre-existing UB or a noise-inertness ` +
    `violation you might have introduced - only report ones you're confident are real. Report ` +
    `cumulative programs/sub-cases tested so far (distinguish compiled-files vs sub-cases now that ` +
    `batching exists), and whether this round feels dry.`,
    { phase: 'Campaign', label: `campaign-round-${roundNum}`, effort: 'high',
      schema: { type: 'object', properties: {
        divergencesFound: { type: 'array', items: { type: 'object', properties: {
          description: { type: 'string' }, repro_path: { type: 'string' },
          config_a_cmd: { type: 'string' }, config_a_output: { type: 'string' },
          config_b_cmd: { type: 'string' }, config_b_output: { type: 'string' },
        }, required: ['description', 'repro_path', 'config_a_cmd', 'config_a_output', 'config_b_cmd', 'config_b_output'] } },
        cumulativeFilesCompiled: { type: 'number' },
        cumulativeSubCasesTested: { type: 'number' },
        feelsDry: { type: 'boolean' },
      }, required: ['divergencesFound', 'cumulativeFilesCompiled', 'cumulativeSubCasesTested', 'feelsDry'] } }
  )

  if (round === null) {
    log(`campaign round ${roundNum} failed to complete (possible [cyber] safeguard false positive) - continuing`)
    continue
  }

  if (round.divergencesFound.length === 0) {
    dryRounds++
    log(`round ${roundNum}: no divergences (dry streak ${dryRounds}), ` +
        `${round.cumulativeFilesCompiled} files / ${round.cumulativeSubCasesTested} sub-cases cumulatively`)
  } else {
    dryRounds = 0
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
        `The differential-fuzzer harness at ${fuzzerDir}/ (v2: batched + noise-injected) reported ` +
        `this possible rustc miscompilation - triage it as bug-hunt candidate ${claim.number} in ` +
        `workspace ${optDir} (ground rules: no patch.diff, no fix, see ${optDir}/README.md). ` +
        `Independently re-run it at least 3x per config, scrutinize the specific generated program ` +
        `(real logic AND any injected noise) for pre-existing UB or a noise-inertness bug, and if ` +
        `the finding came from a BATCHED file, first minimize it down to just the one diverging ` +
        `sub-case (strip the other sub-cases and unrelated noise out one at a time) so the final ` +
        `repro/ is a single clean trigger, not a 1000-line haystack - note in RESULT.md whether ` +
        `noise or batching position turned out to matter (that itself would be a notable result). ` +
        `If confirmed real: write ${optDir}/${claim.number}.<SHORT_NAME>/ (RESULT.md from ` +
        `${skillDir}/templates/RESULT_template.md - "The fix" says "not attempted, out of scope"; ` +
        `repro/; output.txt; justfile from ${optDir}/candidate_justfile_template), best-effort ` +
        `bisect via cargo-bisect-rustc, append rows to ${optDir}/SUMMARY.md and SUMMARY-SHORT.md ` +
        `yourself (Class "miscompile"). If it does NOT hold up, explain what actually caused it and ` +
        `fix ${fuzzerDir}/ if it points at a harness/noise-inertness bug, but do NOT create a ` +
        `candidate folder.\n\nDivergence report: ${JSON.stringify(d)}`,
        { phase: 'Campaign', label: `triage-${claim.number}`, effort: 'high' }
      )
      if (triage !== null) newFindingsCount++
    }
  }
}

const finalT = await nowEpoch('final-clock')
const totalElapsedMin = Math.round((finalT - startEpoch) / 60)
log(`round 2 done: ${newFindingsCount} new confirmed finding(s) from ${roundNum} campaign round(s), ` +
    `~${totalElapsedMin}/${budgetMinutes} min elapsed`)

return {
  newFindingsCount,
  campaignRounds: roundNum,
  elapsedMinutes: totalElapsedMin,
  budgetMinutes,
}
