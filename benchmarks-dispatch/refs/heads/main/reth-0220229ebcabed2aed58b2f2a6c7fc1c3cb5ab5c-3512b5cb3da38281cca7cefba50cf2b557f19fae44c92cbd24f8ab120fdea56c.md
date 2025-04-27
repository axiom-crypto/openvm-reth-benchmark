| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total |  537.81 |  171.27 |
| reth.prove_e2e.block_21000000 |  290.38 |  33.45 |
| leaf |  91.25 |  10.06 |
| internal.0 |  35.11 |  9.32 |
| internal.1 |  8.75 |  6.11 |
| internal.2 |  4.37 |  4.37 |
| root |  23.21 |  23.21 |
| halo2_outer |  49.02 |  49.02 |
| halo2_wrapper |  35.72 |  35.72 |
| agg_keygen |  23.78 |  23.33 |


| reth.prove_e2e.block_21000000 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  26,397.73 |  290,375 |  33,455 |  13,844 |
| `main_cells_used     ` |  1,152,513,485.27 |  12,677,648,338 |  1,513,159,352 |  735,096,389 |
| `total_cycles        ` |  18,583,005.73 |  204,413,063 |  24,897,409 |  4,953,011 |
| `execute_time_ms     ` |  2,857.64 |  31,434 |  6,050 |  730 |
| `trace_gen_time_ms   ` |  8,737.82 |  96,116 |  11,021 |  4,611 |
| `stark_prove_excluding_trace_time_ms` |  14,802.27 |  162,825 |  17,733 |  8,252 |
| `main_trace_commit_time_ms` |  3,363.82 |  37,002 |  5,965 |  2,386 |
| `generate_perm_trace_time_ms` |  815.09 |  8,966 |  972 |  312 |
| `perm_trace_commit_time_ms` |  3,735.64 |  41,092 |  4,561 |  1,606 |
| `quotient_poly_compute_time_ms` |  2,688.09 |  29,569 |  4,653 |  1,859 |
| `quotient_poly_commit_time_ms` |  1,203.73 |  13,241 |  1,512 |  597 |
| `pcs_opening_time_ms ` |  2,983.55 |  32,819 |  3,521 |  1,483 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  8,295.55 |  91,251 |  10,062 |  5,910 |
| `main_cells_used     ` |  222,475,066.64 |  2,447,225,733 |  278,430,604 |  155,251,503 |
| `total_cycles        ` |  2,809,161.18 |  30,900,773 |  3,459,146 |  2,024,350 |
| `execute_time_ms     ` |  917.55 |  10,093 |  1,091 |  710 |
| `trace_gen_time_ms   ` |  1,727.09 |  18,998 |  2,141 |  1,236 |
| `stark_prove_excluding_trace_time_ms` |  5,650.91 |  62,160 |  6,868 |  3,964 |
| `main_trace_commit_time_ms` |  875.36 |  9,629 |  1,072 |  557 |
| `generate_perm_trace_time_ms` |  296.91 |  3,266 |  380 |  208 |
| `perm_trace_commit_time_ms` |  1,668 |  18,348 |  2,052 |  1,090 |
| `quotient_poly_compute_time_ms` |  831.55 |  9,147 |  1,019 |  573 |
| `quotient_poly_commit_time_ms` |  615.64 |  6,772 |  751 |  455 |
| `pcs_opening_time_ms ` |  1,358.64 |  14,945 |  1,624 |  1,022 |

| internal.0 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  8,778.25 |  35,113 |  9,323 |  7,215 |
| `main_cells_used     ` |  194,828,504 |  779,314,016 |  214,268,888 |  140,444,318 |
| `total_cycles        ` |  3,276,554.75 |  13,106,219 |  3,596,330 |  2,365,607 |
| `execute_time_ms     ` |  1,269.25 |  5,077 |  1,414 |  904 |
| `trace_gen_time_ms   ` |  1,435.75 |  5,743 |  1,580 |  1,032 |
| `stark_prove_excluding_trace_time_ms` |  6,073.25 |  24,293 |  6,369 |  5,279 |
| `main_trace_commit_time_ms` |  1,156.50 |  4,626 |  1,232 |  947 |
| `generate_perm_trace_time_ms` |  201.50 |  806 |  221 |  162 |
| `perm_trace_commit_time_ms` |  1,036.75 |  4,147 |  1,093 |  884 |
| `quotient_poly_compute_time_ms` |  1,044.25 |  4,177 |  1,112 |  855 |
| `quotient_poly_commit_time_ms` |  1,179 |  4,716 |  1,247 |  1,021 |
| `pcs_opening_time_ms ` |  1,450.50 |  5,802 |  1,472 |  1,405 |

| internal.1 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  4,374 |  8,748 |  6,105 |  2,643 |
| `main_cells_used     ` |  75,078,914 |  150,157,828 |  111,970,069 |  38,187,759 |
| `total_cycles        ` |  1,532,505 |  3,065,010 |  2,298,793 |  766,217 |
| `execute_time_ms     ` |  431.50 |  863 |  645 |  218 |
| `trace_gen_time_ms   ` |  600.50 |  1,201 |  876 |  325 |
| `stark_prove_excluding_trace_time_ms` |  3,342 |  6,684 |  4,584 |  2,100 |
| `main_trace_commit_time_ms` |  575 |  1,150 |  834 |  316 |
| `generate_perm_trace_time_ms` |  91.50 |  183 |  134 |  49 |
| `perm_trace_commit_time_ms` |  510.50 |  1,021 |  731 |  290 |
| `quotient_poly_compute_time_ms` |  508.50 |  1,017 |  735 |  282 |
| `quotient_poly_commit_time_ms` |  698 |  1,396 |  913 |  483 |
| `pcs_opening_time_ms ` |  953 |  1,906 |  1,232 |  674 |

| internal.2 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  4,372 |  4,372 |  4,372 |  4,372 |
| `main_cells_used     ` |  73,850,454 |  73,850,454 |  73,850,454 |  73,850,454 |
| `total_cycles        ` |  1,516,178 |  1,516,178 |  1,516,178 |  1,516,178 |
| `execute_time_ms     ` |  411 |  411 |  411 |  411 |
| `trace_gen_time_ms   ` |  613 |  613 |  613 |  613 |
| `stark_prove_excluding_trace_time_ms` |  3,348 |  3,348 |  3,348 |  3,348 |
| `main_trace_commit_time_ms` |  547 |  547 |  547 |  547 |
| `generate_perm_trace_time_ms` |  87 |  87 |  87 |  87 |
| `perm_trace_commit_time_ms` |  502 |  502 |  502 |  502 |
| `quotient_poly_compute_time_ms` |  504 |  504 |  504 |  504 |
| `quotient_poly_commit_time_ms` |  775 |  775 |  775 |  775 |
| `pcs_opening_time_ms ` |  928 |  928 |  928 |  928 |

| root |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  23,214 |  23,214 |  23,214 |  23,214 |
| `main_cells_used     ` |  37,522,563 |  37,522,563 |  37,522,563 |  37,522,563 |
| `total_cycles        ` |  759,246 |  759,246 |  759,246 |  759,246 |
| `execute_time_ms     ` |  205 |  205 |  205 |  205 |
| `trace_gen_time_ms   ` |  329 |  329 |  329 |  329 |
| `stark_prove_excluding_trace_time_ms` |  22,680 |  22,680 |  22,680 |  22,680 |
| `main_trace_commit_time_ms` |  6,914 |  6,914 |  6,914 |  6,914 |
| `generate_perm_trace_time_ms` |  42 |  42 |  42 |  42 |
| `perm_trace_commit_time_ms` |  4,292 |  4,292 |  4,292 |  4,292 |
| `quotient_poly_compute_time_ms` |  576 |  576 |  576 |  576 |
| `quotient_poly_commit_time_ms` |  8,438 |  8,438 |  8,438 |  8,438 |
| `pcs_opening_time_ms ` |  2,410 |  2,410 |  2,410 |  2,410 |

| halo2_outer |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  49,018 |  49,018 |  49,018 |  49,018 |
| `main_cells_used     ` |  62,800,919 |  62,800,919 |  62,800,919 |  62,800,919 |

| halo2_wrapper |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  35,718 |  35,718 |  35,718 |  35,718 |

| agg_keygen |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  11,890.50 |  23,781 |  23,330 |  451 |
| `main_cells_used     ` |  43,446,167.50 |  86,892,335 |  86,881,779 |  10,556 |
| `total_cycles        ` |  1,636,779 |  1,636,779 |  1,636,779 |  1,636,779 |
| `execute_time_ms     ` |  106 |  212 |  212 |  0 |
| `trace_gen_time_ms   ` |  172.50 |  345 |  317 |  28 |
| `stark_prove_excluding_trace_time_ms` |  11,612 |  23,224 |  22,801 |  423 |
| `main_trace_commit_time_ms` |  3,459.50 |  6,919 |  6,867 |  52 |
| `generate_perm_trace_time_ms` |  33 |  66 |  53 |  13 |
| `perm_trace_commit_time_ms` |  2,163.50 |  4,327 |  4,276 |  51 |
| `quotient_poly_compute_time_ms` |  298 |  596 |  566 |  30 |
| `quotient_poly_commit_time_ms` |  4,233.50 |  8,467 |  8,408 |  59 |
| `pcs_opening_time_ms ` |  1,409 |  2,818 |  2,604 |  214 |



<details>
<summary>Detailed Metrics</summary>

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

| block_number | execute_time_ms |
| --- | --- |
| 21000000 | 211 | 

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
| internal.0 | AccessAdapterAir<2> | 21000000 | 3 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 0 | 524,288 |  | 12 | 13 | 13,107,200 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 1 | 524,288 |  | 12 | 13 | 13,107,200 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 2 | 524,288 |  | 12 | 13 | 13,107,200 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 3 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 0 | 16,384 |  | 12 | 17 | 475,136 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 1 | 16,384 |  | 12 | 17 | 475,136 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 2 | 16,384 |  | 12 | 17 | 475,136 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 3 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 0 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 1 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 2 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 3 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | JalRangeCheckAir | 21000000 | 0 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | JalRangeCheckAir | 21000000 | 1 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | JalRangeCheckAir | 21000000 | 2 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | JalRangeCheckAir | 21000000 | 3 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 0 | 262,144 |  | 160 | 398 | 146,276,352 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 1 | 262,144 |  | 160 | 398 | 146,276,352 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 2 | 262,144 |  | 160 | 398 | 146,276,352 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 3 | 131,072 |  | 160 | 398 | 73,138,176 | 
| internal.0 | PhantomAir | 21000000 | 0 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 1 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 2 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 3 | 32,768 |  | 8 | 6 | 458,752 | 
| internal.0 | ProgramAir | 21000000 | 0 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 21000000 | 1 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 21000000 | 2 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 21000000 | 3 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 0 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 1 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 2 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 3 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 0 | 524,288 |  | 16 | 23 | 20,447,232 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 1 | 524,288 |  | 16 | 23 | 20,447,232 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 2 | 524,288 |  | 16 | 23 | 20,447,232 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 3 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 0 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 1 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 2 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 3 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 0 | 1,048,576 |  | 24 | 21 | 47,185,920 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 1 | 1,048,576 |  | 24 | 21 | 47,185,920 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 2 | 1,048,576 |  | 24 | 21 | 47,185,920 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 3 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 0 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 1 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 2 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 3 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 0 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 1 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 2 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 3 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmConnectorAir | 21000000 | 0 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VmConnectorAir | 21000000 | 1 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VmConnectorAir | 21000000 | 2 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VmConnectorAir | 21000000 | 3 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 0 | 262,144 |  | 12 | 12 | 6,291,456 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 1 | 262,144 |  | 12 | 12 | 6,291,456 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 2 | 262,144 |  | 12 | 12 | 6,291,456 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 3 | 262,144 |  | 12 | 12 | 6,291,456 | 
| internal.1 | AccessAdapterAir<2> | 21000000 | 4 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.1 | AccessAdapterAir<2> | 21000000 | 5 | 262,144 |  | 12 | 11 | 6,029,312 | 
| internal.1 | AccessAdapterAir<4> | 21000000 | 4 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.1 | AccessAdapterAir<4> | 21000000 | 5 | 131,072 |  | 12 | 13 | 3,276,800 | 
| internal.1 | AccessAdapterAir<8> | 21000000 | 4 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.1 | AccessAdapterAir<8> | 21000000 | 5 | 4,096 |  | 12 | 17 | 118,784 | 
| internal.1 | FriReducedOpeningAir | 21000000 | 4 | 524,288 |  | 44 | 27 | 37,224,448 | 
| internal.1 | FriReducedOpeningAir | 21000000 | 5 | 131,072 |  | 44 | 27 | 9,306,112 | 
| internal.1 | JalRangeCheckAir | 21000000 | 4 | 65,536 |  | 16 | 12 | 1,835,008 | 
| internal.1 | JalRangeCheckAir | 21000000 | 5 | 32,768 |  | 16 | 12 | 917,504 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 4 | 131,072 |  | 160 | 398 | 73,138,176 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 5 | 32,768 |  | 160 | 398 | 18,284,544 | 
| internal.1 | PhantomAir | 21000000 | 4 | 32,768 |  | 8 | 6 | 458,752 | 
| internal.1 | PhantomAir | 21000000 | 5 | 8,192 |  | 8 | 6 | 114,688 | 
| internal.1 | ProgramAir | 21000000 | 4 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.1 | ProgramAir | 21000000 | 5 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 21000000 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 21000000 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 4 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 5 | 524,288 |  | 20 | 29 | 25,690,112 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 4 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 5 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 4 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 5 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 4 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 5 | 262,144 |  | 24 | 21 | 11,796,480 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 4 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 5 | 65,536 |  | 24 | 27 | 3,342,336 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 4 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 5 | 65,536 |  | 20 | 38 | 3,801,088 | 
| internal.1 | VmConnectorAir | 21000000 | 4 | 2 | 1 | 12 | 5 | 34 | 
| internal.1 | VmConnectorAir | 21000000 | 5 | 2 | 1 | 12 | 5 | 34 | 
| internal.1 | VolatileBoundaryAir | 21000000 | 4 | 262,144 |  | 12 | 12 | 6,291,456 | 
| internal.1 | VolatileBoundaryAir | 21000000 | 5 | 131,072 |  | 12 | 12 | 3,145,728 | 
| internal.2 | AccessAdapterAir<2> | 21000000 | 6 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.2 | AccessAdapterAir<4> | 21000000 | 6 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.2 | AccessAdapterAir<8> | 21000000 | 6 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.2 | FriReducedOpeningAir | 21000000 | 6 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.2 | JalRangeCheckAir | 21000000 | 6 | 65,536 |  | 16 | 12 | 1,835,008 | 
| internal.2 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 6 | 65,536 |  | 160 | 398 | 36,569,088 | 
| internal.2 | PhantomAir | 21000000 | 6 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.2 | ProgramAir | 21000000 | 6 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.2 | VariableRangeCheckerAir | 21000000 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.2 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 6 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.2 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 6 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 6 | 64 |  | 16 | 23 | 2,496 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 6 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 6 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 6 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.2 | VmConnectorAir | 21000000 | 6 | 2 | 1 | 12 | 5 | 34 | 
| internal.2 | VolatileBoundaryAir | 21000000 | 6 | 262,144 |  | 12 | 12 | 6,291,456 | 
| leaf | AccessAdapterAir<2> | 21000000 | 0 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<2> | 21000000 | 1 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<2> | 21000000 | 10 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<2> | 21000000 | 2 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<2> | 21000000 | 3 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<2> | 21000000 | 4 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 21000000 | 5 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 21000000 | 6 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 21000000 | 7 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 21000000 | 8 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 21000000 | 9 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<4> | 21000000 | 0 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 21000000 | 1 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 21000000 | 10 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 21000000 | 2 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 21000000 | 3 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 21000000 | 4 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 21000000 | 5 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 21000000 | 6 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 21000000 | 7 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 21000000 | 8 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 21000000 | 9 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<8> | 21000000 | 0 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 21000000 | 1 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 21000000 | 10 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 21000000 | 2 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 21000000 | 3 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 21000000 | 4 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 21000000 | 5 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 21000000 | 6 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 21000000 | 7 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 21000000 | 8 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 21000000 | 9 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | FriReducedOpeningAir | 21000000 | 0 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 21000000 | 1 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | FriReducedOpeningAir | 21000000 | 10 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | FriReducedOpeningAir | 21000000 | 2 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | FriReducedOpeningAir | 21000000 | 3 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | FriReducedOpeningAir | 21000000 | 4 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 21000000 | 5 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 21000000 | 6 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 21000000 | 7 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 21000000 | 8 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 21000000 | 9 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | JalRangeCheckAir | 21000000 | 0 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 1 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 10 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 2 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 3 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 4 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 5 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 6 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 7 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 8 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 21000000 | 9 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 0 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 1 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 10 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 2 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 3 | 131,072 |  | 312 | 398 | 93,061,120 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 4 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 5 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 6 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 7 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 8 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 9 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | PhantomAir | 21000000 | 0 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 21000000 | 1 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 21000000 | 10 | 32,768 |  | 12 | 6 | 589,824 | 
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
| leaf | VariableRangeCheckerAir | 21000000 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 7 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 8 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21000000 | 9 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 0 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 1 | 1,048,576 |  | 36 | 29 | 68,157,440 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 10 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 2 | 1,048,576 |  | 36 | 29 | 68,157,440 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 3 | 1,048,576 |  | 36 | 29 | 68,157,440 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 4 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 5 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 6 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 7 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 8 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 9 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 0 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 1 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 10 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 2 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 3 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 4 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 5 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 6 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 7 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 8 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 9 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 0 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 1 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 10 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 2 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 3 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 4 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 5 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 6 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 7 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 8 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 9 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 0 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 1 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 10 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 2 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 3 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 4 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 5 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 6 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 7 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 8 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 9 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 0 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 1 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 10 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 2 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 3 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 4 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 5 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 6 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 7 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 8 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 9 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 0 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 1 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 10 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 2 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 3 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 4 | 524,288 |  | 36 | 38 | 38,797,312 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 5 | 524,288 |  | 36 | 38 | 38,797,312 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 6 | 524,288 |  | 36 | 38 | 38,797,312 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 7 | 524,288 |  | 36 | 38 | 38,797,312 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 8 | 524,288 |  | 36 | 38 | 38,797,312 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 9 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmConnectorAir | 21000000 | 0 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 1 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 10 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 2 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 3 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 4 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 5 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 6 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 7 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 8 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 21000000 | 9 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VolatileBoundaryAir | 21000000 | 0 | 1,048,576 |  | 20 | 12 | 33,554,432 | 
| leaf | VolatileBoundaryAir | 21000000 | 1 | 524,288 |  | 20 | 12 | 16,777,216 | 
| leaf | VolatileBoundaryAir | 21000000 | 10 | 524,288 |  | 20 | 12 | 16,777,216 | 
| leaf | VolatileBoundaryAir | 21000000 | 2 | 524,288 |  | 20 | 12 | 16,777,216 | 
| leaf | VolatileBoundaryAir | 21000000 | 3 | 524,288 |  | 20 | 12 | 16,777,216 | 
| leaf | VolatileBoundaryAir | 21000000 | 4 | 1,048,576 |  | 20 | 12 | 33,554,432 | 
| leaf | VolatileBoundaryAir | 21000000 | 5 | 1,048,576 |  | 20 | 12 | 33,554,432 | 
| leaf | VolatileBoundaryAir | 21000000 | 6 | 1,048,576 |  | 20 | 12 | 33,554,432 | 
| leaf | VolatileBoundaryAir | 21000000 | 7 | 1,048,576 |  | 20 | 12 | 33,554,432 | 
| leaf | VolatileBoundaryAir | 21000000 | 8 | 1,048,576 |  | 20 | 12 | 33,554,432 | 
| leaf | VolatileBoundaryAir | 21000000 | 9 | 524,288 |  | 20 | 12 | 16,777,216 | 
| root | AccessAdapterAir<2> | 21000000 | 0 | 262,144 |  | 8 | 11 | 4,980,736 | 
| root | AccessAdapterAir<4> | 21000000 | 0 | 131,072 |  | 8 | 13 | 2,752,512 | 
| root | AccessAdapterAir<8> | 21000000 | 0 | 4,096 |  | 8 | 17 | 102,400 | 
| root | FriReducedOpeningAir | 21000000 | 0 | 131,072 |  | 24 | 27 | 6,684,672 | 
| root | JalRangeCheckAir | 21000000 | 0 | 32,768 |  | 12 | 12 | 786,432 | 
| root | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 0 | 32,768 |  | 84 | 398 | 15,794,176 | 
| root | PhantomAir | 21000000 | 0 | 8,192 |  | 8 | 6 | 114,688 | 
| root | ProgramAir | 21000000 | 0 | 131,072 |  | 8 | 10 | 2,359,296 | 
| root | VariableRangeCheckerAir | 21000000 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| root | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 0 | 524,288 |  | 12 | 29 | 21,495,808 | 
| root | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 0 | 131,072 |  | 12 | 23 | 4,587,520 | 
| root | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 0 | 64 |  | 12 | 22 | 2,176 | 
| root | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 0 | 262,144 |  | 16 | 21 | 9,699,328 | 
| root | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 0 | 65,536 |  | 16 | 27 | 2,818,048 | 
| root | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 0 | 65,536 |  | 12 | 38 | 3,276,800 | 
| root | VmConnectorAir | 21000000 | 0 | 2 | 1 | 8 | 5 | 26 | 
| root | VolatileBoundaryAir | 21000000 | 0 | 131,072 |  | 8 | 12 | 2,621,440 | 

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
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 0 | 64 |  | 16 | 25 | 2,624 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 10 | 32 |  | 16 | 25 | 1,312 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 4 | 1,048,576 |  | 16 | 25 | 42,991,616 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 5 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 6 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 7 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 8 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 9 | 8,192 |  | 16 | 25 | 335,872 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<2> | 21000000 | 10 | 131,072 |  | 16 | 11 | 3,538,944 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<2> | 21000000 | 4 | 1,024 |  | 16 | 11 | 27,648 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<2> | 21000000 | 5 | 1,024 |  | 16 | 11 | 27,648 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<2> | 21000000 | 6 | 4,096 |  | 16 | 11 | 110,592 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<2> | 21000000 | 9 | 262,144 |  | 16 | 11 | 7,077,888 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 0 | 32 |  | 16 | 41 | 1,824 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 10 | 16 |  | 16 | 41 | 912 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 4 | 524,288 |  | 16 | 41 | 29,884,416 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 5 | 131,072 |  | 16 | 41 | 7,471,104 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 6 | 131,072 |  | 16 | 41 | 7,471,104 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 7 | 131,072 |  | 16 | 41 | 7,471,104 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 8 | 131,072 |  | 16 | 41 | 7,471,104 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 9 | 4,096 |  | 16 | 41 | 233,472 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 0 | 64 |  | 16 | 13 | 1,856 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 10 | 65,536 |  | 16 | 13 | 1,900,544 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 4 | 512 |  | 16 | 13 | 14,848 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 5 | 512 |  | 16 | 13 | 14,848 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 6 | 4,096 |  | 16 | 13 | 118,784 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 8 | 256 |  | 16 | 13 | 7,424 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 9 | 131,072 |  | 16 | 13 | 3,801,088 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 0 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 1 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 10 | 524,288 |  | 16 | 17 | 17,301,504 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 2 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 3 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 4 | 2,097,152 |  | 16 | 17 | 69,206,016 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 5 | 2,097,152 |  | 16 | 17 | 69,206,016 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 6 | 2,097,152 |  | 16 | 17 | 69,206,016 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 7 | 2,097,152 |  | 16 | 17 | 69,206,016 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 8 | 2,097,152 |  | 16 | 17 | 69,206,016 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 9 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_e2e.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 0 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 1 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 10 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 2 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 3 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 4 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 5 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 6 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 7 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 8 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 9 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 0 | 1 |  | 1,056 | 3,163 | 4,219 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 1 | 1 |  | 1,056 | 3,163 | 4,219 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 10 | 131,072 |  | 1,056 | 3,163 | 552,992,768 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 2 | 1 |  | 1,056 | 3,163 | 4,219 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 3 | 524,288 |  | 1,056 | 3,163 | 2,211,971,072 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 4 | 131,072 |  | 1,056 | 3,163 | 552,992,768 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 5 | 16,384 |  | 1,056 | 3,163 | 69,124,096 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 6 | 16,384 |  | 1,056 | 3,163 | 69,124,096 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 7 | 16,384 |  | 1,056 | 3,163 | 69,124,096 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 8 | 65,536 |  | 1,056 | 3,163 | 276,496,384 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 9 | 262,144 |  | 1,056 | 3,163 | 1,105,985,536 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 0 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 1 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 10 | 2,097,152 |  | 16 | 32 | 100,663,296 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 2 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 3 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 4 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 5 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 6 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 7 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 8 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 9 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 0 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 1 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 10 | 524,288 |  | 12 | 20 | 16,777,216 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 2 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 3 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 4 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 5 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 6 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 7 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 8 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 9 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 0 | 4 |  | 12 | 6 | 72 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 1 | 1 |  | 12 | 6 | 18 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 10 | 1 |  | 12 | 6 | 18 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 2 | 1 |  | 12 | 6 | 18 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 3 | 2 |  | 12 | 6 | 36 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 4 | 256 |  | 12 | 6 | 4,608 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 5 | 4 |  | 12 | 6 | 72 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 6 | 32 |  | 12 | 6 | 576 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 7 | 8 |  | 12 | 6 | 144 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 8 | 32 |  | 12 | 6 | 576 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 9 | 8 |  | 12 | 6 | 144 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 0 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 1 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 10 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 2 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 3 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 4 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 5 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 6 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 7 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 8 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 9 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 0 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 1 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 10 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 2 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 3 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 4 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 5 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 6 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 7 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 8 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 9 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_e2e.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 0 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 1 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 10 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 2 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 3 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 4 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 5 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 6 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 7 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 8 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 9 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 0 | 524,288 |  | 44 | 32 | 39,845,888 | 
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 1 | 524,288 |  | 44 | 32 | 39,845,888 | 
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 2 | 524,288 |  | 44 | 32 | 39,845,888 | 
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 3 | 1,048,576 |  | 44 | 32 | 79,691,776 | 
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 4 | 2,048 |  | 44 | 32 | 155,648 | 
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 5 | 16 |  | 44 | 32 | 1,216 | 
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 6 | 64 |  | 44 | 32 | 4,864 | 
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 7 | 32 |  | 44 | 32 | 2,432 | 
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 8 | 64 |  | 44 | 32 | 4,864 | 
| reth.prove_e2e.block_21000000 | VariableRangeCheckerAir | 21000000 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21000000 | VariableRangeCheckerAir | 21000000 | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21000000 | VariableRangeCheckerAir | 21000000 | 10 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21000000 | VariableRangeCheckerAir | 21000000 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21000000 | VariableRangeCheckerAir | 21000000 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21000000 | VariableRangeCheckerAir | 21000000 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21000000 | VariableRangeCheckerAir | 21000000 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21000000 | VariableRangeCheckerAir | 21000000 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21000000 | VariableRangeCheckerAir | 21000000 | 7 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21000000 | VariableRangeCheckerAir | 21000000 | 8 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21000000 | VariableRangeCheckerAir | 21000000 | 9 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 0 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 1 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 10 | 2,097,152 |  | 52 | 36 | 184,549,376 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 2 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 3 | 2,097,152 |  | 52 | 36 | 184,549,376 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 4 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 5 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 6 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 7 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 8 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 9 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 0 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 1 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 10 | 131,072 |  | 40 | 37 | 10,092,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 2 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 3 | 131,072 |  | 40 | 37 | 10,092,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 4 | 262,144 |  | 40 | 37 | 20,185,088 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 5 | 1,048,576 |  | 40 | 37 | 80,740,352 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 6 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 7 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 8 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 9 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 0 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 1 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 10 | 262,144 |  | 52 | 53 | 27,525,120 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 2 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 3 | 262,144 |  | 52 | 53 | 27,525,120 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 4 | 2,097,152 |  | 52 | 53 | 220,200,960 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 5 | 2,097,152 |  | 52 | 53 | 220,200,960 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 6 | 2,097,152 |  | 52 | 53 | 220,200,960 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 7 | 2,097,152 |  | 52 | 53 | 220,200,960 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 8 | 2,097,152 |  | 52 | 53 | 220,200,960 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 9 | 524,288 |  | 52 | 53 | 55,050,240 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 0 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 1 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 10 | 1,048,576 |  | 28 | 26 | 56,623,104 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 2 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 3 | 524,288 |  | 28 | 26 | 28,311,552 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 4 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 5 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 6 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 7 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 8 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 9 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 0 | 1,048,576 |  | 32 | 32 | 67,108,864 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 1 | 1,048,576 |  | 32 | 32 | 67,108,864 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 10 | 131,072 |  | 32 | 32 | 8,388,608 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 2 | 1,048,576 |  | 32 | 32 | 67,108,864 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 3 | 262,144 |  | 32 | 32 | 16,777,216 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 4 | 2,097,152 |  | 32 | 32 | 134,217,728 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 5 | 2,097,152 |  | 32 | 32 | 134,217,728 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 6 | 4,194,304 |  | 32 | 32 | 268,435,456 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 7 | 2,097,152 |  | 32 | 32 | 134,217,728 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 8 | 2,097,152 |  | 32 | 32 | 134,217,728 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 9 | 262,144 |  | 32 | 32 | 16,777,216 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 0 | 1,048,576 |  | 28 | 18 | 48,234,496 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 1 | 1,048,576 |  | 28 | 18 | 48,234,496 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 10 | 131,072 |  | 28 | 18 | 6,029,312 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 2 | 1,048,576 |  | 28 | 18 | 48,234,496 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 3 | 262,144 |  | 28 | 18 | 12,058,624 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 4 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 5 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 6 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 7 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 8 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 9 | 262,144 |  | 28 | 18 | 12,058,624 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 10 | 1 |  | 192 | 168 | 360 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 4 | 4,096 |  | 192 | 168 | 1,474,560 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 5 | 16,384 |  | 192 | 168 | 5,898,240 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 6 | 16,384 |  | 192 | 168 | 5,898,240 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 7 | 16,384 |  | 192 | 168 | 5,898,240 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 8 | 16,384 |  | 192 | 168 | 5,898,240 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 9 | 512 |  | 192 | 168 | 184,320 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 4 | 2,048 |  | 68 | 169 | 485,376 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 5 | 4,096 |  | 68 | 169 | 970,752 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 6 | 8,192 |  | 68 | 169 | 1,941,504 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 7 | 4,096 |  | 68 | 169 | 970,752 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 8 | 4,096 |  | 68 | 169 | 970,752 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 4 | 512 |  | 192 | 164 | 182,272 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 5 | 2,048 |  | 192 | 164 | 729,088 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 6 | 1,024 |  | 192 | 164 | 364,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 7 | 1,024 |  | 192 | 164 | 364,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 8 | 512 |  | 192 | 164 | 182,272 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 4 | 1,024 |  | 164 | 241 | 414,720 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 5 | 2,048 |  | 164 | 241 | 829,440 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 6 | 2,048 |  | 164 | 241 | 829,440 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 7 | 2,048 |  | 164 | 241 | 829,440 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 8 | 2,048 |  | 164 | 241 | 829,440 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 10 | 2 |  | 48 | 124 | 344 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 4 | 8,192 |  | 48 | 124 | 1,409,024 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 5 | 32,768 |  | 48 | 124 | 5,636,096 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 6 | 16,384 |  | 48 | 124 | 2,818,048 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 7 | 16,384 |  | 48 | 124 | 2,818,048 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 8 | 16,384 |  | 48 | 124 | 2,818,048 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 9 | 512 |  | 48 | 124 | 88,064 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 0 | 1 |  | 56 | 166 | 222 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 4 | 131,072 |  | 56 | 166 | 29,097,984 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 5 | 1,024 |  | 56 | 166 | 227,328 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 6 | 4,096 |  | 56 | 166 | 909,312 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 7 | 2,048 |  | 56 | 166 | 454,656 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 8 | 4,096 |  | 56 | 166 | 909,312 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 0 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 1 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 10 | 262,144 |  | 36 | 28 | 16,777,216 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 2 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 3 | 131,072 |  | 36 | 28 | 8,388,608 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 4 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 5 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 6 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 7 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 8 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 9 | 262,144 |  | 36 | 28 | 16,777,216 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 0 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 1 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 10 | 524,288 |  | 52 | 36 | 46,137,344 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 2 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 3 | 262,144 |  | 52 | 36 | 23,068,672 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 4 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 5 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 6 | 2,097,152 |  | 52 | 36 | 184,549,376 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 7 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 8 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 9 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 0 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 1 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 10 | 2,097,152 |  | 52 | 41 | 195,035,136 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 2 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 3 | 4,194,304 |  | 52 | 41 | 390,070,272 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 4 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 5 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 6 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 7 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 8 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 9 | 4,194,304 |  | 52 | 41 | 390,070,272 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 4 | 64 |  | 72 | 59 | 8,384 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 5 | 256 |  | 72 | 59 | 33,536 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 6 | 256 |  | 72 | 59 | 33,536 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 7 | 64 |  | 72 | 59 | 8,384 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 8 | 128 |  | 72 | 59 | 16,768 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 9 | 512 |  | 72 | 59 | 67,072 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 1 | 512 |  | 72 | 39 | 56,832 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 10 | 64 |  | 72 | 39 | 7,104 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 2 | 1,024 |  | 72 | 39 | 113,664 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 3 | 4,096 |  | 72 | 39 | 454,656 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 4 | 65,536 |  | 72 | 39 | 7,274,496 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 5 | 131,072 |  | 72 | 39 | 14,548,992 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 6 | 131,072 |  | 72 | 39 | 14,548,992 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 7 | 131,072 |  | 72 | 39 | 14,548,992 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 8 | 131,072 |  | 72 | 39 | 14,548,992 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 9 | 16,384 |  | 72 | 39 | 1,818,624 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 0 | 256 |  | 52 | 31 | 21,248 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 1 | 1,024 |  | 52 | 31 | 84,992 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 10 | 16,384 |  | 52 | 31 | 1,359,872 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 2 | 4,096 |  | 52 | 31 | 339,968 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 3 | 8,192 |  | 52 | 31 | 679,936 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 4 | 262,144 |  | 52 | 31 | 21,757,952 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 5 | 262,144 |  | 52 | 31 | 21,757,952 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 6 | 262,144 |  | 52 | 31 | 21,757,952 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 7 | 524,288 |  | 52 | 31 | 43,515,904 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 8 | 262,144 |  | 52 | 31 | 21,757,952 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 9 | 65,536 |  | 52 | 31 | 5,439,488 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 0 | 262,144 |  | 28 | 20 | 12,582,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 1 | 262,144 |  | 28 | 20 | 12,582,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 10 | 131,072 |  | 28 | 20 | 6,291,456 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 2 | 262,144 |  | 28 | 20 | 12,582,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 3 | 65,536 |  | 28 | 20 | 3,145,728 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 4 | 262,144 |  | 28 | 20 | 12,582,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 5 | 65,536 |  | 28 | 20 | 3,145,728 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 6 | 65,536 |  | 28 | 20 | 3,145,728 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 7 | 131,072 |  | 28 | 20 | 6,291,456 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 8 | 131,072 |  | 28 | 20 | 6,291,456 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 9 | 131,072 |  | 28 | 20 | 6,291,456 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 0 | 1 |  | 836 | 547 | 1,383 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 65,536 |  | 836 | 547 | 90,636,288 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 5 | 256 |  | 836 | 547 | 354,048 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 6 | 2,048 |  | 836 | 547 | 2,832,384 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 7 | 512 |  | 836 | 547 | 708,096 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 8 | 2,048 |  | 836 | 547 | 2,832,384 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 0 | 1 |  | 320 | 263 | 583 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 1,024 |  | 320 | 263 | 596,992 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 5 | 4 |  | 320 | 263 | 2,332 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 6 | 32 |  | 320 | 263 | 18,656 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 7 | 8 |  | 320 | 263 | 4,664 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 8 | 32 |  | 320 | 263 | 18,656 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 0 | 1 |  | 860 | 625 | 1,485 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 32,768 |  | 860 | 625 | 48,660,480 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 5 | 256 |  | 860 | 625 | 380,160 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 6 | 1,024 |  | 860 | 625 | 1,520,640 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 7 | 512 |  | 860 | 625 | 760,320 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 8 | 1,024 |  | 860 | 625 | 1,520,640 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 0 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 1 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 10 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 2 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 3 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 4 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 5 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 6 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 7 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 8 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 9 | 2 | 1 | 16 | 5 | 42 | 

| group | block_number | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | num_segments | main_trace_commit_time_ms | main_cells_used | halo2_total_cells | halo2_keygen_time_ms | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 317 | 23,330 | 1,636,779 | 270,872,042 | 22,801 | 566 | 8,408 | 4,276 | 2,604 | 1 | 6,867 | 86,881,779 | 8,037,489 | 17,927 | 53 | 212 | 
| halo2_outer | 21000000 |  | 49,018 |  |  |  |  |  |  |  |  |  | 62,800,919 |  |  |  |  | 
| halo2_wrapper | 21000000 |  | 35,718 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
| reth.prove_e2e.block_21000000 | 21000000 |  |  |  |  |  |  |  |  |  | 11 |  |  |  |  |  |  | 

| group | block_number | cell_tracker_span | simple_advice_cells | lookup_advice_cells | fixed_cells |
| --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | VerifierProgram | 482,930 | 155,510 | 158,234 | 
| agg_keygen | 21000000 | VerifierProgram;CheckTraceHeightConstraints | 4,789 | 972 | 1,738 | 
| agg_keygen | 21000000 | VerifierProgram;PoseidonCell | 29,400 |  | 8,700 | 
| agg_keygen | 21000000 | VerifierProgram;stage-c-build-rounds | 19,526 | 2,717 | 6,696 | 
| agg_keygen | 21000000 | VerifierProgram;stage-c-build-rounds;PoseidonCell | 46,550 |  | 13,775 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs | 1,365,246 | 211,617 | 481,258 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;PoseidonCell | 3,839,150 |  | 1,136,075 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify | 45,125 | 5,543 | 19,412 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;PoseidonCell | 68,600 |  | 20,300 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;cache-generator-powers | 66,304 | 11,396 | 20,384 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;compute-reduced-opening;single-reduced-opening-eval | 7,994,476 | 335,356 | 1,482,124 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;pre-compute-rounds-context | 76,224 | 11,116 | 22,232 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch | 49,728 |  | 6,216 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;PoseidonCell | 9,264,780 |  | 2,744,280 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;verify-batch-reduce-fast;PoseidonCell | 8,263,864 | 237,048 | 2,580,396 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query | 953,456 | 165,676 | 272,356 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext | 102,144 |  | 12,768 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;PoseidonCell | 15,647,184 |  | 4,634,784 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;verify-batch-reduce-fast;PoseidonCell | 1,550,612 | 56,000 | 476,812 | 
| agg_keygen | 21000000 | VerifierProgram;stage-e-verify-constraints | 9,770,542 | 1,967,337 | 3,013,652 | 

| group | block_number | idx | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | 21000000 | 0 | 1,580 | 9,323 | 3,564,039 | 472,992,226 | 6,369 | 1,112 | 1,247 | 1,093 | 1,463 | 1,232 | 211,573,259 | 218 | 1,374 | 
| internal.0 | 21000000 | 1 | 1,571 | 9,270 | 3,580,243 | 472,992,226 | 6,314 | 1,102 | 1,209 | 1,093 | 1,472 | 1,228 | 213,027,551 | 205 | 1,385 | 
| internal.0 | 21000000 | 2 | 1,560 | 9,305 | 3,596,330 | 472,992,226 | 6,331 | 1,108 | 1,239 | 1,077 | 1,462 | 1,219 | 214,268,888 | 221 | 1,414 | 
| internal.0 | 21000000 | 3 | 1,032 | 7,215 | 2,365,607 | 346,728,930 | 5,279 | 855 | 1,021 | 884 | 1,405 | 947 | 140,444,318 | 162 | 904 | 
| internal.1 | 21000000 | 4 | 876 | 6,105 | 2,298,793 | 300,984,802 | 4,584 | 735 | 913 | 731 | 1,232 | 834 | 111,970,069 | 134 | 645 | 
| internal.1 | 21000000 | 5 | 325 | 2,643 | 766,217 | 95,656,418 | 2,100 | 282 | 483 | 290 | 674 | 316 | 38,187,759 | 49 | 218 | 
| internal.2 | 21000000 | 6 | 613 | 4,372 | 1,516,178 | 186,591,714 | 3,348 | 504 | 775 | 502 | 928 | 547 | 73,850,454 | 87 | 411 | 
| leaf | 21000000 | 0 | 1,895 | 9,328 | 3,028,747 | 1,037,143,530 | 6,452 | 919 | 678 | 2,016 | 1,507 | 984 | 247,922,310 | 343 | 981 | 
| leaf | 21000000 | 1 | 1,265 | 6,237 | 2,040,215 | 664,751,594 | 4,243 | 622 | 464 | 1,213 | 1,072 | 659 | 156,789,400 | 209 | 729 | 
| leaf | 21000000 | 10 | 1,383 | 6,630 | 2,256,858 | 732,909,034 | 4,511 | 676 | 488 | 1,307 | 1,072 | 728 | 168,403,082 | 234 | 736 | 
| leaf | 21000000 | 2 | 1,258 | 6,178 | 2,040,178 | 664,751,594 | 4,188 | 622 | 460 | 1,217 | 1,022 | 655 | 156,789,969 | 208 | 732 | 
| leaf | 21000000 | 3 | 1,236 | 5,910 | 2,024,350 | 571,690,474 | 3,964 | 573 | 455 | 1,090 | 1,070 | 557 | 155,251,503 | 214 | 710 | 
| leaf | 21000000 | 4 | 2,141 | 10,038 | 3,459,146 | 1,100,058,090 | 6,812 | 1,009 | 751 | 2,018 | 1,616 | 1,054 | 278,251,864 | 360 | 1,085 | 
| leaf | 21000000 | 5 | 2,130 | 10,043 | 3,457,904 | 1,100,058,090 | 6,824 | 1,006 | 739 | 2,031 | 1,624 | 1,057 | 278,335,104 | 362 | 1,089 | 
| leaf | 21000000 | 6 | 2,118 | 10,062 | 3,458,404 | 1,100,058,090 | 6,853 | 1,010 | 735 | 2,045 | 1,620 | 1,057 | 278,430,604 | 380 | 1,091 | 
| leaf | 21000000 | 7 | 2,105 | 10,044 | 3,378,710 | 1,100,058,090 | 6,863 | 1,019 | 746 | 2,043 | 1,623 | 1,062 | 275,169,550 | 366 | 1,076 | 
| leaf | 21000000 | 8 | 2,102 | 10,049 | 3,418,759 | 1,100,058,090 | 6,868 | 1,008 | 748 | 2,052 | 1,624 | 1,072 | 276,890,960 | 359 | 1,079 | 
| leaf | 21000000 | 9 | 1,365 | 6,732 | 2,337,502 | 746,278,378 | 4,582 | 683 | 508 | 1,316 | 1,095 | 744 | 174,991,387 | 231 | 785 | 
| root | 21000000 | 0 | 329 | 23,214 | 759,246 | 80,435,354 | 22,680 | 576 | 8,438 | 4,292 | 2,410 | 6,914 | 37,522,563 | 42 | 205 | 

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
| internal.0 | 21000000 | 3 | 0 | 9,764,996 | 2,013,265,921 | 
| internal.0 | 21000000 | 3 | 1 | 50,356,480 | 2,013,265,921 | 
| internal.0 | 21000000 | 3 | 2 | 4,882,498 | 2,013,265,921 | 
| internal.0 | 21000000 | 3 | 3 | 50,610,436 | 2,013,265,921 | 
| internal.0 | 21000000 | 3 | 4 | 262,144 | 2,013,265,921 | 
| internal.0 | 21000000 | 3 | 5 | 116,269,770 | 2,013,265,921 | 
| internal.1 | 21000000 | 4 | 0 | 8,323,204 | 2,013,265,921 | 
| internal.1 | 21000000 | 4 | 1 | 40,001,792 | 2,013,265,921 | 
| internal.1 | 21000000 | 4 | 2 | 4,161,602 | 2,013,265,921 | 
| internal.1 | 21000000 | 4 | 3 | 40,124,676 | 2,013,265,921 | 
| internal.1 | 21000000 | 4 | 4 | 262,144 | 2,013,265,921 | 
| internal.1 | 21000000 | 4 | 5 | 93,266,634 | 2,013,265,921 | 
| internal.1 | 21000000 | 5 | 0 | 2,572,420 | 2,013,265,921 | 
| internal.1 | 21000000 | 5 | 1 | 12,005,632 | 2,013,265,921 | 
| internal.1 | 21000000 | 5 | 2 | 1,286,210 | 2,013,265,921 | 
| internal.1 | 21000000 | 5 | 3 | 12,067,076 | 2,013,265,921 | 
| internal.1 | 21000000 | 5 | 4 | 65,536 | 2,013,265,921 | 
| internal.1 | 21000000 | 5 | 5 | 28,390,090 | 2,013,265,921 | 
| internal.2 | 21000000 | 6 | 0 | 5,144,708 | 2,013,265,921 | 
| internal.2 | 21000000 | 6 | 1 | 24,011,008 | 2,013,265,921 | 
| internal.2 | 21000000 | 6 | 2 | 2,572,354 | 2,013,265,921 | 
| internal.2 | 21000000 | 6 | 3 | 24,133,892 | 2,013,265,921 | 
| internal.2 | 21000000 | 6 | 4 | 131,072 | 2,013,265,921 | 
| internal.2 | 21000000 | 6 | 5 | 56,386,250 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 0 | 18,022,532 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 1 | 123,437,312 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 2 | 9,011,266 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 3 | 125,108,484 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 5 | 278,463,178 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 0 | 9,896,068 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 1 | 73,318,656 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 2 | 4,948,034 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 3 | 73,433,348 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 5 | 164,479,690 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 0 | 11,993,220 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 1 | 79,610,112 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 2 | 5,996,610 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 3 | 79,724,804 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 5 | 180,208,330 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 0 | 9,896,068 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 1 | 73,318,656 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 2 | 4,948,034 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 3 | 73,433,348 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 5 | 164,479,690 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 0 | 9,371,780 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 1 | 64,930,048 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 2 | 4,685,890 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 3 | 65,044,740 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 4 | 262,144 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 5 | 146,653,898 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 0 | 18,546,820 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 1 | 129,728,768 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 2 | 9,273,410 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 3 | 129,827,076 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 5 | 290,259,658 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 0 | 18,546,820 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 1 | 129,728,768 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 2 | 9,273,410 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 3 | 129,827,076 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 5 | 290,259,658 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 0 | 18,546,820 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 1 | 129,728,768 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 2 | 9,273,410 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 3 | 129,827,076 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 5 | 290,259,658 | 2,013,265,921 | 
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
| leaf | 21000000 | 9 | 0 | 12,517,508 | 2,013,265,921 | 
| leaf | 21000000 | 9 | 1 | 80,658,688 | 2,013,265,921 | 
| leaf | 21000000 | 9 | 2 | 6,258,754 | 2,013,265,921 | 
| leaf | 21000000 | 9 | 3 | 80,773,380 | 2,013,265,921 | 
| leaf | 21000000 | 9 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 9 | 5 | 183,091,914 | 2,013,265,921 | 
| root | 21000000 | 0 | 0 | 2,252,928 | 2,013,265,921 | 
| root | 21000000 | 0 | 1 | 14,557,184 | 2,013,265,921 | 
| root | 21000000 | 0 | 2 | 1,126,464 | 2,013,265,921 | 
| root | 21000000 | 0 | 3 | 15,540,224 | 2,013,265,921 | 
| root | 21000000 | 0 | 4 | 262,144 | 2,013,265,921 | 
| root | 21000000 | 0 | 5 | 34,263,234 | 2,013,265,921 | 

| group | block_number | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 0 | 28 | 451 |  | 7,747,601 | 423 | 30 | 59 | 51 | 214 | 52 | 10,556 | 13 | 0 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 8,796 | 25,811 | 20,427,755 | 2,548,470,041 | 14,247 | 2,108 | 1,358 | 3,881 | 3,110 | 2,868 | 992,299,176 | 905 | 2,768 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 8,728 | 25,296 | 20,434,219 | 2,548,578,999 | 13,817 | 2,093 | 1,243 | 3,743 | 3,097 | 2,775 | 988,593,979 | 859 | 2,751 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 4,862 | 13,844 | 4,953,011 | 1,606,281,068 | 8,252 | 1,859 | 597 | 1,606 | 1,483 | 2,386 | 735,096,389 | 312 | 730 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 9,152 | 25,658 | 20,452,500 | 2,548,890,807 | 13,750 | 2,119 | 1,173 | 3,714 | 3,097 | 2,788 | 990,119,560 | 850 | 2,756 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 4,611 | 22,692 | 5,823,559 | 3,308,081,230 | 17,258 | 4,653 | 791 | 3,284 | 1,935 | 5,965 | 1,407,488,424 | 620 | 823 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 9,672 | 33,455 | 23,489,166 | 3,343,298,794 | 17,733 | 3,316 | 1,453 | 4,561 | 3,501 | 3,917 | 1,402,416,642 | 972 | 6,050 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 10,271 | 28,375 | 23,850,039 | 2,704,491,686 | 14,941 | 2,346 | 1,376 | 4,038 | 3,403 | 2,850 | 1,093,627,152 | 912 | 3,163 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 10,985 | 30,342 | 24,897,409 | 2,892,926,986 | 15,900 | 2,499 | 1,512 | 4,339 | 3,521 | 3,048 | 1,157,943,576 | 965 | 3,457 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 10,669 | 28,845 | 24,315,144 | 2,686,742,498 | 14,783 | 2,364 | 1,298 | 4,019 | 3,372 | 2,825 | 1,115,686,473 | 891 | 3,393 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 11,021 | 30,120 | 24,112,134 | 2,875,560,842 | 15,683 | 2,669 | 1,355 | 4,169 | 3,367 | 3,189 | 1,281,217,615 | 918 | 3,416 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 7,349 | 25,937 | 11,658,127 | 3,078,921,914 | 16,461 | 3,543 | 1,085 | 3,738 | 2,933 | 4,391 | 1,513,159,352 | 762 | 2,127 | 

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
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 0 | 49,807,902 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 1 | 141,035,676 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 2 | 24,903,951 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 3 | 166,202,506 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 6 | 56,361,113 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 8 | 1,024 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 9 | 448,666,860 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 0 | 49,810,440 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 1 | 141,042,778 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 2 | 24,905,220 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 3 | 166,208,606 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 6 | 56,361,610 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 8 | 8,192 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 9 | 448,691,534 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 0 | 13,926,540 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 1 | 51,544,620 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 2 | 6,963,270 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 3 | 58,425,856 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 4 | 6,815,744 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 5 | 2,621,440 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 6 | 31,588,452 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 8 | 66,048 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 9 | 176,015,202 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 0 | 49,817,608 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 1 | 141,064,282 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 2 | 24,908,804 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 3 | 166,230,110 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 6 | 56,362,122 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 8 | 24,576 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 9 | 448,762,190 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 0 | 19,554,312 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 1 | 103,751,680 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 2 | 9,777,156 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 3 | 112,664,580 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 6 | 89,198,592 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 8 | 65,536 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 9 | 344,842,256 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 0 | 52,798,084 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 1 | 170,270,592 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 2 | 26,399,042 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 3 | 236,365,188 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 6 | 77,454,464 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 8 | 1,589,760 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 9 | 574,707,530 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 0 | 53,497,408 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 1 | 158,818,004 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 2 | 26,748,704 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 3 | 188,050,016 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 6 | 63,141,188 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 8 | 2,164,736 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 9 | 502,250,456 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 0 | 58,725,220 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 1 | 170,345,248 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 2 | 29,362,610 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 3 | 203,713,380 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 6 | 65,770,336 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 8 | 2,131,968 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 9 | 539,879,162 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 0 | 53,072,124 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 1 | 157,245,224 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 2 | 26,536,062 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 3 | 186,677,820 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 6 | 61,880,072 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 8 | 3,179,008 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 9 | 498,420,710 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 0 | 52,653,668 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 1 | 160,140,064 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 2 | 26,326,834 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 3 | 190,370,404 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 6 | 68,669,024 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 8 | 2,114,560 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 9 | 510,104,954 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 0 | 36,080,660 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 1 | 128,861,184 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 2 | 18,040,330 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 3 | 141,562,884 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 6 | 84,052,480 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 8 | 397,312 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 9 | 419,349,538 | 2,013,265,921 | 

| group | block_number | trace_height_constraint | weighted_sum | threshold |
| --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 0 | 5,701,764 | 2,013,265,921 | 
| agg_keygen | 21000000 | 1 | 28,467,456 | 2,013,265,921 | 
| agg_keygen | 21000000 | 2 | 2,850,882 | 2,013,265,921 | 
| agg_keygen | 21000000 | 3 | 28,197,124 | 2,013,265,921 | 
| agg_keygen | 21000000 | 4 | 262,144 | 2,013,265,921 | 
| agg_keygen | 21000000 | 5 | 65,741,514 | 2,013,265,921 | 

</details>


Commit: https://github.com/axiom-crypto/openvm-reth-benchmark/commit/0220229ebcabed2aed58b2f2a6c7fc1c3cb5ab5c

Max Segment Length: 8388508

Instance Type: m8g.24xlarge

Memory Allocator: jemalloc

[Benchmark Workflow](https://github.com/axiom-crypto/openvm-reth-benchmark/actions/runs/14691825051)
