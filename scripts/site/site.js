(() => {
const app = document.getElementById('app');
const D = JSON.parse(document.getElementById('data').textContent);
const page = document.body.dataset.page;
const esc = s => String(s ?? '').replace(/[&<>"]/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;'}[c]));
const el = (html) => { const t = document.createElement('template'); t.innerHTML = html.trim(); return t.content.firstChild; };
const $ = (sel, root = app) => root.querySelector(sel);

const SEV = ['critical', 'high', 'medium', 'low', 'negligible'];
const SEV_COLOR = {critical: '#ef4444', high: '#f97316', medium: '#eab308', low: '#3b82f6', negligible: '#64748b'};
const SEV_CLS = {critical: 'c-red', high: 'c-orange', medium: 'c-amber', low: 'c-blue', negligible: 'c-slate'};

function catClass(cat) {
  const c = cat.toLowerCase();
  if (/strong/.test(c)) return 'c-green';
  if (/small/.test(c)) return 'c-teal';
  if (/security|unsound/.test(c)) return 'c-pink';
  if (/memory|ub\b|race|leak/.test(c)) return 'c-purple';
  if (/crash|panic|oom|memory-exhaust|overflow/.test(c)) return 'c-red';
  if (/logic/.test(c)) return 'c-amber';
  if (/build|tooling|test|doc/.test(c)) return 'c-blue';
  return 'c-slate';
}
const KIND_CLS = {panic: 'c-red', timeout: 'c-amber', 'out of memory': 'c-orange', contract: 'c-blue', SIGSEGV: 'c-pink',
  abort: 'c-purple', 'stack overflow': 'c-purple', 'exit status': 'c-slate'};
const kindClass = k => KIND_CLS[k] || 'c-slate';
const badge = (text, cls) => `<span class="badge ${cls}">${esc(text)}</span>`;
const plural = (n, w) => `${n} ${w}${n === 1 ? '' : 's'}`;

function chipGroup(values, counts, clsOf, onChange) {
  const off = new Set();
  const box = el('<div class="row"></div>');
  values.forEach(v => {
    const chip = el(`<span class="chip ${clsOf(v)}">${esc(v)}<small>${counts[v]}</small></span>`);
    chip.onclick = () => { off.has(v) ? off.delete(v) : off.add(v); chip.classList.toggle('off', off.has(v)); onChange(); };
    box.append(chip);
  });
  const all = el('<span class="btn">all</span>'), none = el('<span class="btn">none</span>');
  const setAll = on => { off.clear(); if (!on) values.forEach(v => off.add(v)); [...box.querySelectorAll('.chip')].forEach(c => c.classList.toggle('off', !on)); onChange(); };
  all.onclick = () => setAll(true); none.onclick = () => setAll(false);
  box.append(all, none);
  return {box, off};
}

/* ------------------------------ index ------------------------------ */
function renderIndex() {
  const P = D.projects;
  const sum = f => P.reduce((a, p) => a + f(p), 0);
  const kindsDef = [
    ['audit', 'Audit', 'var(--audit)', p => p.audit], ['bughunt', 'Bughunt', 'var(--bughunt)', p => p.bughunt],
    ['perf', 'Perf', 'var(--perf)', p => p.perf], ['fuzz', 'Fuzz', 'var(--fuzz)', p => p.fuzz]];
  const bugs = p => (p.bughunt || []).reduce((a, t) => a + t.n, 0);
  const wins = p => (p.perf || []).reduce((a, t) => a + t.n, 0);
  const metrics = {
    name: p => p.name.toLowerCase(), audit_date: p => p.audit ? p.audit.date : '', audit_n: p => p.audit ? p.audit.n : 0,
    bugs, wins, fuzz_n: p => p.fuzz ? p.fuzz.n : 0,
    severe: p => p.audit ? (p.audit.sev.critical || 0) * 1000 + (p.audit.sev.high || 0) : 0,
    total: p => (p.audit ? p.audit.n : 0) + bugs(p) + wins(p) + (p.fuzz ? p.fuzz.n : 0)};
  const sorts = [['audit_date', 'Newest audit'], ['name', 'Name'], ['audit_n', 'Audit findings'], ['severe', 'Severe audit findings'],
    ['bugs', 'Confirmed bugs'], ['wins', 'Perf wins'], ['fuzz_n', 'Fuzz crashes'], ['total', 'Total findings']];
  app.innerHTML = `
    <div class="hero"><div><h1>Claude Code audit reports</h1>
      <div class="sub">Static audits, confirmed bug hunts, measured optimizations and fuzz crashes - one entry per project.
      Source: <a href="${D.repo}">qarmin/ClaudeReports</a></div></div></div>
    <div class="tiles">
      <div class="tile" style="--c:#f1f5f9"><b>${P.length}</b><span>Projects</span></div>
      <div class="tile" style="--c:var(--audit)"><b>${sum(p => p.audit ? p.audit.n : 0)}</b><span>Audit findings, ${sum(p => p.audit ? 1 : 0)} reports</span></div>
      <div class="tile" style="--c:var(--bughunt)"><b>${sum(bugs)}</b><span>Confirmed bugs, ${sum(p => p.bughunt ? 1 : 0)} projects</span></div>
      <div class="tile" style="--c:var(--perf)"><b>${sum(wins)}</b><span>Perf wins, ${sum(p => p.perf ? 1 : 0)} projects</span></div>
      <div class="tile" style="--c:var(--fuzz)"><b>${sum(p => p.fuzz ? p.fuzz.n : 0)}</b><span>Fuzz crashes, ${sum(p => p.fuzz ? 1 : 0)} targets</span></div></div>
    <div class="bar">
      <div class="row"><input type="search" id="q" placeholder="Search projects and descriptions..." autofocus>
        <select id="sort">${sorts.map(([k, l]) => `<option value="${k}">Sort: ${l}</option>`).join('')}</select>
        <select id="dir"><option value="-1">descending</option><option value="1">ascending</option></select></div>
      <div class="row"><span class="lbl">Has</span><span id="has"></span><span class="lbl" style="margin-left:1rem">Severity</span><span id="sevf"></span></div></div>
    <div class="count" id="count"></div><div class="grid" id="grid"></div>`;
  const active = new Set();
  let sev = null;
  const has = $('#has');
  kindsDef.forEach(([k, label, color]) => {
    const chip = el(`<span class="chip tog" style="--c:${color}">${label}</span>`);
    chip.onclick = () => { active.has(k) ? active.delete(k) : active.add(k); chip.classList.toggle('on'); draw(); };
    has.append(chip);
  });
  const sevf = $('#sevf');
  SEV.slice(0, 3).forEach(s => {
    const chip = el(`<span class="chip tog" style="--c:${SEV_COLOR[s]}">audit has ${s}</span>`);
    chip.onclick = () => { sev = sev === s ? null : s; [...sevf.children].forEach(c => c.classList.remove('on')); if (sev) chip.classList.add('on'); draw(); };
    sevf.append(chip);
  });

  const slot = (key, label, color, inner) => inner
    ? `<div class="slot" style="--c:${color}"><span class="k">${label}</span>${inner}</div>`
    : `<div class="slot empty" style="--c:${color}"><span class="k">${label}</span><span class="meta">-</span></div>`;
  const pills = (counts, clsOf) => `<div class="pills">${Object.entries(counts).sort((a, b) => b[1] - a[1]).slice(0, 4)
    .map(([k, n]) => `<span class="badge ${clsOf(k)}">${esc(k)} ${n}</span>`).join('')}</div>`;
  const seg = (counts, order, colorOf) => {
    const total = order.reduce((a, k) => a + (counts[k] || 0), 0) || 1;
    return `<div class="seg">${order.filter(k => counts[k]).map(k => `<i style="width:${counts[k] / total * 100}%;background:${colorOf(k)}" title="${k}: ${counts[k]}"></i>`).join('')}</div>`;
  };
  const card = p => {
    const a = p.audit;
    const audit = a && slot('audit', 'Audit', 'var(--audit)',
      `<a class="main" href="${a.url}">${a.n} findings</a><span class="meta">${a.date}${a.older.length ? ` · +${a.older.length} older` : ''}</span>${seg(a.sev, SEV, k => SEV_COLOR[k])}`);
    const group = (kind, label, color, cats, clsOf) => {
      const t = p[kind]; if (!t) return slot(kind, label, color, '');
      const cc = {}; t.forEach(x => Object.entries(x.cats).forEach(([c, n]) => cc[c] = (cc[c] || 0) + n));
      const links = t.map(x => `<a class="main" href="${x.url}">${x.n} ${cats}${x.sub ? ` <span class="meta">(${esc(x.sub)})</span>` : ''}</a>`).join('');
      return slot(kind, label, color, links + pills(cc, clsOf));
    };
    const f = p.fuzz;
    const fuzz = f ? slot('fuzz', 'Fuzz', 'var(--fuzz)',
      `<a class="main" href="${f.url}">${f.n} crashes${f.reported ? ' <span class="meta">(reported)</span>' : ''}</a>${pills(f.kinds, kindClass)}`) : slot('fuzz', 'Fuzz', 'var(--fuzz)', '');
    return `<div class="card"><h3><a href="${p.overview}">${esc(p.name)}</a>${p.repo ? `<a class="meta" href="${esc(p.repo)}" style="font-size:.7rem;color:var(--dim)">repo</a>` : ''}</h3>
      <div class="desc">${esc(p.desc || '')}</div>
      <div class="slots">${audit || slot('audit', 'Audit', 'var(--audit)', '')}${group('bughunt', 'Bughunt', 'var(--bughunt)', 'bugs', catClass)}${group('perf', 'Perf', 'var(--perf)', 'wins', catClass)}${fuzz}</div></div>`;
  };
  function draw() {
    const q = $('#q').value.toLowerCase(), key = $('#sort').value, dir = +$('#dir').value;
    const list = P.filter(p => [...active].every(k => p[k]))
      .filter(p => !sev || (p.audit && p.audit.sev[sev]))
      .filter(p => !q || (p.name + ' ' + (p.desc || '')).toLowerCase().includes(q));
    list.sort((x, y) => { const a = metrics[key](x), b = metrics[key](y); return (a < b ? -1 : a > b ? 1 : 0) * dir || x.name.localeCompare(y.name); });
    $('#count').textContent = `${list.length} of ${P.length} projects`;
    $('#grid').innerHTML = list.map(card).join('') || '<div class="empty-msg">No project matches.</div>';
  }
  ['#q', '#sort', '#dir'].forEach(s => $(s).addEventListener('input', draw));
  draw();
}

/* ----------------------------- detail ------------------------------ */
function header(extra) {
  const t = D.tabs, defs = [['overview', 'Overview', '#f1f5f9'], ['audit', 'Audit', 'var(--audit)'], ['bughunt', 'Bughunt', 'var(--bughunt)'], ['perf', 'Perf', 'var(--perf)'], ['fuzz', 'Fuzz', 'var(--fuzz)']];
  const root = document.body.dataset.root;
  const tabs = defs.map(([k, l, c]) => {
    const href = t[k];
    return `<a class="tab ${href ? '' : 'none'} ${k === page || (k === 'overview' && page === 'project') ? 'on' : ''}" style="--c:${c}" href="${href ? root + href : '#'}">${l}</a>`;
  }).join('');
  return `<div class="hero"><div><div class="sub"><a href="${root}index.html">&larr; all projects</a></div>
    <h1>${esc(D.name)}</h1><div class="sub">${esc(D.desc || '')}${D.repo ? ` <a href="${esc(D.repo)}">${esc(D.repo.replace('https://', ''))}</a>` : ''}</div></div>
    <div class="hdrlinks">${extra}</div></div><div class="tabs">${tabs}</div>`;
}
const linkBtn = (text, url) => url ? `<a class="btn" href="${esc(url)}">${esc(text)}</a>` : '';

function sortSelect(options) { return `<select id="sort">${options.map(([k, l]) => `<option value="${k}">Sort: ${l}</option>`).join('')}</select>
  <select id="dir"><option value="1">ascending</option><option value="-1">descending</option></select>`; }

function renderTable() {
  const rows = D.rows, label = page === 'perf' ? 'wins' : 'bugs';
  const sibling = (D.sub ? `<span class="badge c-slate">${esc(D.sub)}</span>` : '');
  const cats = {}; rows.forEach(r => cats[r.cat] = (cats[r.cat] || 0) + 1);
  const added = rows.reduce((a, r) => a + (r.fix ? r.fix[0] : 0), 0), removed = rows.reduce((a, r) => a + (r.fix ? r.fix[1] : 0), 0);
  const color = page === 'perf' ? 'var(--perf)' : 'var(--bughunt)';
  app.innerHTML = header(Object.entries(D.files).map(([k, u]) => linkBtn(k, u)).join('') + linkBtn('Workspace on GitHub', D.workspace)) + `
    ${sibling}
    <div class="tiles"><div class="tile" style="--c:${color}"><b>${rows.length}</b><span>${page === 'perf' ? 'Optimizations' : 'Confirmed bugs'}</span></div>
      ${Object.entries(cats).sort((a, b) => b[1] - a[1]).slice(0, 4).map(([c, n]) => `<div class="tile"><b>${n}</b><span>${esc(c)}</span></div>`).join('')}
      <div class="tile" style="--c:#4ade80"><b class="mono" style="font-size:1.1rem">+${added} <span style="color:#f87171">-${removed}</span></b><span>Fix lines total</span></div></div>
    ${D.intro ? `<details class="about" ${rows.length ? '' : 'open'}><summary>About this summary</summary><div class="md">${D.intro}${D.outro}</div></details>` : ''}
    ${rows.length ? `<div class="bar"><div class="row"><input type="search" id="q" placeholder="Search ${label}...">
      ${sortSelect([['id', '#'], ['cat', 'Category'], ['fix', 'Fix size'], ['title', 'Title']])}
      <span class="btn" id="expand">expand all</span></div><div class="row" id="cats"><span class="lbl">Category</span></div></div>
    <div class="count" id="count"></div><div id="list"></div>` : ''}`;
  if (!rows.length) return;
  const {box, off} = chipGroup(Object.keys(cats).sort((a, b) => cats[b] - cats[a]), cats, catClass, draw);
  $('#cats').append(...box.children);
  const fixSize = r => r.fix ? r.fix[0] + r.fix[1] : 0, num = r => parseInt(r.id) || 0;
  const keyOf = {id: num, cat: r => r.cat, fix: fixSize, title: r => r.title.replace(/<[^>]+>/g, '').toLowerCase()};
  function draw() {
    const q = $('#q').value.toLowerCase(), k = $('#sort').value, dir = +$('#dir').value;
    const list = rows.filter(r => !off.has(r.cat)).filter(r => !q || (r.title + r.extra.join(' ') + r.cat).toLowerCase().includes(q.trim()));
    list.sort((a, b) => { const x = keyOf[k](a), y = keyOf[k](b); return (x < y ? -1 : x > y ? 1 : 0) * dir || num(a) - num(b); });
    $('#count').textContent = `${list.length} of ${rows.length}`;
    $('#list').innerHTML = list.map((r, i) => `<div class="item" data-i="${i}"><div class="head"><span class="id">#${esc(r.id)}</span>${badge(r.cat, catClass(r.cat))}
      <span class="ttl">${r.title}</span><span class="fix">${r.fix ? `<span class="a">+${r.fix[0]}</span> <span class="d">-${r.fix[1]}</span>` : ''}</span><span class="caret">&#9656;</span></div>
      <div class="body"><div class="kv">${D.columns.map((c, j) => `<span class="k">${esc(c)}</span><span>${r.extra[j] || ''}</span>`).join('')}</div>
      ${r.url ? linkBtn('Candidate folder: patch, repro, RESULT.md', r.url) : ''}</div></div>`).join('') || '<div class="empty-msg">Nothing matches.</div>';
  }
  $('#list').addEventListener('click', e => { const h = e.target.closest('.head'); if (h && !e.target.closest('a')) h.parentElement.classList.toggle('open'); });
  let open = false; $('#expand').onclick = e => { open = !open; e.target.textContent = open ? 'collapse all' : 'expand all'; document.querySelectorAll('.item').forEach(i => i.classList.toggle('open', open)); };
  ['#q', '#sort', '#dir'].forEach(s => $(s).addEventListener('input', draw));
  draw();
}

function renderFuzz() {
  const C = D.crashes, PAGE = 60;
  const kinds = {}; C.forEach(c => kinds[c.kind] = (kinds[c.kind] || 0) + 1);
  const miri = C.filter(c => c.miri === 'clean').length;
  const files = new Set(C.map(c => c.loc.replace(/:\d+(:\d+)?$/, ''))).size;
  app.innerHTML = header(Object.entries(D.files).map(([k, u]) => linkBtn(k, u)).join('') + linkBtn('Reproducers on GitHub', D.workspace + '/crashes')) + `
    <div class="sub" style="margin-bottom:.8rem">${D.source ? `<a href="${esc(D.source.replace(/\.git$/, ''))}">${esc(D.source)}</a>` : ''} ${D.rev ? `@ <code>${esc(D.rev.slice(0, 12))}</code>` : ''}
      ${D.reported ? ' ' + badge('reported upstream', 'c-green') : ''}</div>
    <div class="tiles"><div class="tile" style="--c:var(--fuzz)"><b>${C.length}</b><span>Distinct crashes</span></div>
      ${Object.entries(kinds).sort((a, b) => b[1] - a[1]).slice(0, 4).map(([k, n]) => `<div class="tile"><b>${n}</b><span>${esc(k)}</span></div>`).join('')}
      <div class="tile"><b>${files}</b><span>Source files hit</span></div><div class="tile" style="--c:#4ade80"><b>${miri}</b><span>Miri-confirmed clean-run</span></div></div>
    <div class="bar"><div class="row"><input type="search" id="q" placeholder="Search message, location, API call, code...">
      ${sortSelect([['kind', 'Kind'], ['loc', 'Location'], ['api', 'API call'], ['msg', 'Message']])}</div>
      <div class="row" id="kinds"><span class="lbl">Kind</span></div></div>
    <div class="count" id="count"></div><div id="list"></div><span class="btn more" id="more" hidden>show more</span>`;
  const {box, off} = chipGroup(Object.keys(kinds).sort((a, b) => kinds[b] - kinds[a]), kinds, kindClass, () => { shown = PAGE; draw(); });
  $('#kinds').append(...box.children);
  let shown = PAGE, list = [];
  const first = s => (s || '').split('\n')[0];
  function draw() {
    const q = $('#q').value.toLowerCase().trim(), k = $('#sort').value, dir = +$('#dir').value;
    list = C.filter(c => !off.has(c.kind)).filter(c => !q || (c.msg + c.loc + c.api + c.code + c.kind).toLowerCase().includes(q));
    list.sort((a, b) => { const x = (a[k] || '').toLowerCase(), y = (b[k] || '').toLowerCase(); return (x < y ? -1 : x > y ? 1 : 0) * dir; });
    $('#count').textContent = `${list.length} of ${C.length}`;
    $('#list').innerHTML = list.slice(0, shown).map((c, i) => `<div class="item fz" data-i="${i}"><div class="head">${badge(c.kind, kindClass(c.kind))}
      <span class="ttl">${esc(first(c.msg)) || '<i>no message</i>'}<span class="api">${esc(c.api)}</span></span>
      <span class="loc">${esc(c.loc)}${c.miri === 'clean' ? ' ' + badge('miri', 'c-green') : ''}</span><span class="caret">&#9656;</span></div>
      <div class="body"></div></div>`).join('') || '<div class="empty-msg">Nothing matches.</div>';
    $('#more').hidden = list.length <= shown;
    $('#more').textContent = `show more (${list.length - shown} left)`;
  }
  $('#list').addEventListener('click', e => {
    const h = e.target.closest('.head'); if (!h) return;
    const item = h.parentElement, body = $('.body', item);
    if (!body.innerHTML) {
      const c = list[+item.dataset.i];
      body.innerHTML = `<div class="kv"><span class="k">Location</span><span class="loc">${esc(c.loc)}</span><span class="k">Last API call</span><span class="loc">${esc(c.api)}</span>
        <span class="k">Message</span><span><pre>${esc(c.msg)}</pre></span></div>
        <div class="codewrap"><span class="btn">copy</span><pre>${esc(c.code)}</pre></div>
        ${c.trace ? `<details><summary class="sub" style="cursor:pointer">Trimmed backtrace</summary><pre>${esc(c.trace)}</pre></details>` : ''}
        <div style="margin-top:.5rem">${linkBtn('Reproducer on GitHub', c.url)}</div>`;
      $('.codewrap .btn', body).onclick = ev => { navigator.clipboard.writeText(c.code); ev.target.textContent = 'copied'; };
    }
    item.classList.toggle('open');
  });
  $('#more').onclick = () => { shown += PAGE; draw(); };
  ['#q', '#sort', '#dir'].forEach(s => $(s).addEventListener('input', () => { shown = PAGE; draw(); }));
  draw();
}


/* ----------------------------- project ----------------------------- */
function renderProject() {
  const A = D.audit, F = D.fuzz, info = D.commit || {};
  const root = document.body.dataset.root;
  const dist = (counts, order, clsOf) => Object.entries(counts).sort((a, b) => order ? order.indexOf(a[0]) - order.indexOf(b[0]) : b[1] - a[1])
    .map(([k, n]) => `<span class="badge ${clsOf(k)}">${esc(k)} ${n}</span>`).join(' ');
  const panel = (title, color, inner) => `<section class="panel" style="--c:${color}"><h2>${title}</h2>${inner}</section>`;
  const links = [D.repo && linkBtn('Upstream repo', D.repo), info.commit_url && linkBtn(`Audited commit ${info.commit_short}`, info.commit_url)].filter(Boolean).join('');
  const sections = [];
  if (A) {
    const sevBar = SEV.filter(s => A.sev[s]).map(s => `<i style="width:${A.sev[s] / A.n * 100}%;background:${SEV_COLOR[s]}" title="${s}: ${A.sev[s]}"></i>`).join('');
    sections.push(panel('Static audit', 'var(--audit)', `
      <div class="sub">Generated ${A.date}${info.branch ? ` on <code>${esc(info.branch)}</code>` : ''}${info.commit_short ? ` @ <code>${esc(info.commit_short)}</code>` : ''} · ${A.kb} KB · ${A.n} findings</div>
      <div class="seg big">${sevBar}</div>
      <div class="kv"><span class="k">Severity</span><span>${dist(A.sev, SEV, s => SEV_CLS[s] || 'c-slate')}</span>
        <span class="k">Difficulty</span><span>${dist(A.diff, ['very-easy', 'easy', 'medium', 'hard'], d => ({easy: 'c-green', 'very-easy': 'c-green', medium: 'c-amber', hard: 'c-red'})[d] || 'c-slate')}</span>
        <span class="k">Type</span><span>${dist(A.types, null, () => 'c-slate')}</span>
        ${A.older.length ? `<span class="k">Older reports</span><span>${A.older.map(o => `<a href="${root}${o.url}">${o.date}</a>`).join(' · ')}</span>` : ''}</div>
      <h3>Most severe findings</h3>
      ${A.top.map(f => `<div class="mini">${badge(f.sev, SEV_CLS[f.sev] || 'c-slate')} <span class="mono id">${esc(f.id)}</span> ${f.diff ? badge(f.diff, 'c-slate') : ''}<div class="md">${esc(f.text)}${f.text.length >= 260 ? '...' : ''}</div></div>`).join('')}
      <div style="margin-top:.7rem">${linkBtn('Open full interactive report', root + A.url)} ${linkBtn('Stable link: latest/' + D.name + '.html', root + 'latest/' + D.name + '.html')}</div>`));
  }
  [['bughunt', 'Bughunt - confirmed bugs', 'var(--bughunt)'], ['perf', 'Optimizations', 'var(--perf)']].forEach(([k, title, color]) => {
    (D[k] || []).forEach(t => {
      const shown = t.rows.slice(0, 14);
      sections.push(panel(`${title}${t.sub ? ` <span class="badge c-slate">${esc(t.sub)}</span>` : ''} <small>${t.n}</small>`, color, `
        <div class="row" style="margin-bottom:.5rem">${dist(t.cats, null, catClass)}</div>
        ${shown.map(r => `<div class="mini"><span class="mono id">#${esc(r.id)}</span> ${badge(r.cat, catClass(r.cat))} <span>${r.title}</span>
          ${r.fix ? `<span class="fix"><span class="a">+${r.fix[0]}</span> <span class="d">-${r.fix[1]}</span></span>` : ''}</div>`).join('')}
        ${t.rows.length > shown.length ? `<div class="sub">... and ${t.rows.length - shown.length} more</div>` : ''}
        <div style="margin-top:.7rem">${linkBtn('Browse all ' + t.n + ' with details', root + t.url)} ${linkBtn('Workspace on GitHub', t.workspace)}</div>`));
    });
  });
  if (F) {
    sections.push(panel('Fuzzing (rfuzz)', 'var(--fuzz)', `
      <div class="sub">${F.source ? `<a href="${esc(F.source.replace(/\.git$/, ''))}">${esc(F.source)}</a>` : ''}${F.rev ? ` @ <code>${esc(F.rev.slice(0, 12))}</code>` : ''}
        ${F.reported ? badge('reported upstream', 'c-green') : ''} · ${F.n} crashes, ${F.miri} confirmed by Miri</div>
      <div class="row" style="margin:.5rem 0">${dist(F.kinds, null, kindClass)}</div>
      <h3>Most crashing source files</h3>
      ${F.files.map(([f, n]) => `<div class="mini"><span class="badge c-slate">${n}</span> <span class="loc">${esc(f)}</span></div>`).join('')}
      <div style="margin-top:.7rem">${linkBtn('Browse all crashes with reproducers', root + F.url)} ${linkBtn('Reproducers on GitHub', F.workspace + '/crashes')}</div>`));
  }
  app.innerHTML = header(links) + `<div class="cols">${sections.join('') || '<div class="empty-msg">No material for this project.</div>'}</div>`;
}

({index: renderIndex, project: renderProject, bughunt: renderTable, perf: renderTable, fuzz: renderFuzz})[page]();
})();
