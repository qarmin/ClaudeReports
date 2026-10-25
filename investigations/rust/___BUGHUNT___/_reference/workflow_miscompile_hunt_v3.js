// Follow-up workflow, round 3: a fundamentally different generation strategy - "identity chains".
// Instead of a template computing a value forward and a separate Python oracle predicting what
// the Rust code should print (today's model, and a real source of oracle-bugs/self-check
// mismatches), build DEEPLY NESTED if/match/loop/call structures around a value that is PROVABLY
// unchanged by construction: every node in the tree is drawn from a small, independently-vetted
// library of "identity transforms" (each one is `v -> v` for every input, with a short, locally
// self-evident soundness argument - no Miri needed). The oracle becomes trivial: the printed value
// must equal the known starting constant. User's explicit framing: build "upward" from the target
// value by composing branches/matches/loops that still produce it, go deep for complexity, keep
// unsafe to a small, clearly-bounded, obviously-sound subset (prefer safe Rust first).
//
// Also: briefly survey github.com/matthiaskrgr/icemaker (user is skeptical it's useful here - it's
// crash/ICE-oriented, not differential-miscompile-oriented - just check for a reusable real-Rust-
// program corpus, don't deeply integrate), and document (do NOT implement) treereduce-rust as a
// future step for minimizing any real finding, pointing at rust-lang/rust#163457 as the reference
// example the user gave - this is explicitly deferred, only relevant once something is found.
//
// args: { projectPath, optDir, fuzzerDir, budgetMinutes, buildMinutes, skillDir }

export const meta = {
  name: 'rustc-miscompile-fuzzer-v3',
  description: 'Build an identity-chain generator (deep if/match/loop nesting around a provably-unchanged value, trivial oracle, safe-Rust-first) then run an expanded campaign',
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
log(`round 3: ${budgetMinutes} min total, build <=${buildMinutes} min, campaign gets the rest ` +
    `(~${Math.round((overallDeadlineEpoch - buildDeadlineEpoch) / 60)} min planned)`)

// --- Build: identity-chain generator ----------------------------------------------------------
phase('Build')

const buildReport = await agent(
  `${NEVER_EDIT}\n\n` +
  `Add a NEW, structurally different generation mode to ${fuzzerDir}/ (read its README.md, ` +
  `templates/, templates_static/, generator.py, diff_runner.py, templates/noise.py first - build ` +
  `on top of what's there, don't replace it). This is a separate generation PARADIGM from the ` +
  `existing per-category templates (which compute a value forward and rely on a parallel Python ` +
  `oracle to predict the printed output - a real source of self-check/oracle bugs). Instead:\n\n` +
  `## The "identity chain" idea\n\n` +
  `Start from a known constant (e.g. \`std::hint::black_box(0x5A_u32)\` or similar, for whichever ` +
  `primitive type the chain is built over - vary types across u8/u16/u32/u64/i32/bool/char and a ` +
  `couple of small Copy structs/tuples/arrays). Build a DEEPLY NESTED tree of if/match/loop/call ` +
  `structures around that value, where EVERY node is drawn from a small, independently-vetted ` +
  `library of "identity transforms" - each one is provably \`v -> v\` for every possible input, with ` +
  `a short, self-evident soundness argument that does NOT require Miri-level stacked-borrows ` +
  `reasoning (the user explicitly wants to avoid depending on Miri, which is slow, and prefers safe ` +
  `Rust first with unsafe kept to an obviously-bounded minimum). Recurse to build the tree as deep/ ` +
  `wide as a --depth/--breadth parameter asks for ("going up" from the target value by composing ` +
  `branches/matches/loops that still produce it, per the user's own framing) - this is the lever for ` +
  `arbitrarily complex, deeply-nested control flow without needing a smarter oracle, because the ` +
  `oracle stays trivial (the final printed value must equal the starting constant) no matter how ` +
  `deep the nesting goes.\n\n` +
  `### Safe-Rust identity transform library (build this FIRST, make it the default/primary path - ` +
  `the user said "best to find problems in safe Rust" first; put these in ` +
  `templates/identity_chain.py):\n` +
  `  - arithmetic round-trips: \`v.wrapping_add(k).wrapping_sub(k)\`, \`v ^ k ^ k\`, ` +
  `\`v.rotate_left(k).rotate_right(k)\` (k a generation-time constant, black_box'd)\n` +
  `  - branch identity: \`if black_box(cond) { v } else { v }\` - cond can be an arbitrarily complex ` +
  `boolean expression (even itself built from a parallel identity-sub-chain over an unrelated ` +
  `counter value) since BOTH arms are textually/semantically \`v\`, so correctness never depends on ` +
  `which branch is actually taken, only the optimizer's handling of the branch does\n` +
  `  - match identity: every arm of a \`match\` independently reduces (via its own recursive sub- ` +
  `chain) back to \`v\` - this is where the tree genuinely branches/grows, since each arm gets its ` +
  `own independently-generated sub-chain\n` +
  `  - loop identity: \`for _ in 0..black_box(n) { v = <per-iteration identity transform>(v) }\` - ` +
  `the per-iteration transform must itself be a true single-application identity (not something ` +
  `that needs N applications to cancel out, since n is runtime/black_box'd and not staticaly known)\n` +
  `  - struct/tuple/array wrap-unwrap: \`let w = Wrapper(v); w.0\`, \`[v][0]\`, \`(v,).0\`\n` +
  `  - enum wrap-unwrap: \`match Some(v) { Some(x) => x, None => unreachable!() }\` (None is ` +
  `genuinely unreachable here, so this never panics)\n` +
  `  - function/closure call indirection: call a vetted identity fn directly, through a function ` +
  `pointer, or via a closure capturing \`v\` by move - adds inlining/devirtualization stress\n` +
  `  - bounded recursion: a recursive fn that applies one identity transform per level down to a ` +
  `depth parameter, still returning \`v\`\n` +
  `  Document EACH transform's one-paragraph soundness argument in the module docstring, same ` +
  `convention as the existing templates' UB-freedom arguments.\n\n` +
  `### A SEPARATE, smaller, clearly-labeled "limited unsafe" extension (templates/` +
  `identity_chain_unsafe.py, lower weight / opt-in, not the default) - ONLY these two patterns, ` +
  `both locally obviously sound without Miri-level reasoning:\n` +
  `  - pointer round-trip: \`unsafe { *(&v as *const T) }\` immediately after taking the reference, ` +
  `same scope, no concurrent access, no aliasing concern - sound by the most basic rule (you can ` +
  `always deref a pointer you just made to still-live local data)\n` +
  `  - union byte-reinterpretation (e.g. a union of \`u32\`/\`[u8;4]\`): NOT a pure identity (the ` +
  `value read back through a different field is a real bit-level reinterpretation, not literally v) ` +
  `- so its oracle is NOT trivial-equals-v, but it IS still simple, mechanical Python struct packing ` +
  `(e.g. \`struct.pack\`/\`struct.unpack\`), not a logic-mirroring oracle - document clearly why this ` +
  `is still low-risk (deterministic byte math, not interpreting arbitrary generated control flow)\n` +
  `  Make clear in the README this extension is intentionally small and opt-in (e.g. its own ` +
  `--categories name, not included by default) - the bulk of coverage should come from the safe ` +
  `path.\n\n` +
  `### Integration\n` +
  `Wire this into generator.py as new categor(ies) (e.g. \`identity_chain_safe\`, ` +
  `\`identity_chain_unsafe\`) with --depth/--breadth flags (reasonable defaults, e.g. depth 6-10, ` +
  `breadth 2-4 branches per match node - tune based on what compiles in reasonable time), compatible ` +
  `with the existing --cases-per-file batching and --noise-level injection from the previous round ` +
  `(an identity-chain case can still have noise.py filler spliced around it). diff_runner.py needs ` +
  `no changes - it already diffs stdout/.expected the same way regardless of how a case's oracle was ` +
  `computed.\n\n` +
  `### Quick survey (5-10 min, don't over-invest - the user is already skeptical this helps)\n` +
  `Look at github.com/matthiaskrgr/icemaker (web search/fetch its README) - it's an ICE/crash-corpus ` +
  `fuzzer (runs rustc's own test-suite files through many flag combinations looking for crashes), ` +
  `not a differential-miscompile generator, so it likely doesn't transfer directly - but check ` +
  `whether it points at a reusable corpus of real, diverse, already-valid Rust source files that ` +
  `could seed variety beyond what templates/identity-chains generate synthetically. Report what you ` +
  `found in one short paragraph; do not build an integration unless something is clearly and ` +
  `trivially reusable in the time available.\n\n` +
  `### Document, do NOT implement, a future step\n` +
  `Add a short "Future: automatic minimization with treereduce-rust" section to ${fuzzerDir}/` +
  `README.md noting that IF a future round confirms a real new finding, treereduce-rust (like a ` +
  `C-Reduce for Rust) could automate the "shrink the repro" step the candidate-triage prompt ` +
  `currently does by hand - the user pointed at rust-lang/rust#163457 as a worked example of this ` +
  `kind of reduction. Explicitly: do not install or wire this up now, there is nothing to reduce yet ` +
  `- just leave the pointer for whichever future session needs it.\n\n` +
  `### Validate before moving on (critical - this is a new oracle paradigm)\n` +
  `1. ${fuzzerDir}/check_canaries.sh must still PASS.\n` +
  `2. Generate a baseline batch of identity-chain cases (safe path primarily, a smaller slice of ` +
  `the unsafe extension too) at a representative depth/breadth/cases-per-file/noise-level and ` +
  `confirm ZERO self-check mismatches - any mismatch here means a transform in your library isn't ` +
  `actually an identity, or the oracle computation for the unsafe/reinterpretation path has a bug - ` +
  `fix until clean, this is not optional given how load-bearing the "trivial oracle" claim is.\n\n` +
  `Report: the transform library (count of safe vs unsafe transforms), the depth/breadth knobs and ` +
  `their defaults, the icemaker survey finding, and the baseline validation result.`,
  { phase: 'Build', label: 'build-v3-identity-chains', effort: 'high' }
)
log(`build v3 done: ${JSON.stringify(buildReport).slice(0, 300)}`)

// --- Campaign: run the new generator alongside the existing ones -------------------------------
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
    `Run one campaign round at ${fuzzerDir}/ for roughly ${Math.round(roundBudgetMin)} minutes of ` +
    `real wall-clock work, now INCLUDING the new identity_chain_safe/identity_chain_unsafe ` +
    `categories alongside the existing 17+4 - weight toward identity_chain_safe (the user's stated ` +
    `priority: safe Rust first), vary --depth/--breadth across rounds to explore both shallow-wide ` +
    `and deep-narrow trees, combine with --cases-per-file and --noise-level from before. Use ` +
    `run_round.sh, extending it with any new flags if it hasn't been already. Append a line to ` +
    `ROUNDS.log. Before reporting a divergence, re-verify it (re-run both configs again) and re-read ` +
    `the specific generated program for a bug in the identity-chain's own transform library (an ` +
    `accidentally-non-identity transform would show up as a "self-check mismatch" against .expected, ` +
    `not a cross-config divergence - make sure you're not misreporting one as the other) or ` +
    `pre-existing UB - only report ones you're confident are real cross-config divergences. Report ` +
    `cumulative sub-cases tested (by category family: existing templates vs new identity chains) ` +
    `and whether this round feels dry.`,
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
    dryRounds++
    log(`round ${roundNum}: no divergences (dry streak ${dryRounds}), ` +
        `${round.cumulativeSubCasesTested} sub-cases cumulatively`)
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
        `The differential-fuzzer harness at ${fuzzerDir}/ (v3: identity-chain generation) reported ` +
        `this possible rustc miscompilation - triage it as bug-hunt candidate ${claim.number} in ` +
        `workspace ${optDir} (ground rules: no patch.diff, no fix, see ${optDir}/README.md). ` +
        `Independently re-run it at least 3x per config. Since identity-chain cases can be deeply ` +
        `nested, MINIMIZE it by hand first (strip transform-tree nodes one at a time, outermost or ` +
        `deepest first, re-checking the divergence still reproduces after each removal, down to the ` +
        `smallest sub-tree that still diverges) before writing it up - do not leave a 1000-line ` +
        `nested repro/ if a 20-line one reproduces identically; note in RESULT.md which specific ` +
        `transform(s) in the chain were load-bearing for the divergence. If confirmed real: write ` +
        `${optDir}/${claim.number}.<SHORT_NAME>/ (RESULT.md from ` +
        `${skillDir}/templates/RESULT_template.md - "The fix" says "not attempted, out of scope"; ` +
        `repro/; output.txt; justfile from ${optDir}/candidate_justfile_template), best-effort ` +
        `bisect via cargo-bisect-rustc, append rows to ${optDir}/SUMMARY.md and SUMMARY-SHORT.md ` +
        `yourself (Class "miscompile"). If it does NOT hold up, explain what actually caused it - if ` +
        `it points at a non-identity bug in the transform library itself, fix ` +
        `${fuzzerDir}/templates/identity_chain*.py so it doesn't recur, but do NOT create a ` +
        `candidate folder for a harness bug.\n\nDivergence report: ${JSON.stringify(d)}`,
        { phase: 'Campaign', label: `triage-${claim.number}`, effort: 'high' }
      )
      if (triage !== null) newFindingsCount++
    }
  }
}

const finalT = await nowEpoch('final-clock')
const totalElapsedMin = Math.round((finalT - startEpoch) / 60)
log(`round 3 done: ${newFindingsCount} new confirmed finding(s) from ${roundNum} campaign round(s), ` +
    `~${totalElapsedMin}/${budgetMinutes} min elapsed`)

return {
  newFindingsCount,
  campaignRounds: roundNum,
  elapsedMinutes: totalElapsedMin,
  budgetMinutes,
}
