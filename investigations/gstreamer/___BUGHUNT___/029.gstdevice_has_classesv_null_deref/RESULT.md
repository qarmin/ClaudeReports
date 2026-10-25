# gstdevice_has_classesv_null_deref

`subprojects/gstreamer/gst/gstdevice.c`'s `gst_device_has_classesv()` (~line 356-385) calls
`strstr(device->priv->device_class, klass)` with no NULL check on `device_class`. The candidate
(findings.toml PANIC_19) hypothesized this is reachable because `device-class` is
`G_PARAM_CONSTRUCT_ONLY` (not `G_PARAM_CONSTRUCT`), so a `GstDevice` subclass that never
explicitly sets it would leave the field NULL.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Isolated minimal-GObject test (bare `GObject` subclass, one `G_PARAM_CONSTRUCT_ONLY` string property with default `""`, constructed via `g_object_new()` without passing the property) | Plain gcc, system `gobject-2.0` | `set_property` IS called during construction with the pspec's default value (`""`), even though the property is `CONSTRUCT_ONLY` not `CONSTRUCT`. The field ends up `""`, not NULL. Refutes the candidate's stated trigger (mere omission). |
| Real `GstDevice` subclass (`MyDevice`), public `g_object_new()`, **omitting** `"device-class"` entirely | System `libgstreamer-1.0` 1.28.2 via pkg-config, plain build | `gst_device_get_device_class()` returns `""`; `gst_device_has_classesv()` returns `FALSE`, no crash. Confirms the negative control - omission alone does not reproduce. |
| Real `GstDevice` subclass, public `g_object_new()`, **explicitly** passing `"device-class", NULL` | Same, then rebuilt against the actual project's own core (see below) | SEGV inside `gst_device_has_classesv()` at the `strstr()` call, `device->priv->device_class == NULL`. Reproducible 5/5 runs against system gstreamer. |
| Same repro rebuilt and linked against this project's own `subprojects/gstreamer` core (gstreamer-only meson build, `--buildtype=debugoptimized -Db_sanitize=address,undefined`, HEAD 65b9547519), run under clang ASAN+UBSAN | Real project source, clang ASAN/UBSAN | UBSan: "null pointer passed as argument 1, which is declared to never be null" at `gstdevice.c:372:13`, immediately followed by ASAN SEGV in `strstr`/`gst_device_has_classesv`. Reproducible 3/3 runs. |
| Variation (CONFIRMATION.md section 7): `classes` array with a leading empty-string entry before the real class name (exercises the loop reaching `strstr()` on the second iteration), plus `gst_device_has_classes()` (the public `"/"`-joined wrapper) | Same ASAN/UBSAN build | Same crash location, same signature - confirms the bug isn't specific to the exact first-triggering input/entry point. |

## Status

**Confirmed**, but the root cause differs from the candidate's original hypothesis - see "Root
cause" below. Reproduces deterministically: 3/3 runs against the project's own ASAN/UBSAN build of
`subprojects/gstreamer` core at HEAD 65b9547519, plus 5/5 against system libgstreamer-1.0 1.28.2
(same code path, same crash site), plus the negative-control test (omission-only, 3/3 runs) showing
no crash, correctly distinguishing the real trigger from the originally-hypothesized one.

**Fully reproducible**: **Yes**. The repro (`repro/repro.c`) exercises the real, public
`gst_device_has_classesv()` entry point on a `GstDevice` constructed through the ordinary public
`g_object_new()` API a real device-provider/subclass author would use - no internal struct field is
touched directly, and no second participant (peer/caller) needs to be faked here: the "attacker
input" is simply the caller's own choice of property values, i.e. a single-process, single-API-call
scenario, not an interaction between two participants (CONFIRMATION.md section 11's single-artifact
exception applies).

## Class

crash (NULL-pointer dereference / UB per UBSan's own diagnostic)

## Root cause

The candidate's stated mechanism is **false**: `device-class` being `G_PARAM_CONSTRUCT_ONLY`
(instead of `G_PARAM_CONSTRUCT`) does **not** leave the field NULL when a subclass simply omits it
from `g_object_new()`. GObject's construction machinery places every `G_PARAM_CONSTRUCT` **and**
`G_PARAM_CONSTRUCT_ONLY` property into the class's `construct_properties` list; for any such
property not explicitly supplied to `g_object_new()`, GObject invokes the setter with the pspec's
*default* value. Since `device-class`'s pspec was installed with default value `""` (see
`g_param_spec_string ("device-class", "Device Class", "The Class of the device", "", ...)` in
`gst_device_class_init()`), an omitted `device-class` construction results in
`gstdevice->priv->device_class = g_strdup("")` - non-NULL. This was verified both with an isolated
minimal-GObject test and with a real `GstDevice` subclass.

The **actual** trigger is different: a caller/subclass author that constructs a `GstDevice` (or
subclass) via `g_object_new(..., "device-class", NULL, ...)` - i.e. explicitly passing a NULL value
for the string-typed `"device-class"` construct property - does leave `priv->device_class == NULL`,
because `gst_device_set_property()`'s `PROP_DEVICE_CLASS` case unconditionally does
`gstdevice->priv->device_class = g_value_dup_string (value)`, and `g_value_dup_string()` on a
GValue holding a NULL string returns NULL. Nothing in `GObject`'s property-validation machinery
rejects passing NULL for a plain `G_TYPE_STRING` property (there is no "not-null" property flag in
this GLib), so this is a normal, permitted use of a completely ordinary public API - not a
contrived edge case. `gst_device_has_classesv()` then calls `strstr(NULL, klass)` for any non-empty
`klass` entry, which is undefined behavior and crashes in practice (glibc's `strstr`/`strchr`
implementations dereference the haystack unconditionally).

None of the ~20 in-tree call sites that construct a `GstDevice`/subclass (grepped across
`gst-plugins-{good,bad}` device providers) pass NULL explicitly for `"device-class"` - they all pass
a real string. So this bug is not known to be triggered by any in-tree code path today, but it is
directly reachable by any third-party `GstDeviceProvider`/`GstDevice` subclass (in-tree or
out-of-tree) that passes a possibly-NULL string through to this construct property (e.g. an
optional config value that wasn't validated before being forwarded).

`gst_device_get_device_class()` (the sibling getter, a few lines above) already defensively checks
`device->priv->device_class != NULL` before using it - `gst_device_has_classesv()` is the one path
in this file that doesn't, which is the actual defect.

## Introduced by

e743fac26bd5f36268bf1035f33ab900d956b1e3 "device: Add \"klass\" to GstDevices" (2014-03-16) - the
commit that introduced both `gst_device_has_classesv()` and the `"device-class"`/`device_class`
property/field, with the unchecked `strstr()` call present from the start (confirmed via `git
blame` - line 372's `strstr()` call itself was later touched cosmetically by
05a9655523 "device: fix comparison in _has_classesv()" (2016-01-21), but the missing NULL check
predates and survives that change unchanged).

## The fix

Added a NULL check on `device->priv->device_class` inside the loop, returning `FALSE` for any
non-empty `klass` when `device_class` is NULL (mirroring `gst_device_get_device_class()`'s own
existing NULL handling, which treats a NULL internal value as "no classes"):

```c
    if (*klass == '\0')
      continue;

+    if (!device->priv->device_class)
+      return FALSE;
+
    found = strstr (device->priv->device_class, klass);
```

Verified:
- `repro/repro.c` (explicit `"device-class", NULL`): crashes on pristine HEAD 65b9547519 (3/3 runs,
  ASAN+UBSAN), no longer crashes after the patch + rebuild (3/3 runs), correctly returns `FALSE`.
- Variation input (leading empty-string `classes` entry + `gst_device_has_classes()` wrapper):
  crashes on pristine, no longer crashes after the patch (per CONFIRMATION.md section 7 - not just
  the exact original input).
- Project test suite (`meson test -C build-029 --suite gstreamer`, gstreamer-core-only ASAN/UBSAN
  build): **101 pass / 8 fail / 1 skipped, identical before and after** the patch (`gst_gstbuffer`,
  `gst_gstutils`, `gst_gstvecdeque`, `libs_bytereader`, `libs_bytereader_noinline`,
  `libs_bytewriter`, `libs_bytewriter_noinline`, `libs_queuearray` fail both before and after -
  these are pre-existing failures under this ASAN/UBSAN build unrelated to `gstdevice.c`, not
  caused or fixed by this patch). `gst_gstdevice`'s own check suite (`tests/check/gst/gstdevice.c`)
  passes both before and after (0 tests failed).

## What's not done / open questions

- Not checked whether any other `GstDevice`-derived getter/method elsewhere in the codebase has the
  same unchecked-`device_class` pattern - a scan of `gstdevice.c` itself shows
  `gst_device_get_device_class()` is the only other consumer, and it already NULL-checks.
- Not checked whether other `GObject`-based GStreamer classes have similarly-unchecked
  `G_PARAM_CONSTRUCT_ONLY` string properties that could be explicitly NULL-constructed the same
  way - this is a narrow, single-function fix, not a sweep of the pattern.
- The realistic exposure is third-party/out-of-tree `GstDevice` subclasses, not any code currently
  in this repository - worth noting to maintainers when deciding whether this is worth a defensive
  fix at all versus documenting `"device-class"` as non-nullable.
