"""Testbed components. Each takes a Ctx and returns a list of failure dicts (see common.failure)."""
import collections
import gzip
import lzma
import os
import random
import re
import shutil
import struct
import subprocess
import tempfile
from compression import zstd
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

from common import (BUILD_FUZZ, BUILD_GCC, KHEADERS, REAL_MODDIR, REALROOT, SRC, SYSTEM_MAP, KVER,
                    TB, failure, real_modules, run, sample, save_input)
import kelf
import synth
from sig import first_line, signature

KINFO = TB / "drivers" / "bin" / "kinfo"
IDXCHECK = TB / "drivers" / "bin" / "idxcheck"
ENVTOK = TB / "drivers" / "bin" / "envtok"
FUZZ_BIN = TB / "fuzz" / "bin"
SIG_KEYS = {"sig_id", "signer", "sig_key", "sig_hashalgo", "signature"}


def _san_failure(component, rc, err, input_path, ctx, save_name=None):
    """Turn a crashed/sanitizer-reporting run into a failure (None when the run was clean)."""
    if rc == "timeout":
        return failure(f"timeout:{component}", component, input_path, "timed out")
    sig = signature(err)
    if sig is None and isinstance(rc, int) and rc < 0:
        sig = f"signal:{-rc}"
    if sig is None:
        return None
    if save_name and input_path and Path(input_path).exists():
        input_path = save_input(ctx, component, save_name, input_path)
    return failure(sig, component, input_path, first_line(err))


def parse_kinfo(text):
    """kinfo output -> {path: {"info": [...], "ver": set, "sym": set, "dep": set, "error": [...]}}"""
    mods = {}
    cur = None
    for line in text.splitlines():
        if line.startswith("== "):
            cur = mods.setdefault(line[3:], collections.defaultdict(list))
            continue
        kind, _, rest = line.partition("\t")
        cur[kind].append(tuple(rest.split("\t")))
    return mods


def _unesc(s):
    return s.replace("\\n", "\n").replace("\\t", "\t").replace("\\\\", "\\")


# --------------------------------------------------------------------------------------------
# Existing test suite under sanitizers (clang ASAN+UBSAN and gcc ASAN+UBSAN builds).
# --------------------------------------------------------------------------------------------
def _suite(ctx, bld, cc, component):
    env = {"CC": cc, "KDIR": str(KHEADERS)}
    rc, out, err = run(["meson", "test", "-C", bld, "--num-processes", "4"], timeout=900, env=env)
    fails = []
    for m in re.finditer(r"^\s*\d+/\d+\s+kmod:(\S+)\s+(FAIL|TIMEOUT|ERROR)", out, re.M):
        log = (Path(bld) / "meson-logs" / "testlog.txt")
        fails.append(failure(f"suite:{component}:{m.group(1)}:{m.group(2)}", component, log,
                             f"meson test {m.group(1)} {m.group(2)}"))
    if rc == "timeout":
        fails.append(failure(f"timeout:{component}", component, bld, "meson test timed out"))
    elif not re.search(r"^Ok:\s+\d+", out, re.M):
        fails.append(failure(f"suite:{component}:did-not-run", component, bld, (out + err)[-300:]))
    return fails


def suite_clang(ctx):
    return _suite(ctx, BUILD_FUZZ, "clang", "suite_clang")


def suite_gcc(ctx):
    return _suite(ctx, BUILD_GCC, "gcc", "suite_gcc")


# --------------------------------------------------------------------------------------------
# libFuzzer targets (fork mode, crashes collected, re-run once each to get a signature).
# --------------------------------------------------------------------------------------------
FUZZ_TARGETS = ["elf", "modfile", "index", "config", "builtin", "modprobe_env", "util", "api"]


def _fuzz(ctx, target):
    component = f"fuzz_{target}"
    wd = ctx.workdir(component)
    corpus = TB / "__corpus-data" / "fuzz" / target
    seeds = TB / "__corpus-data" / "seeds" / target
    corpus.mkdir(parents=True, exist_ok=True)
    secs = 600 if ctx.long else 75
    cmd = [FUZZ_BIN / f"fuzz_{target}", corpus, seeds, f"-max_total_time={secs}", "-timeout=10",
           "-rss_limit_mb=1536", "-max_len=1048576", f"-seed={ctx.seed}", "-fork=2",
           "-ignore_crashes=1", "-ignore_timeouts=1", "-ignore_ooms=1",
           f"-artifact_prefix={wd}/", "-print_final_stats=1"]
    rc, out, err = run(cmd, timeout=secs + 120, env={"TMPDIR": str(wd), "KMOD_API_TREE": str(TB / "__corpus-data" / "api-tree")})
    (wd / "fuzz.log").write_text(err[-200000:])
    fails = []
    for art in sorted(wd.glob("crash-*")) + sorted(wd.glob("timeout-*")) + sorted(wd.glob("oom-*")):
        rc2, _o, e2 = run([FUZZ_BIN / f"fuzz_{target}", art, "-timeout=10", "-rss_limit_mb=1536"],
                          timeout=60, env={"TMPDIR": str(wd), "KMOD_API_TREE": str(TB / "__corpus-data" / "api-tree")})
        sig = signature(e2) or f"{art.name.split('-')[0]}:{component}"
        if art.name.startswith("timeout-"):
            sig = f"timeout:{component}"
        fails.append(failure(sig, component, art, first_line(e2)))
    if not re.search(r"stat::number_of_executed_units|Done \d+ runs|INFO: fuzzed for", err):
        fails.append(failure(f"testbed:{component}:did-not-run", component, wd, err[-300:]))
    for d in wd.glob("kmodfuzz.*"):
        shutil.rmtree(d, ignore_errors=True)
    return fails


def make_fuzz_component(target):
    def f(ctx):
        return _fuzz(ctx, target)
    f.__name__ = f"fuzz_{target}"
    return f


# --------------------------------------------------------------------------------------------
# modinfo differential: libkmod's view of real modules vs the independent Python ELF reader.
# --------------------------------------------------------------------------------------------
def _expected_dep_symbols(e, versions):
    """Undefined .symtab symbols, plus __versions entries no symbol matched (libkmod documents
    these as module_layout/struct_module: "not visited, but needed")."""
    crcs = dict(versions)
    out, used = set(), set()
    for name, shndx, bind in e.symbols():
        if shndx == 0 and name and bind in (1, 2):
            out.add((name, crcs.get(name, 0)))
            used.add(name)
    out |= {(n, c) for n, c in versions if n not in used}
    return out


def modinfo_diff(ctx):
    component = "modinfo_diff"
    mods = sample(real_modules(), None if ctx.long else 400, ctx.seed)
    rc, out, err = run([KINFO], stdin="\n".join(map(str, mods)).encode(), timeout=900)
    fails = []
    f = _san_failure(component, rc, err, "", ctx)
    if f:
        return [f]
    got = parse_kinfo(out)
    for path in mods:
        k = got.get(str(path))
        if k is None:
            fails.append(failure("diff:modinfo-missing-output", component, path, "no kinfo block"))
            continue
        e = kelf.Elf(kelf.load_module_bytes(path))
        exp_info = collections.Counter(e.modinfo())
        bid = e.build_id()
        if bid:
            exp_info[("build-id", bid)] += 1
        got_info = collections.Counter((a, _unesc(b) if len(t) > 1 else "")
                                       for t in k["info"] for a, b in [(t[0], t[1] if len(t) > 1 else "")]
                                       if a not in SIG_KEYS)
        if exp_info != got_info:
            missing = list((exp_info - got_info).elements())[:3]
            extra = list((got_info - exp_info).elements())[:3]
            fails.append(failure("diff:modinfo-info", component, path,
                                 f"missing={missing} extra={extra}"))
        exp_ver = set(e.versions())
        got_ver = {(t[0], int(t[1], 16)) for t in k["ver"]}
        if exp_ver != got_ver:
            fails.append(failure("diff:modinfo-versions", component, path,
                                 f"missing={sorted(exp_ver - got_ver)[:3]} extra={sorted(got_ver - exp_ver)[:3]}"))
        exp_dep = _expected_dep_symbols(e, e.versions())
        got_dep = {(t[0], int(t[1], 16)) for t in k["dep"]}
        if exp_dep != got_dep:
            fails.append(failure("diff:modinfo-depsyms", component, path,
                                 f"missing={sorted(exp_dep - got_dep)[:3]} extra={sorted(got_dep - exp_dep)[:3]}"))
        exp_sym = {n[len("__ksymtab_"):] for n, shndx, _b in e.symbols()
                   if n.startswith("__ksymtab_") and shndx != 0}
        got_sym = {t[0] for t in k["sym"]}
        if exp_sym and exp_sym != got_sym:
            fails.append(failure("diff:modinfo-symbols", component, path,
                                 f"missing={sorted(exp_sym - got_sym)[:3]} extra={sorted(got_sym - exp_sym)[:3]}"))
        trailer = kelf.signature_trailer(kelf.load_module_bytes(path))
        has_sig = any(t[0] == "sig_id" for t in k["info"])
        if bool(trailer) != has_sig:
            fails.append(failure("diff:modinfo-signature-presence", component, path,
                                 f"trailer={bool(trailer)} libkmod={has_sig}"))
    return fails


def signature_diff(ctx):
    """libkmod's PKCS#7 signer/key/hash vs `openssl cms -cmsout -print` on the same signature."""
    component = "signature_diff"
    mods = sample(real_modules(), None if ctx.long else 25, ctx.seed + 7)
    rc, out, err = run([KINFO] + mods, timeout=300)
    f = _san_failure(component, rc, err, "", ctx)
    if f:
        return [f]
    got = parse_kinfo(out)
    fails = []
    for path in mods:
        info = {t[0]: (t[1] if len(t) > 1 else "") for t in got[str(path)]["info"]}
        tr = kelf.signature_trailer(kelf.load_module_bytes(path))
        if not tr or tr["id_type"] != 2:
            continue
        p = subprocess.run(["openssl", "cms", "-inform", "DER", "-cmsout", "-print"],
                           input=tr["sig"], capture_output=True)
        txt = p.stdout.decode("latin-1")
        m_cn = re.search(r"signerInfos:.*?issuer:.*?CN=([^,\n]+)", txt, re.S)
        m_ser = re.search(r"signerInfos:.*?serialNumber:\s*(?:0x)?([0-9A-Fa-f]+)", txt, re.S)
        m_hash = re.search(r"signerInfos:.*?digestAlgorithm:\s*\n\s*algorithm:\s*(\S+)", txt, re.S)
        if m_cn and info.get("signer") != m_cn.group(1).strip():
            fails.append(failure("diff:signature-signer", component, path,
                                 f"libkmod={info.get('signer')!r} openssl={m_cn.group(1)!r}"))
        if m_ser:
            ser = m_ser.group(1).upper()
            ser = ("0" + ser) if len(ser) % 2 else ser
            exp = ":".join(ser[i:i + 2] for i in range(0, len(ser), 2))
            if info.get("sig_key", "").upper() != exp:
                fails.append(failure("diff:signature-key", component, path,
                                     f"libkmod={info.get('sig_key')} openssl={exp}"))
        if m_hash and info.get("sig_hashalgo") != m_hash.group(1):
            fails.append(failure("diff:signature-hash", component, path,
                                 f"libkmod={info.get('sig_hashalgo')} openssl={m_hash.group(1)}"))
    fails += _signature_synthetic(ctx, component)
    return fails


def _signature_synthetic(ctx, component):
    """Self-signed synthetic signed modules with unusual issuer DNs (real modules always have a CN); 029 was found this way."""
    wd = ctx.workdir(component + "_synth")
    subjects = [("O=Foo Org/C=PL", ""), ("C=PL", ""), ("O=A/CN=First/CN=Second", "First"), ("CN=Only", "Only"),
                ("CN=Mid/O=After", "Mid"), ("O=Z/CN=Caf\u00e9/C=PL", "Caf\u00e9")]
    mods, expected = [], {}
    (wd / "payload").write_bytes(b"payload")
    for i, (dn, exp) in enumerate(subjects):
        key, pem, der, ko = (wd / f"{i}.{x}" for x in ("key", "pem", "der", "ko"))
        r = subprocess.run(["openssl", "req", "-x509", "-newkey", "ec", "-pkeyopt", "ec_paramgen_curve:prime256v1", "-nodes",
                            "-keyout", str(key), "-out", str(pem), "-subj", "/" + dn, "-days", "3", "-utf8"], capture_output=True)
        if r.returncode:
            continue
        subprocess.run(["openssl", "smime", "-sign", "-binary", "-noattr", "-in", str(wd / "payload"), "-signer", str(pem),
                        "-inkey", str(key), "-outform", "DER", "-out", str(der)], check=True, capture_output=True)
        sig = der.read_bytes()
        ko.write_bytes(synth.module([], [], ["license=GPL"]) + sig + struct.pack(">BBBBB3xI", 0, 4, 2, 0, 0, len(sig))
                       + b"~Module signature appended~\n")
        mods.append(str(ko))
        expected[str(ko)] = (dn, exp)
    if not mods:
        return []
    rc, out, err = run([KINFO] + mods, timeout=120)
    f = _san_failure(component, rc, err, "", ctx)
    if f:
        return [f]
    got = parse_kinfo(out)
    fails = []
    for path in mods:
        info = {t[0]: (t[1] if len(t) > 1 else "") for t in got[path]["info"]}
        dn, exp = expected[path]
        if info.get("signer", "") != exp:
            fails.append(failure("diff:signature-signer-synthetic", component, path,
                                 f"issuer {dn!r}: libkmod={info.get('signer')!r} expected={exp!r}"))
    return fails


# --------------------------------------------------------------------------------------------
# Compression differential: the same module raw / xz / gzip / zstd must give identical answers.
# --------------------------------------------------------------------------------------------
def compress_diff(ctx):
    component = "compress_diff"
    wd = ctx.workdir(component)
    mods = sample(real_modules(), 120 if ctx.long else 30, ctx.seed + 3)
    fails = []
    for i, path in enumerate(mods):
        raw = kelf.load_module_bytes(path)
        variants = {
            "raw.ko": raw,
            "xz.ko.xz": lzma.compress(raw, check=lzma.CHECK_CRC32),
            "gz.ko.gz": gzip.compress(raw),
            "zst.ko.zst": zstd.compress(raw),
            "trunc.ko.zst": zstd.compress(raw)[:-7],
            "trunc.ko.xz": lzma.compress(raw, check=lzma.CHECK_CRC32)[:-9],
            "trunc.ko.gz": gzip.compress(raw)[:-5],
            "half.ko.zst": zstd.compress(raw)[: len(zstd.compress(raw)) // 2],
            "half.ko.xz": lzma.compress(raw)[: len(lzma.compress(raw)) // 2],
            "half.ko.gz": gzip.compress(raw)[: len(gzip.compress(raw)) // 2],
        }
        d = wd / f"m{i}"
        d.mkdir(exist_ok=True)
        paths = []
        for suffix, data in variants.items():
            p = d / f"{path.name.split('.')[0]}.{suffix}"
            p.write_bytes(data)
            paths.append(p)
        rc, out, err = run([KINFO] + paths, timeout=120)
        f = _san_failure(component, rc, err, d, ctx, save_name=f"m{i}")
        if f:
            fails.append(f)
            continue
        got = parse_kinfo(out)
        norm = {}
        for p in paths:
            blk = got.get(str(p), {})
            norm[p.name] = sorted((kind, t) for kind, v in blk.items() for t in v)
        ref = norm[paths[0].name]
        for p in paths[1:4]:
            if norm[p.name] != ref:
                fails.append(failure(f"diff:compress-{p.name.split('.')[1]}", component,
                                     save_input(ctx, component, f"m{i}", d), f"{p.name} differs from raw"))
        for p in paths[4:]:
            blk = got.get(str(p), {})
            if blk.get("info") and not blk.get("error"):
                kind = "trailer" if ".trunc." in p.name else "half"
                fails.append(failure(f"diff:compress-truncated-{kind}-accepted-{p.name.split('.')[-1]}",
                                     component, save_input(ctx, component, f"m{i}", d),
                                     f"{p.name}: truncated stream returned module info"))
        shutil.rmtree(d, ignore_errors=True)
    return fails


# --------------------------------------------------------------------------------------------
# depmod on the real 7.0.0-38-generic tree vs a model built from each module's own .modinfo.
# --------------------------------------------------------------------------------------------
def _run_depmod(ctx, outdir, extra=()):
    outdir.mkdir(parents=True, exist_ok=True)
    return run([ctx.tool("depmod"), "-b", REALROOT / "usr", "-o", outdir, "-F", SYSTEM_MAP,
                "-e", *extra, KVER], timeout=600)


def _closure(direct, name, memo, stack=()):
    if name in memo:
        return memo[name]
    res = set()
    for d in direct.get(name, ()):
        if d in stack:
            continue
        res.add(d)
        res |= _closure(direct, d, memo, stack + (name,))
    memo[name] = res
    return res


def depmod_model(ctx):
    component = "depmod_model"
    wd = ctx.workdir(component)
    out1, out2 = wd / "out1", wd / "out2"
    fails = []
    rc, o, err = _run_depmod(ctx, out1)
    f = _san_failure(component, rc, err, REALROOT, ctx)
    if f:
        return [f]
    if rc != 0:
        return [failure("depmod:nonzero-exit", component, REALROOT, err[-300:])]
    for line in err.splitlines():
        if "needs unknown symbol" in line or "ERROR" in line:
            fails.append(failure("depmod:unexpected-diagnostic", component, REALROOT, line))
            break
    gen = out1 / "lib" / "modules" / KVER

    names, direct, aliases, softdeps, weakdeps, devnames = {}, {}, set(), {}, {}, set()
    for p in real_modules():
        e = kelf.Elf(kelf.load_module_bytes(p))
        mi = e.modinfo()
        name = p.name.split(".")[0].replace("-", "_")
        names[name] = p.relative_to(REAL_MODDIR).as_posix()
        dep = [v for k, v in mi if k == "depends"]
        direct[name] = {d.replace("-", "_") for v in dep for d in v.split(",") if d}
        for k, v in mi:
            if k == "alias":
                aliases.add((v, name))
            elif k == "softdep":
                softdeps.setdefault(name, []).append(v)
            elif k == "weakdep":
                weakdeps.setdefault(name, []).append(v)
        devname = next((v for k, v in mi if k == "alias" and v.startswith("devname:")), None)
        majmin = next((v for k, v in mi if k == "alias" and re.match(r"(char|block)-major-\d+-\d+$", v)), None)
        if devname and majmin:
            devnames.add((name, devname[len("devname:"):], majmin))

    # modules.dep: each line's dependency set == transitive closure of modinfo depends=.
    memo = {}
    by_path = {v: k for k, v in names.items()}
    seen = set()
    for line in (gen / "modules.dep").read_text().splitlines():
        mpath, _, rest = line.partition(":")
        mod = by_path.get(mpath)
        if mod is None:
            fails.append(failure("model:modules.dep-unknown-path", component, gen / "modules.dep", line))
            continue
        seen.add(mod)
        got = [by_path.get(x, x) for x in rest.split()]
        exp = _closure(direct, mod, memo)
        if set(got) != exp:
            fails.append(failure("model:modules.dep-closure", component, gen / "modules.dep",
                                 f"{mod}: missing={sorted(exp - set(got))[:4]} extra={sorted(set(got) - exp)[:4]}"))
        # Load-order property: every dependency's own deps appear after it on the line.
        pos = {d: i for i, d in enumerate(got)}
        for i, d in enumerate(got):
            for dd in _closure(direct, d, memo):
                if dd in pos and pos[dd] < i:
                    fails.append(failure("model:modules.dep-order", component, gen / "modules.dep",
                                         f"{mod}: {dd} listed before its dependent {d}"))
                    break
    if set(names) - seen:
        fails.append(failure("model:modules.dep-missing-module", component, gen / "modules.dep",
                             f"{sorted(set(names) - seen)[:5]}"))

    got_alias = set()
    for line in (gen / "modules.alias").read_text().splitlines():
        if line.startswith("alias "):
            _a, al, mod = line.split(" ", 2)
            got_alias.add((al, mod))
    exp_alias = {(a, m) for a, m in aliases}
    if exp_alias != got_alias:
        fails.append(failure("model:modules.alias", component, gen / "modules.alias",
                             f"missing={sorted(exp_alias - got_alias)[:3]} extra={sorted(got_alias - exp_alias)[:3]}"))

    got_soft = {}
    for line in (gen / "modules.softdep").read_text().splitlines():
        if line.startswith("softdep "):
            _s, mod, rest = line.split(" ", 2)
            got_soft.setdefault(mod, []).append(rest)
    if {k: sorted(v) for k, v in softdeps.items()} != {k: sorted(v) for k, v in got_soft.items()}:
        diff = set(softdeps) ^ set(got_soft) or {k for k in softdeps if sorted(softdeps[k]) != sorted(got_soft.get(k, []))}
        fails.append(failure("model:modules.softdep", component, gen / "modules.softdep", f"{sorted(diff)[:5]}"))

    got_weak = {}
    for line in (gen / "modules.weakdep").read_text().splitlines():
        if line.startswith("weakdep "):
            _s, mod, rest = line.split(" ", 2)
            got_weak.setdefault(mod, []).extend(rest.split())
    exp_weak = {k: sorted(x for v in vs for x in v.split()) for k, vs in weakdeps.items()}
    if exp_weak != {k: sorted(v) for k, v in got_weak.items()}:
        fails.append(failure("model:modules.weakdep", component, gen / "modules.weakdep",
                             f"{sorted(set(exp_weak) ^ set(got_weak))[:5]}"))

    got_dev = set()
    for line in (gen / "modules.devname").read_text().splitlines():
        if line and not line.startswith("#"):
            mod, dev, typ = line.split()
            got_dev.add((mod, dev, typ))
    exp_dev = {(m, d, ("c" if mm.startswith("char") else "b") + mm.split("major-")[1].replace("-", ":"))
               for m, d, mm in devnames}
    if exp_dev != got_dev:
        fails.append(failure("model:modules.devname", component, gen / "modules.devname",
                             f"missing={sorted(exp_dev - got_dev)[:3]} extra={sorted(got_dev - exp_dev)[:3]}"))

    # Idempotence: a second run over the same input produces byte-identical output.
    rc, _o, err = _run_depmod(ctx, out2)
    gen2 = out2 / "lib" / "modules" / KVER
    for f1 in sorted(gen.iterdir()):
        f2 = gen2 / f1.name
        if not f2.exists() or f1.read_bytes() != f2.read_bytes():
            fails.append(failure(f"model:depmod-not-deterministic:{f1.name}", component, f1, "second run differs"))
    shutil.rmtree(out2, ignore_errors=True)
    return fails


# --------------------------------------------------------------------------------------------
# Index readers on real depmod output: .bin dump vs text file, mmap reader vs stdio reader.
# --------------------------------------------------------------------------------------------
def index_diff(ctx):
    component = "index_diff"
    wd = ctx.workdir(component)
    out = wd / "out"
    rc, _o, err = _run_depmod(ctx, out)
    if rc != 0:
        return [failure("testbed:index_diff:depmod-failed", component, out, err[-300:])]
    gen = out / "lib" / "modules" / KVER
    fails = []

    def dump(t):
        rc, o, e = run([IDXCHECK, "dump", gen, str(t)], timeout=120)
        f = _san_failure(component, rc, e, gen, ctx)
        if f:
            fails.append(f)
        return o

    dep_txt = sorted(l for l in (gen / "modules.dep").read_text().splitlines() if l)
    # The .bin dump prefixes each line with its index key ("<modname> <path>: <deps>").
    dep_bin = sorted(l.split(" ", 1)[1] for l in dump(0).splitlines() if " " in l)
    if dep_txt != dep_bin:
        fails.append(failure("diff:index-dump-modules.dep", component, gen,
                             f"text-only={sorted(set(dep_txt) - set(dep_bin))[:2]} bin-only={sorted(set(dep_bin) - set(dep_txt))[:2]}"))
    def norm_alias(line):
        # depmod stores alias keys normalized ('-' -> '_' outside [...]) in the .bin, by design.
        _a, key, mod = line.split(" ", 2)
        out, depth = [], 0
        for ch in key:
            depth += ch == "["
            depth -= ch == "]"
            out.append("_" if ch == "-" and depth <= 0 else ch)
        return f"alias {''.join(out)} {mod}"

    al_txt = sorted(norm_alias(l) for l in (gen / "modules.alias").read_text().splitlines() if l.startswith("alias "))
    al_bin = sorted(l for l in dump(1).splitlines() if l.startswith("alias "))
    if sorted(set(al_txt)) != sorted(set(al_bin)):
        fails.append(failure("diff:index-dump-modules.alias", component, gen,
                             f"text-only={sorted(set(al_txt) - set(al_bin))[:2]} bin-only={sorted(set(al_bin) - set(al_txt))[:2]}"))
    sy_txt = sorted(l for l in (gen / "modules.symbols").read_text().splitlines() if l.startswith("alias "))
    sy_bin = sorted(l for l in dump(2).splitlines() if l.startswith("alias "))
    if sorted(set(sy_txt)) != sorted(set(sy_bin)):
        fails.append(failure("diff:index-dump-modules.symbols", component, gen,
                             f"text-only={sorted(set(sy_txt) - set(sy_bin))[:2]} bin-only={sorted(set(sy_bin) - set(sy_txt))[:2]}"))

    names = [l.split(":")[0].rsplit("/", 1)[-1].split(".")[0] for l in dep_txt]
    alias_keys = [l.split(" ", 2)[1] for l in al_txt]
    rnd = random.Random(ctx.seed)
    probes = names + alias_keys
    if not ctx.long:
        probes = sample(names, 300, ctx.seed) + sample(alias_keys, 1500, ctx.seed)
    probes += ["pci:v00008086d00001234sv00001028sd00000001bc02sc00i00",
               "usb:v046Dp0825d0010dcEFdsc02dp01ic0Eisc01ip00in00", "", "*", "?", "[", "a" * 300,
               "symbol:printk", "char-major-10-200", "fs-ext4", "net-pf-10"]
    probes += ["".join(rnd.choice("abcdefgh:*?[]-_0123456789") for _ in range(rnd.randint(1, 40)))
               for _ in range(500)]
    rc, o, e = run([IDXCHECK, "lookup", gen], stdin="\n".join(probes).encode(), timeout=600)
    f = _san_failure(component, rc, e, gen, ctx)
    if f:
        fails.append(f)
    for line in o.splitlines():
        if line.startswith("MISMATCH"):
            p = save_input(ctx, component, "mismatch-keys.txt", line.encode())
            fails.append(failure("diff:index-mm-vs-file-lookup", component, p, line))
            break
    shutil.rmtree(out, ignore_errors=True)
    return fails


# --------------------------------------------------------------------------------------------
# CLI matrix: every tool, many flag combinations, against the real tree; ASAN/UBSAN-clean only.
# --------------------------------------------------------------------------------------------
def cli_matrix(ctx):
    component = "cli_matrix"
    wd = ctx.workdir(component)
    out = wd / "out"
    rc, _o, err = _run_depmod(ctx, out)
    base = out
    moddir = out / "lib" / "modules" / KVER
    if rc != 0:
        return [failure("testbed:cli_matrix:depmod-failed", component, out, err[-300:])]
    for f in REAL_MODDIR.glob("modules.builtin*"):
        shutil.copy(f, moddir)
    (moddir / "kernel").exists() or os.symlink(REAL_MODDIR / "kernel", moddir / "kernel")
    cfgdir = wd / "modprobe.d"
    cfgdir.mkdir(exist_ok=True)
    (cfgdir / "t.conf").write_text(
        "alias myalias e1000e\noptions e1000e debug=1 copybreak=$CMDLINE_OPTS\n"
        "install dummy /bin/true $CMDLINE_OPTS\nsoftdep e1000e pre: crc32c post: ptp\n"
        "weakdep e1000e ptp\nblacklist floppy\nremove dummy /bin/true\n")
    relconf = "t.conf"
    some = sample(real_modules(), 6, ctx.seed)
    mods = ["e1000e", "ext4", "myalias", "dummy", "pci:v00008086d000010D3sv*sd*bc*sc*i*",
            "nonexistent_mod", "symbol:printk", "ext4.ko"] + [p.name.split(".")[0] for p in some]
    D, B, C = ["-d", base], ["-b", base], ["-C", cfgdir]
    cmds = []
    for m in mods:
        cmds += [["modinfo", *B, "-k", KVER, m], ["modinfo", *B, "-k", KVER, "-F", "alias", m],
                 ["modinfo", *B, "-k", KVER, "-0", "-a", "-d", "-l", "-p", "-n", m],
                 ["modprobe", *D, *C, "-S", KVER, "-n", "-v", m],
                 ["modprobe", *D, *C, "-S", KVER, "--show-depends", m],
                 ["modprobe", *D, *C, "-S", KVER, "-R", m],
                 ["modprobe", *D, *C, "-S", KVER, "-n", "-r", "-v", m],
                 ["modprobe", *D, *C, "-S", KVER, "-n", "-a", "-i", "-b", m, "ext4"],
                 ["modprobe", *D, *C, "-S", KVER, "-n", "--first-time", "-f", m, "opt=1", "x=$CMDLINE_OPTS"]]
    for p in some:
        cmds += [["modinfo", p], ["insmod", "-f", p, "a=1"], ["insmod", p]]
    cmds += [["modprobe", *D, *C, "-S", KVER, "-c"], ["modprobe", *D, *C, "-S", KVER, "--show-modversions", some[0]],
             ["modprobe", *D, *C, "-S", KVER, "--dump-modversions", some[0]],
             ["depmod", *B, "-n", KVER], ["depmod", *B, "-A", KVER], ["depmod", *B, "-a", "-w", KVER],
             ["depmod", *B, "-n", "-P", "x86_", KVER], ["depmod", *B, "-n", "-v", KVER],
             ["depmod", *B, "-C", cfgdir, "-n", KVER], ["depmod", *B, "-n", KVER, some[0].as_posix()],
             ["lsmod"], ["rmmod", "nonexistent_mod"], ["rmmod", "-f", "nonexistent_mod"],
             ["kmod", "static-nodes", "-o", wd / "sn.txt", "-f", "tmpfiles"],
             ["kmod", "static-nodes", "-o", wd / "sn2.txt", "-f", "devname"],
             ["kmod", "static-nodes", "-f", "human"], ["kmod", "list"], ["kmod", "--version"],
             ["modinfo", "-k", KVER, "-b", base, "-F", "", "e1000e"], ["modinfo", "--field", "x" * 4096, some[0]],
             ["modprobe", "-n", "-S", "x" * 5000, "e1000e"], ["depmod", "-n", "-b", "/" * 3000, KVER]]
    fails = []
    for i, c in enumerate(cmds):
        exe, args = c[0], c[1:]
        cwd = cfgdir if "depmod" == exe and "-C" in args else None
        rc, o, e = run([ctx.tool(exe), *args], timeout=60, cwd=cwd)
        f = _san_failure(component, rc, e, " ".join(map(str, c)), ctx)
        if f:
            f["detail"] = (" ".join(map(str, c)) + " :: " + f["detail"])[:300]
            fails.append(f)
        elif isinstance(rc, int) and rc >= 128:
            fails.append(failure(f"cli:exit-{rc}:{exe}", component, " ".join(map(str, c)), e[-200:]))
    # Relative -C path: depmod run from inside the config dir with a bare file name.
    rc, o, e = run([ctx.tool("depmod"), *B, "-n", "-C", relconf, KVER], timeout=120, cwd=cfgdir)
    f = _san_failure(component, rc, e, f"depmod -C {relconf} (cwd=modprobe.d)", ctx)
    if f:
        fails.append(f)
    return fails


# --------------------------------------------------------------------------------------------
# modprobe -c on generated configs vs a model of the documented modprobe.d(5) semantics.
# --------------------------------------------------------------------------------------------
def _gen_config(rnd):
    names = ["mod_a", "mod_b", "mod-c", "mod_d", "e1000e", "x"]
    lines, model = [], collections.Counter()
    for _ in range(rnd.randint(1, 25)):
        kind = rnd.choice(["alias", "options", "blacklist", "install", "remove", "softdep", "weakdep"])
        m = rnd.choice(names)
        mu = m.replace("-", "_")
        if kind == "alias":
            a = rnd.choice(["pci:v*d*", "net-pf-10", "fs-foo", "char-major-1-2", "my-alias"])
            lines.append(f"alias {a} {m}")
            # kmod normalizes alias names like module names ('-' -> '_' outside [...]), by design.
            model[f"alias {a.replace('-', '_')} {mu}"] += 1
        elif kind == "options":
            opts = " ".join(rnd.choice(["a=1", "b=\"x y\"", "c", "d=1,2"]) for _ in range(rnd.randint(1, 3)))
            lines.append(f"options {m} {opts}")
            model[("options", mu)] += 1
        elif kind == "blacklist":
            lines.append(f"blacklist {m}")
            model[f"blacklist {mu}"] += 1
        elif kind in ("install", "remove"):
            cmd = rnd.choice(["/bin/true", "/sbin/modprobe --ignore-install $CMDLINE_OPTS x", "echo a  b"])
            lines.append(f"{kind} {m} {cmd}")
            model[f"{kind} {mu} {cmd}"] += 1
        elif kind == "softdep":
            pre = rnd.sample(["p1", "p2", "p3"], rnd.randint(0, 2))
            post = rnd.sample(["q1", "q2"], rnd.randint(0, 2))
            if not pre and not post:
                pre = ["p1"]
            spec = (("pre: " + " ".join(pre)) if pre else "") + (" " if pre and post else "") + \
                   (("post: " + " ".join(post)) if post else "")
            lines.append(f"softdep {m} {spec}")
            model[f"softdep {mu} {spec}"] += 1
        else:
            w = rnd.sample(["w1", "w2", "w3"], rnd.randint(1, 2))
            lines.append(f"weakdep {m} {' '.join(w)}")
            model[f"weakdep {mu} {' '.join(w)}"] += 1
    return lines, model


def config_model(ctx):
    component = "config_model"
    wd = ctx.workdir(component)
    rnd = random.Random(ctx.seed)
    fails, seen = [], set()
    empty = wd / "emptyroot" / "lib" / "modules" / KVER
    empty.mkdir(parents=True, exist_ok=True)
    for i in range(300 if ctx.long else 60):
        lines, model = _gen_config(rnd)
        d = wd / f"cfg{i}"
        d.mkdir(exist_ok=True)
        (d / "gen.conf").write_text("\n".join(lines) + "\n")
        rc, o, e = run([ctx.tool("modprobe"), "-d", wd / "emptyroot", "-S", KVER, "-C", d, "-c"], timeout=30)
        f = _san_failure(component, rc, e, d, ctx, save_name=f"cfg{i}")
        if f:
            fails.append(f)
            continue
        got = collections.Counter()
        for line in o.splitlines():
            if line.startswith("options "):
                parts = line.split(" ", 2)
                got[("options", parts[1])] += 0  # options lines are merged per module, compare names only
                got[("options", parts[1])] = max(1, got[("options", parts[1])])
            elif line.split(" ")[0] in ("alias", "blacklist", "install", "remove", "softdep", "weakdep"):
                got[line] += 1
        exp = collections.Counter({k: v for k, v in model.items() if not isinstance(k, tuple)})
        exp_opts = {k for k in model if isinstance(k, tuple)}
        got_cmp = collections.Counter({k: v for k, v in got.items() if not isinstance(k, tuple)})
        got_opts = {k for k in got if isinstance(k, tuple)}
        # Only lines this config produced: modprobe -c also prints kernel-cmdline options.
        if exp != got_cmp or not exp_opts <= got_opts:
            missing = list((exp - got_cmp).elements())
            extra = list((got_cmp - exp).elements())
            kinds = sorted({x.split(" ")[0] for x in missing + extra}) or ["options"]
            sig = "model:modprobe-c-" + "-".join(kinds)
            if sig not in seen:
                seen.add(sig)
                p = save_input(ctx, component, f"cfg{i}", d)
                fails.append(failure(sig, component, p, f"missing={missing[:2]} extra={extra[:2]}"))
        shutil.rmtree(d, ignore_errors=True)
    return fails


# --------------------------------------------------------------------------------------------
# MODPROBE_OPTIONS tokenizer vs a reference quote-aware splitter (no escapes, like the docs say).
# --------------------------------------------------------------------------------------------
def _ref_split(s):
    out, cur, q, have = [], [], None, False
    for ch in s:
        if q:
            if ch == q:
                q = None
            else:
                cur.append(ch)
        elif ch in "\"'":
            q, have = ch, True
        elif ch == " ":
            if cur or have:
                out.append("".join(cur))
            cur, have = [], False
        else:
            cur.append(ch)
    if cur or have:
        out.append("".join(cur))
    return out


def modprobe_env_diff(ctx):
    component = "modprobe_env_diff"
    rnd = random.Random(ctx.seed)
    cases = ["-v", "-v -n", "a  b", " -v", "-v ", "'a b'", "\"a b\" c", "x'a b'y", "'a b'c", "a'b c'",
             "''", "'", "\"unterminated", "a 'b' c", "--first-time  -q", "''a", "a''", "'a''b'"]
    cases += ["".join(rnd.choice("ab '\"-") for _ in range(rnd.randint(1, 12))) for _ in range(3000)]
    rc, o, e = run([ENVTOK], stdin="\n".join(cases).encode(), timeout=120)
    f = _san_failure(component, rc, e, "", ctx)
    if f:
        return [f]
    fails, seen = [], set()
    for case, line in zip(cases, o.splitlines()):
        if case.count("'") % 2 or case.count('"') % 2:
            continue  # unbalanced quotes: no documented meaning, nothing to compare against
        got = line.split("\x1f")[1:] if "\x1f" in line else []
        exp = _ref_split(case)
        if got != exp:
            kind = ("empty-token" if "" in got and "" not in exp else
                    "quote-glued" if len(got) > len(exp) else
                    "unterminated-quote" if case.count("'") % 2 or case.count('"') % 2 else "other")
            if kind not in seen:
                seen.add(kind)
                p = save_input(ctx, component, f"{kind}.txt", case.encode())
                fails.append(failure(f"diff:modprobe-env-{kind}", component, p,
                                     f"env={case!r} modprobe={got} reference={exp}"))
    return fails


# --------------------------------------------------------------------------------------------
# Structured ELF mutations: boundary values in header/section-header fields + truncations.
# --------------------------------------------------------------------------------------------
BOUNDARY32 = [0, 1, 2, 0x7F, 0x80, 0xFF, 0xFFFF, 0x7FFFFFFF, 0x80000000, 0xFFFFFFFE, 0xFFFFFFFF]
BOUNDARY64 = BOUNDARY32 + [0x100000000, 0x7FFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF]


def _elf_variants(raw, rnd, limit):
    e = kelf.Elf(raw)
    out = []
    size = len(raw)
    for off, fmt in [(0x3A, "<H"), (0x3C, "<H"), (0x3E, "<H"), (0x28, "<Q")]:
        for v in [0, 1, 0xFFFF, 0x7FFF] + ([size, size - 64, 0xFFFFFFFFFFFFFFFF] if fmt == "<Q" else []):
            b = bytearray(raw)
            struct.pack_into(fmt, b, off, v & ((1 << (8 * struct.calcsize(fmt))) - 1))
            out.append((f"ehdr{off:x}={v:x}", bytes(b)))
    shoff, = struct.unpack_from("<Q", raw, 0x28)
    interesting = {".modinfo", "__versions", ".symtab", ".strtab", ".shstrtab", "__ksymtab_strings",
                   ".note.gnu.build-id", ".gnu.linkonce.this_module", "__ksymtab", "__kcrctab"}
    for idx, s in enumerate(e.sections):
        if s["name"] not in interesting:
            continue
        base = shoff + idx * 64
        for field, foff, fmt, vals in [("name", 0, "<I", BOUNDARY32), ("offset", 24, "<Q", BOUNDARY64 + [size, size - 1]),
                                       ("size", 32, "<Q", BOUNDARY64 + [size, size - s["offset"], size - s["offset"] + 1]),
                                       ("link", 40, "<I", [0, 0xFFFF, len(e.sections)]),
                                       ("entsize", 56, "<Q", [0, 1, 0xFFFFFFFFFFFFFFFF])]:
            for v in vals:
                b = bytearray(raw)
                struct.pack_into(fmt, b, base + foff, v & ((1 << (8 * struct.calcsize(fmt))) - 1))
                out.append((f"{s['name'].strip('.')}.{field}={v:x}", bytes(b)))
        if s["type"] != 8 and s["size"] > 0:
            b = bytearray(raw)
            end = s["offset"] + s["size"] - 1
            if end < len(b):
                b[end] = 0x41  # remove the section's trailing NUL terminator
                out.append((f"{s['name'].strip('.')}.unterminated", bytes(b)))
    for cut in sorted({s["offset"] + d for s in e.sections for d in (-1, 0, 1)} |
                      {int(size * k / 32) for k in range(1, 32)}):
        if 0 < cut < size:
            out.append((f"trunc{cut}", raw[:cut]))
    rnd.shuffle(out)
    return out[:limit]


def elf_sweep(ctx):
    component = "elf_sweep"
    wd = ctx.workdir(component)
    rnd = random.Random(ctx.seed)
    bases = [p for p in sample(real_modules(), 8 if ctx.long else 3, ctx.seed + 11)]
    bases += sorted((TB.parent / "__build-asan" / "testsuite" / "module-playground").glob("*.ko"))[:3]
    jobs = []
    for bi, p in enumerate(bases):
        raw = kelf.load_module_bytes(p)
        if raw[4] != 2 or raw[5] != 1:
            continue
        for name, data in _elf_variants(raw, rnd, 2000 if ctx.long else 350):
            f = wd / "cases" / f"b{bi}-{name}.ko"
            f.parent.mkdir(exist_ok=True)
            f.write_bytes(data)
            jobs.append(f)

    def one(f):
        rc, o, e = run([KINFO, f], timeout=30)
        return f, rc, e

    fails, seen = [], set()
    with ThreadPoolExecutor(max_workers=4) as ex:
        for f, rc, e in ex.map(one, jobs):
            fl = _san_failure(component, rc, e, f, ctx)
            if fl and fl["signature"] not in seen:
                seen.add(fl["signature"])
                fl["input"] = str(save_input(ctx, component, f.name, f))
                fails.append(fl)
    shutil.rmtree(wd / "cases", ignore_errors=True)
    return fails


# --------------------------------------------------------------------------------------------
# Corpus runner: every module of the real tree + the testsuite's crafted modules through kinfo.
# --------------------------------------------------------------------------------------------
def corpus_run(ctx):
    component = "corpus_run"
    mods = real_modules() if ctx.long else sample(real_modules(), 1500, ctx.seed + 5)
    mods += sorted((SRC / "testsuite" / "rootfs-pristine").rglob("*.ko*"))
    fails = []
    chunks = [mods[i:i + 200] for i in range(0, len(mods), 200)]

    def one(chunk):
        return chunk, run([KINFO], stdin="\n".join(map(str, chunk)).encode(), timeout=600)

    with ThreadPoolExecutor(max_workers=4) as ex:
        for chunk, (rc, o, e) in ex.map(one, chunks):
            f = _san_failure(component, rc, e, "", ctx)
            if f:
                last = o.rsplit("== ", 1)[-1].splitlines()[0] if "== " in o else chunk[0]
                f["input"] = str(save_input(ctx, component, Path(last).name, last))
                fails.append(f)
    return fails


# --------------------------------------------------------------------------------------------
# depmod on random synthetic module graphs (acyclic: dep sets vs transitive closure; cyclic: no crash/hang).
# --------------------------------------------------------------------------------------------
def depmod_graph(ctx):
    import synth
    component = "depmod_graph"
    wd = ctx.workdir(component)
    rnd = random.Random(ctx.seed + 11)
    fails = []
    for case in range(60 if ctx.long else 25):
        n = rnd.randint(2, 9)
        cyclic = case % 3 == 2
        edges = {i: set() for i in range(n)}
        for i in range(n):
            for j in range(n):
                if (j > i or (cyclic and j != i and rnd.random() < 0.3)) and rnd.random() < 0.4:
                    edges[i].add(j)
        root = wd / f"case{case}"
        shutil.rmtree(root, ignore_errors=True)
        kd = root / "lib" / "modules" / "1.0" / "kernel"
        kd.mkdir(parents=True)
        messy = case % 5 == 4
        for i in range(n):
            exports, imports = [f"s{i}"], [f"s{j}" for j in sorted(edges[i])]
            if messy:
                exports.append(f"s{rnd.randrange(n)}")
                imports.append(f"s{i}")
                imports.append("s_missing")
            (kd / f"m{i}.ko").write_bytes(synth.module(exports, imports))
        rc, out, err = run([ctx.tool("depmod"), "-b", root, "1.0"], timeout=60)
        label = "cyclic" if cyclic else "acyclic"
        f = _san_failure(component, rc, err, root, ctx, save_name=f"case{case}")
        if f:
            f["signature"] = f"{f['signature']}:{label}"
            fails.append(f)
            continue
        if rc == "timeout":
            fails.append(failure(f"timeout:depmod-{label}", component, save_input(ctx, component, f"case{case}", root),
                                 f"n={n}"))
            continue
        dep = root / "lib" / "modules" / "1.0" / "modules.dep"
        if not dep.exists():
            if not cyclic_has_cycle(edges) and not messy:
                fails.append(failure("depmod:no-modules.dep-acyclic", component,
                                     save_input(ctx, component, f"case{case}", root), err[-200:]))
            continue
        if cyclic_has_cycle(edges) or messy:
            continue
        got = {}
        for line in dep.read_text().splitlines():
            name, _, deps = line.partition(":")
            got[Path(name).name.split(".")[0]] = {Path(d).name.split(".")[0] for d in deps.split()}
        for i in range(n):
            want = {f"m{j}" for j in reach(edges, i)}
            if got.get(f"m{i}") != want:
                fails.append(failure("model:depmod-closure", component,
                                     save_input(ctx, component, f"case{case}", root),
                                     f"m{i}: got={sorted(got.get(f'm{i}', []))} want={sorted(want)}"))
                break
        shutil.rmtree(root, ignore_errors=True)
    return fails


def reach(edges, i):
    seen, todo = set(), list(edges[i])
    while todo:
        j = todo.pop()
        if j not in seen:
            seen.add(j)
            todo.extend(edges[j])
    return seen


def cyclic_has_cycle(edges):
    return any(i in reach(edges, i) for i in edges)


# --------------------------------------------------------------------------------------------
# Alias glob matching: modprobe --resolve-alias over depmod-built modules.alias.bin vs libc fnmatch(3).
# Keys avoid [ ] (alias_normalize rejects them by design) and '-' (normalized to '_'); duplicate hits are deduplicated.
# --------------------------------------------------------------------------------------------
def alias_glob(ctx):
    import ctypes
    import synth
    component = "alias_glob"
    wd = ctx.workdir(component)
    rnd = random.Random(ctx.seed + 23)
    libc = ctypes.CDLL(None)
    libc.fnmatch.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_int]
    atoms = ["a", "b", "c", ":", "_", "0", "1", "*", "?", "[ab]", "[a-c]", "[^a]", "[!b]", "[0-9]", "[a-]",
             "ab", "**", "*?"]
    plain = list("abc:_01*?") + ["ab", "pci:v"]
    fails = []
    for rnd_round in range(3 if ctx.long else 1):
        pats = ["".join(rnd.choice(atoms) for _ in range(rnd.randint(1, 6))) for _ in range(14)]
        root = wd / f"round{rnd_round}"
        shutil.rmtree(root, ignore_errors=True)
        kd = root / "lib" / "modules" / "1.0" / "kernel"
        kd.mkdir(parents=True)
        for i in range(0, len(pats), 2):
            (kd / f"m{i // 2}.ko").write_bytes(
                synth.module([f"s{i}"], [], modinfo=[f"alias={pats[i]}", f"alias={pats[i + 1]}"]))
        rc, _o, err = run([ctx.tool("depmod"), "-b", root, "1.0"], timeout=60)
        f = _san_failure(component, rc, err, root, ctx, save_name=f"round{rnd_round}")
        if f:
            fails.append(f)
            continue
        mods = {i // 2: (pats[i], pats[i + 1]) for i in range(0, len(pats), 2)}
        keys = {"".join(rnd.choice(plain) for _ in range(rnd.randint(0, 8))) for _ in range(40)}
        for k in sorted(keys):
            if not k or any(c in k for c in "\\\n"):
                continue
            rc, out, err = run([ctx.tool("modprobe"), "-d", root, "-S", "1.0", "--resolve-alias", "--", k], timeout=60)
            f = _san_failure(component, rc, err, root, ctx, save_name=f"round{rnd_round}")
            if f:
                fails.append(f)
                break
            want = sorted(f"m{i}" for i, ps in mods.items()
                          if any(libc.fnmatch(p.encode(), k.encode(), 0) == 0 for p in ps))
            got = sorted(set(out.split()))
            if got != want:
                fails.append(failure("diff:alias-glob-vs-fnmatch", component,
                                     save_input(ctx, component, f"round{rnd_round}", root),
                                     f"key={k!r} got={got} want={want} pats={mods}"))
                break
    return fails


# --------------------------------------------------------------------------------------------
# Allocation-failure sweep: fail each malloc/calloc/realloc/strdup of a run in turn (LD_PRELOAD shim, plain build).
# --------------------------------------------------------------------------------------------
def oom_sweep(ctx):
    import synth
    component = "oom_sweep"
    wd = ctx.workdir(component)
    plain, shim = TB / "__build-plain", TB / "__build-plain" / "failmalloc.so"
    root = wd / "root"
    shutil.rmtree(root, ignore_errors=True)
    kd = root / "lib" / "modules" / "1.0" / "kernel"
    kd.mkdir(parents=True)
    (root / "etc").mkdir()
    (kd / "foo.ko").write_bytes(synth.module(["sf"], ["sb"], modinfo=["alias=pci:v1*", "license=GPL"]))
    (kd / "bar.ko").write_bytes(synth.module(["sb"], [], modinfo=["license=GPL"]))
    (root / "etc" / "a.conf").write_text("softdep foo pre: bar\noptions foo x=1\ninstall baz /bin/true\nalias qq foo\n")
    subprocess.run([str(plain / "depmod"), "-b", str(root), "1.0"], capture_output=True)
    targets = {
        "modprobe-n": [plain / "modprobe", "-n", "-v", "-C", root / "etc", "-d", root, "-S", "1.0", "foo"],
        "modprobe-deps": [plain / "modprobe", "-C", root / "etc", "-d", root, "-S", "1.0", "--show-depends", "qq"],
        "modprobe-c": [plain / "modprobe", "-C", root / "etc", "-d", root, "-S", "1.0", "-c"],
        "modinfo": [plain / "modinfo", "-b", root, "-k", "1.0", "foo"],
        "depmod": [plain / "depmod", "-n", "-b", root, "1.0"],
    }
    fails = []
    for name, cmd in targets.items():
        env = dict(os.environ, LD_PRELOAD=str(shim), FAILMALLOC_COUNT="1")
        p = subprocess.run([str(c) for c in cmd], capture_output=True, text=True, env=env, timeout=60)
        m = re.search(r"FAILMALLOC_TOTAL=(\d+)", p.stderr)
        total = int(m.group(1)) if m else 0
        for n in range(1, total + 1, max(1, total // (1500 if ctx.long else 400) + 1)):
            env = dict(os.environ, LD_PRELOAD=str(shim), FAILMALLOC_N=str(n))
            try:
                r = subprocess.run([str(c) for c in cmd], capture_output=True, env=env, timeout=30)
            except subprocess.TimeoutExpired:
                fails.append(failure(f"oom:{name}:hang", component, root, f"alloc #{n}"))
                break
            if r.returncode < 0:
                fails.append(failure(f"oom:{name}:signal{-r.returncode}", component, root, f"alloc #{n} of {total}"))
                break
    return fails


def modprobe_order(ctx):
    """modprobe -n -v on random hard-dependency DAGs plus random softdep pre/post edges: order and uniqueness invariants."""
    component = "modprobe_order"
    wd = ctx.workdir(component)
    rnd = random.Random(ctx.seed + 31)
    fails = []
    for case in range(80 if ctx.long else 30):
        n = rnd.randint(3, 8)
        hard = {i: {j for j in range(i + 1, n) if rnd.random() < 0.3} for i in range(n)}
        pre = {i: set() for i in range(n)}
        post = {i: set() for i in range(n)}
        for i in range(n):
            for j in range(n):
                if i != j and rnd.random() < 0.12:
                    pre[i].add(j)
                if i != j and rnd.random() < 0.08:
                    post[i].add(j)
        for i in range(n):
            if hard[i] and rnd.random() < 0.4:
                pre[i].add(rnd.choice(sorted(hard[i])))
        root = wd / f"case{case}"
        shutil.rmtree(root, ignore_errors=True)
        kd = root / "lib" / "modules" / "1.0" / "kernel"
        kd.mkdir(parents=True)
        (root / "conf").mkdir()
        for i in range(n):
            (kd / f"m{i}.ko").write_bytes(synth.module([f"s{i}"], [f"s{j}" for j in sorted(hard[i])]))
        lines = []
        for i in range(n):
            if pre[i] or post[i]:
                lines.append(f"softdep m{i}" + (" pre: " + " ".join(f"m{j}" for j in sorted(pre[i])) if pre[i] else "")
                             + (" post: " + " ".join(f"m{j}" for j in sorted(post[i])) if post[i] else ""))
        (root / "conf" / "a.conf").write_text("\n".join(lines) + "\n")
        run([ctx.tool("depmod"), "-b", root, "1.0"], timeout=60)
        target = rnd.randrange(n)
        rc, out, err = run([ctx.tool("modprobe"), "-d", root, "-S", "1.0", "-C", root / "conf", "-n", "-v",
                            f"m{target}"], timeout=30)
        f = _san_failure(component, rc, err, root, ctx, save_name=f"case{case}")
        if f:
            fails.append(f)
            continue
        if rc == "timeout":
            fails.append(failure("timeout:modprobe-order", component, save_input(ctx, component, f"case{case}", root), ""))
            continue
        order = [Path(m.group(1)).name[1:-3] for m in re.finditer(r"insmod (\S+)", out)]
        order = [int(x) for x in order]
        closure = set()
        todo = [target]
        while todo:
            x = todo.pop()
            for d in hard[x] | pre[x] | post[x]:
                if d not in closure:
                    closure.add(d)
                    todo.append(d)
        def bad(sig, detail):
            fails.append(failure(sig, component, save_input(ctx, component, f"case{case}", root), detail))
        acyclic = not cyclic_has_cycle({x: hard[x] | pre[x] | {y for y in range(n) if x in post[y]} for x in range(n)})
        if acyclic and len(order) != len(set(order)):
            bad("model:modprobe-order-duplicate", f"target=m{target} order={order}")
            continue
        if target not in order and rc == 0:
            bad("model:modprobe-order-target-missing", f"target=m{target} order={order}")
            continue
        before = {x: hard[x] | pre[x] | {y for y in range(n) if x in post[y]} for x in range(n)}
        if cyclic_has_cycle({x: before[x] for x in range(n)}):
            shutil.rmtree(root, ignore_errors=True)
            continue
        pos = {m: k for k, m in enumerate(order)}
        for x in order:
            for d in hard[x]:
                if d in pos and pos[d] > pos[x]:
                    bad("model:modprobe-order-dep-after-dependent", f"m{d} after m{x} order={order}")
                    break
            for d in pre[x]:
                if d in pos and pos[d] > pos[x]:
                    bad("model:modprobe-order-pre-after-module", f"pre m{d} after m{x} order={order}")
                    break
        shutil.rmtree(root, ignore_errors=True)
    return fails


REGRESSION_SKIP = {"030", "031"}  # 030 is sensitive to 011's assert on this build, 031 needs a -Dmoduledir build


def regression(ctx):
    """Run every candidate's repro/run.sh against the tested HEAD build; a repro that stops reproducing means the bug was fixed upstream (see SKILL.md, CORRECTION.md)."""
    fails = []
    ws = TB.parent
    for script in sorted(ws.glob("0*/repro/run.sh")):
        num = script.parts[-3][:3]
        if num in REGRESSION_SKIP:
            continue
        rc, out, err = run(["bash", script, SRC, BUILD_GCC], timeout=240, cap_mb=3072)
        if rc == 0:
            fails.append(failure(f"regression:{num}:no-longer-reproduces", "regression", script,
                                 "repro exits 0 on the tested HEAD: fixed upstream? check git log and write a CORRECTION.md"))
        elif rc == "timeout":
            fails.append(failure(f"regression:{num}:timeout", "regression", script, "repro timed out"))
    return fails


ARCHS = [(64, False, 62), (64, True, 21), (32, False, 3), (32, True, 20), (32, True, 8)]


def arch_matrix(ctx):
    """Same module built as ELF32/ELF64, LE/BE: modprobe --show-modversions/--show-exports/modinfo and depmod's CRC check must agree with the generated values."""
    import synth_arch
    fails = []
    wd = ctx.workdir("arch_matrix")
    for bits, be, mach in ARCHS:
        tag = f"{bits}{'be' if be else 'le'}-{mach}"
        root = wd / tag
        k = root / "lib" / "modules" / "1.0" / "kernel"
        k.mkdir(parents=True, exist_ok=True)
        for f in ("modules.order", "modules.builtin", "modules.builtin.modinfo"):
            (k.parent / f).write_bytes(b"")
        exp, imp = [("foo", 0x11111111), ("bar", 0x22222222)], [("foo", 0x11111111), ("bar", 0x99999999), ("kf", 0x33333333)]
        kw = dict(bits=bits, be=be, machine=mach)
        (k / "a.ko").write_bytes(synth_arch.module(exp, [], ["license=GPL"], **kw))
        (k / "b.ko").write_bytes(synth_arch.module([], imp, ["license=GPL"], **kw))
        (root / "Module.symvers").write_text("0x33333333\tkf\tvmlinux\tEXPORT_SYMBOL\n")
        rc, out, err = run([BUILD_GCC / "modprobe", "--show-exports", k / "a.ko"])
        want = "".join(f"0x{c:08x}\t{n}\n" for n, c in exp)
        if out != want:
            fails.append(failure(f"arch:{tag}:exports", "arch_matrix", k / "a.ko", f"{out!r} != {want!r}"))
        rc, out, err = run([BUILD_GCC / "modprobe", "--show-modversions", k / "b.ko"])
        want = "".join(f"0x{c:08x}\t{n}\n" for n, c in imp)
        if out != want:
            fails.append(failure(f"arch:{tag}:modversions", "arch_matrix", k / "b.ko", f"{out!r} != {want!r}"))
        rc, out, err = run([BUILD_GCC / "depmod", "-e", "-E", root / "Module.symvers", "-b", root, "1.0"])
        warn = sorted(re.findall(r"disagrees about version of symbol (\w+)", err))
        if warn != ["bar"] or (k.parent / "modules.dep").read_text() != "kernel/b.ko: kernel/a.ko\nkernel/a.ko:\n":
            fails.append(failure(f"arch:{tag}:depmod", "arch_matrix", root, f"warn={warn}"))
    return fails


COMPONENTS = {
    "suite_clang": suite_clang, "suite_gcc": suite_gcc,
    "modinfo_diff": modinfo_diff, "signature_diff": signature_diff, "compress_diff": compress_diff,
    "depmod_model": depmod_model, "index_diff": index_diff, "cli_matrix": cli_matrix,
    "config_model": config_model, "modprobe_env_diff": modprobe_env_diff,
    "elf_sweep": elf_sweep, "corpus_run": corpus_run, "depmod_graph": depmod_graph, "alias_glob": alias_glob, "oom_sweep": oom_sweep,
    "modprobe_order": modprobe_order, "regression": regression, "arch_matrix": arch_matrix,
}
for _t in FUZZ_TARGETS:
    COMPONENTS[f"fuzz_{_t}"] = make_fuzz_component(_t)
