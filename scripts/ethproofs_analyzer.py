#!/usr/bin/env python3
"""
Fetches data from ethproofs.org API and analyzes proving times.

Finds:
- Block with maximum gas used
- Proving time statistics across all proofs: MAX, MEDIAN, AVG, MIN

Usage:
    python3 ethproofs_analyzer.py                         # Fetch 1 page (100 blocks)
    python3 ethproofs_analyzer.py --pages 5               # Fetch 5 pages (500 blocks)
    python3 ethproofs_analyzer.py --pages 10 --size 50    # Fetch 10 pages of 50 blocks each
    python3 ethproofs_analyzer.py --file data.json        # Load from file
"""

import argparse
import json
import sys
import urllib.parse
import urllib.request

API_URL = "https://ethproofs.org/api/blocks"


def fetch_blocks(
    page_index: int = 0, page_size: int = 100, machine_type: str = "multi"
) -> dict:
    """Fetch a single page of blocks from the ethproofs API."""
    params = urllib.parse.urlencode(
        {"page_index": page_index, "page_size": page_size, "machine_type": machine_type}
    )
    url = f"{API_URL}?{params}"

    with urllib.request.urlopen(url, timeout=30) as response:
        return json.loads(response.read().decode())


def fetch_multiple_pages(
    num_pages: int = 1, page_size: int = 100, machine_type: str = "multi"
) -> dict:
    """Fetch multiple pages and combine results."""
    all_rows = []

    for page_idx in range(num_pages):
        # Show progress on same line
        print(f"\r  Fetching page {page_idx + 1}/{num_pages}...", end="", flush=True)
        try:
            data = fetch_blocks(
                page_index=page_idx, page_size=page_size, machine_type=machine_type
            )
            rows = data.get("rows", [])
            all_rows.extend(rows)

            # Stop if we got fewer blocks than requested (no more data)
            if len(rows) < page_size:
                print(
                    f"\r  Fetched {len(all_rows):,} blocks ({page_idx + 1} pages, reached end of data)"
                )
                break
        except Exception as e:
            print(f"\r  Error on page {page_idx + 1}: {e}")
            break
    else:
        print(
            f"\r  Fetched {len(all_rows):,} blocks ({num_pages} pages)                    "
        )

    return {"rows": all_rows}


def load_from_file(filepath: str) -> dict:
    """Load JSON data from a file."""
    with open(filepath, "r") as f:
        return json.load(f)


def analyze_blocks(data: dict) -> None:
    """Analyze blocks to find max gas used and proving time statistics."""
    rows = data.get("rows", [])

    if not rows:
        print("No blocks found in the response.")
        return

    # Track max gas
    max_gas_block = None
    max_gas_used = -1
    blocks_with_gas = 0

    # For each block, calculate stats across its provers
    block_stats = []

    for block in rows:
        gas_used = block.get("gas_used")
        proofs = block.get("proofs", [])

        if gas_used is not None:
            blocks_with_gas += 1
            if gas_used > max_gas_used:
                max_gas_used = gas_used
                max_gas_block = block

        proving_times = [
            p.get("proving_time") for p in proofs if p.get("proving_time") is not None
        ]

        if proving_times:
            sorted_times = sorted(proving_times)
            n = len(sorted_times)

            block_min = sorted_times[0]
            block_max = sorted_times[-1]
            block_avg = sum(sorted_times) / n
            if n % 2 == 0:
                block_median = (sorted_times[n // 2 - 1] + sorted_times[n // 2]) / 2
            else:
                block_median = sorted_times[n // 2]

            block_stats.append(
                (block, block_median, block_avg, block_max, block_min, proving_times)
            )

    # Find blocks with extreme values
    if block_stats:
        max_median_entry = max(block_stats, key=lambda x: x[1])
        max_avg_entry = max(block_stats, key=lambda x: x[2])
        max_max_entry = max(block_stats, key=lambda x: x[3])
        max_min_entry = max(block_stats, key=lambda x: x[4])

    total_proofs = sum(len(entry[5]) for entry in block_stats)

    def fmt_time(ms):
        return f"{ms:,.0f}ms ({ms / 1000:.1f}s)"

    def fmt_gas(gas):
        return f"{gas:,}" if gas else "N/A"

    print(
        f"Fetched {len(rows):,} blocks ({blocks_with_gas:,} with gas, {total_proofs:,} proofs)\n"
    )

    # Max gas section
    print("## Max Gas Used\n")
    if max_gas_block:
        b = max_gas_block
        print(f"| {'Block':<10} | {'Gas':>14} | {'Txs':>5} | {'Timestamp':<26} |")
        print(f"|{'-' * 12}|{'-' * 16}|{'-' * 7}|{'-' * 28}|")
        print(
            f"| {b.get('block_number'):<10} | {fmt_gas(max_gas_used):>14} | {b.get('transaction_count'):>5} | {b.get('timestamp'):<26} |"
        )
    else:
        print("No blocks with gas data found")

    # Proving time section
    if block_stats:
        print("\n## Proving Time (per-block stats → max across blocks)\n")
        print(
            f"| {'Metric':<12} | {'Block':<10} | {'Time':<18} | {'Gas':>14} | {'Txs':>5} |"
        )
        print(f"|{'-' * 14}|{'-' * 12}|{'-' * 20}|{'-' * 16}|{'-' * 7}|")

        stats = [
            ("MAX median", max_median_entry[0], max_median_entry[1]),
            ("MAX avg", max_avg_entry[0], max_avg_entry[2]),
            ("MAX max", max_max_entry[0], max_max_entry[3]),
            ("MAX min", max_min_entry[0], max_min_entry[4]),
        ]

        for label, block, time_ms in stats:
            bn = block.get("block_number")
            gas = block.get("gas_used")
            txs = block.get("transaction_count") or "N/A"
            print(
                f"| {label:<12} | {bn:<10} | {fmt_time(time_ms):<18} | {fmt_gas(gas):>14} | {txs:>5} |"
            )
    else:
        print("\nNo proofs with proving time data found")


def main():
    parser = argparse.ArgumentParser(
        description="Analyze ethproofs.org block data to find max gas used and max median proving time",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s                         Fetch 1 page (100 blocks)
  %(prog)s --pages 5               Fetch 5 pages (500 blocks)
  %(prog)s --pages 10 --size 50    Fetch 10 pages of 50 blocks each
  %(prog)s --file data.json        Load from local JSON file
        """,
    )
    parser.add_argument(
        "--pages",
        "-p",
        type=int,
        default=1,
        help="Number of pages to fetch (default: 1)",
    )
    parser.add_argument(
        "--size",
        "-s",
        type=int,
        default=100,
        help="Number of blocks per page (default: 100)",
    )
    parser.add_argument(
        "--file", "-f", type=str, help="Load data from a JSON file instead of fetching"
    )
    parser.add_argument(
        "--machine-type",
        "-m",
        type=str,
        default="multi",
        choices=["multi", "single"],
        help="Machine type filter (default: multi)",
    )

    args = parser.parse_args()

    print("\n# ETHPROOFS ANALYZER\n")

    if args.file:
        print(f"**Source:** {args.file}\n")
        try:
            data = load_from_file(args.file)
            analyze_blocks(data)
        except FileNotFoundError:
            print(f"Error: File not found: {args.file}")
            sys.exit(1)
        except json.JSONDecodeError as e:
            print(f"Error parsing JSON: {e}")
            sys.exit(1)
    else:
        print(f"**Source:** {API_URL}  ")
        print(
            f"**Config:** {args.pages} × {args.size} blocks, filter={args.machine_type}\n"
        )

        try:
            data = fetch_multiple_pages(
                num_pages=args.pages,
                page_size=args.size,
                machine_type=args.machine_type,
            )
            print()
            analyze_blocks(data)
        except urllib.error.URLError as e:
            print(f"\nError: {e}")
            print("Try: python3 ethproofs_analyzer.py --file data.json")
            sys.exit(1)
        except json.JSONDecodeError as e:
            print(f"Error parsing response: {e}")
            sys.exit(1)


if __name__ == "__main__":
    main()
