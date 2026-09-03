# Correction (candidate 070, 2026-08-24)

This candidate's second checked fix shape ("compute NoWrap `Orientation::Vertical` height from
font metrics instead of shaping") was dismissed with: "the horizontal branch already forces the
same shape unconditionally (candidate 044) and shares the exact same `TextLayoutCache` key as the
vertical branch's `NoWrap` call, so there is never an unshaped item left for a vertical-only fast
path to catch."

That reasoning holds for a `Text` hosted in a `HorizontalLayout` (candidate 044's own scene), where
`Orientation::Horizontal` really is queried on every child regardless of stretch/elide, and so a
`Vertical` query afterwards always hits the cache the `Horizontal` query already populated. It does
**not** hold for the symmetric case - a `Text` hosted in a `VerticalLayout` - which this candidate's
own scope never checked. Candidate 070 measured directly (a temporary orientation-call counter):

```
VerticalLayout, fixed-width NoWrap Text x3:   H=0 V=3
HorizontalLayout, fixed-width NoWrap Text x3: H=3 V=0
```

`Orientation::Horizontal` is never queried at all for fixed-width `Text` in a `VerticalLayout`, so
there is no prior `Horizontal` call to have already populated the cache - the fast path this
candidate ruled out as pointless does have a real, unshaped target in that construct. Candidate 070
built and measured it: -82% first-frame time on a non-virtualized `VerticalLayout`-of-`Text` scene
at 20 000-100 000 rows.

This candidate's other two conclusions (the fixed-width-alone fix is disproved by 044's own
measurement; the compiler-provable "width unused" flag is out of scope, tracked under candidate 012
option 2) are unaffected by this correction.

See [`070.VERTICALLAYOUT_TEXT_ORIENTATION_QUERY/RESULT.md`](../070.VERTICALLAYOUT_TEXT_ORIENTATION_QUERY/RESULT.md)
for the full measurement.
