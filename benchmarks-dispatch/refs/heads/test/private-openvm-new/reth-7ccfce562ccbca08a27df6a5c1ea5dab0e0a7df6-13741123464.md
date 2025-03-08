| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total |  575.14 |  175 |
| reth.prove_e2e.block_21000000 |  295.97 |  36.47 |
| leaf |  104.51 |  11.51 |
| internal.0 |  44.37 |  7.99 |
| internal.1 |  13.06 |  4.44 |
| internal.2 |  6.93 |  4.28 |
| internal.3 |  4.30 |  4.30 |
| root |  22.74 |  22.74 |
| halo2_outer |  48.10 |  48.10 |
| halo2_wrapper |  35.17 |  35.17 |
| agg_keygen |  23.41 |  22.97 |


| reth.prove_e2e.block_21000000 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  26,906.64 |  295,973 |  36,470 |  7,347 |
| `main_cells_used     ` |  1,153,940,203.91 |  12,693,342,243 |  1,939,000,338 |  290,363,484 |
| `total_cycles        ` |  18,533,831.82 |  203,872,150 |  24,818,326 |  1,960,929 |
| `execute_time_ms     ` |  3,020 |  33,220 |  6,208 |  284 |
| `trace_gen_time_ms   ` |  8,742 |  96,162 |  11,120 |  3,167 |
| `stark_prove_excluding_trace_time_ms` |  15,144.64 |  166,591 |  24,283 |  3,896 |
| `main_trace_commit_time_ms` |  3,473.09 |  38,204 |  7,575 |  850 |
| `generate_perm_trace_time_ms` |  802.09 |  8,823 |  1,017 |  146 |
| `perm_trace_commit_time_ms` |  3,807.91 |  41,887 |  5,219 |  726 |
| `quotient_poly_compute_time_ms` |  2,798.09 |  30,779 |  5,812 |  636 |
| `quotient_poly_commit_time_ms` |  1,232.73 |  13,560 |  1,523 |  448 |
| `pcs_opening_time_ms ` |  3,017.64 |  33,194 |  3,596 |  1,079 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  9,500.55 |  104,506 |  11,513 |  7,121 |
| `main_cells_used     ` |  239,626,315.55 |  2,635,889,471 |  301,153,407 |  167,644,411 |
| `total_cycles        ` |  3,255,364.45 |  35,809,009 |  4,077,547 |  2,301,161 |
| `execute_time_ms     ` |  995.36 |  10,949 |  1,195 |  759 |
| `trace_gen_time_ms   ` |  1,879.36 |  20,673 |  2,362 |  1,315 |
| `stark_prove_excluding_trace_time_ms` |  6,625.82 |  72,884 |  7,982 |  5,004 |
| `main_trace_commit_time_ms` |  1,018 |  11,198 |  1,214 |  753 |
| `generate_perm_trace_time_ms` |  367.27 |  4,040 |  470 |  243 |
| `perm_trace_commit_time_ms` |  1,860.27 |  20,463 |  2,370 |  1,282 |
| `quotient_poly_compute_time_ms` |  962.09 |  10,583 |  1,197 |  689 |
| `quotient_poly_commit_time_ms` |  745 |  8,195 |  862 |  606 |
| `pcs_opening_time_ms ` |  1,668.18 |  18,350 |  1,906 |  1,427 |

| internal.0 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  7,394.83 |  44,369 |  7,991 |  4,496 |
| `main_cells_used     ` |  135,162,901.50 |  810,977,409 |  147,362,225 |  74,686,303 |
| `total_cycles        ` |  2,289,255.67 |  13,735,534 |  2,497,560 |  1,248,950 |
| `execute_time_ms     ` |  897.67 |  5,386 |  987 |  492 |
| `trace_gen_time_ms   ` |  1,011.67 |  6,070 |  1,119 |  557 |
| `stark_prove_excluding_trace_time_ms` |  5,485.50 |  32,913 |  5,917 |  3,447 |
| `main_trace_commit_time_ms` |  1,067.50 |  6,405 |  1,162 |  617 |
| `generate_perm_trace_time_ms` |  193.67 |  1,162 |  212 |  129 |
| `perm_trace_commit_time_ms` |  933 |  5,598 |  1,021 |  554 |
| `quotient_poly_compute_time_ms` |  938.67 |  5,632 |  1,018 |  560 |
| `quotient_poly_commit_time_ms` |  1,041.83 |  6,251 |  1,131 |  703 |
| `pcs_opening_time_ms ` |  1,304.33 |  7,826 |  1,393 |  879 |

| internal.1 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  4,353 |  13,059 |  4,442 |  4,269 |
| `main_cells_used     ` |  78,562,532.67 |  235,687,598 |  78,779,464 |  78,128,877 |
| `total_cycles        ` |  1,633,343.67 |  4,900,031 |  1,635,785 |  1,628,484 |
| `execute_time_ms     ` |  452.67 |  1,358 |  457 |  447 |
| `trace_gen_time_ms   ` |  643 |  1,929 |  665 |  612 |
| `stark_prove_excluding_trace_time_ms` |  3,257.33 |  9,772 |  3,343 |  3,203 |
| `main_trace_commit_time_ms` |  538.33 |  1,615 |  546 |  534 |
| `generate_perm_trace_time_ms` |  95.67 |  287 |  103 |  91 |
| `perm_trace_commit_time_ms` |  490.33 |  1,471 |  494 |  487 |
| `quotient_poly_compute_time_ms` |  492 |  1,476 |  501 |  486 |
| `quotient_poly_commit_time_ms` |  762 |  2,286 |  799 |  742 |
| `pcs_opening_time_ms ` |  873 |  2,619 |  909 |  850 |

| internal.2 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  3,464 |  6,928 |  4,281 |  2,647 |
| `main_cells_used     ` |  58,534,597 |  117,069,194 |  77,463,643 |  39,605,551 |
| `total_cycles        ` |  1,215,859.50 |  2,431,719 |  1,620,899 |  810,820 |
| `execute_time_ms     ` |  331 |  662 |  441 |  221 |
| `trace_gen_time_ms   ` |  484.50 |  969 |  626 |  343 |
| `stark_prove_excluding_trace_time_ms` |  2,648.50 |  5,297 |  3,214 |  2,083 |
| `main_trace_commit_time_ms` |  424 |  848 |  532 |  316 |
| `generate_perm_trace_time_ms` |  72.50 |  145 |  95 |  50 |
| `perm_trace_commit_time_ms` |  383.50 |  767 |  483 |  284 |
| `quotient_poly_compute_time_ms` |  382.50 |  765 |  486 |  279 |
| `quotient_poly_commit_time_ms` |  630.50 |  1,261 |  708 |  553 |
| `pcs_opening_time_ms ` |  748 |  1,496 |  903 |  593 |

| internal.3 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  4,302 |  4,302 |  4,302 |  4,302 |
| `main_cells_used     ` |  76,941,672 |  76,941,672 |  76,941,672 |  76,941,672 |
| `total_cycles        ` |  1,613,501 |  1,613,501 |  1,613,501 |  1,613,501 |
| `execute_time_ms     ` |  432 |  432 |  432 |  432 |
| `trace_gen_time_ms   ` |  618 |  618 |  618 |  618 |
| `stark_prove_excluding_trace_time_ms` |  3,252 |  3,252 |  3,252 |  3,252 |
| `main_trace_commit_time_ms` |  543 |  543 |  543 |  543 |
| `generate_perm_trace_time_ms` |  103 |  103 |  103 |  103 |
| `perm_trace_commit_time_ms` |  490 |  490 |  490 |  490 |
| `quotient_poly_compute_time_ms` |  503 |  503 |  503 |  503 |
| `quotient_poly_commit_time_ms` |  729 |  729 |  729 |  729 |
| `pcs_opening_time_ms ` |  879 |  879 |  879 |  879 |

| root |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  22,736 |  22,736 |  22,736 |  22,736 |
| `main_cells_used     ` |  39,627,559 |  39,627,559 |  39,627,559 |  39,627,559 |
| `total_cycles        ` |  811,303 |  811,303 |  811,303 |  811,303 |
| `execute_time_ms     ` |  219 |  219 |  219 |  219 |
| `trace_gen_time_ms   ` |  339 |  339 |  339 |  339 |
| `stark_prove_excluding_trace_time_ms` |  22,178 |  22,178 |  22,178 |  22,178 |
| `main_trace_commit_time_ms` |  6,856 |  6,856 |  6,856 |  6,856 |
| `generate_perm_trace_time_ms` |  51 |  51 |  51 |  51 |
| `perm_trace_commit_time_ms` |  4,322 |  4,322 |  4,322 |  4,322 |
| `quotient_poly_compute_time_ms` |  554 |  554 |  554 |  554 |
| `quotient_poly_commit_time_ms` |  7,971 |  7,971 |  7,971 |  7,971 |
| `pcs_opening_time_ms ` |  2,398 |  2,398 |  2,398 |  2,398 |

| halo2_outer |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  48,098 |  48,098 |  48,098 |  48,098 |
| `main_cells_used     ` |  61,697,112 |  61,697,112 |  61,697,112 |  61,697,112 |

| halo2_wrapper |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  35,167 |  35,167 |  35,167 |  35,167 |

| agg_keygen |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  11,703 |  23,406 |  22,969 |  437 |
| `main_cells_used     ` |  46,697,945 |  93,395,890 |  92,467,821 |  928,069 |
| `total_cycles        ` |  1,761,791 |  1,761,791 |  1,761,791 |  1,761,791 |
| `execute_time_ms     ` |  109.50 |  219 |  219 |  0 |
| `trace_gen_time_ms   ` |  187 |  374 |  347 |  27 |
| `stark_prove_excluding_trace_time_ms` |  11,406.50 |  22,813 |  22,403 |  410 |
| `main_trace_commit_time_ms` |  3,454 |  6,908 |  6,857 |  51 |
| `generate_perm_trace_time_ms` |  31 |  62 |  51 |  11 |
| `perm_trace_commit_time_ms` |  2,176 |  4,352 |  4,301 |  51 |
| `quotient_poly_compute_time_ms` |  293.50 |  587 |  559 |  28 |
| `quotient_poly_commit_time_ms` |  4,145 |  8,290 |  8,231 |  59 |
| `pcs_opening_time_ms ` |  1,298 |  2,596 |  2,392 |  204 |



<details>
<summary>Detailed Metrics</summary>

| air_name | block_number | quotient_deg | interactions | constraints |
| --- | --- | --- | --- | --- |
| AccessAdapterAir<16> | 21000000 | 2 | 5 | 12 | 
| AccessAdapterAir<2> | 21000000 | 2 | 5 | 12 | 
| AccessAdapterAir<32> | 21000000 | 2 | 5 | 12 | 
| AccessAdapterAir<4> | 21000000 | 2 | 5 | 12 | 
| AccessAdapterAir<64> | 21000000 | 2 | 5 | 12 | 
| AccessAdapterAir<8> | 21000000 | 2 | 5 | 12 | 
| BitwiseOperationLookupAir<8> | 21000000 | 2 | 2 | 4 | 
| KeccakVmAir | 21000000 | 2 | 321 | 4,511 | 
| MemoryMerkleAir<8> | 21000000 | 2 | 4 | 39 | 
| PersistentBoundaryAir<8> | 21000000 | 2 | 3 | 6 | 
| PhantomAir | 21000000 | 2 | 3 | 5 | 
| Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 2 | 1 | 286 | 
| ProgramAir | 21000000 | 1 | 1 | 4 | 
| RangeTupleCheckerAir<2> | 21000000 | 1 | 1 | 4 | 
| Rv32HintStoreAir | 21000000 | 2 | 18 | 28 | 
| Sha256VmAir | 21000000 | 2 | 50 | 664 | 
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
| VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 2 | 25 | 223 | 
| VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 2 | 16 | 20 | 
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 2 | 18 | 33 | 
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 2 | 17 | 40 | 
| VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 2 | 25 | 84 | 
| VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 2 | 24 | 31 | 
| VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 2 | 19 | 19 | 
| VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 2 | 12 | 14 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 2 | 411 | 476 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 4, 8, 32, 32>, FieldExpressionCoreAir> | 21000000 | 2 | 1,716 | 1,712 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 2 | 156 | 188 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 12, 12, 32, 32>, FieldExpressionCoreAir> | 21000000 | 2 | 4,370 | 4,350 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 2 | 422 | 451 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 4, 10, 32, 32>, FieldExpressionCoreAir> | 21000000 | 2 | 1,303 | 1,291 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 4, 12, 32, 32>, FieldExpressionCoreAir> | 21000000 | 2 | 2,903 | 2,896 | 
| VmAirWrapper<Rv32VecHeapTwoReadsAdapterAir<12, 10, 12, 32, 32>, FieldExpressionCoreAir> | 21000000 | 2 | 3,977 | 3,958 | 
| VmAirWrapper<Rv32VecHeapTwoReadsAdapterAir<4, 2, 4, 32, 32>, FieldExpressionCoreAir> | 21000000 | 2 | 565 | 557 | 
| VmConnectorAir | 21000000 | 2 | 3 | 9 | 

| block_number | execute_time_ms |
| --- | --- |
| 21000000 | 219 | 

| group | air_name | block_number | rows | quotient_deg | prep_cols | perm_cols | main_cols | interactions | constraints | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | AccessAdapterAir<16> | 21000000 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<2> | 21000000 | 524,288 | 8 |  | 16 | 11 | 5 | 12 | 14,155,776 | 
| agg_keygen | AccessAdapterAir<32> | 21000000 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<4> | 21000000 | 262,144 | 8 |  | 16 | 13 | 5 | 12 | 7,602,176 | 
| agg_keygen | AccessAdapterAir<64> | 21000000 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<8> | 21000000 | 8,192 | 8 |  | 16 | 17 | 5 | 12 | 270,336 | 
| agg_keygen | BitwiseOperationLookupAir<8> | 21000000 |  | 2 |  |  |  | 2 | 4 |  | 
| agg_keygen | FriReducedOpeningAir | 21000000 | 524,288 | 8 |  | 84 | 27 | 39 | 70 | 58,195,968 | 
| agg_keygen | MemoryMerkleAir<8> | 21000000 |  | 2 |  |  |  | 4 | 39 |  | 
| agg_keygen | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 131,072 | 8 |  | 312 | 399 | 136 | 571 | 93,192,192 | 
| agg_keygen | PersistentBoundaryAir<8> | 21000000 |  | 2 |  |  |  | 3 | 6 |  | 
| agg_keygen | PhantomAir | 21000000 | 32,768 | 4 |  | 12 | 6 | 3 | 5 | 589,824 | 
| agg_keygen | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 |  | 2 |  |  |  | 1 | 286 |  | 
| agg_keygen | ProgramAir | 21000000 | 131,072 | 1 |  | 8 | 10 | 1 | 4 | 2,359,296 | 
| agg_keygen | RangeTupleCheckerAir<2> | 21000000 |  | 1 |  |  |  | 1 | 4 |  | 
| agg_keygen | Rv32HintStoreAir | 21000000 |  | 2 |  |  |  | 18 | 28 |  | 
| agg_keygen | VariableRangeCheckerAir | 21000000 | 262,144 | 1 | 2 | 8 | 1 | 1 | 4 | 2,359,296 | 
| agg_keygen | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 1,048,576 | 8 |  | 36 | 29 | 15 | 27 | 68,157,440 | 
| agg_keygen | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 262,144 | 8 |  | 28 | 23 | 11 | 25 | 13,369,344 | 
| agg_keygen | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 32,768 | 8 |  | 20 | 9 | 7 | 9 | 950,272 | 
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
| agg_keygen | VmConnectorAir | 21000000 | 2 | 4 | 1 | 12 | 4 | 3 | 9 | 32 | 
| agg_keygen | VolatileBoundaryAir | 21000000 | 262,144 | 4 |  | 12 | 11 | 4 | 17 | 6,029,312 | 

| group | air_name | block_number | idx | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | AccessAdapterAir<2> | 21000000 | 0 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 1 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 2 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 3 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 4 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 5 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 0 | 524,288 |  | 12 | 13 | 13,107,200 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 1 | 524,288 |  | 12 | 13 | 13,107,200 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 2 | 524,288 |  | 12 | 13 | 13,107,200 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 3 | 524,288 |  | 12 | 13 | 13,107,200 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 4 | 524,288 |  | 12 | 13 | 13,107,200 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 5 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 0 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 1 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 2 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 3 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 4 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 5 | 4,096 |  | 12 | 17 | 118,784 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 0 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 1 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 2 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 3 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 4 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 5 | 524,288 |  | 44 | 27 | 37,224,448 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 0 | 262,144 |  | 160 | 399 | 146,538,496 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 1 | 262,144 |  | 160 | 399 | 146,538,496 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 2 | 262,144 |  | 160 | 399 | 146,538,496 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 3 | 262,144 |  | 160 | 399 | 146,538,496 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 4 | 262,144 |  | 160 | 399 | 146,538,496 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 5 | 131,072 |  | 160 | 399 | 73,269,248 | 
| internal.0 | PhantomAir | 21000000 | 0 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 1 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 2 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 3 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 4 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 5 | 32,768 |  | 8 | 6 | 458,752 | 
| internal.0 | ProgramAir | 21000000 | 0 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 21000000 | 1 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 21000000 | 2 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 21000000 | 3 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 21000000 | 4 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 21000000 | 5 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 0 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 1 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 2 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 3 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 4 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 5 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 0 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 1 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 2 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 3 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 4 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 5 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 0 | 65,536 |  | 12 | 9 | 1,376,256 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 1 | 65,536 |  | 12 | 9 | 1,376,256 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 2 | 65,536 |  | 12 | 9 | 1,376,256 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 3 | 65,536 |  | 12 | 9 | 1,376,256 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 4 | 65,536 |  | 12 | 9 | 1,376,256 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 5 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 0 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 1 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 2 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 3 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 4 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 5 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 0 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 1 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 2 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 3 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 4 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 5 | 262,144 |  | 24 | 21 | 11,796,480 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 0 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 1 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 2 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 3 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 4 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 5 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 0 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 1 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 2 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 3 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 4 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 5 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmConnectorAir | 21000000 | 0 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21000000 | 1 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21000000 | 2 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21000000 | 3 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21000000 | 4 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21000000 | 5 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 0 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 1 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 2 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 3 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 4 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 5 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.1 | AccessAdapterAir<2> | 21000000 | 6 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.1 | AccessAdapterAir<2> | 21000000 | 7 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.1 | AccessAdapterAir<2> | 21000000 | 8 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.1 | AccessAdapterAir<4> | 21000000 | 6 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.1 | AccessAdapterAir<4> | 21000000 | 7 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.1 | AccessAdapterAir<4> | 21000000 | 8 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.1 | AccessAdapterAir<8> | 21000000 | 6 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.1 | AccessAdapterAir<8> | 21000000 | 7 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.1 | AccessAdapterAir<8> | 21000000 | 8 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.1 | FriReducedOpeningAir | 21000000 | 6 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.1 | FriReducedOpeningAir | 21000000 | 7 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.1 | FriReducedOpeningAir | 21000000 | 8 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 6 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 7 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 8 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.1 | PhantomAir | 21000000 | 6 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.1 | PhantomAir | 21000000 | 7 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.1 | PhantomAir | 21000000 | 8 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.1 | ProgramAir | 21000000 | 6 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.1 | ProgramAir | 21000000 | 7 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.1 | ProgramAir | 21000000 | 8 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 21000000 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 21000000 | 7 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 21000000 | 8 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 6 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 7 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 8 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 6 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 7 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 8 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 6 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 7 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 8 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 6 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 7 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 8 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 6 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 7 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 8 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 6 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 7 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 8 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 6 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 7 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 8 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.1 | VmConnectorAir | 21000000 | 6 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VmConnectorAir | 21000000 | 7 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VmConnectorAir | 21000000 | 8 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VolatileBoundaryAir | 21000000 | 6 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.1 | VolatileBoundaryAir | 21000000 | 7 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.1 | VolatileBoundaryAir | 21000000 | 8 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.2 | AccessAdapterAir<2> | 21000000 | 10 | 262,144 |  | 12 | 11 | 6,029,312 | 
| internal.2 | AccessAdapterAir<2> | 21000000 | 9 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.2 | AccessAdapterAir<4> | 21000000 | 10 | 131,072 |  | 12 | 13 | 3,276,800 | 
| internal.2 | AccessAdapterAir<4> | 21000000 | 9 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.2 | AccessAdapterAir<8> | 21000000 | 10 | 4,096 |  | 12 | 17 | 118,784 | 
| internal.2 | AccessAdapterAir<8> | 21000000 | 9 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.2 | FriReducedOpeningAir | 21000000 | 10 | 131,072 |  | 44 | 27 | 9,306,112 | 
| internal.2 | FriReducedOpeningAir | 21000000 | 9 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.2 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 10 | 32,768 |  | 160 | 399 | 18,317,312 | 
| internal.2 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 9 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.2 | PhantomAir | 21000000 | 10 | 8,192 |  | 8 | 6 | 114,688 | 
| internal.2 | PhantomAir | 21000000 | 9 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.2 | ProgramAir | 21000000 | 10 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.2 | ProgramAir | 21000000 | 9 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.2 | VariableRangeCheckerAir | 21000000 | 10 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.2 | VariableRangeCheckerAir | 21000000 | 9 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.2 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 10 | 524,288 |  | 20 | 29 | 25,690,112 | 
| internal.2 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 9 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.2 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 10 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.2 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 9 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.2 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 10 | 16,384 |  | 12 | 9 | 344,064 | 
| internal.2 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 9 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 10 | 64 |  | 16 | 23 | 2,496 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 9 | 64 |  | 16 | 23 | 2,496 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 10 | 262,144 |  | 24 | 21 | 11,796,480 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 9 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 10 | 65,536 |  | 24 | 27 | 3,342,336 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 9 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 10 | 65,536 |  | 20 | 38 | 3,801,088 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 9 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.2 | VmConnectorAir | 21000000 | 10 | 2 | 1 | 8 | 4 | 24 | 
| internal.2 | VmConnectorAir | 21000000 | 9 | 2 | 1 | 8 | 4 | 24 | 
| internal.2 | VolatileBoundaryAir | 21000000 | 10 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.2 | VolatileBoundaryAir | 21000000 | 9 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.3 | AccessAdapterAir<2> | 21000000 | 11 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.3 | AccessAdapterAir<4> | 21000000 | 11 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.3 | AccessAdapterAir<8> | 21000000 | 11 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.3 | FriReducedOpeningAir | 21000000 | 11 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.3 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 11 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.3 | PhantomAir | 21000000 | 11 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.3 | ProgramAir | 21000000 | 11 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.3 | VariableRangeCheckerAir | 21000000 | 11 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.3 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 11 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.3 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 11 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.3 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 11 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.3 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 11 | 64 |  | 16 | 23 | 2,496 | 
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 11 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 11 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.3 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 11 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.3 | VmConnectorAir | 21000000 | 11 | 2 | 1 | 8 | 4 | 24 | 
| internal.3 | VolatileBoundaryAir | 21000000 | 11 | 262,144 |  | 8 | 11 | 4,980,736 | 
| leaf | AccessAdapterAir<2> | 21000000 | 0 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
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
| leaf | AccessAdapterAir<4> | 21000000 | 0 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
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
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 0 | 262,144 |  | 312 | 399 | 186,384,384 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 1 | 262,144 |  | 312 | 399 | 186,384,384 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 10 | 131,072 |  | 312 | 399 | 93,192,192 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 2 | 262,144 |  | 312 | 399 | 186,384,384 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 3 | 131,072 |  | 312 | 399 | 93,192,192 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 4 | 262,144 |  | 312 | 399 | 186,384,384 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 5 | 262,144 |  | 312 | 399 | 186,384,384 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 6 | 262,144 |  | 312 | 399 | 186,384,384 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 7 | 262,144 |  | 312 | 399 | 186,384,384 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 8 | 262,144 |  | 312 | 399 | 186,384,384 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 9 | 262,144 |  | 312 | 399 | 186,384,384 | 
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
| leaf | ProgramAir | 21000000 | 0 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21000000 | 1 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21000000 | 10 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21000000 | 2 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21000000 | 3 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21000000 | 4 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21000000 | 5 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21000000 | 6 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21000000 | 7 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21000000 | 8 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21000000 | 9 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
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
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 1 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 10 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 2 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 3 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 4 | 4,194,304 |  | 36 | 29 | 272,629,760 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 5 | 4,194,304 |  | 36 | 29 | 272,629,760 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 6 | 4,194,304 |  | 36 | 29 | 272,629,760 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 7 | 4,194,304 |  | 36 | 29 | 272,629,760 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 8 | 4,194,304 |  | 36 | 29 | 272,629,760 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 9 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 0 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 1 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 10 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 2 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 3 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 4 | 1,048,576 |  | 28 | 23 | 53,477,376 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 5 | 1,048,576 |  | 28 | 23 | 53,477,376 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 6 | 1,048,576 |  | 28 | 23 | 53,477,376 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 7 | 1,048,576 |  | 28 | 23 | 53,477,376 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 8 | 1,048,576 |  | 28 | 23 | 53,477,376 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 9 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 0 | 65,536 |  | 20 | 9 | 1,900,544 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 1 | 32,768 |  | 20 | 9 | 950,272 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 10 | 32,768 |  | 20 | 9 | 950,272 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 2 | 32,768 |  | 20 | 9 | 950,272 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 3 | 32,768 |  | 20 | 9 | 950,272 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 4 | 65,536 |  | 20 | 9 | 1,900,544 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 5 | 65,536 |  | 20 | 9 | 1,900,544 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 6 | 65,536 |  | 20 | 9 | 1,900,544 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 7 | 65,536 |  | 20 | 9 | 1,900,544 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 8 | 65,536 |  | 20 | 9 | 1,900,544 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 9 | 65,536 |  | 20 | 9 | 1,900,544 | 
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
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 10 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 2 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 3 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 4 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 5 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 6 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 7 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 8 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 9 | 262,144 |  | 40 | 27 | 17,563,648 | 
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
| leaf | VmConnectorAir | 21000000 | 0 | 2 | 1 | 12 | 4 | 32 | 
| leaf | VmConnectorAir | 21000000 | 1 | 2 | 1 | 12 | 4 | 32 | 
| leaf | VmConnectorAir | 21000000 | 10 | 2 | 1 | 12 | 4 | 32 | 
| leaf | VmConnectorAir | 21000000 | 2 | 2 | 1 | 12 | 4 | 32 | 
| leaf | VmConnectorAir | 21000000 | 3 | 2 | 1 | 12 | 4 | 32 | 
| leaf | VmConnectorAir | 21000000 | 4 | 2 | 1 | 12 | 4 | 32 | 
| leaf | VmConnectorAir | 21000000 | 5 | 2 | 1 | 12 | 4 | 32 | 
| leaf | VmConnectorAir | 21000000 | 6 | 2 | 1 | 12 | 4 | 32 | 
| leaf | VmConnectorAir | 21000000 | 7 | 2 | 1 | 12 | 4 | 32 | 
| leaf | VmConnectorAir | 21000000 | 8 | 2 | 1 | 12 | 4 | 32 | 
| leaf | VmConnectorAir | 21000000 | 9 | 2 | 1 | 12 | 4 | 32 | 
| leaf | VolatileBoundaryAir | 21000000 | 0 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 21000000 | 1 | 524,288 |  | 12 | 11 | 12,058,624 | 
| leaf | VolatileBoundaryAir | 21000000 | 10 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 21000000 | 2 | 524,288 |  | 12 | 11 | 12,058,624 | 
| leaf | VolatileBoundaryAir | 21000000 | 3 | 524,288 |  | 12 | 11 | 12,058,624 | 
| leaf | VolatileBoundaryAir | 21000000 | 4 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 21000000 | 5 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 21000000 | 6 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 21000000 | 7 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 21000000 | 8 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 21000000 | 9 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| root | AccessAdapterAir<2> | 21000000 | 0 | 262,144 |  | 8 | 11 | 4,980,736 | 
| root | AccessAdapterAir<4> | 21000000 | 0 | 131,072 |  | 8 | 13 | 2,752,512 | 
| root | AccessAdapterAir<8> | 21000000 | 0 | 4,096 |  | 8 | 17 | 102,400 | 
| root | FriReducedOpeningAir | 21000000 | 0 | 131,072 |  | 24 | 27 | 6,684,672 | 
| root | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 0 | 32,768 |  | 84 | 399 | 15,826,944 | 
| root | PhantomAir | 21000000 | 0 | 8,192 |  | 8 | 6 | 114,688 | 
| root | ProgramAir | 21000000 | 0 | 131,072 |  | 8 | 10 | 2,359,296 | 
| root | VariableRangeCheckerAir | 21000000 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| root | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 0 | 524,288 |  | 12 | 29 | 21,495,808 | 
| root | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 0 | 131,072 |  | 12 | 23 | 4,587,520 | 
| root | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 0 | 16,384 |  | 8 | 9 | 278,528 | 
| root | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 0 | 64 |  | 12 | 22 | 2,176 | 
| root | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 0 | 262,144 |  | 16 | 21 | 9,699,328 | 
| root | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 0 | 65,536 |  | 16 | 27 | 2,818,048 | 
| root | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 0 | 65,536 |  | 12 | 38 | 3,276,800 | 
| root | VmConnectorAir | 21000000 | 0 | 2 | 1 | 8 | 4 | 24 | 
| root | VolatileBoundaryAir | 21000000 | 0 | 131,072 |  | 8 | 11 | 2,490,368 | 

| group | air_name | block_number | segment | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | AccessAdapterAir<16> | 21000000 | 0 | 1 |  | 16 | 25 | 41 | 
| agg_keygen | AccessAdapterAir<2> | 21000000 | 0 | 1 |  | 16 | 11 | 27 | 
| agg_keygen | AccessAdapterAir<32> | 21000000 | 0 | 1 |  | 16 | 41 | 57 | 
| agg_keygen | AccessAdapterAir<4> | 21000000 | 0 | 1 |  | 16 | 13 | 29 | 
| agg_keygen | AccessAdapterAir<64> | 21000000 | 0 | 1 |  | 16 | 73 | 89 | 
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
| agg_keygen | VmConnectorAir | 21000000 | 0 | 2 | 1 | 12 | 4 | 32 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 0 | 64 |  | 16 | 25 | 2,624 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 10 | 16 |  | 16 | 25 | 656 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 4 | 1,048,576 |  | 16 | 25 | 42,991,616 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 5 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 6 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 7 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 8 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 9 | 8,192 |  | 16 | 25 | 335,872 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<2> | 21000000 | 4 | 1,024 |  | 16 | 11 | 27,648 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<2> | 21000000 | 5 | 1,024 |  | 16 | 11 | 27,648 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<2> | 21000000 | 6 | 4,096 |  | 16 | 11 | 110,592 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<2> | 21000000 | 9 | 262,144 |  | 16 | 11 | 7,077,888 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 0 | 32 |  | 16 | 41 | 1,824 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 10 | 8 |  | 16 | 41 | 456 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 4 | 524,288 |  | 16 | 41 | 29,884,416 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 5 | 131,072 |  | 16 | 41 | 7,471,104 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 6 | 131,072 |  | 16 | 41 | 7,471,104 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 7 | 131,072 |  | 16 | 41 | 7,471,104 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 8 | 131,072 |  | 16 | 41 | 7,471,104 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 9 | 4,096 |  | 16 | 41 | 233,472 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 0 | 64 |  | 16 | 13 | 1,856 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 4 | 512 |  | 16 | 13 | 14,848 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 5 | 512 |  | 16 | 13 | 14,848 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 6 | 4,096 |  | 16 | 13 | 118,784 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 8 | 256 |  | 16 | 13 | 7,424 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 9 | 131,072 |  | 16 | 13 | 3,801,088 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 0 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 1 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 10 | 262,144 |  | 16 | 17 | 8,650,752 | 
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
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 10 | 16,384 |  | 1,056 | 3,163 | 69,124,096 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 2 | 1 |  | 1,056 | 3,163 | 4,219 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 3 | 524,288 |  | 1,056 | 3,163 | 2,211,971,072 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 4 | 131,072 |  | 1,056 | 3,163 | 552,992,768 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 5 | 16,384 |  | 1,056 | 3,163 | 69,124,096 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 6 | 16,384 |  | 1,056 | 3,163 | 69,124,096 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 7 | 16,384 |  | 1,056 | 3,163 | 69,124,096 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 8 | 65,536 |  | 1,056 | 3,163 | 276,496,384 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 9 | 524,288 |  | 1,056 | 3,163 | 2,211,971,072 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 0 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 1 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 10 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 2 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 3 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 4 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 5 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 6 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 7 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 8 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 9 | 2,097,152 |  | 16 | 32 | 100,663,296 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 0 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 1 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 10 | 262,144 |  | 12 | 20 | 8,388,608 | 
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
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 10 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 2 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 3 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 4 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 5 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 6 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 7 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 8 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 9 | 2,097,152 |  | 8 | 300 | 645,922,816 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 0 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 1 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 10 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 2 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 3 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 4 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 5 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 6 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 7 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 8 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21000000 | ProgramAir | 21000000 | 9 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
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
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 10 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
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
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 10 | 32,768 |  | 40 | 37 | 2,523,136 | 
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
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 10 | 131,072 |  | 52 | 53 | 13,762,560 | 
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
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 10 | 262,144 |  | 28 | 26 | 14,155,776 | 
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
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 10 | 32,768 |  | 32 | 32 | 2,097,152 | 
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
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 10 | 32,768 |  | 28 | 18 | 1,507,328 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 2 | 1,048,576 |  | 28 | 18 | 48,234,496 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 3 | 262,144 |  | 28 | 18 | 12,058,624 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 4 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 5 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 6 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 7 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 8 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 9 | 262,144 |  | 28 | 18 | 12,058,624 | 
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
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 10 | 32,768 |  | 52 | 36 | 2,883,584 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 2 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 3 | 524,288 |  | 52 | 36 | 46,137,344 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 4 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 5 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 6 | 2,097,152 |  | 52 | 36 | 184,549,376 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 7 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 8 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 9 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 0 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 1 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 10 | 1,048,576 |  | 52 | 41 | 97,517,568 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 2 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 3 | 4,194,304 |  | 52 | 41 | 390,070,272 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 4 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 5 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 6 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 7 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 8 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 9 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 4 | 64 |  | 72 | 59 | 8,384 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 5 | 256 |  | 72 | 59 | 33,536 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 6 | 256 |  | 72 | 59 | 33,536 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 7 | 64 |  | 72 | 59 | 8,384 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 8 | 128 |  | 72 | 59 | 16,768 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 9 | 512 |  | 72 | 59 | 67,072 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 1 | 512 |  | 72 | 39 | 56,832 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 10 | 32 |  | 72 | 39 | 3,552 | 
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
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 10 | 8,192 |  | 52 | 31 | 679,936 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 2 | 4,096 |  | 52 | 31 | 339,968 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 3 | 16,384 |  | 52 | 31 | 1,359,872 | 
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
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 0 | 1 |  | 828 | 543 | 1,371 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 65,536 |  | 828 | 543 | 89,849,856 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 5 | 256 |  | 828 | 543 | 350,976 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 6 | 2,048 |  | 828 | 543 | 2,807,808 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 7 | 512 |  | 828 | 543 | 701,952 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 8 | 2,048 |  | 828 | 543 | 2,807,808 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 0 | 1 |  | 316 | 261 | 577 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 1,024 |  | 316 | 261 | 590,848 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 5 | 4 |  | 316 | 261 | 2,308 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 6 | 32 |  | 316 | 261 | 18,464 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 7 | 8 |  | 316 | 261 | 4,616 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 8 | 32 |  | 316 | 261 | 18,464 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 0 | 1 |  | 848 | 619 | 1,467 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 32,768 |  | 848 | 619 | 48,070,656 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 5 | 256 |  | 848 | 619 | 375,552 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 6 | 1,024 |  | 848 | 619 | 1,502,208 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 7 | 512 |  | 848 | 619 | 751,104 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 8 | 1,024 |  | 848 | 619 | 1,502,208 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 0 | 2 | 1 | 12 | 4 | 32 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 1 | 2 | 1 | 12 | 4 | 32 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 10 | 2 | 1 | 12 | 4 | 32 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 2 | 2 | 1 | 12 | 4 | 32 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 3 | 2 | 1 | 12 | 4 | 32 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 4 | 2 | 1 | 12 | 4 | 32 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 5 | 2 | 1 | 12 | 4 | 32 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 6 | 2 | 1 | 12 | 4 | 32 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 7 | 2 | 1 | 12 | 4 | 32 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 8 | 2 | 1 | 12 | 4 | 32 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 9 | 2 | 1 | 12 | 4 | 32 | 

| group | block_number | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | num_segments | main_trace_commit_time_ms | main_cells_used | halo2_total_cells | halo2_keygen_time_ms | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 347 | 22,969 | 1,761,791 | 317,697,504 | 22,403 | 559 | 8,231 | 4,301 | 2,392 | 1 | 6,857 | 92,467,821 | 7,911,006 | 18,562 | 51 | 219 | 
| halo2_outer | 21000000 |  | 48,098 |  |  |  |  |  |  |  |  |  | 61,697,112 |  |  |  |  | 
| halo2_wrapper | 21000000 |  | 35,167 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
| reth.prove_e2e.block_21000000 | 21000000 |  |  |  |  |  |  |  |  |  | 11 |  |  |  |  |  |  | 

| group | block_number | cell_tracker_span | simple_advice_cells | lookup_advice_cells | fixed_cells |
| --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | VerifierProgram | 467,386 | 151,016 | 152,991 | 
| agg_keygen | 21000000 | VerifierProgram;CheckTraceHeightConstraints | 4,789 | 972 | 1,738 | 
| agg_keygen | 21000000 | VerifierProgram;PoseidonCell | 19,600 |  | 5,800 | 
| agg_keygen | 21000000 | VerifierProgram;stage-c-build-rounds | 19,567 | 2,735 | 6,722 | 
| agg_keygen | 21000000 | VerifierProgram;stage-c-build-rounds;PoseidonCell | 46,550 |  | 13,775 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs | 1,337,374 | 207,297 | 471,434 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;PoseidonCell | 3,760,750 |  | 1,112,875 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify | 44,285 | 5,263 | 19,132 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;PoseidonCell | 68,600 |  | 20,300 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;cache-generator-powers | 66,304 | 11,396 | 20,384 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;compute-reduced-opening;single-reduced-opening-eval | 7,776,888 | 319,256 | 1,435,728 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;pre-compute-rounds-context | 76,416 | 11,144 | 22,288 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch | 49,728 |  | 6,216 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;PoseidonCell | 9,470,664 |  | 2,805,264 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;verify-batch-reduce-fast;PoseidonCell | 8,155,644 | 230,328 | 2,544,780 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query | 938,168 | 161,588 | 266,812 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext | 102,144 |  | 12,768 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;PoseidonCell | 15,647,184 |  | 4,634,784 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;verify-batch-reduce-fast;PoseidonCell | 1,544,732 | 54,096 | 474,908 | 
| agg_keygen | 21000000 | VerifierProgram;stage-e-verify-constraints | 9,112,436 | 1,832,811 | 2,808,517 | 

| group | block_number | idx | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | 21000000 | 0 | 1,119 | 7,973 | 2,497,321 | 435,595,736 | 5,882 | 1,016 | 1,092 | 1,012 | 1,387 | 1,157 | 147,280,266 | 212 | 972 | 
| internal.0 | 21000000 | 1 | 1,107 | 7,950 | 2,496,919 | 435,595,736 | 5,859 | 1,010 | 1,090 | 1,005 | 1,390 | 1,156 | 147,312,987 | 203 | 984 | 
| internal.0 | 21000000 | 2 | 1,117 | 7,991 | 2,497,560 | 435,595,736 | 5,893 | 1,013 | 1,131 | 986 | 1,386 | 1,162 | 147,168,183 | 209 | 981 | 
| internal.0 | 21000000 | 3 | 1,083 | 7,968 | 2,497,478 | 435,595,736 | 5,915 | 1,018 | 1,119 | 1,020 | 1,391 | 1,158 | 147,167,445 | 202 | 970 | 
| internal.0 | 21000000 | 4 | 1,087 | 7,991 | 2,497,306 | 435,595,736 | 5,917 | 1,015 | 1,116 | 1,021 | 1,393 | 1,155 | 147,362,225 | 207 | 987 | 
| internal.0 | 21000000 | 5 | 557 | 4,496 | 1,248,950 | 222,648,792 | 3,447 | 560 | 703 | 554 | 879 | 617 | 74,686,303 | 129 | 492 | 
| internal.1 | 21000000 | 6 | 612 | 4,269 | 1,635,785 | 181,709,272 | 3,203 | 489 | 742 | 487 | 850 | 535 | 78,779,464 | 93 | 454 | 
| internal.1 | 21000000 | 7 | 665 | 4,348 | 1,635,762 | 181,709,272 | 3,226 | 486 | 745 | 494 | 860 | 534 | 78,779,257 | 103 | 457 | 
| internal.1 | 21000000 | 8 | 652 | 4,442 | 1,628,484 | 184,199,640 | 3,343 | 501 | 799 | 490 | 909 | 546 | 78,128,877 | 91 | 447 | 
| internal.2 | 21000000 | 10 | 343 | 2,647 | 810,820 | 94,460,376 | 2,083 | 279 | 553 | 284 | 593 | 316 | 39,605,551 | 50 | 221 | 
| internal.2 | 21000000 | 9 | 626 | 4,281 | 1,620,899 | 181,709,272 | 3,214 | 486 | 708 | 483 | 903 | 532 | 77,463,643 | 95 | 441 | 
| internal.3 | 21000000 | 11 | 618 | 4,302 | 1,613,501 | 184,199,640 | 3,252 | 503 | 729 | 490 | 879 | 543 | 76,941,672 | 103 | 432 | 
| leaf | 21000000 | 0 | 2,117 | 10,166 | 3,572,778 | 1,108,512,224 | 6,958 | 1,032 | 778 | 2,031 | 1,689 | 1,023 | 268,383,792 | 400 | 1,091 | 
| leaf | 21000000 | 1 | 1,333 | 7,426 | 2,316,055 | 777,899,488 | 5,314 | 718 | 626 | 1,396 | 1,444 | 854 | 169,289,822 | 270 | 779 | 
| leaf | 21000000 | 10 | 1,384 | 7,267 | 2,389,610 | 705,547,744 | 5,124 | 711 | 632 | 1,300 | 1,456 | 772 | 172,045,930 | 248 | 759 | 
| leaf | 21000000 | 2 | 1,315 | 7,405 | 2,315,993 | 777,899,488 | 5,309 | 724 | 620 | 1,381 | 1,438 | 866 | 169,290,411 | 274 | 781 | 
| leaf | 21000000 | 3 | 1,358 | 7,121 | 2,301,161 | 684,707,296 | 5,004 | 689 | 606 | 1,282 | 1,427 | 753 | 167,644,411 | 243 | 759 | 
| leaf | 21000000 | 4 | 2,362 | 11,513 | 4,077,547 | 1,290,964,448 | 7,966 | 1,187 | 861 | 2,359 | 1,871 | 1,214 | 300,972,726 | 470 | 1,185 | 
| leaf | 21000000 | 5 | 2,299 | 11,431 | 4,076,454 | 1,290,964,448 | 7,937 | 1,188 | 858 | 2,308 | 1,906 | 1,207 | 301,058,182 | 466 | 1,195 | 
| leaf | 21000000 | 6 | 2,297 | 11,470 | 4,076,890 | 1,290,964,448 | 7,982 | 1,189 | 862 | 2,370 | 1,885 | 1,211 | 301,153,407 | 459 | 1,191 | 
| leaf | 21000000 | 7 | 2,303 | 11,411 | 3,993,895 | 1,290,964,448 | 7,925 | 1,197 | 853 | 2,315 | 1,876 | 1,211 | 297,908,772 | 469 | 1,183 | 
| leaf | 21000000 | 8 | 2,341 | 11,435 | 4,035,594 | 1,290,964,448 | 7,908 | 1,190 | 860 | 2,296 | 1,875 | 1,214 | 299,602,892 | 468 | 1,186 | 
| leaf | 21000000 | 9 | 1,564 | 7,861 | 2,653,032 | 799,690,208 | 5,457 | 758 | 639 | 1,425 | 1,483 | 873 | 188,539,126 | 273 | 840 | 
| root | 21000000 | 0 | 339 | 22,736 | 811,303 | 79,829,144 | 22,178 | 554 | 7,971 | 4,322 | 2,398 | 6,856 | 39,627,559 | 51 | 219 | 

| group | block_number | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 0 | 27 | 437 |  | 7,747,680 | 410 | 28 | 59 | 51 | 204 | 51 | 928,069 | 11 | 0 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 8,812 | 25,826 | 20,354,934 | 2,557,907,173 | 14,232 | 2,122 | 1,354 | 3,864 | 3,170 | 2,934 | 992,536,548 | 774 | 2,782 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 8,656 | 25,335 | 20,434,623 | 2,558,016,173 | 13,775 | 2,126 | 1,255 | 3,714 | 3,071 | 2,779 | 991,775,300 | 818 | 2,904 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 3,167 | 7,347 | 1,960,929 | 589,214,658 | 3,896 | 636 | 448 | 726 | 1,079 | 850 | 290,363,484 | 146 | 284 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 9,062 | 25,775 | 20,453,005 | 2,558,327,981 | 13,810 | 2,123 | 1,239 | 3,721 | 3,126 | 2,775 | 993,297,698 | 817 | 2,903 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 4,931 | 22,653 | 5,880,422 | 3,341,267,012 | 16,832 | 4,665 | 779 | 3,322 | 1,990 | 5,420 | 1,454,946,918 | 648 | 890 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 9,777 | 33,781 | 23,436,040 | 3,351,350,496 | 17,796 | 3,298 | 1,418 | 4,669 | 3,528 | 3,902 | 1,360,397,819 | 967 | 6,208 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 9,998 | 28,415 | 23,882,662 | 2,713,921,144 | 15,005 | 2,392 | 1,406 | 4,030 | 3,432 | 2,859 | 1,097,000,238 | 873 | 3,412 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 11,120 | 30,999 | 24,818,326 | 2,902,320,864 | 16,109 | 2,521 | 1,523 | 4,428 | 3,596 | 3,063 | 1,161,286,967 | 962 | 3,770 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 10,599 | 29,222 | 24,327,870 | 2,696,164,240 | 14,984 | 2,367 | 1,414 | 4,053 | 3,407 | 2,859 | 1,121,174,350 | 866 | 3,639 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 10,732 | 30,150 | 24,013,861 | 2,884,954,720 | 15,869 | 2,717 | 1,431 | 4,141 | 3,439 | 3,188 | 1,291,562,583 | 935 | 3,549 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 9,308 | 36,470 | 14,309,478 | 4,957,707,952 | 24,283 | 5,812 | 1,293 | 5,219 | 3,356 | 7,575 | 1,939,000,338 | 1,017 | 2,879 | 

</details>


Commit: https://github.com/axiom-crypto/openvm-reth-benchmark/commit/7ccfce562ccbca08a27df6a5c1ea5dab0e0a7df6

Max Segment Length: 8388508

Instance Type: m8g.24xlarge

Memory Allocator: jemalloc

[Benchmark Workflow](https://github.com/axiom-crypto/openvm-reth-benchmark/actions/runs/13741123464)
