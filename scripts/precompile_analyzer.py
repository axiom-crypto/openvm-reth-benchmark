#!/usr/bin/env python3
"""
Analyzes precompile calls in Ethereum blocks using debug_traceBlockByNumber.

Usage:
    python3 precompile_analyzer.py <block_number>                    # Analyze single block
    python3 precompile_analyzer.py <block_number> --rpc <url>        # Use custom RPC endpoint
    python3 precompile_analyzer.py <block_number> -v                 # Verbose output
    python3 precompile_analyzer.py --check                           # Check if RPC is working
"""

import argparse
import json
import sys
import urllib.request

DEFAULT_RPC_URL = "http://localhost:8545"

# Precompile addresses
PRECOMPILES = {
    # Frontier
    "0x0000000000000000000000000000000000000001": "ecRecover",
    "0x0000000000000000000000000000000000000002": "SHA2-256",
    "0x0000000000000000000000000000000000000003": "RIPEMD-160",
    "0x0000000000000000000000000000000000000004": "identity",
    # Byzantium
    "0x0000000000000000000000000000000000000005": "modexp",
    "0x0000000000000000000000000000000000000006": "ecAdd",
    "0x0000000000000000000000000000000000000007": "ecMul",
    "0x0000000000000000000000000000000000000008": "ecPairing",
    # Istanbul
    "0x0000000000000000000000000000000000000009": "blake2f",
    # Cancun
    "0x000000000000000000000000000000000000000a": "KZG point evaluation",
    # Prague
    "0x000000000000000000000000000000000000000b": "BLS12-381 G1 add",
    "0x000000000000000000000000000000000000000c": "BLS12-381 G1 msm",
    "0x000000000000000000000000000000000000000d": "BLS12-381 G2 add",
    "0x000000000000000000000000000000000000000e": "BLS12-381 G2 msm",
    "0x000000000000000000000000000000000000000f": "BLS12-381 pairing",
    "0x0000000000000000000000000000000000000010": "BLS12-381 map Fp to G1",
    "0x0000000000000000000000000000000000000011": "BLS12-381 map Fp2 to G2",
    # Osaka (RIP-7212)
    "0x0000000000000000000000000000000000000100": "P256 verify",
}

# Table column widths
COL_PRECOMPILE = 22
COL_CALLS = 8

# Table formatting
TABLE_HEADER = f"| {'Precompile':<{COL_PRECOMPILE}} | {'Calls':>{COL_CALLS}} |"
TABLE_SEP = f"|{'-' * (COL_PRECOMPILE + 2)}|{'-' * (COL_CALLS + 2)}|"


def rpc_call(url: str, method: str, params: list) -> dict:
    """Make a JSON-RPC call."""
    payload = {
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1,
    }
    req = urllib.request.Request(
        url,
        data=json.dumps(payload).encode(),
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(req, timeout=120) as response:
        return json.loads(response.read().decode())


def process_call(call: dict, counts: dict[str, int]) -> None:
    """Process a single call and its subcalls, counting precompile calls."""
    to_addr = call.get("to", "").lower()
    if to_addr in PRECOMPILES:
        counts[PRECOMPILES[to_addr]] += 1

    for subcall in call.get("calls", []):
        process_call(subcall, counts)


def count_call_frames(call: dict) -> int:
    """Count total call frames in a call tree."""
    n = 1
    for sub in call.get("calls", []):
        n += count_call_frames(sub)
    return n


def count_precompile_calls(trace: list) -> dict[str, int]:
    """Count precompile calls from debug_traceBlockByNumber result."""
    counts: dict[str, int] = {name: 0 for name in PRECOMPILES.values()}

    for tx_trace in trace:
        if "result" in tx_trace:
            process_call(tx_trace["result"], counts)

    return counts


def analyze_block(
    rpc_url: str, block_number: int, verbose: bool = False
) -> dict[str, int]:
    """Analyze a single block and return precompile counts."""
    block_hex = hex(block_number)
    tracer_config = {"tracer": "callTracer"}

    result = rpc_call(rpc_url, "debug_traceBlockByNumber", [block_hex, tracer_config])
    if result is None:
        raise Exception("RPC returned None")
    if "error" in result:
        raise Exception(f"RPC error: {result['error']}")

    trace = result.get("result", [])
    if verbose:
        total_calls = 0
        for tx in trace:
            if "result" in tx:
                total_calls += count_call_frames(tx["result"])
        print(f"  Transactions: {len(trace)}, Call frames: {total_calls}")

    return count_precompile_calls(trace)


def check_rpc(rpc_url: str) -> bool:
    """Check if RPC endpoint supports debug_traceBlockByNumber."""
    print("Checking for debug_traceBlockByNumber support...")

    try:
        result = rpc_call(rpc_url, "eth_blockNumber", [])
        if "error" in result:
            print(f"  eth_blockNumber failed: {result['error']}")
            return False
        block_num = int(result["result"], 16)
        print(f"  eth_blockNumber: {block_num}")

        # Use an older block for testing (100 blocks back)
        test_block = block_num - 100
        tracer_config = {"tracer": "callTracer"}

        result = rpc_call(
            rpc_url, "debug_traceBlockByNumber", [hex(test_block), tracer_config]
        )

        if "error" in result:
            print(f"  debug_traceBlockByNumber failed: {result['error']}")
            return False

        trace = result.get("result", [])
        print(f"  debug_traceBlockByNumber: OK ({len(trace)} transactions)")
        return True
    except Exception as e:
        print(f"  Error: {e}")
        return False


def print_counts(counts: dict[str, int], block_number: int) -> None:
    """Print precompile counts in a table."""
    total = sum(counts.values())

    print(f"\n## Block {block_number}\n")

    print(TABLE_HEADER)
    print(TABLE_SEP)

    for name, count in sorted(counts.items(), key=lambda x: -x[1]):
        if count > 0:
            print(f"| {name:<{COL_PRECOMPILE}} | {count:>{COL_CALLS}} |")

    print(TABLE_SEP)
    print(f"| {'Total':<{COL_PRECOMPILE}} | {total:>{COL_CALLS}} |")


def main():
    parser = argparse.ArgumentParser(
        description="Analyze precompile calls in Ethereum blocks using debug_traceBlockByNumber",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s 21000000                          Analyze block 21000000
  %(prog)s 21000000 --rpc http://localhost:8545
  %(prog)s 21000000 -v                       Verbose output
  %(prog)s --check                           Check if RPC supports trace method
        """,
    )
    parser.add_argument(
        "block",
        type=int,
        nargs="?",
        help="Block number to analyze",
    )
    parser.add_argument(
        "--rpc",
        type=str,
        default=DEFAULT_RPC_URL,
        help=f"RPC endpoint URL (default: {DEFAULT_RPC_URL})",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Check if RPC endpoint supports debug_traceBlockByNumber",
    )
    parser.add_argument(
        "--verbose",
        "-v",
        action="store_true",
        help="Print debug information about the trace response",
    )

    args = parser.parse_args()

    if args.check:
        success = check_rpc(args.rpc)
        sys.exit(0 if success else 1)

    if args.block is None:
        parser.error("block number is required (or use --check)")

    print("\n# PRECOMPILE ANALYZER\n")
    print(f"**Block:** {args.block}\n")

    try:
        counts = analyze_block(args.rpc, args.block, args.verbose)
        print_counts(counts, args.block)
    except urllib.error.URLError as e:
        print(f"\nError: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"\nError: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
