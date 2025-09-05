#!/usr/bin/env python3
"""
OpenVM Execute Handler Analyzer

Analyzes execute handler functions in the OpenVM binary with detailed assembly output,
source code mapping, and instruction analysis.

Features:
- Disassembles execute handlers with interleaved source code
- Counts and categorizes instructions (loads, stores, branches, etc.)
- Detects tail call optimization (TCO) usage
- Categorizes handlers by circuit type and operation
- Cross-platform support (macOS/Linux)
- Auto-builds binary if missing

Usage:
    ./scripts/analyze_handlers.py [options] [search_terms]

Options:
    --first              Analyze first matching handler
    --offset <N>         Analyze handler at offset N (0-indexed)
    --output <file>      Save output to file
    --rebuild            Force rebuild of binary before analysis

Examples:
    ./scripts/analyze_handlers.py                       # List all execute handlers
    ./scripts/analyze_handlers.py rv32 base_alu         # List rv32 base_alu handlers
    ./scripts/analyze_handlers.py --first bigint slt    # Analyze first bigint less-than
    ./scripts/analyze_handlers.py --offset 2 native field_extension  # 3rd field extension handler
    ./scripts/analyze_handlers.py --rebuild --first rv32 base_alu  # Force rebuild and analyze
    ./scripts/analyze_handlers.py 0x1000ea478          # Analyze handler at address

Search terms match against:
- Function names (e.g., "base_alu", "less_than")
- Circuit types (e.g., "rv32im", "bigint", "native", "algebra")
- Operations (e.g., "add", "mul", "branch", "castf")

Supported operations include:
- RV32IM: base_alu, shift, load/store, branch, less_than, mul, div/rem
- BigInt: 256-bit arithmetic, comparisons, branches
- Native: field arithmetic, field extensions, castf, range checks
- Algebra: modular arithmetic, equality checks
- Cryptography: SHA256, Keccak256, Poseidon2
- ECC: elliptic curve operations
"""

import subprocess
import sys
import re
import platform
from pathlib import Path
from collections import defaultdict
from dataclasses import dataclass
from typing import Optional

# Constants
BINARY_NAME = "openvm-reth-benchmark-bin"
BINARY_SEARCH_PATHS = ["target/profiling", "target/release", "target/debug"]

# OpenVM Handler Categorizer
@dataclass
class HandlerInfo:
    """Information about a TCO handler function."""
    circuit: str      # e.g., "rv32im", "bigint", "native"
    extension: str    # e.g., "base_alu", "shift", "loadstore"
    operation: str    # e.g., "ADD", "SUB", "load", "store"
    handler_type: str # e.g., "e1", "e2"
    is_imm: Optional[bool] = None  # True for immediate, False for register, None if unknown

class OpenVMCategorizer:
    """Categorizes OpenVM TCO handler functions."""

    def __init__(self):
        self.circuit_patterns = {
            "rv32im": "openvm_rv32im_circuit",
            "bigint": "openvm_bigint_circuit",
            "native": "openvm_native_circuit",
            "ecc": "openvm_ecc_circuit",
            "keccak": "openvm_keccak_circuit",
            "system": "openvm_circuit",
        }

        self.extension_patterns = {
            "base_alu": ["base_alu"],
            "shift": ["shift"],
            "loadstore": ["loadstore", "load_store", "hintstore", "load_sign_extend"],
            "branch": ["branch"],
            "jump": ["jal", "jump"],
            "mul": ["mul"],
            "divrem": ["divrem"],
            "less_than": ["less_than"],
            "castf": ["castf"],
            "field_extension": ["field_extension"],
            "field_arithmetic": ["field_arithmetic"],
            "modular_chip": ["modular_chip"],
            "poseidon2": ["poseidon2"],
            "fri": ["fri"],
            "weierstrass": ["weierstrass"],
            "keccak": ["keccak"],
            "sha256": ["sha256"],
            "public_values": ["public_values"],
            "phantom": ["phantom"],
            "system": ["terminate", "unreachable"],
        }

        self.operation_patterns = {
            "load": ["load", "lb", "lh", "lw", "lbu", "lhu", "loadw", "loadbu", "loadhu", "loadb", "loadh"],
            "store": ["store", "sb", "sh", "sw", "storew", "storeh", "storeb", "hintstore", "hint_store"],
            "mul": ["mul", "mulh", "mulhsu", "mulhu"],
            "div": ["div", "divu"],
            "rem": ["rem", "remu"],
            "sll": ["sll"], "srl": ["srl"], "sra": ["sra"],
            "slt": ["slt", "less_than"],
            "sltu": ["sltu"],
            "beq": ["beq", "branch_eq"],
            "bne": ["bne"], "blt": ["blt"], "bge": ["bge"], "bltu": ["bltu"], "bgeu": ["bgeu"],
            "jal": ["jal"], "jalr": ["jalr"],
            "lui": ["lui"], "auipc": ["auipc"],
            "add": ["add"], "sub": ["sub"], "xor": ["xor"], "or": ["or"], "and": ["and"],
            "castf": ["castf"],
            "field_add": ["fe4add", "field_add"],
            "field_sub": ["fe4sub", "field_sub"],
            "field_mul": ["fe4mul", "bbe4mul", "field_mul"],
            "field_div": ["fe4div", "bbe4div", "field_div"],
            "is_eq": ["is_eq"],
            "range_check": ["range_check"],
            "poseidon2_perm": ["perm_pos2"],
            "poseidon2_comp": ["comp_pos2"],
            "fri_opening": ["fri_reduced_opening"],
            "verify_batch": ["verify_batch"],
            "terminate": ["terminate"],
            "phantom": ["phantom"],
            "publish": ["publish"],
            "ecall": ["ecall"], "ebreak": ["ebreak"],
        }

    def parse_function_name(self, function_name: str) -> HandlerInfo:
        """Parse a mangled function name to extract instruction information."""
        # Extract circuit type
        circuit = "unknown"
        for circuit_name, pattern in self.circuit_patterns.items():
            if pattern in function_name:
                circuit = circuit_name
                break

        # Extract extension type
        extension = "unknown"
        for ext_name, patterns in self.extension_patterns.items():
            if any(pattern in function_name for pattern in patterns):
                extension = ext_name
                break

        # Extract handler type (e1 or e2)
        handler_type = "unknown"
        if "execute_e1_tco_handler" in function_name:
            handler_type = "e1"
        elif "execute_e2_tco_handler" in function_name:
            handler_type = "e2"

        operation = self._determine_operation(function_name, circuit, extension)

        return HandlerInfo(
            circuit=circuit,
            extension=extension,
            operation=operation,
            handler_type=handler_type
        )

    def _determine_operation(self, function_name: str, circuit: str, extension: str) -> str:
        """Determine the specific operation from the function name."""
        if extension == "base_alu":
            return self._categorize_base_alu_operation(function_name, circuit)

        # For specific extensions, check for sub-operations first
        if extension == "modular_chip":
            if "is_eq" in function_name.lower():
                return "IS_EQ"
            return "MODULAR_CHIP"

        # For specific extensions, return the extension name as the operation
        specific_extensions = ["castf", "field_extension", "less_than"]
        if extension in specific_extensions:
            return extension.upper()

        for op_name, patterns in self.operation_patterns.items():
            if any(pattern in function_name.lower() for pattern in patterns):
                return op_name.upper()

        return extension.upper()

    def _categorize_base_alu_operation(self, function_name: str, circuit: str) -> str:
        """Categorize base_alu operations by their hash suffix."""
        hash_match = re.search(r'17h([a-f0-9]+)E$', function_name)
        if not hash_match:
            return "UNKNOWN"

        hash_suffix = hash_match.group(1)

        if circuit == "rv32im":
            return f"RV32_BASE_ALU_{hash_suffix[:8].upper()}"
        elif circuit == "bigint":
            return f"BIGINT_BASE_ALU_{hash_suffix[:8].upper()}"
        else:
            return f"BASE_ALU_{hash_suffix[:8].upper()}"

    def get_category_key(self, handler_info: HandlerInfo) -> str:
        """Get a key for grouping handlers by category."""
        if handler_info.extension == "base_alu":
            return f"{handler_info.circuit}_base_alu"
        elif handler_info.circuit != "unknown" and handler_info.extension != "unknown":
            return f"{handler_info.circuit}_{handler_info.extension}"
        elif handler_info.circuit != "unknown":
            return handler_info.circuit
        else:
            return "other"

# Assembly analysis patterns
ASSEMBLY_INSTRUCTION_REGEX = r'^\s*[0-9a-fA-F]+:'

# Cross-platform instruction patterns (ARM64 + x86)
INSTRUCTION_PATTERNS = {
    'loads': [
        # ARM64
        r'\bldr\b', r'\bldur\b', r'\bldrb\b', r'\bldrh\b', r'\bldp\b',
        # x86
        r'\bmov.*,.*\[', r'\blea\b', r'\bpop\b'
    ],
    'stores': [
        # ARM64
        r'\bstr\b', r'\bstur\b', r'\bstrb\b', r'\bstrh\b', r'\bstp\b',
        # x86
        r'\bmov.*\[.*,', r'\bpush\b'
    ],
    'branches': [
        # ARM64
        r'\bb\.', r'\bbr\b', r'\bret\b', r'\bcbz\b', r'\bcbnz\b', r'\btbz\b', r'\btbnz\b',
        # x86
        r'\bjmp\b', r'\bje\b', r'\bjne\b', r'\bjl\b', r'\bjg\b', r'\bjle\b', r'\bjge\b', r'\bret\b'
    ],
    'calls': [
        # ARM64
        r'\bbl\b', r'\bblr\b',
        # x86
        r'\bcall\b'
    ],
    'arithmetic': [
        # ARM64
        r'\badd\b', r'\bsub\b', r'\bmul\b', r'\bdiv\b', r'\band\b', r'\borr\b', r'\beor\b',
        r'\blsl\b', r'\blsr\b', r'\basr\b', r'\bmadd\b', r'\bmsub\b', r'\bcmp\b', r'\bcmn\b',
        # x86
        r'\badd\b', r'\bsub\b', r'\bmul\b', r'\bdiv\b', r'\band\b', r'\bor\b', r'\bxor\b',
        r'\bshl\b', r'\bshr\b', r'\bsar\b', r'\bcmp\b', r'\btest\b'
    ],
    'moves': [
        # ARM64
        r'\bmov\b', r'\bmovk\b', r'\bmovz\b', r'\bmovn\b',
        # x86
        r'\bmov(?!.*\[).*,.*(?<!\[)', r'\blea\b(?!.*\[)'
    ]
}

# Platform-specific tail call patterns
def get_tail_call_patterns():
    """Get tail call instruction patterns for the current architecture."""
    # Detect from the first assembly instruction we see, or use platform as fallback
    return {
        'arm64': [r'\bbr\s+x', r'\bbr\t'],
        'x86_64': [r'\bjmp\b', r'\btail\s+call']
    }

# Output formatting
HEADER_WIDTH = 80
SECTION_WIDTH = 80

def find_binary_path():
    """Find the OpenVM benchmark binary."""
    for search_path in BINARY_SEARCH_PATHS:
        binary_path = Path(search_path) / BINARY_NAME
        if binary_path.exists():
            return binary_path
    return None

def get_all_symbols(binary_path):
    """Get all symbols from the binary using nm."""
    try:
        result = subprocess.run([
            "nm", "-n", str(binary_path)
        ], stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True)
        return result.stdout
    except FileNotFoundError:
        print("nm not found. Install with: brew install binutils")
        return ""

def find_execute_handlers(symbols_text):
    """Find all execute handler functions in the binary (both TCO and non-TCO)."""
    handlers = []
    lines = symbols_text.split('\n')

    for line in lines:
        # Check if the line contains an execute handler function
        # This includes both TCO handlers (execute_e1_tco_handler, execute_e2_tco_handler)
        # and regular execute handlers if they exist
        if 'execute_e' in line and 'handler' in line:
            # Extract address and full symbol name
            parts = line.strip().split()
            if len(parts) >= 3:
                addr = parts[0]
                symbol_type = parts[1]
                symbol_name = parts[2]

                # Only include text symbols (lowercase 't' also indicates text section)
                if symbol_type.upper() == 'T' or symbol_type == 't':
                    handlers.append((addr, symbol_name))

    return handlers

def demangle_symbol(symbol):
    """Demangle a Rust symbol using c++filt."""
    try:
        result = subprocess.run(
            ["c++filt", "-r"],
            input=symbol,
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            text=True
        )
        return result.stdout.strip()
    except FileNotFoundError:
        # If c++filt is not available, return the original symbol
        return symbol

def categorize_handler(handler_name):
    """Categorize a handler based on its name."""
    categorizer = OpenVMCategorizer()
    handler_info = categorizer.parse_function_name(handler_name)
    return categorizer.get_category_key(handler_info), handler_info.operation

def get_objdump_command():
    """Get the appropriate objdump command for the current platform."""
    return "objdump"

def generate_dsym(binary_path, verbose=False):
    """Generate dSYM file for source code mapping on macOS."""
    if platform.system().lower() != "darwin":
        return True  # Not needed on non-macOS platforms

    dsym_path = Path(f"{binary_path}.dSYM")
    if verbose:
        print("Generating dSYM file for source mapping...")

    try:
        result = subprocess.run([
            "dsymutil", str(binary_path)
        ], capture_output=True, text=True, timeout=60)

        if result.returncode == 0:
            if verbose:
                print("dSYM file generated successfully!")
            return True
        else:
            # Filter out the many warnings about temporary files
            stderr_lines = result.stderr.split('\n')
            error_lines = [line for line in stderr_lines if 'error:' in line.lower()]
            if error_lines and verbose:
                print("Warning: dSYM generation had errors:")
                for error in error_lines[:3]:  # Show first 3 errors only
                    print(f"  {error}")
            elif verbose:
                print("dSYM file generated (with warnings about temporary files)")
            return dsym_path.exists()

    except subprocess.TimeoutExpired:
        if verbose:
            print("Warning: dSYM generation timed out")
        return False
    except FileNotFoundError:
        if verbose:
            print("Warning: dsymutil not found - source mapping may not work")
        return False

def ensure_debug_symbols(binary_path):
    """Ensure debug symbols are available for source mapping."""
    if platform.system().lower() == "darwin":  # macOS needs dSYM
        dsym_path = Path(f"{binary_path}.dSYM")
        if dsym_path.exists():
            return True
        return generate_dsym(binary_path, verbose=True)
    else:  # Linux uses embedded debug info
        return True

def get_assembly_with_source(binary_path, start_addr, end_addr=None):
    """Get assembly for a specific address range with interleaved source code."""
    objdump_cmd = get_objdump_command()

    try:
        if isinstance(start_addr, str):
            start_addr = int(start_addr, 16)

        cmd = [objdump_cmd, "-d", "-S", "-l", f"--start-address={start_addr:#x}"]

        if end_addr:
            if isinstance(end_addr, str):
                end_addr = int(end_addr, 16)
            cmd.append(f"--stop-address={end_addr:#x}")

        cmd.append(str(binary_path))
        result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True)
        return result.stdout
    except FileNotFoundError:
        print(f"objdump not found ({objdump_cmd}). Install binutils or build tools.")
        return ""

def analyze_execute_handler(binary_path, handler_addr, handler_name):
    """Analyze a specific execute handler function."""
    print("=" * HEADER_WIDTH)
    print("OpenVM Handler Analysis")
    print("=" * HEADER_WIDTH)
    print(f"Function: {handler_name}")

    demangled = demangle_symbol(handler_name)
    if demangled != handler_name:
        print(f"Demangled: {demangled}")

    category, opcode = categorize_handler(handler_name)
    print(f"Category: {category.upper()}")
    print(f"Opcode: {opcode.upper()}")
    print(f"Address: 0x{handler_addr}")

    # Ensure debug symbols are available for source code interleaving
    if not ensure_debug_symbols(binary_path):
        print("Warning: Debug symbols not available, source mapping may not work")

    print("-" * SECTION_WIDTH)

    # Get assembly for this function
    assembly_text = get_assembly_with_source(binary_path, handler_addr)

    if not assembly_text:
        print("Failed to get assembly")
        return

    # Display the assembly with interleaved source code
    print("Assembly with Source Code:")
    print()

    # gobjdump -S output has source code interleaved, parse and display it
    lines = assembly_text.split('\n')
    in_function = False
    asm_instructions = []

    for line in lines:
        # Check if we're in our function
        if handler_addr in line and ":" in line:
            in_function = True
            continue

        # Check for end of function (next function starts)
        if in_function and re.match(r'^[0-9a-fA-F]+\s+<.*>:', line):
            break

        if in_function and line.strip():
            # Count assembly instructions for analysis
            if re.match(ASSEMBLY_INSTRUCTION_REGEX, line):
                asm_instructions.append(line.strip())

            # Display all content (source comments and assembly)
            print(f"  {line}")

    # Analyze instruction counts
    print()
    print("-" * SECTION_WIDTH)
    print("Instruction Analysis:")
    print(f"Total assembly instructions: {len(asm_instructions)}")

    # Count instruction types
    instruction_counts = {name: 0 for name in INSTRUCTION_PATTERNS.keys()}

    for asm_line in asm_instructions:
        match = re.search(r'[0-9a-fA-F]+:\s+[0-9a-fA-F]+\s+(\w+)', asm_line)
        if match:
            mnemonic = match.group(1)
            for inst_type, patterns in INSTRUCTION_PATTERNS.items():
                if any(re.match(pattern.replace(r'\b', ''), mnemonic) for pattern in patterns):
                    instruction_counts[inst_type] += 1
                    break

    total_instructions = len(asm_instructions)
    categorized = sum(instruction_counts.values())
    instruction_counts['other'] = total_instructions - categorized

    for inst_type, count in instruction_counts.items():
        percentage = count * 100 // total_instructions if total_instructions > 0 else 0
        print(f"  {inst_type.replace('_', ' ').title():14s} {count:4d} ({percentage:3d}%)")

    # TCO analysis - detect architecture-specific tail calls
    print()
    print("TCO Analysis:")

    # Auto-detect architecture from assembly
    is_arm64 = any('ldr' in line or 'str' in line for line in asm_instructions[:5])
    is_x86 = any('mov' in line and ('rax' in line or 'eax' in line) for line in asm_instructions[:5])

    has_tail_call = False
    if is_arm64:
        has_tail_call = any('br\tx' in line or 'br\t' in line for line in asm_instructions)
    elif is_x86:
        has_tail_call = any('jmp' in line and 'call' not in line for line in asm_instructions)
    else:
        # Fallback: look for common tail call patterns
        has_tail_call = any('br\t' in line or ('jmp' in line and 'call' not in line) for line in asm_instructions)

    print(f"  Tail call: {'Yes' if has_tail_call else 'No'}")

def list_all_handlers(binary_path):
    """List all execute handlers grouped by category."""
    symbols_text = get_all_symbols(binary_path)
    if not symbols_text:
        return

    handlers = find_execute_handlers(symbols_text)

    if not handlers:
        print("No execute handlers found in binary")
        return

    # Group handlers by category
    categorized_handlers = defaultdict(list)

    for addr, name in handlers:
        category, opcode = categorize_handler(name)
        categorized_handlers[category].append((addr, name, opcode))

    print("=" * HEADER_WIDTH)
    print("OpenVM Execute Handlers")
    print("=" * HEADER_WIDTH)
    print(f"Total handlers found: {len(handlers)}")
    print()

    for category in sorted(categorized_handlers.keys()):
        handler_list = categorized_handlers[category]
        print(f"{category.upper()} ({len(handler_list)} handlers):")

        for addr, name, opcode in sorted(handler_list, key=lambda x: x[2]):
            # Shorten the name for display
            short_name = f"{name[:50]}...{name[-15:]}" if len(name) > 70 else name
            print(f"  {addr} {opcode:12s} {short_name}")
        print()

def get_build_environment():
    """Get the build environment variables."""
    import os
    arch = subprocess.run(["uname", "-m"], capture_output=True, text=True).stdout.strip()
    env = os.environ.copy()

    if arch in ["arm64", "aarch64", "x86_64", "amd64"]:
        env["RUSTFLAGS"] = "-Ctarget-cpu=native"
    else:
        print(f"Warning: Unsupported architecture: {arch}")

    env["JEMALLOC_SYS_WITH_MALLOC_CONF"] = ("retain:true,background_thread:true,"
                                           "metadata_thp:always,dirty_decay_ms:10000,"
                                           "muzzy_decay_ms:10000,abort_conf:true")
    return env

def get_build_command():
    """Get the build command arguments."""
    return [
        "cargo", "+nightly-2025-08-19", "build",
        "--bin", "openvm-reth-benchmark-bin",
        "--profile=profiling",
        "--no-default-features",
        "--features=metrics,jemalloc,tco,unprotected,nightly-features"
    ]

def build_binary(force_rebuild=False):
    """Build the binary using the run.sh build command."""
    status_msg = "Forcing rebuild of binary..." if force_rebuild else "Binary not found, building..."
    print(status_msg)

    try:
        build_cmd = get_build_command()
        env = get_build_environment()

        # Show build command for transparency
        if force_rebuild:
            print(f"Running: {' '.join(build_cmd)}")

        # Show build progress instead of capturing output
        result = subprocess.run(build_cmd, env=env, cwd=".")

        if result.returncode == 0:
            print("Build completed successfully!")

            # Generate dSYM file for source code mapping
            binary_path = find_binary_path()
            if binary_path:
                generate_dsym(binary_path, verbose=True)
            else:
                print("Warning: Could not find binary path for dSYM generation")

            return True
        else:
            print(f"Build failed with exit code: {result.returncode}")
            return False
    except Exception as e:
        print(f"Error building binary: {e}")
        return False

@dataclass
class AnalysisConfig:
    """Configuration for handler analysis."""
    output_file: Optional[str] = None
    offset: int = 0
    first_match: bool = False
    force_rebuild: bool = False
    search_terms: Optional[list] = None

    def __post_init__(self):
        if self.search_terms is None:
            self.search_terms = []

def parse_arguments() -> AnalysisConfig:
    """Parse command line arguments into configuration."""
    args = sys.argv[1:]
    config = AnalysisConfig()

    # Handle --rebuild option
    if "--rebuild" in args:
        config.force_rebuild = True
        args.remove("--rebuild")

    # Handle --output option
    if "--output" in args:
        output_idx = args.index("--output")
        if output_idx + 1 >= len(args):
            print("Error: --output requires a filename")
            sys.exit(1)
        config.output_file = args[output_idx + 1]
        args = args[:output_idx] + args[output_idx + 2:]

    # Handle --offset option
    if "--offset" in args:
        offset_idx = args.index("--offset")
        if offset_idx + 1 >= len(args):
            print("Error: --offset requires a number")
            sys.exit(1)
        try:
            config.offset = int(args[offset_idx + 1])
        except ValueError:
            print("Error: --offset requires an integer")
            sys.exit(1)
        args = args[:offset_idx] + args[offset_idx + 2:]

    # Handle --first option
    if "--first" in args:
        config.first_match = True
        args.remove("--first")

    config.search_terms = args
    return config

def setup_output_redirection(output_file: Optional[str]):
    """Setup output redirection if specified."""
    if output_file:
        return sys.stdout, open(output_file, 'w')
    return None, sys.stdout

def cleanup_output_redirection(original_stdout, current_stdout, output_file: Optional[str]):
    """Cleanup output redirection."""
    if original_stdout:
        current_stdout.close()
        sys.stdout = original_stdout
        print(f"Output saved to {output_file}")

def show_help():
    """Show usage help."""
    print("Usage:")
    print("  analyze_handlers.py [options] [search_terms]")
    print()
    print("Options:")
    print("  --first              Analyze first matching handler")
    print("  --offset <N>         Analyze handler at offset N (0-indexed)")
    print("  --output <file>      Save output to file")
    print("  --rebuild            Force rebuild of binary before analysis")
    print()
    print("Examples:")
    print("  analyze_handlers.py                          # List all execute handlers")
    print("  analyze_handlers.py rv32 base_alu            # Filter rv32 base_alu handlers")
    print("  analyze_handlers.py --first rv32 base_alu    # Analyze first match")
    print("  analyze_handlers.py --offset 2 rv32 base_alu # Analyze 3rd match (0-indexed)")
    print("  analyze_handlers.py --rebuild --first bigint # Rebuild and analyze bigint handler")
    print("  analyze_handlers.py --output out.txt bigint  # Save to file")

def find_matching_handlers(handlers, search_terms):
    """Find handlers matching search terms."""
    matches = []
    for addr, name in handlers:
        category, operation = categorize_handler(name)
        if all(term.lower() in name.lower() or
               term.lower() in category.lower() or
               term.lower() in operation.lower()
               for term in search_terms):
            matches.append((addr, name))
    return matches

def handle_single_handler_analysis(config, binary_path, handlers):
    """Handle analysis of a single handler (first match or offset)."""
    matches = find_matching_handlers(handlers, config.search_terms)

    if not matches:
        print(f"No handlers found matching: {' '.join(config.search_terms)}")
        return

    target_idx = 0 if config.first_match else config.offset

    if target_idx >= len(matches):
        print(f"Offset {config.offset} is out of range. Found {len(matches)} matches (max offset: {len(matches) - 1})")
        return

    addr, name = matches[target_idx]
    if config.first_match:
        print(f"Analyzing first handler matching: {' '.join(config.search_terms)}\n")
    else:
        print(f"Analyzing handler at offset {config.offset} matching: {' '.join(config.search_terms)}\n")

    analyze_execute_handler(binary_path, addr, name)

def handle_address_search(config, binary_path, handlers):
    """Handle search by specific address."""
    if len(config.search_terms) != 1:
        return False

    arg = config.search_terms[0]
    if not (arg.startswith("0x") or (len(arg) == 16 and all(c in "0123456789abcdefABCDEF" for c in arg))):
        return False

    search_addr = arg.lower().replace("0x", "")
    for addr, name in handlers:
        if addr.lower() == search_addr:
            analyze_execute_handler(binary_path, addr, name)
            return True

    print(f"No handler found at address {arg}")
    return True

def show_multiple_matches(matches, search_terms):
    """Show multiple matching handlers grouped by opcode."""
    print(f"Found {len(matches)} handlers matching: {' '.join(search_terms)}")
    print()

    by_opcode = defaultdict(list)
    for addr, name in matches:
        _, opcode = categorize_handler(name)
        by_opcode[opcode].append((addr, name))

    for opcode in sorted(by_opcode.keys()):
        handlers_list = by_opcode[opcode]
        print(f"{opcode.upper()} ({len(handlers_list)} handlers):")
        for addr, name in handlers_list[:5]:
            handler_type = "e1" if "execute_e1_tco_handler" in name else "e2"
            print(f"  [{handler_type}] {addr}")
        if len(handlers_list) > 5:
            print(f"  ... and {len(handlers_list) - 5} more")
        print()
    print("Use --first to analyze the first match")

def main():
    config = parse_arguments()
    original_stdout, current_stdout = setup_output_redirection(config.output_file)
    sys.stdout = current_stdout

    try:
        # Find binary or force rebuild
        binary_path = find_binary_path()
        if not binary_path or config.force_rebuild:
            if not build_binary(config.force_rebuild):
                print("Failed to build binary")
                return
            binary_path = find_binary_path()
            if not binary_path:
                print(f"Binary '{BINARY_NAME}' still not found after build")
                return

        print(f"Using binary: {binary_path}")
        print()

        # Show help and list all handlers if no search terms
        if not config.search_terms:
            list_all_handlers(binary_path)
            show_help()
            return

        # Get handlers
        symbols_text = get_all_symbols(binary_path)
        if not symbols_text:
            return
        handlers = find_execute_handlers(symbols_text)

        # Handle single handler analysis (first match or offset)
        if config.first_match or config.offset > 0:
            handle_single_handler_analysis(config, binary_path, handlers)
            return

        # Handle address search
        if handle_address_search(config, binary_path, handlers):
            return

        # Search by name/category
        matches = find_matching_handlers(handlers, config.search_terms)
        if not matches:
            print(f"No handlers found matching: {' '.join(config.search_terms)}")
            return

        if len(matches) == 1:
            addr, name = matches[0]
            analyze_execute_handler(binary_path, addr, name)
        else:
            show_multiple_matches(matches, config.search_terms)

    finally:
        cleanup_output_redirection(original_stdout, current_stdout, config.output_file)

if __name__ == "__main__":
    main()
