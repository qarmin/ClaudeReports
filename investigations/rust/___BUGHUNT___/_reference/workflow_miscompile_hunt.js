// Custom workflow for this session - NOT the stock templates/workflow_autonomous.js.
//
// Goal, per the user's explicit correction mid-session: the 33 open I-miscompile GitHub issues
// are PATTERN-MINING INPUT ONLY, not candidates - re-confirming one is not a "finding" and never
// gets a SUMMARY.md row. The actual deliverable is miscompile-fuzzer/: a reusable, parameterized,
// deliberately UB-free Rust-program generator + differential-testing harness, built as real code,
// that this session both builds AND runs for as much of the budget as possible to find genuinely
// NEW miscompilations. Never edits/builds rust-lang/rust itself, never attempts a fix (no
// reviewer named - AGENTS.md Reviewer gate).
//
// args: { projectPath, optDir, budgetMinutes, reconMinutes, buildMinutes, skillDir }
// (reconMinutes + buildMinutes are reserved off the front of budgetMinutes; everything left over
// goes to the campaign phase that runs the harness and triages findings.)

export const meta = {
  name: 'rustc-miscompile-fuzzer-hunt',
  description: 'Quick recon over known rustc I-miscompile issues (pattern-mining only), then build + run a reusable UB-free differential-testing generator to find NEW miscompilations',
  phases: [
    { title: 'Recon' },
    { title: 'Build' },
    { title: 'Campaign' },
  ],
}

const { projectPath, optDir, budgetMinutes, reconMinutes, buildMinutes, skillDir } = args
const fuzzerDir = `${optDir}/miscompile-fuzzer`

const NEVER_EDIT = `HARD RULES, no exceptions: (1) NEVER edit, create, or delete any file inside ` +
  `${projectPath} except under ${optDir}/ - never run ./x build, ./x test, ./x fmt, or cargo ` +
  `inside the rust-lang/rust checkout itself, and never build rustc from source (entirely ` +
  `unnecessary - every test uses a prebuilt toolchain via rustup). (2) NEVER attempt a fix/patch ` +
  `for anything found - "not attempted, out of scope for this session" every time, no exceptions. ` +
  `(3) NEVER write to GitHub - gh issue view/--comments is read-only and fine, gh issue comment / ` +
  `gh issue close / gh pr create / any write operation is forbidden. (4) Build/run generated or ` +
  `repro programs only under ${optDir}/ or a /tmp scratch dir, never inside ${projectPath} itself.`

async function nowEpoch(label) {
  const r = await agent(
    'Run `date +%s` in bash and return exactly its numeric stdout as epochSeconds - nothing else.',
    { phase: 'Recon', label: label || 'clock-check',
      schema: { type: 'object', properties: { epochSeconds: { type: 'number' } }, required: ['epochSeconds'] } }
  )
  return r.epochSeconds
}

const startEpoch = await nowEpoch('session-start-clock')
const reconDeadlineEpoch = startEpoch + reconMinutes * 60
const buildDeadlineEpoch = reconDeadlineEpoch + buildMinutes * 60
const overallDeadlineEpoch = startEpoch + budgetMinutes * 60
log(`time budget: ${budgetMinutes} min total - recon <=${reconMinutes} min, build <=${buildMinutes} min, ` +
    `campaign gets the rest (~${Math.round((overallDeadlineEpoch - buildDeadlineEpoch) / 60)} min planned)`)

// --- Phase 1: Recon (pattern-mining only, NOT candidates) ------------------------------------
phase('Recon')

const ISSUES = [
  61275, 100914, 107975, 118223, 153222, 159618, 160004, 161898, 163777, 163782, 163856,
  149294, 155952, 161973, 162078, 163311,
  98132, 127336, 132898, 144371, 147538, 149429, 156008, 159268, 160267, 160722, 163734, 163737, 163825, 163911,
  130388, 162214, 163541,
]
// Split into 3 roughly-even slices for parallel recon - a barrier is fine here, we want every
// slice's patterns collected before designing the generator in the Build phase.
const SLICES = [[], [], []]
ISSUES.forEach((n, i) => SLICES[i % 3].push(n))

const RECON_SCHEMA = { type: 'object', properties: {
  results: { type: 'array', items: { type: 'object', properties: {
    issue: { type: 'number' },
    stillReproduces: { type: 'string', enum: ['Yes', 'No', 'Not checked'] },
    note: { type: 'string' },
  }, required: ['issue', 'stillReproduces', 'note'] } },
}, required: ['results'] }

const reconResults = (await parallel(SLICES.map((slice, i) => () => agent(
  `${NEVER_EDIT}\n\n` +
  `Quick recon pass ONLY - NOT a full bug-hunt investigation. For each of these rust-lang/rust ` +
  `issue numbers: ${slice.join(', ')} - run \`gh issue view <N> --repo rust-lang/rust --comments\` ` +
  `(read-only), extract the minimal Rust repro, and spend at most ~3-4 minutes per issue trying it ` +
  `with the current \`nightly\` toolchain (install via rustup if needed) using the flags/opt-level ` +
  `the issue mentions. Just answer: does it still reproduce the wrong behavior today (Yes/No), and ` +
  `a one-line note on which MIR/LLVM pass or language feature it exercises (for designing a ` +
  `differential-fuzzer generator later - e.g. "GVN + union with reference field", "pointer ` +
  `provenance round-trip via expose_provenance", "EarlyOtherwiseBranch on a match tree"). Do NOT ` +
  `write any repro/ folder, RESULT.md, or SUMMARY.md row for these - that is explicitly out of ` +
  `scope, this is pattern-mining input only. If an issue needs unusual setup not worth the time ` +
  `(a rare -Z nightly feature, cross-compilation), mark "Not checked" with why instead of spending ` +
  `a long time on it.`,
  { phase: 'Recon', label: `recon-slice-${i}`, schema: RECON_SCHEMA, effort: 'medium' }
)))).filter(Boolean)

const allRecon = reconResults.flatMap(r => r.results)
log(`recon done: ${allRecon.length}/${ISSUES.length} issues checked`)

await agent(
  `Update the "Still reproduces?" column (and refine the "Pattern (for generator)" column if the ` +
  `recon note sharpens it) in ${optDir}/_reference/known_miscompile_issues.md's table, for each of ` +
  `these issues, using this data (do not touch any other row, do not reformat the table beyond ` +
  `these cell updates): ${JSON.stringify(allRecon)}`,
  { phase: 'Recon' }
)

// --- Phase 2: Build the generator + differential-testing harness ------------------------------
phase('Build')
log('building the differential-testing fuzzer harness')

const buildReport = await agent(
  `${NEVER_EDIT}\n\n` +
  `Build the main deliverable of this session: ${fuzzerDir}/, a reusable, parameterized, ` +
  `deliberately UB-free Rust-program generator + differential-testing harness for finding NEW ` +
  `rustc miscompilations. Read ${optDir}/README.md and ${optDir}/_reference/known_miscompile_issues.md ` +
  `first (the latter's "Pattern (for generator)" column, now filled in from recon, tells you which ` +
  `MIR/LLVM passes to weight the generator toward - GVN/ReferencePropagation/unions/packed structs ` +
  `and pointer-provenance round-trips were the two biggest clusters in the open-issue list as of ` +
  `2026-10-07, but don't limit yourself to only those).\n\n` +
  `Build this as REAL, reusable code (Python and/or a shell driver - whatever's simplest to get ` +
  `right), not one-off ad-hoc snippets re-authored each round:\n` +
  `1. A GENERATOR that produces many small, self-contained, parameterized Rust programs into a ` +
  `corpus/ dir. "Parameterized" means real combinatorial/seeded variation (field type choices, ` +
  `struct/union layouts, operation sequences, branch structures) driven by a deterministic RNG ` +
  `seeded by an index - not just asking yourself to improvise N examples by hand each run. Cover ` +
  `AS BROAD a set of template categories as you reasonably can in the time available, e.g.: unions/ ` +
  `packed structs with reference or pointer fields read across branches (GVN/ReferencePropagation- ` +
  `sensitive), reference reborrow chains through nested field projections, pointer provenance ` +
  `round-trips (expose_provenance/with_exposed_provenance/ptr casts, pointer equality after ` +
  `optimization), match/if-trees with shared conditions and aliasing locals (EarlyOtherwiseBranch/ ` +
  `JumpThreading-sensitive), enum niche-layout patterns (Option<T> and custom niche types), static ` +
  `merging/const-eval of packed structs, checked/wrapping arithmetic interacting with inlining and ` +
  `overflow-checks. EVERY generated program must be provably UB-free by construction per Rust's ` +
  `actual language guarantees (no uninitialized reads, no data races, no out-of-bounds, no ` +
  `reliance on unspecified-but-not-UB ordering, no printing raw addresses/anything ASLR- or ` +
  `allocator-dependent) - this matters more than breadth; a generator that produces UB "finds" ` +
  `nothing but noise. Each generated program must print one or more values via println! whose ` +
  `correct value is mechanically derivable from the program's own source (so a diff is a real ` +
  `signal, not a judgment call).\n` +
  `2. A DIFF RUNNER that compiles+runs every corpus/ program under 2+ configurations that MUST ` +
  `produce byte-identical stdout+exit-code for any UB-free Rust program - e.g. same nightly at ` +
  `-Copt-level=0 vs -Copt-level=3, stable vs nightly with no unstable features, or default vs ` +
  `-Zmir-opt-level=0. Compile directly with rustc (not cargo, to keep per-program overhead low), ` +
  `and PARALLELIZE compilation/execution (xargs -P$(nproc) or similar - this host has 24 threads, ` +
  `serial compilation of a real-sized corpus wastes most of the budget). Any mismatch gets copied ` +
  `into findings/ with both outputs and the exact commands, not silently dropped.\n` +
  `3. VALIDATE it before calling it done: generate a baseline batch (~100-200 programs) across your ` +
  `template categories, run the diff pass, and confirm it reports zero (or very few, and you've ` +
  `root-caused each as a harness/template bug you then fixed, not a real compiler divergence) false ` +
  `positives. Debug the harness/templates until this baseline run is clean - an unvalidated harness ` +
  `that cries wolf on its own test programs is worse than useless for the campaign phase that comes ` +
  `next.\n` +
  `4. Write ${fuzzerDir}/README.md: what it generates, exactly how to run a campaign (the commands), ` +
  `how to extend it with a new template category, and the validation result from step 3.\n\n` +
  `Report: how many template categories you built, the baseline validation result (clean or what ` +
  `you fixed), and the exact command to run one more campaign round (generate N + diff).`,
  { phase: 'Build', label: 'build-harness', effort: 'high' }
)
log(`harness built: ${JSON.stringify(buildReport).slice(0, 300)}`)

// --- Phase 3: Campaign - run the harness, triage any real divergence -------------------------
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

  const round = await agent(
    `${NEVER_EDIT}\n\n` +
    `Run one campaign round of the differential-testing harness at ${fuzzerDir}/ for roughly ` +
    `${Math.round(roundBudgetMin)} minutes of real wall-clock work: generate a fresh batch of ` +
    `programs (reasonable size for this time budget - use the parallel diff runner, this host has ` +
    `24 threads), run the diff pass, and collect any genuine divergences. Before reporting a ` +
    `divergence, re-verify it yourself (re-run both configs at least once more) and re-read the ` +
    `specific generated program for pre-existing UB you might have missed - only report ones you're ` +
    `confident are real. Also report the cumulative corpus size tested so far across all rounds, ` +
    `and whether this round's template categories feel like they're running dry (same kinds of ` +
    `programs producing no new signal) versus still finding fresh variety.`,
    { phase: 'Campaign', label: `campaign-round-${roundNum}`, effort: 'high',
      schema: { type: 'object', properties: {
        divergencesFound: { type: 'array', items: { type: 'object', properties: {
          description: { type: 'string' }, repro_path: { type: 'string' },
          config_a_cmd: { type: 'string' }, config_a_output: { type: 'string' },
          config_b_cmd: { type: 'string' }, config_b_output: { type: 'string' },
        }, required: ['description', 'repro_path', 'config_a_cmd', 'config_a_output', 'config_b_cmd', 'config_b_output'] } },
        cumulativeProgramsTested: { type: 'number' },
        feelsDry: { type: 'boolean' },
      }, required: ['divergencesFound', 'cumulativeProgramsTested', 'feelsDry'] } }
  )

  if (round === null) {
    log(`campaign round ${roundNum} failed to complete (possible [cyber] safeguard false ` +
        `positive, see SKILL.md) - continuing to the next round`)
    continue
  }

  if (round.divergencesFound.length === 0) {
    dryRounds++
    log(`round ${roundNum}: no divergences (dry streak ${dryRounds}), ` +
        `${round.cumulativeProgramsTested} programs tested cumulatively`)
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
        `The differential-fuzzer harness at ${fuzzerDir}/ reported this possible rustc ` +
        `miscompilation - triage it as bug-hunt candidate ${claim.number} in workspace ${optDir} ` +
        `(ground rules: no patch.diff, no fix, see ${optDir}/README.md). Independently re-run it ` +
        `yourself at least 3x per config to confirm it's real and not a transcription error, and ` +
        `scrutinize the generated program one more time for pre-existing UB before trusting it. If ` +
        `confirmed real: write ${optDir}/${claim.number}.<SHORT_NAME>/ (RESULT.md using ` +
        `${skillDir}/templates/RESULT_template.md as the base - "The fix" section says "not ` +
        `attempted, out of scope for this session"; repro/; output.txt; a justfile copied+filled-in ` +
        `from ${optDir}/candidate_justfile_template, no clone/patch), best-effort bisect via ` +
        `cargo-bisect-rustc (fallback: manual rustup nightly binary search) for an Introduced-by, ` +
        `and append rows to ${optDir}/SUMMARY.md and ${optDir}/SUMMARY-SHORT.md yourself (Class ` +
        `"miscompile", Fixed by blank). If it does NOT hold up under re-verification, explain what ` +
        `actually caused the apparent divergence (harness/template bug, environment nondeterminism, ` +
        `etc.) and do NOT create a candidate folder or SUMMARY row - report that outcome in your ` +
        `final text instead, and if it points at a harness bug, fix ${fuzzerDir}/ so it doesn't ` +
        `recur.\n\nDivergence report: ${JSON.stringify(d)}`,
        { phase: 'Campaign', label: `triage-${claim.number}`, effort: 'high' }
      )
      if (triage !== null) newFindingsCount++
    }
  }

  // Broaden the generator's template coverage after a sustained dry streak, so a long campaign
  // doesn't just keep re-running the same narrow set of patterns for hours - "as broad as
  // possible" was an explicit part of the user's ask, not just "run it once and wait".
  if ((dryRounds >= 3 || round.feelsDry) && dryRounds >= 2) {
    const remAfter = (overallDeadlineEpoch - (await nowEpoch('broaden-check-clock'))) / 60
    if (remAfter > roundBudgetMin * 2) {
      log(`dry streak (${dryRounds}) - broadening the generator's template categories`)
      await agent(
        `${NEVER_EDIT}\n\n` +
        `The differential-fuzzer harness at ${fuzzerDir}/ has gone ${dryRounds} rounds with no new ` +
        `signal. Add at least 2 genuinely new template categories to its generator (not just more ` +
        `of the same shapes) - consider areas not yet covered: const-eval of more complex constant ` +
        `expressions, SIMD/autovectorization-safe loops, niche-filling enum layouts with nested ` +
        `generics, trait-object vtable/devirtualization interactions, closures capturing by move vs ` +
        `reference across inlining boundaries, drop-order-sensitive patterns, or anything else you ` +
        `judge under-explored given ${optDir}/_reference/known_miscompile_issues.md's pattern ` +
        `column. Every new template must still be provably UB-free by construction. Re-validate ` +
        `with a small baseline batch the same way the Build phase did, fix anything that cries ` +
        `wolf, and update ${fuzzerDir}/README.md's category list.`,
        { phase: 'Campaign', label: `broaden-templates-round-${roundNum}`, effort: 'high' }
      )
      dryRounds = 0
    }
  }
}

const finalT = await nowEpoch('final-clock')
const totalElapsedMin = Math.round((finalT - startEpoch) / 60)
log(`session done: ${newFindingsCount} new confirmed finding(s) from ${roundNum} campaign round(s), ` +
    `~${totalElapsedMin}/${budgetMinutes} min elapsed`)

return {
  newFindingsCount,
  campaignRounds: roundNum,
  elapsedMinutes: totalElapsedMin,
  budgetMinutes,
}
