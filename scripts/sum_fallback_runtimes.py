#!/usr/bin/env python3
"""Aggregate fallback runtimes by type from an output log."""

from __future__ import annotations

import argparse
from collections import defaultdict
from decimal import Decimal, InvalidOperation, getcontext
from pathlib import Path
import re
import sys


getcontext().prec = 28

# Map supported duration suffixes to seconds.
_UNIT_FACTORS = {
    "ns": Decimal("1e-9"),
    "us": Decimal("1e-6"),
    "\u00b5s": Decimal("1e-6"),
    "ms": Decimal("1e-3"),
    "s": Decimal("1"),
}

# Lines look like: "<fallback> [0.000012345s]"
_LINE_RE = re.compile(
    r"^(?P<name>.+?)\s*\[\s*(?P<value>[0-9]+(?:\.[0-9]+)?)\s*(?P<unit>ns|us|\u00b5s|ms|s)\s*\]\s*$"
)


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Sum runtimes for each fallback type found in the log. "
            "Only lines ending with '[<duration><unit>]' are considered."
        )
    )
    parser.add_argument(
        "path",
        nargs="?",
        default="output.txt",
        help="Path to the log file (default: %(default)s)",
    )
    parser.add_argument(
        "--sort",
        choices=("total", "name"),
        default="total",
        help="Sort output by total runtime or by fallback name (default: %(default)s)",
    )
    parser.add_argument(
        "--ascending",
        action="store_true",
        help="Sort in ascending order instead of descending.",
    )
    return parser.parse_args()


def _format_seconds(value: Decimal) -> str:
    text = format(value, ".9f").rstrip("0").rstrip(".")
    return text or "0"


def main() -> int:
    args = _parse_args()
    path = Path(args.path)

    if not path.is_file():
        print(f"File not found: {path}", file=sys.stderr)
        return 1

    totals: dict[str, Decimal] = defaultdict(Decimal)
    counts: dict[str, int] = defaultdict(int)

    with path.open("r", encoding="utf-8") as file_obj:
        for line_no, raw_line in enumerate(file_obj, start=1):
            line = raw_line.strip()
            if not line:
                continue

            match = _LINE_RE.match(line)
            if not match:
                continue

            name = match.group("name").strip()
            if not name:
                continue

            value_text = match.group("value")
            unit = match.group("unit")

            try:
                base_value = Decimal(value_text)
            except InvalidOperation:
                print(
                    f"Skipping line {line_no}: invalid numeric value '{value_text}'",
                    file=sys.stderr,
                )
                continue

            factor = _UNIT_FACTORS[unit]
            totals[name] += base_value * factor
            counts[name] += 1

    if not totals:
        print("No fallback runtimes found.")
        return 0

    if args.sort == "total":
        sorted_items = sorted(
            totals.items(), key=lambda item: item[1], reverse=not args.ascending
        )
    else:
        sorted_items = sorted(
            totals.items(), key=lambda item: item[0], reverse=not args.ascending
        )

    header = f"{'fallback':<40} {'total_seconds':>16} {'count':>8}"
    print(header)
    print("-" * len(header))
    for name, total_seconds in sorted_items:
        print(
            f"{name:<40} {_format_seconds(total_seconds):>16} {counts[name]:>8}"
        )

    return 0


if __name__ == "__main__":
    raise SystemExit(main())

