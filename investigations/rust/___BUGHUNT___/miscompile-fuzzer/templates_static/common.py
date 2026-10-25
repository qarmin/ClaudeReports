"""Shared helpers for static (deterministic) template modules."""

from __future__ import annotations

import itertools


def product_scenarios(axes: dict) -> list[dict]:
    """`axes` IS the declarative spec: `{axis_name: [allowed values]}`.
    Returns one dict per combination of the cartesian product, in the stable
    order `itertools.product` gives for the axes in declaration order - so
    the same `axes` dict always enumerates the same scenarios in the same
    order, and the combination count is exactly the product of the list
    lengths (never a surprise, never open-ended). To broaden coverage, add a
    value to one of the axis lists in the calling module - nothing here or
    in the calling module's generation logic needs to change.
    """
    keys = list(axes.keys())
    return [dict(zip(keys, combo)) for combo in itertools.product(*(axes[k] for k in keys))]
