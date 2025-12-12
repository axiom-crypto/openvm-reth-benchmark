#!/usr/bin/env python3
"""
Fetches data from ethproofs.org API and analyzes proving times.

Finds:
- Top K blocks with maximum gas used
- Top K blocks by proving time statistics: MAX median, MAX avg, MAX max, MAX min

Usage:
    python3 ethproofs_analyzer.py                         # Fetch 1 page (100 blocks)
    python3 ethproofs_analyzer.py --pages 5               # Fetch 5 pages (500 blocks)
    python3 ethproofs_analyzer.py --pages 10 --size 50    # Fetch 10 pages of 50 blocks each
    python3 ethproofs_analyzer.py --file data.json        # Load from file
    python3 ethproofs_analyzer.py --top-k 5               # Show top 5 blocks per metric
"""

import argparse
import json
import sys
import urllib.parse
import urllib.request

API_URL = "https://ethproofs.org/api/blocks"

# Table column widths
COL_RANK = 4
COL_BLOCK = 10
COL_GAS = 14
COL_TXS = 5
COL_TIME = 13
COL_TIMESTAMP = 19

# Table separators
TIME_TABLE_SEP = f"|{'-' * (COL_RANK + 2)}|{'-' * (COL_BLOCK + 2)}|{'-' * (COL_TIME + 2)}|{'-' * (COL_GAS + 2)}|{'-' * (COL_TXS + 2)}|"


def fmt_timestamp(ts: str | None) -> str:
    """Format timestamp without timezone."""
    if ts and len(ts) > 19:
        return ts[:19]  # Keep only YYYY-MM-DD HH:MM:SS
    return ts or "N/A"


def fmt_time(ms: float) -> str:
    """Format milliseconds as seconds."""
    return f"{ms / 1000:.2f}s"


def fmt_gas(gas: int | None) -> str:
    """Format gas with commas."""
    return f"{gas:,}" if gas else "N/A"


def time_table_header(label: str) -> str:
    """Generate header row for proving time tables."""
    col = f"Time ({label})"
    return f"| {'Rank':>{COL_RANK}} | {'Block':<{COL_BLOCK}} | {col:<{COL_TIME}} | {'Gas':>{COL_GAS}} | {'Txs':>{COL_TXS}} |"


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


def analyze_blocks(data: dict, top_k: int = 1, metric: str = "all") -> None:
    """Analyze blocks to find max gas used and proving time statistics."""
    rows = data.get("rows", [])

    if not rows:
        print("No blocks found in the response.")
        return

    # Track blocks with gas for sorting
    blocks_with_gas_list = []
    blocks_with_gas = 0

    # For each block, calculate stats across its provers
    block_stats = []

    for block in rows:
        gas_used = block.get("gas_used")
        proofs = block.get("proofs", [])

        if gas_used is not None:
            blocks_with_gas += 1
            blocks_with_gas_list.append((block, gas_used))

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

    # Sort and get top K for each metric
    top_gas = sorted(blocks_with_gas_list, key=lambda x: x[1], reverse=True)[:top_k]
    top_median = sorted(block_stats, key=lambda x: x[1], reverse=True)[:top_k]
    top_avg = sorted(block_stats, key=lambda x: x[2], reverse=True)[:top_k]
    top_max = sorted(block_stats, key=lambda x: x[3], reverse=True)[:top_k]
    top_min = sorted(block_stats, key=lambda x: x[4], reverse=True)[:top_k]

    total_proofs = sum(len(entry[5]) for entry in block_stats)

    print(
        f"Fetched {len(rows):,} blocks ({blocks_with_gas:,} with gas, {total_proofs:,} proofs)\n"
    )

    # Max gas section
    if metric in ("all", "gas"):
        print(f"## Top {top_k} by Gas Used\n")
        if top_gas:
            print(
                f"| {'Rank':>{COL_RANK}} | {'Block':<{COL_BLOCK}} | {'Gas':>{COL_GAS}} | {'Txs':>{COL_TXS}} | {'Timestamp':<{COL_TIMESTAMP}} |"
            )
            print(
                f"|{'-' * (COL_RANK + 2)}|{'-' * (COL_BLOCK + 2)}|{'-' * (COL_GAS + 2)}|{'-' * (COL_TXS + 2)}|{'-' * (COL_TIMESTAMP + 2)}|"
            )
            for rank, (block, gas) in enumerate(top_gas, 1):
                print(
                    f"| {rank:>{COL_RANK}} | {block.get('block_number'):<{COL_BLOCK}} | {fmt_gas(gas):>{COL_GAS}} | {block.get('transaction_count'):>{COL_TXS}} | {fmt_timestamp(block.get('timestamp')):<{COL_TIMESTAMP}} |"
                )
        else:
            print("No blocks with gas data found")

    # Proving time sections
    if not block_stats and metric in ("all", "max", "median", "avg", "min"):
        print("\nNo proofs with proving time data found")
        return

    if metric in ("all", "max"):
        print(f"\n## Top {top_k} by MAX Proving Time\n")
        print(time_table_header("Max"))
        print(TIME_TABLE_SEP)
        for rank, entry in enumerate(top_max, 1):
            block, _, _, time_ms, _, _ = entry
            bn = block.get("block_number")
            gas = block.get("gas_used")
            txs = block.get("transaction_count") or "N/A"
            print(
                f"| {rank:>{COL_RANK}} | {bn:<{COL_BLOCK}} | {fmt_time(time_ms):<{COL_TIME}} | {fmt_gas(gas):>{COL_GAS}} | {txs:>{COL_TXS}} |"
            )

    if metric in ("all", "median"):
        print(f"\n## Top {top_k} by MEDIAN Proving Time\n")
        print(time_table_header("Median"))
        print(TIME_TABLE_SEP)
        for rank, entry in enumerate(top_median, 1):
            block, time_ms, _, _, _, _ = entry
            bn = block.get("block_number")
            gas = block.get("gas_used")
            txs = block.get("transaction_count") or "N/A"
            print(
                f"| {rank:>{COL_RANK}} | {bn:<{COL_BLOCK}} | {fmt_time(time_ms):<{COL_TIME}} | {fmt_gas(gas):>{COL_GAS}} | {txs:>{COL_TXS}} |"
            )

    if metric in ("all", "avg"):
        print(f"\n## Top {top_k} by AVG Proving Time\n")
        print(time_table_header("Avg"))
        print(TIME_TABLE_SEP)
        for rank, entry in enumerate(top_avg, 1):
            block, _, time_ms, _, _, _ = entry
            bn = block.get("block_number")
            gas = block.get("gas_used")
            txs = block.get("transaction_count") or "N/A"
            print(
                f"| {rank:>{COL_RANK}} | {bn:<{COL_BLOCK}} | {fmt_time(time_ms):<{COL_TIME}} | {fmt_gas(gas):>{COL_GAS}} | {txs:>{COL_TXS}} |"
            )

    if metric in ("all", "min"):
        print(f"\n## Top {top_k} by MIN Proving Time\n")
        print(time_table_header("Min"))
        print(TIME_TABLE_SEP)
        for rank, entry in enumerate(top_min, 1):
            block, _, _, _, time_ms, _ = entry
            bn = block.get("block_number")
            gas = block.get("gas_used")
            txs = block.get("transaction_count") or "N/A"
            print(
                f"| {rank:>{COL_RANK}} | {bn:<{COL_BLOCK}} | {fmt_time(time_ms):<{COL_TIME}} | {fmt_gas(gas):>{COL_GAS}} | {txs:>{COL_TXS}} |"
            )


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
    parser.add_argument(
        "--top-k",
        "-k",
        type=int,
        default=1,
        help="Number of top blocks to show per metric (default: 1)",
    )
    parser.add_argument(
        "--metric",
        type=str,
        default="all",
        choices=["all", "gas", "max", "median", "avg", "min"],
        help="Which metric to show (default: all)",
    )

    args = parser.parse_args()

    print("\n# ETHPROOFS ANALYZER\n")

    if args.file:
        print(f"**Source:** {args.file}\n")
        try:
            data = load_from_file(args.file)
            analyze_blocks(data, top_k=args.top_k, metric=args.metric)
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
            analyze_blocks(data, top_k=args.top_k, metric=args.metric)
        except urllib.error.URLError as e:
            print(f"\nError: {e}")
            print("Try: python3 ethproofs_analyzer.py --file data.json")
            sys.exit(1)
        except json.JSONDecodeError as e:
            print(f"Error parsing response: {e}")
            sys.exit(1)


if __name__ == "__main__":
    main()
