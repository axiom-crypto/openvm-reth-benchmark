#!/usr/bin/env python3
"""Sum durations from output.txt-style files and report milliseconds."""

from __future__ import annotations

import sys
from decimal import Decimal, getcontext
from pathlib import Path
import re


getcontext().prec = 28

FACTORS = {
    "ns": Decimal("0.000001"),   # nanoseconds to milliseconds
    "us": Decimal("0.001"),      # microseconds (ASCII)
    "\u00b5s": Decimal("0.001"),  # microseconds (µ)
    "ms": Decimal("1"),          # already milliseconds
    "s": Decimal("1000"),        # seconds to milliseconds
}

_DURATION_RE = re.compile(r"([0-9]+(?:\.[0-9]+)?)(ns|us|\u00b5s|ms|s)\s*$")


def parse_line(line: str, line_no: int) -> Decimal:
    line = line.strip()
    if not line:
        return Decimal(0)

    match = _DURATION_RE.search(line)
    if not match:
        return Decimal(0)

    number_part, unit = match.groups()

    if unit not in FACTORS:
        raise ValueError(f"Unsupported unit '{unit}' on line {line_no}")

    try:
        value = Decimal(number_part)
    except Exception as exc:  # pragma: no cover - defensive
        raise ValueError(f"Invalid numeric value '{number_part}' on line {line_no}") from exc

    return value * FACTORS[unit]


def main() -> int:
    path = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("output.txt")

    if not path.is_file():
        print(f"File not found: {path}", file=sys.stderr)
        return 1

    total_ms = Decimal(0)
    with path.open("r", encoding="utf-8") as file_obj:
        for line_no, line in enumerate(file_obj, start=1):
            total_ms += parse_line(line, line_no)

    print(f"{total_ms} ms")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
