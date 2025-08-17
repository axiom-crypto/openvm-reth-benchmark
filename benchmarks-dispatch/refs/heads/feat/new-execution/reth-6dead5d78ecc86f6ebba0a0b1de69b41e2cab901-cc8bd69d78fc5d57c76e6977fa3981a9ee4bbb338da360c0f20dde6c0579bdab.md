| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total |  350.45 |  47.50 |
| reth.prove_stark.block_21000000 |  207.47 |  17.90 |
| leaf |  85.66 |  8.05 |
| internal.0 |  39.58 |  8.39 |
| internal.1 |  10.69 |  6.11 |
| internal.2 |  4.51 |  4.51 |


| reth.prove_stark.block_21000000 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  14,819.36 |  207,471 |  17,897 |  5,684 |
| `main_cells_used     ` |  807,102,329.64 |  11,299,432,615 |  1,345,207,093 |  190,562,557 |
| `total_cells_used    ` |  1,564,955,655.93 |  21,909,379,183 |  2,055,719,459 |  361,583,411 |
| `execute_e1_time_ms  ` |  963 |  963 |  963 |  963 |
| `execute_e1_insn_mi/s` |  177.46 | -          |  177.46 |  177.46 |
| `execute_metered_time_ms` |  1,579 | -          | -          | -          |
| `execute_metered_insn_mi/s` |  108.26 | -          |  108.26 |  108.26 |
| `execute_preflight_insns` |  12,214,459.50 |  171,002,433 |  16,325,000 |  2,062,433 |
| `execute_preflight_time_ms` |  685.43 |  9,596 |  1,830 |  188 |
| `execute_preflight_insn_mi/s` |  30.51 | -          |  35.70 |  5.74 |
| `trace_gen_time_ms   ` |  2,732.36 |  38,253 |  4,682 |  1,121 |
| `memory_finalize_time_ms` |  28 |  392 |  49 |  13 |
| `stark_prove_excluding_trace_time_ms` |  11,220.86 |  157,092 |  15,409 |  4,196 |
| `main_trace_commit_time_ms` |  2,973.36 |  41,627 |  5,699 |  1,207 |
| `generate_perm_trace_time_ms` |  743.36 |  10,407 |  967 |  200 |
| `perm_trace_commit_time_ms` |  1,962.71 |  27,478 |  2,318 |  521 |
| `quotient_poly_compute_time_ms` |  1,987.64 |  27,827 |  4,070 |  656 |
| `quotient_poly_commit_time_ms` |  659.57 |  9,234 |  922 |  307 |
| `pcs_opening_time_ms ` |  2,881.93 |  40,347 |  3,517 |  1,290 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  6,118.64 |  85,661 |  8,054 |  4,043 |
| `main_cells_used     ` |  214,560,718.21 |  3,003,850,055 |  312,252,945 |  101,453,950 |
| `total_cells_used    ` |  540,697,372.79 |  7,569,763,219 |  802,875,131 |  233,739,624 |
| `execute_preflight_insns` |  2,449,773.36 |  34,296,827 |  3,438,764 |  1,458,332 |
| `execute_preflight_time_ms` |  643.43 |  9,008 |  747 |  535 |
| `execute_preflight_insn_mi/s` |  5 | -          |  5.97 |  3.79 |
| `trace_gen_time_ms   ` |  427 |  5,978 |  621 |  197 |
| `memory_finalize_time_ms` |  20.86 |  292 |  57 |  10 |
| `stark_prove_excluding_trace_time_ms` |  4,206.93 |  58,897 |  5,857 |  2,480 |
| `main_trace_commit_time_ms` |  667.07 |  9,339 |  937 |  377 |
| `generate_perm_trace_time_ms` |  312.36 |  4,373 |  435 |  159 |
| `perm_trace_commit_time_ms` |  913.36 |  12,787 |  1,395 |  432 |
| `quotient_poly_compute_time_ms` |  496.93 |  6,957 |  706 |  249 |
| `quotient_poly_commit_time_ms` |  346.57 |  4,852 |  512 |  234 |
| `pcs_opening_time_ms ` |  1,464.79 |  20,507 |  1,996 |  1,014 |

| internal.0 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  7,915 |  39,575 |  8,388 |  6,704 |
| `main_cells_used     ` |  201,846,345 |  1,009,231,725 |  219,307,130 |  141,242,457 |
| `total_cells_used    ` |  359,460,137.40 |  1,797,300,687 |  389,822,564 |  251,901,971 |
| `execute_preflight_insns` |  3,281,775.20 |  16,408,876 |  3,545,858 |  2,333,488 |
| `execute_preflight_time_ms` |  1,067.40 |  5,337 |  1,165 |  793 |
| `execute_preflight_insn_mi/s` |  3.59 | -          |  3.64 |  3.51 |
| `trace_gen_time_ms   ` |  432.20 |  2,161 |  465 |  313 |
| `memory_finalize_time_ms` |  12.20 |  61 |  14 |  11 |
| `stark_prove_excluding_trace_time_ms` |  5,571 |  27,855 |  5,968 |  4,765 |
| `main_trace_commit_time_ms` |  1,081.20 |  5,406 |  1,178 |  847 |
| `generate_perm_trace_time_ms` |  239 |  1,195 |  256 |  204 |
| `perm_trace_commit_time_ms` |  817.20 |  4,086 |  860 |  723 |
| `quotient_poly_compute_time_ms` |  771 |  3,855 |  820 |  620 |
| `quotient_poly_commit_time_ms` |  874.80 |  4,374 |  992 |  713 |
| `pcs_opening_time_ms ` |  1,782.80 |  8,914 |  1,857 |  1,653 |

| internal.1 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  5,344 |  10,688 |  6,110 |  4,578 |
| `main_cells_used     ` |  99,578,496.50 |  199,156,993 |  117,149,042 |  82,007,951 |
| `total_cells_used    ` |  178,123,702.50 |  356,247,405 |  208,594,932 |  147,652,473 |
| `execute_preflight_insns` |  1,948,854.50 |  3,897,709 |  2,338,827 |  1,558,882 |
| `execute_preflight_time_ms` |  539 |  1,078 |  617 |  461 |
| `execute_preflight_insn_mi/s` |  5.01 | -          |  5.04 |  4.99 |
| `trace_gen_time_ms   ` |  238.50 |  477 |  282 |  195 |
| `memory_finalize_time_ms` |  10 |  20 |  11 |  9 |
| `stark_prove_excluding_trace_time_ms` |  3,723.50 |  7,447 |  4,368 |  3,079 |
| `main_trace_commit_time_ms` |  648.50 |  1,297 |  786 |  511 |
| `generate_perm_trace_time_ms` |  147 |  294 |  172 |  122 |
| `perm_trace_commit_time_ms` |  501 |  1,002 |  612 |  390 |
| `quotient_poly_compute_time_ms` |  455.50 |  911 |  542 |  369 |
| `quotient_poly_commit_time_ms` |  697.50 |  1,395 |  862 |  533 |
| `pcs_opening_time_ms ` |  1,269.50 |  2,539 |  1,389 |  1,150 |

| internal.2 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  4,509 |  4,509 |  4,509 |  4,509 |
| `main_cells_used     ` |  81,343,205 |  81,343,205 |  81,343,205 |  81,343,205 |
| `total_cells_used    ` |  146,722,167 |  146,722,167 |  146,722,167 |  146,722,167 |
| `execute_preflight_insns` |  1,550,610 |  1,550,610 |  1,550,610 |  1,550,610 |
| `execute_preflight_time_ms` |  456 |  456 |  456 |  456 |
| `execute_preflight_insn_mi/s` |  5.06 | -          |  5.06 |  5.06 |
| `trace_gen_time_ms   ` |  199 |  199 |  199 |  199 |
| `memory_finalize_time_ms` |  10 |  10 |  10 |  10 |
| `stark_prove_excluding_trace_time_ms` |  3,013 |  3,013 |  3,013 |  3,013 |
| `main_trace_commit_time_ms` |  489 |  489 |  489 |  489 |
| `generate_perm_trace_time_ms` |  164 |  164 |  164 |  164 |
| `perm_trace_commit_time_ms` |  405 |  405 |  405 |  405 |
| `quotient_poly_compute_time_ms` |  376 |  376 |  376 |  376 |
| `quotient_poly_commit_time_ms` |  526 |  526 |  526 |  526 |
| `pcs_opening_time_ms ` |  1,047 |  1,047 |  1,047 |  1,047 |

| agg_keygen |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  3,240 |  6,480 |  5,049 |  1,431 |
| `main_cells_used     ` |  46,038,424 |  92,076,848 |  91,157,468 |  919,380 |
| `total_cells_used    ` |  115,856,768 |  231,713,536 |  222,207,970 |  9,505,566 |
| `execute_metered_time_ms` |  0 | -          | -          | -          |
| `execute_metered_insn_mi/s` |  0.20 | -          |  0.20 |  0.20 |
| `execute_preflight_insns` |  811,171.50 |  1,622,343 |  1,622,342 |  1 |
| `execute_preflight_time_ms` |  150.50 |  301 |  299 |  2 |
| `execute_preflight_insn_mi/s` |  9,223,372,036,854,775,807 | -          |  9,223,372,036,854,775,807 |  5.12 |
| `trace_gen_time_ms   ` |  65 |  130 |  109 |  21 |
| `memory_finalize_time_ms` |  5.50 |  11 |  9 |  2 |
| `stark_prove_excluding_trace_time_ms` |  1,863.50 |  3,727 |  3,276 |  451 |
| `main_trace_commit_time_ms` |  300 |  600 |  549 |  51 |
| `generate_perm_trace_time_ms` |  87 |  174 |  156 |  18 |
| `perm_trace_commit_time_ms` |  239.50 |  479 |  435 |  44 |
| `quotient_poly_compute_time_ms` |  184.50 |  369 |  349 |  20 |
| `quotient_poly_commit_time_ms` |  320 |  640 |  584 |  56 |
| `pcs_opening_time_ms ` |  727 |  1,454 |  1,198 |  256 |



<details>
<summary>Detailed Metrics</summary>

|  | reth-block_time_ms |
| --- |
|  | 378,585 | 

| air_name | block_number | quotient_deg | interactions | constraints |
| --- | --- | --- | --- | --- |
| AccessAdapterAir<16> | 21000000 | 2 | 5 | 12 | 
| AccessAdapterAir<2> | 21000000 | 2 | 5 | 12 | 
| AccessAdapterAir<32> | 21000000 | 2 | 5 | 12 | 
| AccessAdapterAir<4> | 21000000 | 2 | 5 | 12 | 
| AccessAdapterAir<8> | 21000000 | 2 | 5 | 12 | 
| BitwiseOperationLookupAir<8> | 21000000 | 2 | 2 | 4 | 
| KeccakVmAir | 21000000 | 2 | 321 | 4,513 | 
| MemoryMerkleAir<8> | 21000000 | 2 | 4 | 39 | 
| PersistentBoundaryAir<8> | 21000000 | 2 | 3 | 7 | 
| PhantomAir | 21000000 | 2 | 3 | 5 | 
| Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 2 | 1 | 286 | 
| ProgramAir | 21000000 | 1 | 1 | 4 | 
| RangeTupleCheckerAir<2> | 21000000 | 1 | 1 | 4 | 
| Rv32HintStoreAir | 21000000 | 2 | 18 | 28 | 
| Sha256VmAir | 21000000 | 2 | 50 | 663 | 
| VariableRangeCheckerAir | 21000000 | 1 | 1 | 4 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 2 | 20 | 37 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 2 | 18 | 40 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 2 | 24 | 91 | 
| VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 2 | 11 | 20 | 
| VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 2 | 13 | 35 | 
| VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 2 | 10 | 18 | 
| VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 2 | 61 | 126 | 
| VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 2 | 31 | 129 | 
| VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 2 | 61 | 57 | 
| VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 2 | 79 | 2,161 | 
| VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 2 | 20 | 55 | 
| VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchLessThanCoreAir<32, 8> | 21000000 | 2 | 22 | 126 | 
| VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 2 | 25 | 225 | 
| VmAirWrapper<Rv32IsEqualModAdapterAir<2, 3, 16, 48>, ModularIsEqualCoreAir<48, 4, 8> | 21000000 | 2 | 41 | 333 | 
| VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 2 | 16 | 20 | 
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 2 | 18 | 33 | 
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 2 | 17 | 40 | 
| VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 2 | 25 | 84 | 
| VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 2 | 24 | 31 | 
| VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 2 | 19 | 19 | 
| VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 2 | 12 | 14 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 2 | 415 | 480 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 6, 6, 16, 16>, FieldExpressionCoreAir> | 21000000 | 2 | 832 | 921 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 2 | 158 | 190 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 2 | 428 | 457 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 3, 3, 16, 16>, FieldExpressionCoreAir> | 21000000 | 2 | 246 | 288 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 6, 6, 16, 16>, FieldExpressionCoreAir> | 21000000 | 2 | 668 | 701 | 
| VmConnectorAir | 21000000 | 2 | 5 | 11 | 

| block_number | vm.create_initial_state_time_ms | memory_to_vec_partition_time_ms | execute_e1_time_ms | dummy_proof_and_keygen_time_ms | app proof_time_ms | agg_layer_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| 21000000 | 0 | 21 | 1,218 | 13,467 | 209,526 | 4,511 | 

| group | air_name | block_number | rows | quotient_deg | prep_cols | perm_cols | main_cols | interactions | constraints | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | AccessAdapterAir<16> | 21000000 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<2> | 21000000 | 524,288 | 8 |  | 16 | 11 | 5 | 12 | 14,155,776 | 
| agg_keygen | AccessAdapterAir<32> | 21000000 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<4> | 21000000 | 262,144 | 8 |  | 16 | 13 | 5 | 12 | 7,602,176 | 
| agg_keygen | AccessAdapterAir<8> | 21000000 | 8,192 | 8 |  | 16 | 17 | 5 | 12 | 270,336 | 
| agg_keygen | BitwiseOperationLookupAir<8> | 21000000 |  | 2 |  |  |  | 2 | 4 |  | 
| agg_keygen | FriReducedOpeningAir | 21000000 | 524,288 | 8 |  | 84 | 27 | 39 | 71 | 58,195,968 | 
| agg_keygen | JalRangeCheckAir | 21000000 | 65,536 | 8 |  | 28 | 12 | 9 | 14 | 2,621,440 | 
| agg_keygen | MemoryMerkleAir<8> | 21000000 |  | 2 |  |  |  | 4 | 39 |  | 
| agg_keygen | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 65,536 | 8 |  | 312 | 398 | 136 | 572 | 46,530,560 | 
| agg_keygen | PersistentBoundaryAir<8> | 21000000 |  | 2 |  |  |  | 3 | 7 |  | 
| agg_keygen | PhantomAir | 21000000 | 32,768 | 4 |  | 12 | 6 | 3 | 5 | 589,824 | 
| agg_keygen | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 |  | 2 |  |  |  | 1 | 286 |  | 
| agg_keygen | ProgramAir | 21000000 | 131,072 | 1 |  | 8 | 10 | 1 | 4 | 2,359,296 | 
| agg_keygen | RangeTupleCheckerAir<2> | 21000000 |  | 1 |  |  |  | 1 | 4 |  | 
| agg_keygen | Rv32HintStoreAir | 21000000 |  | 2 |  |  |  | 18 | 28 |  | 
| agg_keygen | VariableRangeCheckerAir | 21000000 | 262,144 | 1 | 2 | 8 | 1 | 1 | 4 | 2,359,296 | 
| agg_keygen | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 1,048,576 | 8 |  | 36 | 29 | 15 | 27 | 68,157,440 | 
| agg_keygen | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 262,144 | 8 |  | 28 | 23 | 11 | 25 | 13,369,344 | 
| agg_keygen | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 64 | 8 |  | 28 | 27 | 11 | 30 | 3,520 | 
| agg_keygen | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 524,288 | 8 |  | 40 | 21 | 15 | 20 | 31,981,568 | 
| agg_keygen | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 131,072 | 8 |  | 40 | 27 | 15 | 20 | 8,781,824 | 
| agg_keygen | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 131,072 | 8 |  | 36 | 38 | 15 | 27 | 9,699,328 | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 20 | 37 |  | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 18 | 40 |  | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 24 | 91 |  | 
| agg_keygen | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 |  | 2 |  |  |  | 11 | 20 |  | 
| agg_keygen | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 13 | 35 |  | 
| agg_keygen | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 |  | 2 |  |  |  | 10 | 18 |  | 
| agg_keygen | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 |  | 2 |  |  |  | 16 | 20 |  | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 18 | 33 |  | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 |  | 2 |  |  |  | 17 | 40 |  | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 25 | 84 |  | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 24 | 31 |  | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 19 | 19 |  | 
| agg_keygen | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 |  | 2 |  |  |  | 12 | 14 |  | 
| agg_keygen | VmConnectorAir | 21000000 | 2 | 8 | 1 | 16 | 5 | 5 | 11 | 42 | 
| agg_keygen | VolatileBoundaryAir | 21000000 | 131,072 | 8 |  | 20 | 12 | 7 | 19 | 4,194,304 | 

| group | air_name | block_number | idx | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | AccessAdapterAir<2> | 21000000 | 0 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 1 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 2 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 3 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 4 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 0 | 524,288 |  | 12 | 13 | 13,107,200 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 1 | 524,288 |  | 12 | 13 | 13,107,200 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 2 | 524,288 |  | 12 | 13 | 13,107,200 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 3 | 524,288 |  | 12 | 13 | 13,107,200 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 4 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 0 | 16,384 |  | 12 | 17 | 475,136 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 1 | 16,384 |  | 12 | 17 | 475,136 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 2 | 16,384 |  | 12 | 17 | 475,136 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 3 | 16,384 |  | 12 | 17 | 475,136 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 4 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 0 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 1 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 2 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 3 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 4 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | JalRangeCheckAir | 21000000 | 0 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | JalRangeCheckAir | 21000000 | 1 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | JalRangeCheckAir | 21000000 | 2 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | JalRangeCheckAir | 21000000 | 3 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | JalRangeCheckAir | 21000000 | 4 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 0 | 262,144 |  | 160 | 398 | 146,276,352 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 1 | 262,144 |  | 160 | 398 | 146,276,352 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 2 | 262,144 |  | 160 | 398 | 146,276,352 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 3 | 262,144 |  | 160 | 398 | 146,276,352 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 4 | 131,072 |  | 160 | 398 | 73,138,176 | 
| internal.0 | PhantomAir | 21000000 | 0 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 1 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 2 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 3 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 4 | 32,768 |  | 8 | 6 | 458,752 | 
| internal.0 | ProgramAir | 21000000 | 0 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 21000000 | 1 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 21000000 | 2 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 21000000 | 3 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 21000000 | 4 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 0 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 1 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 2 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 3 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 4 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 0 | 524,288 |  | 16 | 23 | 20,447,232 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 1 | 524,288 |  | 16 | 23 | 20,447,232 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 2 | 524,288 |  | 16 | 23 | 20,447,232 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 3 | 524,288 |  | 16 | 23 | 20,447,232 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 4 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 0 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 1 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 2 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 3 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 4 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 0 | 1,048,576 |  | 24 | 21 | 47,185,920 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 1 | 1,048,576 |  | 24 | 21 | 47,185,920 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 2 | 1,048,576 |  | 24 | 21 | 47,185,920 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 3 | 1,048,576 |  | 24 | 21 | 47,185,920 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 4 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 0 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 1 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 2 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 3 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 4 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 0 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 1 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 2 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 3 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 4 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmConnectorAir | 21000000 | 0 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VmConnectorAir | 21000000 | 1 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VmConnectorAir | 21000000 | 2 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VmConnectorAir | 21000000 | 3 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VmConnectorAir | 21000000 | 4 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 0 | 262,144 |  | 12 | 12 | 6,291,456 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 1 | 262,144 |  | 12 | 12 | 6,291,456 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 2 | 262,144 |  | 12 | 12 | 6,291,456 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 3 | 262,144 |  | 12 | 12 | 6,291,456 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 4 | 262,144 |  | 12 | 12 | 6,291,456 | 
| internal.1 | AccessAdapterAir<2> | 21000000 | 5 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.1 | AccessAdapterAir<2> | 21000000 | 6 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.1 | AccessAdapterAir<4> | 21000000 | 5 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.1 | AccessAdapterAir<4> | 21000000 | 6 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.1 | AccessAdapterAir<8> | 21000000 | 5 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.1 | AccessAdapterAir<8> | 21000000 | 6 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.1 | FriReducedOpeningAir | 21000000 | 5 | 524,288 |  | 44 | 27 | 37,224,448 | 
| internal.1 | FriReducedOpeningAir | 21000000 | 6 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.1 | JalRangeCheckAir | 21000000 | 5 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.1 | JalRangeCheckAir | 21000000 | 6 | 65,536 |  | 16 | 12 | 1,835,008 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 5 | 131,072 |  | 160 | 398 | 73,138,176 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 6 | 65,536 |  | 160 | 398 | 36,569,088 | 
| internal.1 | PhantomAir | 21000000 | 5 | 32,768 |  | 8 | 6 | 458,752 | 
| internal.1 | PhantomAir | 21000000 | 6 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.1 | ProgramAir | 21000000 | 5 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.1 | ProgramAir | 21000000 | 6 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 21000000 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 21000000 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 5 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 6 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 5 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 6 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 5 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 6 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 5 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 6 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 5 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 6 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 5 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 6 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.1 | VmConnectorAir | 21000000 | 5 | 2 | 1 | 12 | 5 | 34 | 
| internal.1 | VmConnectorAir | 21000000 | 6 | 2 | 1 | 12 | 5 | 34 | 
| internal.1 | VolatileBoundaryAir | 21000000 | 5 | 262,144 |  | 12 | 12 | 6,291,456 | 
| internal.1 | VolatileBoundaryAir | 21000000 | 6 | 131,072 |  | 12 | 12 | 3,145,728 | 
| internal.2 | AccessAdapterAir<2> | 21000000 | 7 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.2 | AccessAdapterAir<4> | 21000000 | 7 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.2 | AccessAdapterAir<8> | 21000000 | 7 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.2 | FriReducedOpeningAir | 21000000 | 7 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.2 | JalRangeCheckAir | 21000000 | 7 | 65,536 |  | 16 | 12 | 1,835,008 | 
| internal.2 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 7 | 65,536 |  | 160 | 398 | 36,569,088 | 
| internal.2 | PhantomAir | 21000000 | 7 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.2 | ProgramAir | 21000000 | 7 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.2 | VariableRangeCheckerAir | 21000000 | 7 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.2 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 7 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.2 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 7 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 7 | 64 |  | 16 | 23 | 2,496 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 7 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 7 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 7 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.2 | VmConnectorAir | 21000000 | 7 | 2 | 1 | 12 | 5 | 34 | 
| internal.2 | VolatileBoundaryAir | 21000000 | 7 | 262,144 |  | 12 | 12 | 6,291,456 | 
| leaf | AccessAdapterAir<2> | 21000000 | 0 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<2> | 21000000 | 1 | 524,288 |  | 16 | 11 | 14,155,776 | 
| leaf | AccessAdapterAir<2> | 21000000 | 10 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 21000000 | 11 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<2> | 21000000 | 12 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<2> | 21000000 | 13 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<2> | 21000000 | 2 | 524,288 |  | 16 | 11 | 14,155,776 | 
| leaf | AccessAdapterAir<2> | 21000000 | 3 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<2> | 21000000 | 4 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<2> | 21000000 | 5 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<2> | 21000000 | 6 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 21000000 | 7 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 21000000 | 8 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 21000000 | 9 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<4> | 21000000 | 0 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 21000000 | 1 | 262,144 |  | 16 | 13 | 7,602,176 | 
| leaf | AccessAdapterAir<4> | 21000000 | 10 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 21000000 | 11 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 21000000 | 12 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 21000000 | 13 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 21000000 | 2 | 262,144 |  | 16 | 13 | 7,602,176 | 
| leaf | AccessAdapterAir<4> | 21000000 | 3 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 21000000 | 4 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 21000000 | 5 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 21000000 | 6 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 21000000 | 7 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 21000000 | 8 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 21000000 | 9 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<8> | 21000000 | 0 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 21000000 | 1 | 4,096 |  | 16 | 17 | 135,168 | 
| leaf | AccessAdapterAir<8> | 21000000 | 10 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 21000000 | 11 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 21000000 | 12 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 21000000 | 13 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 21000000 | 2 | 4,096 |  | 16 | 17 | 135,168 | 
| leaf | AccessAdapterAir<8> | 21000000 | 3 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 21000000 | 4 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 21000000 | 5 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 21000000 | 6 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 21000000 | 7 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 21000000 | 8 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 21000000 | 9 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | FriReducedOpeningAir | 21000000 | 0 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | FriReducedOpeningAir | 21000000 | 1 | 524,288 |  | 84 | 27 | 58,195,968 | 
| leaf | FriReducedOpeningAir | 21000000 | 10 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 21000000 | 11 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | FriReducedOpeningAir | 21000000 | 12 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | FriReducedOpeningAir | 21000000 | 13 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | FriReducedOpeningAir | 21000000 | 2 | 524,288 |  | 84 | 27 | 58,195,968 | 
| leaf | FriReducedOpeningAir | 21000000 | 3 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | FriReducedOpeningAir | 21000000 | 4 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | FriReducedOpeningAir | 21000000 | 5 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | FriReducedOpeningAir | 21000000 | 6 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 21000000 | 7 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 21000000 | 8 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 21000000 | 9 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | JalRangeCheckAir | 21000000 | 0 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 1 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 10 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 11 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 12 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 13 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 2 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 3 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 4 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 5 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 6 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 7 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 8 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 9 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 0 | 131,072 |  | 312 | 398 | 93,061,120 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 1 | 131,072 |  | 312 | 398 | 93,061,120 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 10 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 11 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 12 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 13 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 2 | 131,072 |  | 312 | 398 | 93,061,120 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 3 | 131,072 |  | 312 | 398 | 93,061,120 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 4 | 131,072 |  | 312 | 398 | 93,061,120 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 5 | 131,072 |  | 312 | 398 | 93,061,120 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 6 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 7 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 8 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 9 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | PhantomAir | 21000000 | 0 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 21000000 | 1 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 21000000 | 10 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 21000000 | 11 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 21000000 | 12 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 21000000 | 13 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 21000000 | 2 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 21000000 | 3 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 21000000 | 4 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 21000000 | 5 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 21000000 | 6 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 21000000 | 7 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 21000000 | 8 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 21000000 | 9 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | ProgramAir | 21000000 | 0 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 21000000 | 1 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 21000000 | 10 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 21000000 | 11 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 21000000 | 12 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 21000000 | 13 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 21000000 | 2 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 21000000 | 3 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 21000000 | 4 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 21000000 | 5 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 21000000 | 6 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 21000000 | 7 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 21000000 | 8 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 21000000 | 9 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | VariableRangeCheckerAir | 21000000 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 10 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 11 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 12 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 13 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 7 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 8 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 9 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 0 | 1,048,576 |  | 36 | 29 | 68,157,440 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 1 | 1,048,576 |  | 36 | 29 | 68,157,440 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 10 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 11 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 12 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 13 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 2 | 1,048,576 |  | 36 | 29 | 68,157,440 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 3 | 1,048,576 |  | 36 | 29 | 68,157,440 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 4 | 1,048,576 |  | 36 | 29 | 68,157,440 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 5 | 1,048,576 |  | 36 | 29 | 68,157,440 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 6 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 7 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 8 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 9 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 0 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 1 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 10 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 11 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 12 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 13 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 2 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 3 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 4 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 5 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 6 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 7 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 8 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 9 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 0 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 1 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 10 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 11 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 12 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 13 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 2 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 3 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 4 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 5 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 6 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 7 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 8 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 9 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 0 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 1 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 10 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 11 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 12 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 13 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 2 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 3 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 4 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 5 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 6 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 7 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 8 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 9 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 0 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 1 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 10 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 11 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 12 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 13 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 2 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 3 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 4 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 5 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 6 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 7 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 8 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 9 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 0 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 1 | 131,072 |  | 36 | 38 | 9,699,328 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 10 | 524,288 |  | 36 | 38 | 38,797,312 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 11 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 12 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 13 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 2 | 131,072 |  | 36 | 38 | 9,699,328 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 3 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 4 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 5 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 6 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 7 | 524,288 |  | 36 | 38 | 38,797,312 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 8 | 524,288 |  | 36 | 38 | 38,797,312 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 9 | 524,288 |  | 36 | 38 | 38,797,312 | 
| leaf | VmConnectorAir | 21000000 | 0 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 1 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 10 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 11 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 12 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 13 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 2 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 3 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 4 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 5 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 6 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 7 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 8 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 9 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VolatileBoundaryAir | 21000000 | 0 | 524,288 |  | 20 | 12 | 16,777,216 | 
| leaf | VolatileBoundaryAir | 21000000 | 1 | 131,072 |  | 20 | 12 | 4,194,304 | 
| leaf | VolatileBoundaryAir | 21000000 | 10 | 1,048,576 |  | 20 | 12 | 33,554,432 | 
| leaf | VolatileBoundaryAir | 21000000 | 11 | 524,288 |  | 20 | 12 | 16,777,216 | 
| leaf | VolatileBoundaryAir | 21000000 | 12 | 524,288 |  | 20 | 12 | 16,777,216 | 
| leaf | VolatileBoundaryAir | 21000000 | 13 | 524,288 |  | 20 | 12 | 16,777,216 | 
| leaf | VolatileBoundaryAir | 21000000 | 2 | 131,072 |  | 20 | 12 | 4,194,304 | 
| leaf | VolatileBoundaryAir | 21000000 | 3 | 524,288 |  | 20 | 12 | 16,777,216 | 
| leaf | VolatileBoundaryAir | 21000000 | 4 | 524,288 |  | 20 | 12 | 16,777,216 | 
| leaf | VolatileBoundaryAir | 21000000 | 5 | 524,288 |  | 20 | 12 | 16,777,216 | 
| leaf | VolatileBoundaryAir | 21000000 | 6 | 1,048,576 |  | 20 | 12 | 33,554,432 | 
| leaf | VolatileBoundaryAir | 21000000 | 7 | 1,048,576 |  | 20 | 12 | 33,554,432 | 
| leaf | VolatileBoundaryAir | 21000000 | 8 | 1,048,576 |  | 20 | 12 | 33,554,432 | 
| leaf | VolatileBoundaryAir | 21000000 | 9 | 1,048,576 |  | 20 | 12 | 33,554,432 | 

| group | air_name | block_number | segment | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | AccessAdapterAir<16> | 21000000 | 0 | 1 |  | 16 | 25 | 41 | 
| agg_keygen | AccessAdapterAir<2> | 21000000 | 0 | 1 |  | 16 | 11 | 27 | 
| agg_keygen | AccessAdapterAir<32> | 21000000 | 0 | 1 |  | 16 | 41 | 57 | 
| agg_keygen | AccessAdapterAir<4> | 21000000 | 0 | 1 |  | 16 | 13 | 29 | 
| agg_keygen | AccessAdapterAir<8> | 21000000 | 0 | 1 |  | 16 | 17 | 33 | 
| agg_keygen | BitwiseOperationLookupAir<8> | 21000000 | 0 | 65,536 | 3 | 8 | 2 | 655,360 | 
| agg_keygen | MemoryMerkleAir<8> | 21000000 | 0 | 64 |  | 16 | 32 | 3,072 | 
| agg_keygen | PersistentBoundaryAir<8> | 21000000 | 0 | 1 |  | 12 | 20 | 32 | 
| agg_keygen | PhantomAir | 21000000 | 0 | 1 |  | 12 | 6 | 18 | 
| agg_keygen | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 0 | 32 |  | 8 | 300 | 9,856 | 
| agg_keygen | ProgramAir | 21000000 | 0 | 1 |  | 8 | 10 | 18 | 
| agg_keygen | RangeTupleCheckerAir<2> | 21000000 | 0 | 524,288 | 2 | 8 | 1 | 4,718,592 | 
| agg_keygen | Rv32HintStoreAir | 21000000 | 0 | 1 |  | 44 | 32 | 76 | 
| agg_keygen | VariableRangeCheckerAir | 21000000 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 0 | 1 |  | 52 | 36 | 88 | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 0 | 1 |  | 40 | 37 | 77 | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 0 | 1 |  | 52 | 53 | 105 | 
| agg_keygen | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 0 | 1 |  | 28 | 26 | 54 | 
| agg_keygen | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 0 | 1 |  | 32 | 32 | 64 | 
| agg_keygen | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 0 | 1 |  | 28 | 18 | 46 | 
| agg_keygen | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 0 | 1 |  | 36 | 28 | 64 | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 0 | 1 |  | 52 | 36 | 88 | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 0 | 1 |  | 52 | 41 | 93 | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 0 | 1 |  | 72 | 59 | 131 | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 0 | 1 |  | 72 | 39 | 111 | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 0 | 1 |  | 52 | 31 | 83 | 
| agg_keygen | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 0 | 1 |  | 28 | 20 | 48 | 
| agg_keygen | VmConnectorAir | 21000000 | 0 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<16> | 21000000 | 10 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<16> | 21000000 | 11 | 65,536 |  | 16 | 25 | 2,686,976 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<16> | 21000000 | 12 | 2,048 |  | 16 | 25 | 83,968 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<16> | 21000000 | 13 | 16 |  | 16 | 25 | 656 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<16> | 21000000 | 6 | 131,072 |  | 16 | 25 | 5,373,952 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<16> | 21000000 | 7 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<16> | 21000000 | 8 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<16> | 21000000 | 9 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<32> | 21000000 | 10 | 131,072 |  | 16 | 41 | 7,471,104 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<32> | 21000000 | 11 | 32,768 |  | 16 | 41 | 1,867,776 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<32> | 21000000 | 12 | 1,024 |  | 16 | 41 | 58,368 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<32> | 21000000 | 13 | 8 |  | 16 | 41 | 456 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<32> | 21000000 | 6 | 65,536 |  | 16 | 41 | 3,735,552 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<32> | 21000000 | 7 | 131,072 |  | 16 | 41 | 7,471,104 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<32> | 21000000 | 8 | 131,072 |  | 16 | 41 | 7,471,104 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<32> | 21000000 | 9 | 131,072 |  | 16 | 41 | 7,471,104 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<8> | 21000000 | 0 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<8> | 21000000 | 1 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<8> | 21000000 | 10 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<8> | 21000000 | 11 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<8> | 21000000 | 12 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<8> | 21000000 | 13 | 524,288 |  | 16 | 17 | 17,301,504 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<8> | 21000000 | 2 | 2,097,152 |  | 16 | 17 | 69,206,016 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<8> | 21000000 | 3 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<8> | 21000000 | 4 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<8> | 21000000 | 5 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<8> | 21000000 | 6 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<8> | 21000000 | 7 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<8> | 21000000 | 8 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_stark.block_21000000 | AccessAdapterAir<8> | 21000000 | 9 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_stark.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 0 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_stark.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 1 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_stark.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 10 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_stark.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 11 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_stark.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 12 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_stark.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 13 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_stark.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 2 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_stark.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 3 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_stark.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 4 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_stark.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 5 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_stark.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 6 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_stark.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 7 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_stark.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 8 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_stark.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 9 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_stark.block_21000000 | KeccakVmAir | 21000000 | 0 | 16,384 |  | 1,056 | 3,163 | 69,124,096 | 
| reth.prove_stark.block_21000000 | KeccakVmAir | 21000000 | 10 | 16,384 |  | 1,056 | 3,163 | 69,124,096 | 
| reth.prove_stark.block_21000000 | KeccakVmAir | 21000000 | 11 | 262,144 |  | 1,056 | 3,163 | 1,105,985,536 | 
| reth.prove_stark.block_21000000 | KeccakVmAir | 21000000 | 12 | 262,144 |  | 1,056 | 3,163 | 1,105,985,536 | 
| reth.prove_stark.block_21000000 | KeccakVmAir | 21000000 | 13 | 32,768 |  | 1,056 | 3,163 | 138,248,192 | 
| reth.prove_stark.block_21000000 | KeccakVmAir | 21000000 | 3 | 262,144 |  | 1,056 | 3,163 | 1,105,985,536 | 
| reth.prove_stark.block_21000000 | KeccakVmAir | 21000000 | 4 | 262,144 |  | 1,056 | 3,163 | 1,105,985,536 | 
| reth.prove_stark.block_21000000 | KeccakVmAir | 21000000 | 5 | 524,288 |  | 1,056 | 3,163 | 2,211,971,072 | 
| reth.prove_stark.block_21000000 | KeccakVmAir | 21000000 | 6 | 262,144 |  | 1,056 | 3,163 | 1,105,985,536 | 
| reth.prove_stark.block_21000000 | KeccakVmAir | 21000000 | 7 | 32,768 |  | 1,056 | 3,163 | 138,248,192 | 
| reth.prove_stark.block_21000000 | KeccakVmAir | 21000000 | 8 | 16,384 |  | 1,056 | 3,163 | 69,124,096 | 
| reth.prove_stark.block_21000000 | KeccakVmAir | 21000000 | 9 | 16,384 |  | 1,056 | 3,163 | 69,124,096 | 
| reth.prove_stark.block_21000000 | MemoryMerkleAir<8> | 21000000 | 0 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_stark.block_21000000 | MemoryMerkleAir<8> | 21000000 | 1 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_stark.block_21000000 | MemoryMerkleAir<8> | 21000000 | 10 | 524,288 |  | 16 | 32 | 25,165,824 | 
| reth.prove_stark.block_21000000 | MemoryMerkleAir<8> | 21000000 | 11 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_stark.block_21000000 | MemoryMerkleAir<8> | 21000000 | 12 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_stark.block_21000000 | MemoryMerkleAir<8> | 21000000 | 13 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_stark.block_21000000 | MemoryMerkleAir<8> | 21000000 | 2 | 2,097,152 |  | 16 | 32 | 100,663,296 | 
| reth.prove_stark.block_21000000 | MemoryMerkleAir<8> | 21000000 | 3 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_stark.block_21000000 | MemoryMerkleAir<8> | 21000000 | 4 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_stark.block_21000000 | MemoryMerkleAir<8> | 21000000 | 5 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_stark.block_21000000 | MemoryMerkleAir<8> | 21000000 | 6 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_stark.block_21000000 | MemoryMerkleAir<8> | 21000000 | 7 | 262,144 |  | 16 | 32 | 12,582,912 | 
| reth.prove_stark.block_21000000 | MemoryMerkleAir<8> | 21000000 | 8 | 262,144 |  | 16 | 32 | 12,582,912 | 
| reth.prove_stark.block_21000000 | MemoryMerkleAir<8> | 21000000 | 9 | 524,288 |  | 16 | 32 | 25,165,824 | 
| reth.prove_stark.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 0 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_stark.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 1 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_stark.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 10 | 262,144 |  | 12 | 20 | 8,388,608 | 
| reth.prove_stark.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 11 | 524,288 |  | 12 | 20 | 16,777,216 | 
| reth.prove_stark.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 12 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_stark.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 13 | 524,288 |  | 12 | 20 | 16,777,216 | 
| reth.prove_stark.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 2 | 2,097,152 |  | 12 | 20 | 67,108,864 | 
| reth.prove_stark.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 3 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_stark.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 4 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_stark.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 5 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_stark.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 6 | 524,288 |  | 12 | 20 | 16,777,216 | 
| reth.prove_stark.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 7 | 262,144 |  | 12 | 20 | 8,388,608 | 
| reth.prove_stark.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 8 | 262,144 |  | 12 | 20 | 8,388,608 | 
| reth.prove_stark.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 9 | 262,144 |  | 12 | 20 | 8,388,608 | 
| reth.prove_stark.block_21000000 | PhantomAir | 21000000 | 0 | 4 |  | 12 | 6 | 72 | 
| reth.prove_stark.block_21000000 | PhantomAir | 21000000 | 10 | 8 |  | 12 | 6 | 144 | 
| reth.prove_stark.block_21000000 | PhantomAir | 21000000 | 11 | 8 |  | 12 | 6 | 144 | 
| reth.prove_stark.block_21000000 | PhantomAir | 21000000 | 13 | 1 |  | 12 | 6 | 18 | 
| reth.prove_stark.block_21000000 | PhantomAir | 21000000 | 3 | 2 |  | 12 | 6 | 36 | 
| reth.prove_stark.block_21000000 | PhantomAir | 21000000 | 6 | 256 |  | 12 | 6 | 4,608 | 
| reth.prove_stark.block_21000000 | PhantomAir | 21000000 | 7 | 64 |  | 12 | 6 | 1,152 | 
| reth.prove_stark.block_21000000 | PhantomAir | 21000000 | 8 | 2 |  | 12 | 6 | 36 | 
| reth.prove_stark.block_21000000 | PhantomAir | 21000000 | 9 | 8 |  | 12 | 6 | 144 | 
| reth.prove_stark.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 0 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_stark.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 1 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_stark.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 10 | 131,072 |  | 8 | 300 | 40,370,176 | 
| reth.prove_stark.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 11 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_stark.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 12 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_stark.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 13 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_stark.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 2 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_stark.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 3 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_stark.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 4 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_stark.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 5 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_stark.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 6 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_stark.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 7 | 131,072 |  | 8 | 300 | 40,370,176 | 
| reth.prove_stark.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 8 | 131,072 |  | 8 | 300 | 40,370,176 | 
| reth.prove_stark.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 9 | 131,072 |  | 8 | 300 | 40,370,176 | 
| reth.prove_stark.block_21000000 | ProgramAir | 21000000 | 0 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_stark.block_21000000 | ProgramAir | 21000000 | 1 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_stark.block_21000000 | ProgramAir | 21000000 | 10 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_stark.block_21000000 | ProgramAir | 21000000 | 11 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_stark.block_21000000 | ProgramAir | 21000000 | 12 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_stark.block_21000000 | ProgramAir | 21000000 | 13 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_stark.block_21000000 | ProgramAir | 21000000 | 2 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_stark.block_21000000 | ProgramAir | 21000000 | 3 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_stark.block_21000000 | ProgramAir | 21000000 | 4 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_stark.block_21000000 | ProgramAir | 21000000 | 5 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_stark.block_21000000 | ProgramAir | 21000000 | 6 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_stark.block_21000000 | ProgramAir | 21000000 | 7 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_stark.block_21000000 | ProgramAir | 21000000 | 8 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_stark.block_21000000 | ProgramAir | 21000000 | 9 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_stark.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 0 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_stark.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 1 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_stark.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 10 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_stark.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 11 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_stark.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 12 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_stark.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 13 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_stark.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 2 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_stark.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 3 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_stark.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 4 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_stark.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 5 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_stark.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 6 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_stark.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 7 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_stark.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 8 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_stark.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 9 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_stark.block_21000000 | Rv32HintStoreAir | 21000000 | 0 | 524,288 |  | 44 | 32 | 39,845,888 | 
| reth.prove_stark.block_21000000 | Rv32HintStoreAir | 21000000 | 1 | 524,288 |  | 44 | 32 | 39,845,888 | 
| reth.prove_stark.block_21000000 | Rv32HintStoreAir | 21000000 | 10 | 64 |  | 44 | 32 | 4,864 | 
| reth.prove_stark.block_21000000 | Rv32HintStoreAir | 21000000 | 2 | 1,048,576 |  | 44 | 32 | 79,691,776 | 
| reth.prove_stark.block_21000000 | Rv32HintStoreAir | 21000000 | 3 | 32,768 |  | 44 | 32 | 2,490,368 | 
| reth.prove_stark.block_21000000 | Rv32HintStoreAir | 21000000 | 6 | 2,048 |  | 44 | 32 | 155,648 | 
| reth.prove_stark.block_21000000 | Rv32HintStoreAir | 21000000 | 7 | 512 |  | 44 | 32 | 38,912 | 
| reth.prove_stark.block_21000000 | Rv32HintStoreAir | 21000000 | 8 | 32 |  | 44 | 32 | 2,432 | 
| reth.prove_stark.block_21000000 | Rv32HintStoreAir | 21000000 | 9 | 64 |  | 44 | 32 | 4,864 | 
| reth.prove_stark.block_21000000 | VariableRangeCheckerAir | 21000000 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_stark.block_21000000 | VariableRangeCheckerAir | 21000000 | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_stark.block_21000000 | VariableRangeCheckerAir | 21000000 | 10 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_stark.block_21000000 | VariableRangeCheckerAir | 21000000 | 11 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_stark.block_21000000 | VariableRangeCheckerAir | 21000000 | 12 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_stark.block_21000000 | VariableRangeCheckerAir | 21000000 | 13 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_stark.block_21000000 | VariableRangeCheckerAir | 21000000 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_stark.block_21000000 | VariableRangeCheckerAir | 21000000 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_stark.block_21000000 | VariableRangeCheckerAir | 21000000 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_stark.block_21000000 | VariableRangeCheckerAir | 21000000 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_stark.block_21000000 | VariableRangeCheckerAir | 21000000 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_stark.block_21000000 | VariableRangeCheckerAir | 21000000 | 7 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_stark.block_21000000 | VariableRangeCheckerAir | 21000000 | 8 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_stark.block_21000000 | VariableRangeCheckerAir | 21000000 | 9 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 0 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 1 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 10 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 11 | 4,194,304 |  | 52 | 36 | 369,098,752 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 12 | 4,194,304 |  | 52 | 36 | 369,098,752 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 13 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 2 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 3 | 4,194,304 |  | 52 | 36 | 369,098,752 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 4 | 4,194,304 |  | 52 | 36 | 369,098,752 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 5 | 4,194,304 |  | 52 | 36 | 369,098,752 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 6 | 4,194,304 |  | 52 | 36 | 369,098,752 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 7 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 8 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 9 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 0 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 1 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 10 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 11 | 262,144 |  | 40 | 37 | 20,185,088 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 12 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 13 | 32,768 |  | 40 | 37 | 2,523,136 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 2 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 3 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 4 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 5 | 262,144 |  | 40 | 37 | 20,185,088 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 6 | 131,072 |  | 40 | 37 | 10,092,544 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 7 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 8 | 1,048,576 |  | 40 | 37 | 80,740,352 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 9 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 0 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 1 | 524,288 |  | 52 | 53 | 55,050,240 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 10 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 11 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 12 | 262,144 |  | 52 | 53 | 27,525,120 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 13 | 32,768 |  | 52 | 53 | 3,440,640 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 2 | 524,288 |  | 52 | 53 | 55,050,240 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 3 | 131,072 |  | 52 | 53 | 13,762,560 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 4 | 131,072 |  | 52 | 53 | 13,762,560 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 5 | 131,072 |  | 52 | 53 | 13,762,560 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 6 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 7 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 8 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 9 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 0 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 1 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 10 | 1,048,576 |  | 28 | 26 | 56,623,104 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 11 | 1,048,576 |  | 28 | 26 | 56,623,104 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 12 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 13 | 524,288 |  | 28 | 26 | 28,311,552 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 2 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 3 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 4 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 5 | 1,048,576 |  | 28 | 26 | 56,623,104 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 6 | 1,048,576 |  | 28 | 26 | 56,623,104 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 7 | 1,048,576 |  | 28 | 26 | 56,623,104 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 8 | 1,048,576 |  | 28 | 26 | 56,623,104 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 9 | 1,048,576 |  | 28 | 26 | 56,623,104 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 0 | 1,048,576 |  | 32 | 32 | 67,108,864 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 1 | 1,048,576 |  | 32 | 32 | 67,108,864 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 10 | 524,288 |  | 32 | 32 | 33,554,432 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 11 | 524,288 |  | 32 | 32 | 33,554,432 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 12 | 131,072 |  | 32 | 32 | 8,388,608 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 13 | 16,384 |  | 32 | 32 | 1,048,576 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 2 | 1,048,576 |  | 32 | 32 | 67,108,864 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 3 | 262,144 |  | 32 | 32 | 16,777,216 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 4 | 262,144 |  | 32 | 32 | 16,777,216 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 5 | 131,072 |  | 32 | 32 | 8,388,608 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 6 | 262,144 |  | 32 | 32 | 16,777,216 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 7 | 524,288 |  | 32 | 32 | 33,554,432 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 8 | 524,288 |  | 32 | 32 | 33,554,432 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 9 | 524,288 |  | 32 | 32 | 33,554,432 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 0 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 1 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 10 | 262,144 |  | 28 | 18 | 12,058,624 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 11 | 131,072 |  | 28 | 18 | 6,029,312 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 12 | 262,144 |  | 28 | 18 | 12,058,624 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 13 | 16,384 |  | 28 | 18 | 753,664 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 2 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 3 | 262,144 |  | 28 | 18 | 12,058,624 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 4 | 262,144 |  | 28 | 18 | 12,058,624 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 5 | 131,072 |  | 28 | 18 | 6,029,312 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 6 | 262,144 |  | 28 | 18 | 12,058,624 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 7 | 262,144 |  | 28 | 18 | 12,058,624 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 8 | 131,072 |  | 28 | 18 | 6,029,312 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 9 | 131,072 |  | 28 | 18 | 6,029,312 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 10 | 16,384 |  | 192 | 168 | 5,898,240 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 11 | 8,192 |  | 192 | 168 | 2,949,120 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 13 | 1 |  | 192 | 168 | 360 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 6 | 256 |  | 192 | 168 | 92,160 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 7 | 16,384 |  | 192 | 168 | 5,898,240 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 8 | 16,384 |  | 192 | 168 | 5,898,240 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 9 | 16,384 |  | 192 | 168 | 5,898,240 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 10 | 4,096 |  | 68 | 169 | 970,752 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 11 | 1,024 |  | 68 | 169 | 242,688 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 6 | 256 |  | 68 | 169 | 60,672 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 7 | 4,096 |  | 68 | 169 | 970,752 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 8 | 8,192 |  | 68 | 169 | 1,941,504 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 9 | 4,096 |  | 68 | 169 | 970,752 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 10 | 512 |  | 192 | 164 | 182,272 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 11 | 64 |  | 192 | 164 | 22,784 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 7 | 1,024 |  | 192 | 164 | 364,544 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 8 | 2,048 |  | 192 | 164 | 729,088 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 9 | 512 |  | 192 | 164 | 182,272 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 10 | 2,048 |  | 164 | 241 | 829,440 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 11 | 512 |  | 164 | 241 | 207,360 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 7 | 2,048 |  | 164 | 241 | 829,440 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 8 | 2,048 |  | 164 | 241 | 829,440 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 9 | 2,048 |  | 164 | 241 | 829,440 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 10 | 16,384 |  | 48 | 124 | 2,818,048 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 11 | 4,096 |  | 48 | 124 | 704,512 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 12 | 512 |  | 48 | 124 | 88,064 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 6 | 16 |  | 48 | 124 | 2,752 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 7 | 16,384 |  | 48 | 124 | 2,818,048 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 8 | 32,768 |  | 48 | 124 | 5,636,096 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 9 | 16,384 |  | 48 | 124 | 2,818,048 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 10 | 8,192 |  | 56 | 166 | 1,818,624 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 6 | 131,072 |  | 56 | 166 | 29,097,984 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 7 | 32,768 |  | 56 | 166 | 7,274,496 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 8 | 2,048 |  | 56 | 166 | 454,656 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 9 | 4,096 |  | 56 | 166 | 909,312 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 0 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 1 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 10 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 11 | 262,144 |  | 36 | 28 | 16,777,216 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 12 | 262,144 |  | 36 | 28 | 16,777,216 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 13 | 262,144 |  | 36 | 28 | 16,777,216 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 2 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 3 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 4 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 5 | 262,144 |  | 36 | 28 | 16,777,216 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 6 | 131,072 |  | 36 | 28 | 8,388,608 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 7 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 8 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 9 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 0 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 1 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 10 | 65,536 |  | 52 | 36 | 5,767,168 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 11 | 262,144 |  | 52 | 36 | 23,068,672 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 12 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 13 | 131,072 |  | 52 | 36 | 11,534,336 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 2 | 524,288 |  | 52 | 36 | 46,137,344 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 3 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 4 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 5 | 524,288 |  | 52 | 36 | 46,137,344 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 6 | 262,144 |  | 52 | 36 | 23,068,672 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 7 | 131,072 |  | 52 | 36 | 11,534,336 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 8 | 65,536 |  | 52 | 36 | 5,767,168 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 9 | 65,536 |  | 52 | 36 | 5,767,168 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 0 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 1 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 10 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 11 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 12 | 4,194,304 |  | 52 | 41 | 390,070,272 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 13 | 1,048,576 |  | 52 | 41 | 97,517,568 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 2 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 3 | 4,194,304 |  | 52 | 41 | 390,070,272 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 4 | 4,194,304 |  | 52 | 41 | 390,070,272 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 5 | 2,097,152 |  | 52 | 41 | 195,035,136 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 6 | 4,194,304 |  | 52 | 41 | 390,070,272 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 7 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 8 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 9 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 10 | 512 |  | 72 | 59 | 67,072 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 11 | 128 |  | 72 | 59 | 16,768 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 12 | 256 |  | 72 | 59 | 33,536 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 7 | 256 |  | 72 | 59 | 33,536 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 8 | 256 |  | 72 | 59 | 33,536 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 9 | 256 |  | 72 | 59 | 33,536 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 1 | 2,048 |  | 72 | 39 | 227,328 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 10 | 131,072 |  | 72 | 39 | 14,548,992 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 11 | 65,536 |  | 72 | 39 | 7,274,496 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 12 | 4,096 |  | 72 | 39 | 454,656 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 13 | 16 |  | 72 | 39 | 1,776 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 2 | 16,384 |  | 72 | 39 | 1,818,624 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 5 | 2,048 |  | 72 | 39 | 227,328 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 6 | 65,536 |  | 72 | 39 | 7,274,496 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 7 | 131,072 |  | 72 | 39 | 14,548,992 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 8 | 131,072 |  | 72 | 39 | 14,548,992 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 9 | 131,072 |  | 72 | 39 | 14,548,992 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 0 | 8,192 |  | 52 | 31 | 679,936 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 1 | 2,048 |  | 52 | 31 | 169,984 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 10 | 262,144 |  | 52 | 31 | 21,757,952 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 11 | 131,072 |  | 52 | 31 | 10,878,976 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 12 | 16,384 |  | 52 | 31 | 1,359,872 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 13 | 4,096 |  | 52 | 31 | 339,968 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 2 | 16,384 |  | 52 | 31 | 1,359,872 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 3 | 8,192 |  | 52 | 31 | 679,936 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 4 | 8,192 |  | 52 | 31 | 679,936 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 5 | 8,192 |  | 52 | 31 | 679,936 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 6 | 65,536 |  | 52 | 31 | 5,439,488 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 7 | 131,072 |  | 52 | 31 | 10,878,976 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 8 | 262,144 |  | 52 | 31 | 21,757,952 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 9 | 262,144 |  | 52 | 31 | 21,757,952 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 0 | 262,144 |  | 28 | 20 | 12,582,912 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 1 | 262,144 |  | 28 | 20 | 12,582,912 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 10 | 65,536 |  | 28 | 20 | 3,145,728 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 11 | 131,072 |  | 28 | 20 | 6,291,456 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 12 | 131,072 |  | 28 | 20 | 6,291,456 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 13 | 131,072 |  | 28 | 20 | 6,291,456 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 2 | 262,144 |  | 28 | 20 | 12,582,912 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 3 | 262,144 |  | 28 | 20 | 12,582,912 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 4 | 131,072 |  | 28 | 20 | 6,291,456 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 5 | 131,072 |  | 28 | 20 | 6,291,456 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 6 | 65,536 |  | 28 | 20 | 3,145,728 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 7 | 65,536 |  | 28 | 20 | 3,145,728 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 8 | 65,536 |  | 28 | 20 | 3,145,728 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 9 | 65,536 |  | 28 | 20 | 3,145,728 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 10 | 2,048 |  | 836 | 547 | 2,832,384 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 6 | 65,536 |  | 836 | 547 | 90,636,288 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 7 | 16,384 |  | 836 | 547 | 22,659,072 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 8 | 512 |  | 836 | 547 | 708,096 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 9 | 2,048 |  | 836 | 547 | 2,832,384 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 10 | 32 |  | 320 | 263 | 18,656 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 6 | 1,024 |  | 320 | 263 | 596,992 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 7 | 256 |  | 320 | 263 | 149,248 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 8 | 8 |  | 320 | 263 | 4,664 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 9 | 32 |  | 320 | 263 | 18,656 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 10 | 1,024 |  | 860 | 625 | 1,520,640 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 6 | 32,768 |  | 860 | 625 | 48,660,480 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 7 | 8,192 |  | 860 | 625 | 12,165,120 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 8 | 512 |  | 860 | 625 | 760,320 | 
| reth.prove_stark.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 9 | 1,024 |  | 860 | 625 | 1,520,640 | 
| reth.prove_stark.block_21000000 | VmConnectorAir | 21000000 | 0 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_stark.block_21000000 | VmConnectorAir | 21000000 | 1 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_stark.block_21000000 | VmConnectorAir | 21000000 | 10 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_stark.block_21000000 | VmConnectorAir | 21000000 | 11 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_stark.block_21000000 | VmConnectorAir | 21000000 | 12 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_stark.block_21000000 | VmConnectorAir | 21000000 | 13 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_stark.block_21000000 | VmConnectorAir | 21000000 | 2 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_stark.block_21000000 | VmConnectorAir | 21000000 | 3 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_stark.block_21000000 | VmConnectorAir | 21000000 | 4 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_stark.block_21000000 | VmConnectorAir | 21000000 | 5 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_stark.block_21000000 | VmConnectorAir | 21000000 | 6 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_stark.block_21000000 | VmConnectorAir | 21000000 | 7 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_stark.block_21000000 | VmConnectorAir | 21000000 | 8 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_stark.block_21000000 | VmConnectorAir | 21000000 | 9 | 2 | 1 | 16 | 5 | 42 | 

| group | block_number | vm.reset_state_time_ms | vm.create_initial_state_time_ms | trace_gen_time_ms | total_proof_time_ms | total_cells_used | total_cells | system_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | single_trace_gen_time_ms | single_leaf_agg_time_ms | single_internal_agg_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | prove_segment_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | num_children | memory_to_vec_partition_time_ms | memory_finalize_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | fri.log_blowup | execute_preflight_time_ms | execute_preflight_insns | execute_preflight_insn_mi/s | execute_metered_time_ms | execute_metered_insns | execute_metered_insn_mi/s | execute_e1_time_ms | execute_e1_insns | execute_e1_insn_mi/s | compute_user_public_values_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 0 | 0 | 109 | 5,049 | 222,207,970 | 270,872,042 | 109 | 3,276 | 0 |  |  | 349 | 584 | 1,431 | 435 | 1,198 |  | 11 | 9 | 549 | 91,157,468 | 156 |  | 299 | 1,622,342 | 5.12 | 0 | 1 | 0.20 |  |  |  | 12 | 
| internal.0 | 21000000 |  |  |  |  |  |  |  |  |  |  | 6,706 |  |  |  |  |  | 3 |  |  |  |  |  | 2 |  |  |  |  |  |  |  |  |  |  | 
| internal.1 | 21000000 |  |  |  |  |  |  |  |  |  |  | 4,579 |  |  |  |  |  | 3 |  |  |  |  |  | 2 |  |  |  |  |  |  |  |  |  |  | 
| internal.2 | 21000000 |  |  |  |  |  |  |  |  |  |  | 4,510 |  |  |  |  |  | 3 |  |  |  |  |  | 2 |  |  |  |  |  |  |  |  |  |  | 
| leaf | 21000000 |  |  |  |  |  |  |  |  |  | 5,637 |  |  |  |  |  |  | 1 |  |  |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  | 
| reth.prove_stark.block_21000000 | 21000000 | 5 |  |  |  |  |  |  |  |  |  |  |  |  | 5,684 |  |  |  | 35 |  |  |  |  | 1 |  |  |  | 1,579 | 171,002,433 | 108.26 | 963 | 171,002,433 | 177.46 | 422 | 

| group | block_number | idx | vm.reset_state_time_ms | trace_gen_time_ms | total_proof_time_ms | total_cells_used | total_cells | system_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | single_trace_gen_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | memory_finalize_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_preflight_time_ms | execute_preflight_insns | execute_preflight_insn_mi/s |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | 21000000 | 0 | 0 | 459 | 8,388 | 383,712,018 | 472,992,226 | 459 | 5,968 | 0 | 820 | 992 | 860 | 1,857 | 14 | 1,178 | 215,286,632 | 256 | 1,120 | 3,499,215 | 3.62 | 
| internal.0 | 21000000 | 1 | 4 | 461 | 8,157 | 383,624,648 | 472,992,226 | 461 | 5,736 | 0 | 799 | 949 | 830 | 1,791 | 11 | 1,122 | 215,220,350 | 240 | 1,112 | 3,499,841 | 3.64 | 
| internal.0 | 21000000 | 2 | 4 | 465 | 8,104 | 389,822,564 | 472,992,226 | 465 | 5,621 | 0 | 803 | 830 | 830 | 1,802 | 13 | 1,095 | 219,307,130 | 256 | 1,165 | 3,545,858 | 3.51 | 
| internal.0 | 21000000 | 3 | 4 | 463 | 8,222 | 388,239,486 | 472,992,226 | 463 | 5,765 | 0 | 813 | 890 | 843 | 1,811 | 12 | 1,164 | 218,175,156 | 239 | 1,147 | 3,530,474 | 3.54 | 
| internal.0 | 21000000 | 4 | 4 | 313 | 6,704 | 251,901,971 | 346,728,930 | 313 | 4,765 | 0 | 620 | 713 | 723 | 1,653 | 11 | 847 | 141,242,457 | 204 | 793 | 2,333,488 | 3.63 | 
| internal.1 | 21000000 | 5 | 4 | 282 | 6,110 | 208,594,932 | 302,819,810 | 282 | 4,368 | 0 | 542 | 862 | 612 | 1,389 | 11 | 786 | 117,149,042 | 172 | 617 | 2,338,827 | 5.04 | 
| internal.1 | 21000000 | 6 | 4 | 195 | 4,578 | 147,652,473 | 183,445,986 | 195 | 3,079 | 0 | 369 | 533 | 390 | 1,150 | 9 | 511 | 82,007,951 | 122 | 461 | 1,558,882 | 4.99 | 
| internal.2 | 21000000 | 7 | 4 | 199 | 4,509 | 146,722,167 | 186,591,714 | 199 | 3,013 | 0 | 376 | 526 | 405 | 1,047 | 10 | 489 | 81,343,205 | 164 | 456 | 1,550,610 | 5.06 | 
| leaf | 21000000 | 0 | 0 | 349 | 5,169 | 431,359,954 | 571,690,474 | 349 | 3,390 | 0 | 386 | 348 | 696 | 1,183 | 15 | 533 | 173,891,944 | 239 | 606 | 1,974,682 | 4.42 | 
| leaf | 21000000 | 1 | 4 | 198 | 4,047 | 233,799,465 | 352,656,874 | 198 | 2,480 | 0 | 249 | 235 | 432 | 1,014 | 10 | 383 | 101,488,563 | 160 | 535 | 1,458,332 | 3.82 | 
| leaf | 21000000 | 10 | 4 | 615 | 8,014 | 802,697,643 | 1,100,058,090 | 615 | 5,816 | 0 | 699 | 435 | 1,395 | 1,960 | 16 | 906 | 312,157,065 | 415 | 735 | 3,438,182 | 5.97 | 
| leaf | 21000000 | 11 | 4 | 428 | 5,829 | 524,683,085 | 778,259,946 | 428 | 3,919 | 0 | 489 | 312 | 863 | 1,306 | 16 | 657 | 208,729,231 | 286 | 640 | 2,477,195 | 5.15 | 
| leaf | 21000000 | 12 | 4 | 370 | 5,614 | 453,073,104 | 732,909,034 | 370 | 3,799 | 0 | 469 | 298 | 822 | 1,269 | 15 | 638 | 181,735,190 | 299 | 599 | 2,111,045 | 4.79 | 
| leaf | 21000000 | 13 | 4 | 370 | 5,635 | 453,095,111 | 732,909,034 | 370 | 3,828 | 0 | 461 | 298 | 826 | 1,264 | 16 | 641 | 181,249,645 | 331 | 590 | 2,107,667 | 4.89 | 
| leaf | 21000000 | 2 | 4 | 197 | 4,043 | 233,739,624 | 352,656,874 | 197 | 2,480 | 0 | 256 | 234 | 432 | 1,014 | 10 | 377 | 101,453,950 | 159 | 539 | 1,458,511 | 3.79 | 
| leaf | 21000000 | 3 | 4 | 347 | 5,131 | 428,381,135 | 571,690,474 | 347 | 3,314 | 0 | 384 | 284 | 699 | 1,193 | 56 | 501 | 172,361,497 | 247 | 633 | 1,959,279 | 4.53 | 
| leaf | 21000000 | 4 | 4 | 336 | 5,082 | 419,249,631 | 571,690,474 | 336 | 3,318 | 0 | 379 | 285 | 696 | 1,191 | 15 | 507 | 168,761,681 | 255 | 585 | 1,876,117 | 4.40 | 
| leaf | 21000000 | 5 | 4 | 342 | 5,168 | 426,412,252 | 571,690,474 | 342 | 3,396 | 0 | 394 | 281 | 700 | 1,200 | 15 | 516 | 171,509,290 | 299 | 588 | 1,924,234 | 4.49 | 
| leaf | 21000000 | 6 | 4 | 577 | 7,961 | 754,948,407 | 1,080,659,434 | 576 | 5,788 | 0 | 706 | 476 | 1,298 | 1,970 | 57 | 924 | 293,914,201 | 407 | 747 | 3,196,690 | 5.87 | 
| leaf | 21000000 | 7 | 4 | 621 | 7,990 | 802,875,131 | 1,100,058,090 | 621 | 5,789 | 0 | 688 | 426 | 1,313 | 1,996 | 16 | 937 | 312,252,945 | 424 | 735 | 3,438,764 | 5.97 | 
| leaf | 21000000 | 8 | 4 | 614 | 7,924 | 802,752,592 | 1,100,058,090 | 614 | 5,723 | 0 | 699 | 428 | 1,307 | 1,959 | 18 | 907 | 312,188,878 | 417 | 738 | 3,437,952 | 5.97 | 
| leaf | 21000000 | 9 | 4 | 614 | 8,054 | 802,696,085 | 1,100,058,090 | 614 | 5,857 | 0 | 698 | 512 | 1,308 | 1,988 | 17 | 912 | 312,155,975 | 435 | 738 | 3,438,177 | 5.95 | 

| group | block_number | idx | trace_height_constraint | weighted_sum | threshold |
| --- | --- | --- | --- | --- | --- |
| internal.0 | 21000000 | 0 | 0 | 11,927,684 | 2,013,265,921 | 
| internal.0 | 21000000 | 0 | 1 | 65,323,264 | 2,013,265,921 | 
| internal.0 | 21000000 | 0 | 2 | 5,963,842 | 2,013,265,921 | 
| internal.0 | 21000000 | 0 | 3 | 64,782,596 | 2,013,265,921 | 
| internal.0 | 21000000 | 0 | 4 | 524,288 | 2,013,265,921 | 
| internal.0 | 21000000 | 0 | 5 | 148,914,890 | 2,013,265,921 | 
| internal.0 | 21000000 | 1 | 0 | 11,927,684 | 2,013,265,921 | 
| internal.0 | 21000000 | 1 | 1 | 65,323,264 | 2,013,265,921 | 
| internal.0 | 21000000 | 1 | 2 | 5,963,842 | 2,013,265,921 | 
| internal.0 | 21000000 | 1 | 3 | 64,782,596 | 2,013,265,921 | 
| internal.0 | 21000000 | 1 | 4 | 524,288 | 2,013,265,921 | 
| internal.0 | 21000000 | 1 | 5 | 148,914,890 | 2,013,265,921 | 
| internal.0 | 21000000 | 2 | 0 | 11,927,684 | 2,013,265,921 | 
| internal.0 | 21000000 | 2 | 1 | 65,323,264 | 2,013,265,921 | 
| internal.0 | 21000000 | 2 | 2 | 5,963,842 | 2,013,265,921 | 
| internal.0 | 21000000 | 2 | 3 | 64,782,596 | 2,013,265,921 | 
| internal.0 | 21000000 | 2 | 4 | 524,288 | 2,013,265,921 | 
| internal.0 | 21000000 | 2 | 5 | 148,914,890 | 2,013,265,921 | 
| internal.0 | 21000000 | 3 | 0 | 11,927,684 | 2,013,265,921 | 
| internal.0 | 21000000 | 3 | 1 | 65,323,264 | 2,013,265,921 | 
| internal.0 | 21000000 | 3 | 2 | 5,963,842 | 2,013,265,921 | 
| internal.0 | 21000000 | 3 | 3 | 64,782,596 | 2,013,265,921 | 
| internal.0 | 21000000 | 3 | 4 | 524,288 | 2,013,265,921 | 
| internal.0 | 21000000 | 3 | 5 | 148,914,890 | 2,013,265,921 | 
| internal.0 | 21000000 | 4 | 0 | 9,764,996 | 2,013,265,921 | 
| internal.0 | 21000000 | 4 | 1 | 50,356,480 | 2,013,265,921 | 
| internal.0 | 21000000 | 4 | 2 | 4,882,498 | 2,013,265,921 | 
| internal.0 | 21000000 | 4 | 3 | 50,610,436 | 2,013,265,921 | 
| internal.0 | 21000000 | 4 | 4 | 262,144 | 2,013,265,921 | 
| internal.0 | 21000000 | 4 | 5 | 116,269,770 | 2,013,265,921 | 
| internal.1 | 21000000 | 5 | 0 | 8,454,276 | 2,013,265,921 | 
| internal.1 | 21000000 | 5 | 1 | 40,132,864 | 2,013,265,921 | 
| internal.1 | 21000000 | 5 | 2 | 4,227,138 | 2,013,265,921 | 
| internal.1 | 21000000 | 5 | 3 | 40,386,820 | 2,013,265,921 | 
| internal.1 | 21000000 | 5 | 4 | 262,144 | 2,013,265,921 | 
| internal.1 | 21000000 | 5 | 5 | 93,856,458 | 2,013,265,921 | 
| internal.1 | 21000000 | 6 | 0 | 5,144,708 | 2,013,265,921 | 
| internal.1 | 21000000 | 6 | 1 | 23,748,864 | 2,013,265,921 | 
| internal.1 | 21000000 | 6 | 2 | 2,572,354 | 2,013,265,921 | 
| internal.1 | 21000000 | 6 | 3 | 23,478,532 | 2,013,265,921 | 
| internal.1 | 21000000 | 6 | 4 | 131,072 | 2,013,265,921 | 
| internal.1 | 21000000 | 6 | 5 | 55,468,746 | 2,013,265,921 | 
| internal.2 | 21000000 | 7 | 0 | 5,144,708 | 2,013,265,921 | 
| internal.2 | 21000000 | 7 | 1 | 24,011,008 | 2,013,265,921 | 
| internal.2 | 21000000 | 7 | 2 | 2,572,354 | 2,013,265,921 | 
| internal.2 | 21000000 | 7 | 3 | 24,133,892 | 2,013,265,921 | 
| internal.2 | 21000000 | 7 | 4 | 131,072 | 2,013,265,921 | 
| internal.2 | 21000000 | 7 | 5 | 56,386,250 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 0 | 9,371,780 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 1 | 64,930,048 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 2 | 4,685,890 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 3 | 65,044,740 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 4 | 262,144 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 5 | 146,653,898 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 0 | 5,963,908 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 1 | 32,649,472 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 2 | 2,981,954 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 3 | 32,383,236 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 4 | 262,144 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 5 | 76,600,010 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 0 | 18,546,820 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 1 | 129,728,768 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 2 | 9,273,410 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 3 | 129,827,076 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 5 | 290,259,658 | 2,013,265,921 | 
| leaf | 21000000 | 11 | 0 | 13,566,084 | 2,013,265,921 | 
| leaf | 21000000 | 11 | 1 | 83,804,416 | 2,013,265,921 | 
| leaf | 21000000 | 11 | 2 | 6,783,042 | 2,013,265,921 | 
| leaf | 21000000 | 11 | 3 | 83,919,108 | 2,013,265,921 | 
| leaf | 21000000 | 11 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 11 | 5 | 190,956,234 | 2,013,265,921 | 
| leaf | 21000000 | 12 | 0 | 11,993,220 | 2,013,265,921 | 
| leaf | 21000000 | 12 | 1 | 79,610,112 | 2,013,265,921 | 
| leaf | 21000000 | 12 | 2 | 5,996,610 | 2,013,265,921 | 
| leaf | 21000000 | 12 | 3 | 79,724,804 | 2,013,265,921 | 
| leaf | 21000000 | 12 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 12 | 5 | 180,208,330 | 2,013,265,921 | 
| leaf | 21000000 | 13 | 0 | 11,993,220 | 2,013,265,921 | 
| leaf | 21000000 | 13 | 1 | 79,610,112 | 2,013,265,921 | 
| leaf | 21000000 | 13 | 2 | 5,996,610 | 2,013,265,921 | 
| leaf | 21000000 | 13 | 3 | 79,724,804 | 2,013,265,921 | 
| leaf | 21000000 | 13 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 13 | 5 | 180,208,330 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 0 | 5,963,908 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 1 | 32,649,472 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 2 | 2,981,954 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 3 | 32,383,236 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 4 | 262,144 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 5 | 76,600,010 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 0 | 9,371,780 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 1 | 64,930,048 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 2 | 4,685,890 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 3 | 65,044,740 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 4 | 262,144 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 5 | 146,653,898 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 0 | 9,371,780 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 1 | 64,930,048 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 2 | 4,685,890 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 3 | 65,044,740 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 4 | 262,144 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 5 | 146,653,898 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 0 | 9,371,780 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 1 | 64,930,048 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 2 | 4,685,890 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 3 | 65,044,740 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 4 | 262,144 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 5 | 146,653,898 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 0 | 18,022,532 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 1 | 128,155,904 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 2 | 9,011,266 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 3 | 128,254,212 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 5 | 286,327,498 | 2,013,265,921 | 
| leaf | 21000000 | 7 | 0 | 18,546,820 | 2,013,265,921 | 
| leaf | 21000000 | 7 | 1 | 129,728,768 | 2,013,265,921 | 
| leaf | 21000000 | 7 | 2 | 9,273,410 | 2,013,265,921 | 
| leaf | 21000000 | 7 | 3 | 129,827,076 | 2,013,265,921 | 
| leaf | 21000000 | 7 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 7 | 5 | 290,259,658 | 2,013,265,921 | 
| leaf | 21000000 | 8 | 0 | 18,546,820 | 2,013,265,921 | 
| leaf | 21000000 | 8 | 1 | 129,728,768 | 2,013,265,921 | 
| leaf | 21000000 | 8 | 2 | 9,273,410 | 2,013,265,921 | 
| leaf | 21000000 | 8 | 3 | 129,827,076 | 2,013,265,921 | 
| leaf | 21000000 | 8 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 8 | 5 | 290,259,658 | 2,013,265,921 | 
| leaf | 21000000 | 9 | 0 | 18,546,820 | 2,013,265,921 | 
| leaf | 21000000 | 9 | 1 | 129,728,768 | 2,013,265,921 | 
| leaf | 21000000 | 9 | 2 | 9,273,410 | 2,013,265,921 | 
| leaf | 21000000 | 9 | 3 | 129,827,076 | 2,013,265,921 | 
| leaf | 21000000 | 9 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 9 | 5 | 290,259,658 | 2,013,265,921 | 

| group | block_number | segment | trace_gen_time_ms | total_proof_time_ms | total_cells_used | total_cells | system_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | single_trace_gen_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | memory_to_vec_partition_time_ms | memory_finalize_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_preflight_time_ms | execute_preflight_insns | execute_preflight_insn_mi/s |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 0 | 21 | 1,431 | 9,505,566 | 7,747,601 | 21 | 451 |  | 20 | 56 | 44 | 256 | 12 | 2 | 51 | 919,380 | 18 | 2 | 1 | 9,223,372,036,854,775,807 | 
| reth.prove_stark.block_21000000 | 21000000 | 0 | 3,826 | 16,717 | 1,547,237,834 | 2,594,119,794 | 3,826 | 12,128 | 0 | 1,553 | 922 | 2,318 | 3,444 | 7 | 38 | 2,914 | 698,570,160 | 967 | 565 | 15,928,000 | 35.49 | 
| reth.prove_stark.block_21000000 | 21000000 | 1 | 3,946 | 15,938 | 1,519,693,477 | 2,469,662,762 | 3,946 | 11,163 | 485 | 1,446 | 751 | 2,214 | 3,347 | 10 | 37 | 2,529 | 668,733,079 | 866 | 652 | 16,291,000 | 35.70 | 
| reth.prove_stark.block_21000000 | 21000000 | 10 | 2,741 | 14,039 | 1,586,457,345 | 2,094,019,546 | 2,741 | 10,400 | 0 | 1,342 | 727 | 2,143 | 3,285 | 30 | 16 | 2,014 | 715,285,155 | 871 | 720 | 16,053,000 | 29.21 | 
| reth.prove_stark.block_21000000 | 21000000 | 11 | 2,581 | 17,073 | 1,746,969,482 | 2,849,225,018 | 2,581 | 13,768 | 0 | 2,731 | 692 | 2,279 | 3,464 | 34 | 23 | 3,859 | 969,502,920 | 733 | 543 | 11,316,000 | 34.14 | 
| reth.prove_stark.block_21000000 | 21000000 | 12 | 2,716 | 15,110 | 1,737,140,142 | 2,656,941,866 | 2,716 | 11,733 | 308 | 2,595 | 577 | 1,771 | 2,461 | 35 | 34 | 3,662 | 1,022,017,968 | 658 | 480 | 9,250,000 | 34.52 | 
| reth.prove_stark.block_21000000 | 21000000 | 13 | 1,121 | 5,684 | 361,583,411 | 837,762,284 | 1,121 | 4,196 | 0 | 656 | 307 | 521 | 1,290 | 36 | 19 | 1,207 | 190,562,557 | 200 | 188 | 2,062,433 | 34.12 | 
| reth.prove_stark.block_21000000 | 21000000 | 2 | 4,682 | 17,164 | 1,526,687,768 | 2,584,641,578 | 4,682 | 11,669 | 431 | 1,496 | 789 | 2,283 | 3,517 | 12 | 49 | 2,627 | 681,693,366 | 953 | 639 | 15,396,000 | 35.11 | 
| reth.prove_stark.block_21000000 | 21000000 | 3 | 2,583 | 15,305 | 1,748,486,736 | 2,675,728,462 | 2,583 | 12,022 | 0 | 2,604 | 602 | 1,867 | 2,486 | 16 | 37 | 3,781 | 987,162,486 | 672 | 520 | 10,866,000 | 35.51 | 
| reth.prove_stark.block_21000000 | 21000000 | 4 | 2,496 | 14,870 | 1,722,895,793 | 2,666,946,602 | 2,496 | 11,674 | 368 | 2,593 | 534 | 1,774 | 2,457 | 17 | 38 | 3,645 | 970,250,991 | 658 | 518 | 10,813,000 | 35.51 | 
| reth.prove_stark.block_21000000 | 21000000 | 5 | 1,870 | 17,897 | 2,055,719,459 | 3,423,983,658 | 1,870 | 15,409 | 208 | 4,070 | 475 | 2,057 | 2,385 | 17 | 36 | 5,699 | 1,345,207,093 | 713 | 437 | 6,104,000 | 33.88 | 
| reth.prove_stark.block_21000000 | 21000000 | 6 | 2,071 | 15,783 | 1,573,821,235 | 2,591,985,642 | 2,071 | 11,699 | 0 | 2,658 | 548 | 1,849 | 2,452 | 16 | 23 | 3,509 | 879,378,401 | 669 | 1,830 | 9,183,000 | 5.74 | 
| reth.prove_stark.block_21000000 | 21000000 | 7 | 2,440 | 14,356 | 1,591,449,656 | 2,181,936,426 | 2,440 | 10,623 | 0 | 1,452 | 783 | 2,160 | 3,244 | 16 | 14 | 2,127 | 736,093,706 | 841 | 1,112 | 15,239,000 | 16.38 | 
| reth.prove_stark.block_21000000 | 21000000 | 8 | 2,512 | 13,804 | 1,593,815,634 | 2,115,782,742 | 2,512 | 10,432 | 0 | 1,327 | 807 | 2,133 | 3,284 | 20 | 13 | 2,033 | 714,721,644 | 832 | 679 | 16,325,000 | 32.12 | 
| reth.prove_stark.block_21000000 | 21000000 | 9 | 2,668 | 13,731 | 1,597,421,211 | 2,087,047,386 | 2,668 | 10,176 | 0 | 1,304 | 720 | 2,109 | 3,231 | 25 | 15 | 2,021 | 720,253,089 | 774 | 713 | 16,176,000 | 29.72 | 

| group | block_number | segment | trace_height_constraint | weighted_sum | threshold |
| --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 0 | 0 | 34 | 2,013,265,921 | 
| agg_keygen | 21000000 | 0 | 1 | 86 | 2,013,265,921 | 
| agg_keygen | 21000000 | 0 | 2 | 17 | 2,013,265,921 | 
| agg_keygen | 21000000 | 0 | 3 | 98 | 2,013,265,921 | 
| agg_keygen | 21000000 | 0 | 4 | 193 | 2,013,265,921 | 
| agg_keygen | 21000000 | 0 | 5 | 65 | 2,013,265,921 | 
| agg_keygen | 21000000 | 0 | 6 | 29 | 2,013,265,921 | 
| agg_keygen | 21000000 | 0 | 7 | 20 | 2,013,265,921 | 
| agg_keygen | 21000000 | 0 | 8 | 918,079 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 0 | 0 | 48,807,948 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 0 | 1 | 141,508,608 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 0 | 2 | 24,403,974 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 0 | 3 | 166,674,436 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 0 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 0 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 0 | 6 | 57,049,088 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 0 | 7 |  | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 0 | 8 | 32,768 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 0 | 9 | 448,831,510 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 1 | 0 | 47,718,404 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 1 | 1 | 136,863,744 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 1 | 2 | 23,859,202 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 1 | 3 | 159,408,132 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 1 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 1 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 1 | 6 | 52,692,992 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 1 | 7 |  | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 1 | 8 | 24,576 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 1 | 9 | 430,921,738 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 10 | 0 | 42,602,964 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 10 | 1 | 128,496,768 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 10 | 2 | 21,301,482 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 10 | 3 | 152,372,452 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 10 | 4 | 1,835,008 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 10 | 5 | 786,432 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 10 | 6 | 53,441,024 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 10 | 7 |  | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 10 | 8 | 2,117,632 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 10 | 9 | 406,099,490 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 11 | 0 | 33,451,412 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 11 | 1 | 121,687,552 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 11 | 2 | 16,725,706 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 11 | 3 | 143,888,388 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 11 | 4 | 3,670,016 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 11 | 5 | 1,572,864 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 11 | 6 | 64,849,792 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 11 | 7 |  | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 11 | 8 | 1,051,648 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 11 | 9 | 390,436,322 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 12 | 0 | 26,781,188 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 12 | 1 | 100,014,592 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 12 | 2 | 13,390,594 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 12 | 3 | 111,808,004 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 12 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 12 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 12 | 6 | 61,740,032 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 12 | 7 |  | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 12 | 8 | 100,352 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 12 | 9 | 324,189,450 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 13 | 0 | 6,561,832 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 13 | 1 | 22,339,764 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 13 | 2 | 3,280,916 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 13 | 3 | 24,993,952 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 13 | 4 | 3,670,016 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 13 | 5 | 1,572,864 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 13 | 6 | 10,993,714 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 13 | 7 |  | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 13 | 8 | 16,512 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 13 | 9 | 77,492,802 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 2 | 0 | 47,775,748 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 2 | 1 | 141,230,080 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 2 | 2 | 23,887,874 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 2 | 3 | 160,104,452 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 2 | 4 | 8,388,608 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 2 | 5 | 4,194,304 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 2 | 6 | 54,280,192 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 2 | 7 |  | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 2 | 8 | 196,608 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 2 | 9 | 444,121,098 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 3 | 0 | 27,607,048 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 3 | 1 | 101,171,200 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 3 | 2 | 13,803,524 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 3 | 3 | 113,360,900 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 3 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 3 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 3 | 6 | 62,488,576 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 3 | 7 |  | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 3 | 8 | 32,768 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 3 | 9 | 328,818,704 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 4 | 0 | 27,279,364 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 4 | 1 | 100,712,448 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 4 | 2 | 13,639,682 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 4 | 3 | 112,902,148 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 4 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 4 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 4 | 6 | 61,734,912 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 4 | 7 |  | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 4 | 8 | 32,768 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 4 | 9 | 326,656,010 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 5 | 0 | 18,894,852 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 5 | 1 | 100,986,880 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 5 | 2 | 9,447,426 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 5 | 3 | 106,360,836 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 5 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 5 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 5 | 6 | 96,208,896 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 5 | 7 |  | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 5 | 8 | 49,152 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 5 | 9 | 342,302,730 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 6 | 0 | 24,457,764 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 6 | 1 | 96,790,656 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 6 | 2 | 12,228,882 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 6 | 3 | 148,271,748 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 6 | 4 | 3,670,016 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 6 | 5 | 1,572,864 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 6 | 6 | 63,986,192 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 6 | 7 |  | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 6 | 8 | 786,432 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 6 | 9 | 355,303,498 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 7 | 0 | 42,600,324 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 7 | 1 | 130,143,232 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 7 | 2 | 21,300,162 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 7 | 3 | 162,594,436 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 7 | 4 | 1,048,576 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 7 | 5 | 524,288 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 7 | 6 | 55,783,680 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 7 | 7 |  | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 7 | 8 | 1,607,680 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 7 | 9 | 418,748,106 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 8 | 0 | 43,416,216 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 8 | 1 | 131,486,848 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 8 | 2 | 21,708,108 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 8 | 3 | 154,562,972 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 8 | 4 | 1,048,576 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 8 | 5 | 524,288 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 8 | 6 | 54,640,816 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 8 | 7 |  | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 8 | 8 | 2,164,736 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 8 | 9 | 412,698,288 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 9 | 0 | 42,332,116 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 9 | 1 | 128,192,128 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 9 | 2 | 21,166,058 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 9 | 3 | 152,067,812 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 9 | 4 | 1,835,008 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 9 | 5 | 786,432 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 9 | 6 | 53,039,104 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 9 | 7 |  | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 9 | 8 | 2,115,584 | 2,013,265,921 | 
| reth.prove_stark.block_21000000 | 21000000 | 9 | 9 | 404,679,970 | 2,013,265,921 | 

| group | block_number | trace_height_constraint | weighted_sum | threshold |
| --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 0 | 5,701,764 | 2,013,265,921 | 
| agg_keygen | 21000000 | 1 | 28,467,456 | 2,013,265,921 | 
| agg_keygen | 21000000 | 2 | 2,850,882 | 2,013,265,921 | 
| agg_keygen | 21000000 | 3 | 28,197,124 | 2,013,265,921 | 
| agg_keygen | 21000000 | 4 | 131,072 | 2,013,265,921 | 
| agg_keygen | 21000000 | 5 | 65,741,514 | 2,013,265,921 | 

</details>


Commit: https://github.com/axiom-crypto/openvm-reth-benchmark/commit/6dead5d78ecc86f6ebba0a0b1de69b41e2cab901

Max Segment Length: 8388508

Instance Type: m8g.24xlarge

Memory Allocator: jemalloc

[Benchmark Workflow](https://github.com/axiom-crypto/openvm-reth-benchmark/actions/runs/17014459415)
