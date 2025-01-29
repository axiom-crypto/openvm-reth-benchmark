| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total |  1,244.78 |  281.81 |
| reth.prove_e2e.block_21000000 |  748.85 |  93.54 |
| leaf |  232.47 |  27.71 |
| internal.0 |  80.79 |  14.58 |
| internal.1 |  41.38 |  13.95 |
| internal.2 |  23.02 |  13.77 |
| internal.3 |  13.94 |  13.94 |
| root |  31.65 |  31.65 |
| halo2_outer |  49.46 |  49.46 |
| halo2_wrapper |  23.21 |  23.21 |
| agg_keygen |  34.14 |  32.72 |


| reth.prove_e2e.block_21000000 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  68,077.55 |  748,853 |  93,537 |  30,044 |
| `main_cells_used     ` |  1,144,172,234.55 |  12,585,894,580 |  1,924,190,592 |  289,588,357 |
| `total_cycles        ` |  18,480,502.91 |  203,285,532 |  24,800,651 |  1,959,808 |
| `execute_time_ms     ` |  5,610.36 |  61,714 |  9,376 |  468 |
| `trace_gen_time_ms   ` |  20,999.64 |  230,996 |  26,622 |  10,209 |
| `stark_prove_excluding_trace_time_ms` |  41,467.55 |  456,143 |  61,982 |  12,697 |
| `main_trace_commit_time_ms` |  6,848.55 |  75,334 |  12,166 |  1,736 |
| `generate_perm_trace_time_ms` |  6,266.64 |  68,933 |  8,027 |  968 |
| `perm_trace_commit_time_ms` |  3,581.09 |  39,392 |  4,601 |  1,078 |
| `quotient_poly_compute_time_ms` |  6,178.82 |  67,967 |  13,523 |  1,519 |
| `quotient_poly_commit_time_ms` |  4,987.27 |  54,860 |  6,233 |  2,218 |
| `pcs_opening_time_ms ` |  13,590.36 |  149,494 |  19,692 |  5,171 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  21,133.82 |  232,472 |  27,709 |  16,503 |
| `main_cells_used     ` |  193,838,613.45 |  2,132,224,748 |  277,127,109 |  141,754,386 |
| `total_cycles        ` |  4,163,407.36 |  45,797,481 |  5,811,277 |  3,059,437 |
| `execute_time_ms     ` |  672.09 |  7,393 |  885 |  433 |
| `trace_gen_time_ms   ` |  3,831.55 |  42,147 |  5,869 |  2,748 |
| `stark_prove_excluding_trace_time_ms` |  16,630.18 |  182,932 |  20,960 |  13,264 |
| `main_trace_commit_time_ms` |  1,754.18 |  19,296 |  2,092 |  1,333 |
| `generate_perm_trace_time_ms` |  2,134.64 |  23,481 |  3,904 |  1,035 |
| `perm_trace_commit_time_ms` |  1,438.45 |  15,823 |  1,862 |  1,071 |
| `quotient_poly_compute_time_ms` |  1,987.18 |  21,859 |  2,600 |  1,528 |
| `quotient_poly_commit_time_ms` |  3,161.82 |  34,780 |  3,640 |  2,697 |
| `pcs_opening_time_ms ` |  6,149.27 |  67,642 |  7,072 |  5,410 |

| internal.0 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  13,465.17 |  80,791 |  14,580 |  8,873 |
| `main_cells_used     ` |  136,073,449 |  816,440,694 |  148,291,304 |  75,283,283 |
| `total_cycles        ` |  3,579,228.50 |  21,475,371 |  3,904,971 |  1,952,375 |
| `execute_time_ms     ` |  492.17 |  2,953 |  566 |  247 |
| `trace_gen_time_ms   ` |  2,675.33 |  16,052 |  2,933 |  1,518 |
| `stark_prove_excluding_trace_time_ms` |  10,297.67 |  61,786 |  11,123 |  7,108 |
| `main_trace_commit_time_ms` |  1,081.67 |  6,490 |  1,345 |  811 |
| `generate_perm_trace_time_ms` |  1,100.50 |  6,603 |  1,507 |  406 |
| `perm_trace_commit_time_ms` |  899.33 |  5,396 |  1,002 |  720 |
| `quotient_poly_compute_time_ms` |  1,179.83 |  7,079 |  1,280 |  756 |
| `quotient_poly_commit_time_ms` |  2,277.67 |  13,666 |  2,425 |  1,813 |
| `pcs_opening_time_ms ` |  3,754.50 |  22,527 |  4,069 |  2,597 |

| internal.1 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  13,794.67 |  41,384 |  13,952 |  13,684 |
| `main_cells_used     ` |  142,376,311.67 |  427,128,935 |  142,962,394 |  141,209,697 |
| `total_cycles        ` |  3,745,431.67 |  11,236,295 |  3,766,005 |  3,704,840 |
| `execute_time_ms     ` |  528 |  1,584 |  552 |  514 |
| `trace_gen_time_ms   ` |  2,806.33 |  8,419 |  2,867 |  2,742 |
| `stark_prove_excluding_trace_time_ms` |  10,460.33 |  31,381 |  10,624 |  10,329 |
| `main_trace_commit_time_ms` |  1,034.67 |  3,104 |  1,070 |  1,007 |
| `generate_perm_trace_time_ms` |  1,051.33 |  3,154 |  1,216 |  941 |
| `perm_trace_commit_time_ms` |  957.67 |  2,873 |  980 |  916 |
| `quotient_poly_compute_time_ms` |  1,255.67 |  3,767 |  1,265 |  1,246 |
| `quotient_poly_commit_time_ms` |  2,269 |  6,807 |  2,337 |  2,227 |
| `pcs_opening_time_ms ` |  3,887.67 |  11,663 |  3,921 |  3,853 |

| internal.2 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  11,509 |  23,018 |  13,768 |  9,250 |
| `main_cells_used     ` |  107,805,978.50 |  215,611,957 |  142,958,344 |  72,653,613 |
| `total_cycles        ` |  2,824,438.50 |  5,648,877 |  3,765,600 |  1,883,277 |
| `execute_time_ms     ` |  390.50 |  781 |  541 |  240 |
| `trace_gen_time_ms   ` |  2,143.50 |  4,287 |  2,863 |  1,424 |
| `stark_prove_excluding_trace_time_ms` |  8,975 |  17,950 |  10,364 |  7,586 |
| `main_trace_commit_time_ms` |  1,037 |  2,074 |  1,056 |  1,018 |
| `generate_perm_trace_time_ms` |  799.50 |  1,599 |  1,028 |  571 |
| `perm_trace_commit_time_ms` |  842.50 |  1,685 |  967 |  718 |
| `quotient_poly_compute_time_ms` |  1,013.50 |  2,027 |  1,254 |  773 |
| `quotient_poly_commit_time_ms` |  2,086 |  4,172 |  2,327 |  1,845 |
| `pcs_opening_time_ms ` |  3,191.50 |  6,383 |  3,727 |  2,656 |

| internal.3 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  13,940 |  13,940 |  13,940 |  13,940 |
| `main_cells_used     ` |  141,204,197 |  141,204,197 |  141,204,197 |  141,204,197 |
| `total_cycles        ` |  3,704,290 |  3,704,290 |  3,704,290 |  3,704,290 |
| `execute_time_ms     ` |  516 |  516 |  516 |  516 |
| `trace_gen_time_ms   ` |  2,826 |  2,826 |  2,826 |  2,826 |
| `stark_prove_excluding_trace_time_ms` |  10,598 |  10,598 |  10,598 |  10,598 |
| `main_trace_commit_time_ms` |  1,063 |  1,063 |  1,063 |  1,063 |
| `generate_perm_trace_time_ms` |  1,175 |  1,175 |  1,175 |  1,175 |
| `perm_trace_commit_time_ms` |  932 |  932 |  932 |  932 |
| `quotient_poly_compute_time_ms` |  1,259 |  1,259 |  1,259 |  1,259 |
| `quotient_poly_commit_time_ms` |  2,232 |  2,232 |  2,232 |  2,232 |
| `pcs_opening_time_ms ` |  3,933 |  3,933 |  3,933 |  3,933 |

| root |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  31,651 |  31,651 |  31,651 |  31,651 |
| `main_cells_used     ` |  72,676,675 |  72,676,675 |  72,676,675 |  72,676,675 |
| `total_cycles        ` |  1,883,941 |  1,883,941 |  1,883,941 |  1,883,941 |
| `execute_time_ms     ` |  225 |  225 |  225 |  225 |
| `trace_gen_time_ms   ` |  1,397 |  1,397 |  1,397 |  1,397 |
| `stark_prove_excluding_trace_time_ms` |  30,029 |  30,029 |  30,029 |  30,029 |
| `main_trace_commit_time_ms` |  7,042 |  7,042 |  7,042 |  7,042 |
| `generate_perm_trace_time_ms` |  561 |  561 |  561 |  561 |
| `perm_trace_commit_time_ms` |  4,723 |  4,723 |  4,723 |  4,723 |
| `quotient_poly_compute_time_ms` |  1,271 |  1,271 |  1,271 |  1,271 |
| `quotient_poly_commit_time_ms` |  10,884 |  10,884 |  10,884 |  10,884 |
| `pcs_opening_time_ms ` |  5,543 |  5,543 |  5,543 |  5,543 |

| halo2_outer |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  49,456 |  49,456 |  49,456 |  49,456 |
| `main_cells_used     ` |  62,033,753 |  62,033,753 |  62,033,753 |  62,033,753 |

| halo2_wrapper |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  23,215 |  23,215 |  23,215 |  23,215 |

| agg_keygen |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  17,071 |  34,142 |  32,718 |  1,424 |
| `main_cells_used     ` |  87,431,251.50 |  174,862,503 |  173,934,434 |  928,069 |
| `total_cycles        ` |  4,501,474 |  4,501,474 |  4,501,474 |  4,501,474 |
| `execute_time_ms     ` |  111.50 |  223 |  222 |  1 |
| `trace_gen_time_ms   ` |  677 |  1,354 |  1,336 |  18 |
| `stark_prove_excluding_trace_time_ms` |  16,282.50 |  32,565 |  31,160 |  1,405 |
| `main_trace_commit_time_ms` |  3,477 |  6,954 |  6,810 |  144 |
| `generate_perm_trace_time_ms` |  272.50 |  545 |  411 |  134 |
| `perm_trace_commit_time_ms` |  2,609 |  5,218 |  5,084 |  134 |
| `quotient_poly_compute_time_ms` |  825 |  1,650 |  1,565 |  85 |
| `quotient_poly_commit_time_ms` |  6,001.50 |  12,003 |  11,805 |  198 |
| `pcs_opening_time_ms ` |  3,094 |  6,188 |  5,481 |  707 |



<details>
<summary>Detailed Metrics</summary>

| air_name | block_number | quotient_deg | interactions | constraints |
| --- | --- | --- | --- | --- |
| AccessAdapterAir<16> | 21000000 | 4 | 5 | 11 | 
| AccessAdapterAir<2> | 21000000 | 4 | 5 | 11 | 
| AccessAdapterAir<32> | 21000000 | 4 | 5 | 11 | 
| AccessAdapterAir<4> | 21000000 | 4 | 5 | 11 | 
| AccessAdapterAir<64> | 21000000 | 4 | 5 | 11 | 
| AccessAdapterAir<8> | 21000000 | 4 | 5 | 11 | 
| BitwiseOperationLookupAir<8> | 21000000 | 2 | 2 | 4 | 
| KeccakVmAir | 21000000 | 4 | 321 | 4,382 | 
| MemoryMerkleAir<8> | 21000000 | 4 | 4 | 38 | 
| PersistentBoundaryAir<8> | 21000000 | 4 | 3 | 5 | 
| PhantomAir | 21000000 | 4 | 3 | 4 | 
| Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 2 | 1 | 286 | 
| ProgramAir | 21000000 | 1 | 1 | 4 | 
| RangeTupleCheckerAir<2> | 21000000 | 1 | 1 | 4 | 
| Rv32HintStoreAir | 21000000 | 4 | 19 | 21 | 
| VariableRangeCheckerAir | 21000000 | 1 | 1 | 4 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 4 | 19 | 30 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 4 | 17 | 35 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 4 | 23 | 84 | 
| VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 4 | 11 | 17 | 
| VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 4 | 13 | 32 | 
| VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 4 | 10 | 15 | 
| VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 4 | 61 | 103 | 
| VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 4 | 31 | 121 | 
| VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 4 | 61 | 34 | 
| VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 4 | 79 | 2,141 | 
| VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 4 | 20 | 50 | 
| VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchLessThanCoreAir<32, 8> | 21000000 | 4 | 22 | 120 | 
| VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 4 | 25 | 217 | 
| VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 4 | 16 | 16 | 
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 4 | 18 | 21 | 
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 4 | 17 | 27 | 
| VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 4 | 25 | 72 | 
| VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 4 | 24 | 23 | 
| VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 4 | 19 | 13 | 
| VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 4 | 11 | 12 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 411 | 378 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 4, 8, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 1,716 | 1,310 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 156 | 150 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 12, 12, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 4,370 | 3,323 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 422 | 351 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 4, 10, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 1,303 | 988 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 4, 12, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 2,903 | 2,221 | 
| VmAirWrapper<Rv32VecHeapTwoReadsAdapterAir<12, 10, 12, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 3,977 | 3,023 | 
| VmAirWrapper<Rv32VecHeapTwoReadsAdapterAir<4, 2, 4, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 565 | 423 | 
| VmConnectorAir | 21000000 | 4 | 3 | 8 | 

| block_number | execute_time_ms |
| --- | --- |
| 21000000 | 236 | 

| group | air_name | block_number | rows | quotient_deg | prep_cols | perm_cols | main_cols | interactions | constraints | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | AccessAdapterAir<16> | 21000000 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<2> | 21000000 | 524,288 | 8 |  | 12 | 11 | 5 | 12 | 12,058,624 | 
| agg_keygen | AccessAdapterAir<32> | 21000000 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<4> | 21000000 | 262,144 | 8 |  | 12 | 13 | 5 | 12 | 6,553,600 | 
| agg_keygen | AccessAdapterAir<64> | 21000000 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<8> | 21000000 | 512 | 8 |  | 12 | 17 | 5 | 12 | 14,848 | 
| agg_keygen | BitwiseOperationLookupAir<8> | 21000000 |  | 2 |  |  |  | 2 | 4 |  | 
| agg_keygen | FriReducedOpeningAir | 21000000 | 524,288 | 8 |  | 36 | 26 | 31 | 53 | 32,505,856 | 
| agg_keygen | MemoryMerkleAir<8> | 21000000 |  | 2 |  |  |  | 4 | 39 |  | 
| agg_keygen | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 65,536 | 8 |  | 216 | 399 | 176 | 555 | 40,304,640 | 
| agg_keygen | PersistentBoundaryAir<8> | 21000000 |  | 2 |  |  |  | 3 | 6 |  | 
| agg_keygen | PhantomAir | 21000000 | 65,536 | 4 |  | 8 | 6 | 3 | 5 | 917,504 | 
| agg_keygen | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 |  | 2 |  |  |  | 1 | 286 |  | 
| agg_keygen | ProgramAir | 21000000 | 262,144 | 1 |  | 8 | 10 | 1 | 4 | 4,718,592 | 
| agg_keygen | RangeTupleCheckerAir<2> | 21000000 |  | 1 |  |  |  | 1 | 4 |  | 
| agg_keygen | Rv32HintStoreAir | 21000000 |  | 2 |  |  |  | 19 | 26 |  | 
| agg_keygen | VariableRangeCheckerAir | 21000000 | 262,144 | 1 | 2 | 8 | 1 | 1 | 4 | 2,359,296 | 
| agg_keygen | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 1,048,576 | 8 |  | 16 | 23 | 11 | 20 | 40,894,464 | 
| agg_keygen | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 131,072 | 8 |  | 12 | 10 | 7 | 6 | 2,883,584 | 
| agg_keygen | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 64 | 8 |  | 16 | 23 | 11 | 23 | 2,496 | 
| agg_keygen | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 4,194,304 | 8 |  | 20 | 30 | 15 | 23 | 209,715,200 | 
| agg_keygen | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 2,097,152 | 8 |  | 24 | 25 | 15 | 17 | 102,760,448 | 
| agg_keygen | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 131,072 | 8 |  | 24 | 34 | 15 | 17 | 7,602,176 | 
| agg_keygen | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 131,072 | 8 |  | 20 | 40 | 15 | 23 | 7,864,320 | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 19 | 36 |  | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 17 | 39 |  | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 23 | 90 |  | 
| agg_keygen | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 |  | 2 |  |  |  | 11 | 20 |  | 
| agg_keygen | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 13 | 35 |  | 
| agg_keygen | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 |  | 2 |  |  |  | 10 | 18 |  | 
| agg_keygen | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 |  | 2 |  |  |  | 16 | 20 |  | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 18 | 26 |  | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 |  | 2 |  |  |  | 17 | 33 |  | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 25 | 80 |  | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 24 | 31 |  | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 19 | 19 |  | 
| agg_keygen | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 |  | 2 |  |  |  | 11 | 15 |  | 
| agg_keygen | VmConnectorAir | 21000000 | 2 | 4 | 1 | 8 | 4 | 3 | 9 | 24 | 
| agg_keygen | VolatileBoundaryAir | 21000000 | 1,048,576 | 4 |  | 8 | 11 | 4 | 16 | 19,922,944 | 

| group | air_name | block_number | idx | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | AccessAdapterAir<2> | 21000000 | 0 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 1 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 2 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 3 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 4 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 5 | 262,144 |  | 12 | 11 | 6,029,312 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 0 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 1 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 2 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 3 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 4 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 5 | 131,072 |  | 12 | 13 | 3,276,800 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 0 | 512 |  | 12 | 17 | 14,848 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 1 | 512 |  | 12 | 17 | 14,848 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 2 | 512 |  | 12 | 17 | 14,848 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 3 | 512 |  | 12 | 17 | 14,848 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 4 | 512 |  | 12 | 17 | 14,848 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 5 | 256 |  | 12 | 17 | 7,424 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 0 | 262,144 |  | 36 | 26 | 16,252,928 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 1 | 262,144 |  | 36 | 26 | 16,252,928 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 2 | 262,144 |  | 36 | 26 | 16,252,928 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 3 | 262,144 |  | 36 | 26 | 16,252,928 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 4 | 262,144 |  | 36 | 26 | 16,252,928 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 5 | 131,072 |  | 36 | 26 | 8,126,464 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 0 | 65,536 |  | 216 | 399 | 40,304,640 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 1 | 65,536 |  | 216 | 399 | 40,304,640 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 2 | 65,536 |  | 216 | 399 | 40,304,640 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 3 | 65,536 |  | 216 | 399 | 40,304,640 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 4 | 65,536 |  | 216 | 399 | 40,304,640 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 5 | 32,768 |  | 216 | 399 | 20,152,320 | 
| internal.0 | PhantomAir | 21000000 | 0 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 1 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 2 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 3 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 4 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 5 | 32,768 |  | 8 | 6 | 458,752 | 
| internal.0 | ProgramAir | 21000000 | 0 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | ProgramAir | 21000000 | 1 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | ProgramAir | 21000000 | 2 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | ProgramAir | 21000000 | 3 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | ProgramAir | 21000000 | 4 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | ProgramAir | 21000000 | 5 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21000000 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 0 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 1 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 2 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 3 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 4 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 5 | 524,288 |  | 16 | 23 | 20,447,232 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 0 | 131,072 |  | 12 | 10 | 2,883,584 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 1 | 131,072 |  | 12 | 10 | 2,883,584 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 2 | 131,072 |  | 12 | 10 | 2,883,584 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 3 | 131,072 |  | 12 | 10 | 2,883,584 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 4 | 131,072 |  | 12 | 10 | 2,883,584 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 5 | 65,536 |  | 12 | 10 | 1,441,792 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 0 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 1 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 2 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 3 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 4 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 5 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 0 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 1 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 2 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 3 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 4 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 5 | 1,048,576 |  | 20 | 30 | 52,428,800 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 0 | 1,048,576 |  | 24 | 25 | 51,380,224 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 1 | 1,048,576 |  | 24 | 25 | 51,380,224 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 2 | 1,048,576 |  | 24 | 25 | 51,380,224 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 3 | 1,048,576 |  | 24 | 25 | 51,380,224 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 4 | 1,048,576 |  | 24 | 25 | 51,380,224 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 5 | 524,288 |  | 24 | 25 | 25,690,112 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 0 | 131,072 |  | 24 | 34 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 1 | 131,072 |  | 24 | 34 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 2 | 131,072 |  | 24 | 34 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 3 | 131,072 |  | 24 | 34 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 4 | 131,072 |  | 24 | 34 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 5 | 65,536 |  | 24 | 34 | 3,801,088 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 0 | 131,072 |  | 20 | 40 | 7,864,320 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 1 | 131,072 |  | 20 | 40 | 7,864,320 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 2 | 131,072 |  | 20 | 40 | 7,864,320 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 3 | 131,072 |  | 20 | 40 | 7,864,320 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 4 | 131,072 |  | 20 | 40 | 7,864,320 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 5 | 65,536 |  | 20 | 40 | 3,932,160 | 
| internal.0 | VmConnectorAir | 21000000 | 0 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21000000 | 1 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21000000 | 2 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21000000 | 3 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21000000 | 4 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21000000 | 5 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 0 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 1 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 2 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 3 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 4 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 5 | 524,288 |  | 8 | 11 | 9,961,472 | 
| internal.1 | AccessAdapterAir<2> | 21000000 | 6 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.1 | AccessAdapterAir<2> | 21000000 | 7 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.1 | AccessAdapterAir<2> | 21000000 | 8 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.1 | AccessAdapterAir<4> | 21000000 | 6 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.1 | AccessAdapterAir<4> | 21000000 | 7 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.1 | AccessAdapterAir<4> | 21000000 | 8 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.1 | AccessAdapterAir<8> | 21000000 | 6 | 512 |  | 12 | 17 | 14,848 | 
| internal.1 | AccessAdapterAir<8> | 21000000 | 7 | 512 |  | 12 | 17 | 14,848 | 
| internal.1 | AccessAdapterAir<8> | 21000000 | 8 | 512 |  | 12 | 17 | 14,848 | 
| internal.1 | FriReducedOpeningAir | 21000000 | 6 | 262,144 |  | 36 | 26 | 16,252,928 | 
| internal.1 | FriReducedOpeningAir | 21000000 | 7 | 262,144 |  | 36 | 26 | 16,252,928 | 
| internal.1 | FriReducedOpeningAir | 21000000 | 8 | 262,144 |  | 36 | 26 | 16,252,928 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 6 | 65,536 |  | 216 | 399 | 40,304,640 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 7 | 65,536 |  | 216 | 399 | 40,304,640 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 8 | 65,536 |  | 216 | 399 | 40,304,640 | 
| internal.1 | PhantomAir | 21000000 | 6 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.1 | PhantomAir | 21000000 | 7 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.1 | PhantomAir | 21000000 | 8 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.1 | ProgramAir | 21000000 | 6 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.1 | ProgramAir | 21000000 | 7 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.1 | ProgramAir | 21000000 | 8 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.1 | VariableRangeCheckerAir | 21000000 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 21000000 | 7 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 21000000 | 8 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 6 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 7 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 8 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 6 | 131,072 |  | 12 | 10 | 2,883,584 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 7 | 131,072 |  | 12 | 10 | 2,883,584 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 8 | 131,072 |  | 12 | 10 | 2,883,584 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 6 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 7 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 8 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 6 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 7 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 8 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 6 | 1,048,576 |  | 24 | 25 | 51,380,224 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 7 | 1,048,576 |  | 24 | 25 | 51,380,224 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 8 | 1,048,576 |  | 24 | 25 | 51,380,224 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 6 | 131,072 |  | 24 | 34 | 7,602,176 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 7 | 131,072 |  | 24 | 34 | 7,602,176 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 8 | 131,072 |  | 24 | 34 | 7,602,176 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 6 | 131,072 |  | 20 | 40 | 7,864,320 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 7 | 131,072 |  | 20 | 40 | 7,864,320 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 8 | 131,072 |  | 20 | 40 | 7,864,320 | 
| internal.1 | VmConnectorAir | 21000000 | 6 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VmConnectorAir | 21000000 | 7 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VmConnectorAir | 21000000 | 8 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VolatileBoundaryAir | 21000000 | 6 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| internal.1 | VolatileBoundaryAir | 21000000 | 7 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| internal.1 | VolatileBoundaryAir | 21000000 | 8 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| internal.2 | AccessAdapterAir<2> | 21000000 | 10 | 262,144 |  | 12 | 11 | 6,029,312 | 
| internal.2 | AccessAdapterAir<2> | 21000000 | 9 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.2 | AccessAdapterAir<4> | 21000000 | 10 | 131,072 |  | 12 | 13 | 3,276,800 | 
| internal.2 | AccessAdapterAir<4> | 21000000 | 9 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.2 | AccessAdapterAir<8> | 21000000 | 10 | 256 |  | 12 | 17 | 7,424 | 
| internal.2 | AccessAdapterAir<8> | 21000000 | 9 | 512 |  | 12 | 17 | 14,848 | 
| internal.2 | FriReducedOpeningAir | 21000000 | 10 | 131,072 |  | 36 | 26 | 8,126,464 | 
| internal.2 | FriReducedOpeningAir | 21000000 | 9 | 262,144 |  | 36 | 26 | 16,252,928 | 
| internal.2 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 10 | 32,768 |  | 216 | 399 | 20,152,320 | 
| internal.2 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 9 | 65,536 |  | 216 | 399 | 40,304,640 | 
| internal.2 | PhantomAir | 21000000 | 10 | 32,768 |  | 8 | 6 | 458,752 | 
| internal.2 | PhantomAir | 21000000 | 9 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.2 | ProgramAir | 21000000 | 10 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.2 | ProgramAir | 21000000 | 9 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.2 | VariableRangeCheckerAir | 21000000 | 10 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.2 | VariableRangeCheckerAir | 21000000 | 9 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.2 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 10 | 524,288 |  | 16 | 23 | 20,447,232 | 
| internal.2 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 9 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.2 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 10 | 65,536 |  | 12 | 10 | 1,441,792 | 
| internal.2 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 9 | 131,072 |  | 12 | 10 | 2,883,584 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 10 | 64 |  | 16 | 23 | 2,496 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 9 | 64 |  | 16 | 23 | 2,496 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 10 | 1,048,576 |  | 20 | 30 | 52,428,800 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 9 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 10 | 524,288 |  | 24 | 25 | 25,690,112 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 9 | 1,048,576 |  | 24 | 25 | 51,380,224 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 10 | 65,536 |  | 24 | 34 | 3,801,088 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 9 | 131,072 |  | 24 | 34 | 7,602,176 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 10 | 65,536 |  | 20 | 40 | 3,932,160 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 9 | 131,072 |  | 20 | 40 | 7,864,320 | 
| internal.2 | VmConnectorAir | 21000000 | 10 | 2 | 1 | 8 | 4 | 24 | 
| internal.2 | VmConnectorAir | 21000000 | 9 | 2 | 1 | 8 | 4 | 24 | 
| internal.2 | VolatileBoundaryAir | 21000000 | 10 | 524,288 |  | 8 | 11 | 9,961,472 | 
| internal.2 | VolatileBoundaryAir | 21000000 | 9 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| internal.3 | AccessAdapterAir<2> | 21000000 | 11 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.3 | AccessAdapterAir<4> | 21000000 | 11 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.3 | AccessAdapterAir<8> | 21000000 | 11 | 512 |  | 12 | 17 | 14,848 | 
| internal.3 | FriReducedOpeningAir | 21000000 | 11 | 262,144 |  | 36 | 26 | 16,252,928 | 
| internal.3 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 11 | 65,536 |  | 216 | 399 | 40,304,640 | 
| internal.3 | PhantomAir | 21000000 | 11 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.3 | ProgramAir | 21000000 | 11 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.3 | VariableRangeCheckerAir | 21000000 | 11 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.3 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 11 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.3 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 11 | 131,072 |  | 12 | 10 | 2,883,584 | 
| internal.3 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 11 | 64 |  | 16 | 23 | 2,496 | 
| internal.3 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 11 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 11 | 1,048,576 |  | 24 | 25 | 51,380,224 | 
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 11 | 131,072 |  | 24 | 34 | 7,602,176 | 
| internal.3 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 11 | 131,072 |  | 20 | 40 | 7,864,320 | 
| internal.3 | VmConnectorAir | 21000000 | 11 | 2 | 1 | 8 | 4 | 24 | 
| internal.3 | VolatileBoundaryAir | 21000000 | 11 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | AccessAdapterAir<2> | 21000000 | 0 | 2,097,152 |  | 12 | 11 | 48,234,496 | 
| leaf | AccessAdapterAir<2> | 21000000 | 1 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21000000 | 10 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21000000 | 2 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21000000 | 3 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21000000 | 4 | 2,097,152 |  | 12 | 11 | 48,234,496 | 
| leaf | AccessAdapterAir<2> | 21000000 | 5 | 2,097,152 |  | 12 | 11 | 48,234,496 | 
| leaf | AccessAdapterAir<2> | 21000000 | 6 | 2,097,152 |  | 12 | 11 | 48,234,496 | 
| leaf | AccessAdapterAir<2> | 21000000 | 7 | 2,097,152 |  | 12 | 11 | 48,234,496 | 
| leaf | AccessAdapterAir<2> | 21000000 | 8 | 2,097,152 |  | 12 | 11 | 48,234,496 | 
| leaf | AccessAdapterAir<2> | 21000000 | 9 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<4> | 21000000 | 0 | 1,048,576 |  | 12 | 13 | 26,214,400 | 
| leaf | AccessAdapterAir<4> | 21000000 | 1 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21000000 | 10 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21000000 | 2 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21000000 | 3 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21000000 | 4 | 1,048,576 |  | 12 | 13 | 26,214,400 | 
| leaf | AccessAdapterAir<4> | 21000000 | 5 | 1,048,576 |  | 12 | 13 | 26,214,400 | 
| leaf | AccessAdapterAir<4> | 21000000 | 6 | 1,048,576 |  | 12 | 13 | 26,214,400 | 
| leaf | AccessAdapterAir<4> | 21000000 | 7 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21000000 | 8 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21000000 | 9 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<8> | 21000000 | 0 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21000000 | 1 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21000000 | 10 | 512 |  | 12 | 17 | 14,848 | 
| leaf | AccessAdapterAir<8> | 21000000 | 2 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21000000 | 3 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21000000 | 4 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21000000 | 5 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21000000 | 6 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21000000 | 7 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21000000 | 8 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21000000 | 9 | 256 |  | 12 | 17 | 7,424 | 
| leaf | FriReducedOpeningAir | 21000000 | 0 | 2,097,152 |  | 36 | 26 | 130,023,424 | 
| leaf | FriReducedOpeningAir | 21000000 | 1 | 524,288 |  | 36 | 26 | 32,505,856 | 
| leaf | FriReducedOpeningAir | 21000000 | 10 | 524,288 |  | 36 | 26 | 32,505,856 | 
| leaf | FriReducedOpeningAir | 21000000 | 2 | 524,288 |  | 36 | 26 | 32,505,856 | 
| leaf | FriReducedOpeningAir | 21000000 | 3 | 524,288 |  | 36 | 26 | 32,505,856 | 
| leaf | FriReducedOpeningAir | 21000000 | 4 | 1,048,576 |  | 36 | 26 | 65,011,712 | 
| leaf | FriReducedOpeningAir | 21000000 | 5 | 1,048,576 |  | 36 | 26 | 65,011,712 | 
| leaf | FriReducedOpeningAir | 21000000 | 6 | 1,048,576 |  | 36 | 26 | 65,011,712 | 
| leaf | FriReducedOpeningAir | 21000000 | 7 | 1,048,576 |  | 36 | 26 | 65,011,712 | 
| leaf | FriReducedOpeningAir | 21000000 | 8 | 1,048,576 |  | 36 | 26 | 65,011,712 | 
| leaf | FriReducedOpeningAir | 21000000 | 9 | 524,288 |  | 36 | 26 | 32,505,856 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 0 | 131,072 |  | 216 | 399 | 80,609,280 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 1 | 65,536 |  | 216 | 399 | 40,304,640 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 10 | 65,536 |  | 216 | 399 | 40,304,640 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 2 | 65,536 |  | 216 | 399 | 40,304,640 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 3 | 65,536 |  | 216 | 399 | 40,304,640 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 4 | 131,072 |  | 216 | 399 | 80,609,280 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 5 | 131,072 |  | 216 | 399 | 80,609,280 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 6 | 131,072 |  | 216 | 399 | 80,609,280 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 7 | 131,072 |  | 216 | 399 | 80,609,280 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 8 | 131,072 |  | 216 | 399 | 80,609,280 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 9 | 65,536 |  | 216 | 399 | 40,304,640 | 
| leaf | PhantomAir | 21000000 | 0 | 65,536 |  | 8 | 6 | 917,504 | 
| leaf | PhantomAir | 21000000 | 1 | 32,768 |  | 8 | 6 | 458,752 | 
| leaf | PhantomAir | 21000000 | 10 | 32,768 |  | 8 | 6 | 458,752 | 
| leaf | PhantomAir | 21000000 | 2 | 32,768 |  | 8 | 6 | 458,752 | 
| leaf | PhantomAir | 21000000 | 3 | 32,768 |  | 8 | 6 | 458,752 | 
| leaf | PhantomAir | 21000000 | 4 | 65,536 |  | 8 | 6 | 917,504 | 
| leaf | PhantomAir | 21000000 | 5 | 65,536 |  | 8 | 6 | 917,504 | 
| leaf | PhantomAir | 21000000 | 6 | 65,536 |  | 8 | 6 | 917,504 | 
| leaf | PhantomAir | 21000000 | 7 | 65,536 |  | 8 | 6 | 917,504 | 
| leaf | PhantomAir | 21000000 | 8 | 65,536 |  | 8 | 6 | 917,504 | 
| leaf | PhantomAir | 21000000 | 9 | 32,768 |  | 8 | 6 | 458,752 | 
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
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 0 | 2,097,152 |  | 16 | 23 | 81,788,928 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 1 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 10 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 2 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 3 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 4 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 5 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 6 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 7 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 8 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 9 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 0 | 65,536 |  | 12 | 10 | 1,441,792 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 1 | 65,536 |  | 12 | 10 | 1,441,792 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 10 | 65,536 |  | 12 | 10 | 1,441,792 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 2 | 65,536 |  | 12 | 10 | 1,441,792 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 3 | 65,536 |  | 12 | 10 | 1,441,792 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 4 | 65,536 |  | 12 | 10 | 1,441,792 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 5 | 65,536 |  | 12 | 10 | 1,441,792 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 6 | 65,536 |  | 12 | 10 | 1,441,792 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 7 | 65,536 |  | 12 | 10 | 1,441,792 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 8 | 65,536 |  | 12 | 10 | 1,441,792 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 9 | 65,536 |  | 12 | 10 | 1,441,792 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 0 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 1 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 10 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 2 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 3 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 4 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 5 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 6 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 7 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 8 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 9 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 0 | 4,194,304 |  | 20 | 30 | 209,715,200 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 1 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 10 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 2 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 3 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 4 | 4,194,304 |  | 20 | 30 | 209,715,200 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 5 | 4,194,304 |  | 20 | 30 | 209,715,200 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 6 | 4,194,304 |  | 20 | 30 | 209,715,200 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 7 | 4,194,304 |  | 20 | 30 | 209,715,200 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 8 | 4,194,304 |  | 20 | 30 | 209,715,200 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 9 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 0 | 2,097,152 |  | 24 | 25 | 102,760,448 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 1 | 1,048,576 |  | 24 | 25 | 51,380,224 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 10 | 1,048,576 |  | 24 | 25 | 51,380,224 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 2 | 1,048,576 |  | 24 | 25 | 51,380,224 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 3 | 1,048,576 |  | 24 | 25 | 51,380,224 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 4 | 2,097,152 |  | 24 | 25 | 102,760,448 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 5 | 2,097,152 |  | 24 | 25 | 102,760,448 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 6 | 2,097,152 |  | 24 | 25 | 102,760,448 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 7 | 2,097,152 |  | 24 | 25 | 102,760,448 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 8 | 2,097,152 |  | 24 | 25 | 102,760,448 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 9 | 1,048,576 |  | 24 | 25 | 51,380,224 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 0 | 131,072 |  | 24 | 34 | 7,602,176 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 1 | 65,536 |  | 24 | 34 | 3,801,088 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 10 | 65,536 |  | 24 | 34 | 3,801,088 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 2 | 65,536 |  | 24 | 34 | 3,801,088 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 3 | 65,536 |  | 24 | 34 | 3,801,088 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 4 | 131,072 |  | 24 | 34 | 7,602,176 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 5 | 131,072 |  | 24 | 34 | 7,602,176 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 6 | 131,072 |  | 24 | 34 | 7,602,176 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 7 | 131,072 |  | 24 | 34 | 7,602,176 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 8 | 131,072 |  | 24 | 34 | 7,602,176 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 9 | 131,072 |  | 24 | 34 | 7,602,176 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 0 | 524,288 |  | 20 | 40 | 31,457,280 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 1 | 262,144 |  | 20 | 40 | 15,728,640 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 10 | 262,144 |  | 20 | 40 | 15,728,640 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 2 | 262,144 |  | 20 | 40 | 15,728,640 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 3 | 262,144 |  | 20 | 40 | 15,728,640 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 4 | 524,288 |  | 20 | 40 | 31,457,280 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 5 | 524,288 |  | 20 | 40 | 31,457,280 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 6 | 524,288 |  | 20 | 40 | 31,457,280 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 7 | 524,288 |  | 20 | 40 | 31,457,280 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 8 | 524,288 |  | 20 | 40 | 31,457,280 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 9 | 262,144 |  | 20 | 40 | 15,728,640 | 
| leaf | VmConnectorAir | 21000000 | 0 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21000000 | 1 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21000000 | 10 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21000000 | 2 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21000000 | 3 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21000000 | 4 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21000000 | 5 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21000000 | 6 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21000000 | 7 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21000000 | 8 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21000000 | 9 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VolatileBoundaryAir | 21000000 | 0 | 2,097,152 |  | 8 | 11 | 39,845,888 | 
| leaf | VolatileBoundaryAir | 21000000 | 1 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21000000 | 10 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21000000 | 2 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21000000 | 3 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21000000 | 4 | 2,097,152 |  | 8 | 11 | 39,845,888 | 
| leaf | VolatileBoundaryAir | 21000000 | 5 | 2,097,152 |  | 8 | 11 | 39,845,888 | 
| leaf | VolatileBoundaryAir | 21000000 | 6 | 2,097,152 |  | 8 | 11 | 39,845,888 | 
| leaf | VolatileBoundaryAir | 21000000 | 7 | 2,097,152 |  | 8 | 11 | 39,845,888 | 
| leaf | VolatileBoundaryAir | 21000000 | 8 | 2,097,152 |  | 8 | 11 | 39,845,888 | 
| leaf | VolatileBoundaryAir | 21000000 | 9 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| root | AccessAdapterAir<2> | 21000000 | 0 | 262,144 |  | 8 | 11 | 4,980,736 | 
| root | AccessAdapterAir<4> | 21000000 | 0 | 131,072 |  | 8 | 13 | 2,752,512 | 
| root | AccessAdapterAir<8> | 21000000 | 0 | 256 |  | 8 | 17 | 6,400 | 
| root | FriReducedOpeningAir | 21000000 | 0 | 131,072 |  | 20 | 26 | 6,029,312 | 
| root | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 0 | 32,768 |  | 112 | 399 | 16,744,448 | 
| root | PhantomAir | 21000000 | 0 | 32,768 |  | 8 | 6 | 458,752 | 
| root | ProgramAir | 21000000 | 0 | 262,144 |  | 8 | 10 | 4,718,592 | 
| root | VariableRangeCheckerAir | 21000000 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| root | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 0 | 524,288 |  | 12 | 23 | 18,350,080 | 
| root | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 0 | 65,536 |  | 8 | 10 | 1,179,648 | 
| root | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 0 | 64 |  | 12 | 22 | 2,176 | 
| root | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 21000000 | 0 | 1,048,576 |  | 12 | 30 | 44,040,192 | 
| root | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 0 | 524,288 |  | 16 | 25 | 21,495,808 | 
| root | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 0 | 65,536 |  | 16 | 34 | 3,276,800 | 
| root | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 0 | 65,536 |  | 12 | 40 | 3,407,872 | 
| root | VmConnectorAir | 21000000 | 0 | 2 | 1 | 8 | 4 | 24 | 
| root | VolatileBoundaryAir | 21000000 | 0 | 524,288 |  | 8 | 11 | 9,961,472 | 

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
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 0 | 1 |  | 48 | 35 | 83 | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 0 | 1 |  | 52 | 40 | 92 | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 0 | 1 |  | 72 | 57 | 129 | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 0 | 1 |  | 72 | 39 | 111 | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 0 | 1 |  | 52 | 31 | 83 | 
| agg_keygen | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 0 | 1 |  | 28 | 21 | 49 | 
| agg_keygen | VmConnectorAir | 21000000 | 0 | 2 | 1 | 12 | 4 | 32 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 0 | 128 |  | 12 | 25 | 4,736 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 10 | 16 |  | 12 | 25 | 592 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 4 | 1,048,576 |  | 12 | 25 | 38,797,312 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 5 | 262,144 |  | 12 | 25 | 9,699,328 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 6 | 262,144 |  | 12 | 25 | 9,699,328 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 7 | 262,144 |  | 12 | 25 | 9,699,328 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 8 | 262,144 |  | 12 | 25 | 9,699,328 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 9 | 8,192 |  | 12 | 25 | 303,104 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<2> | 21000000 | 4 | 1,024 |  | 12 | 11 | 23,552 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<2> | 21000000 | 5 | 1,024 |  | 12 | 11 | 23,552 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<2> | 21000000 | 6 | 4,096 |  | 12 | 11 | 94,208 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<2> | 21000000 | 9 | 262,144 |  | 12 | 11 | 6,029,312 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 0 | 64 |  | 12 | 41 | 3,392 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 10 | 8 |  | 12 | 41 | 424 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 4 | 524,288 |  | 12 | 41 | 27,787,264 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 5 | 131,072 |  | 12 | 41 | 6,946,816 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 6 | 131,072 |  | 12 | 41 | 6,946,816 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 7 | 131,072 |  | 12 | 41 | 6,946,816 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 8 | 131,072 |  | 12 | 41 | 6,946,816 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 9 | 4,096 |  | 12 | 41 | 217,088 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 0 | 128 |  | 12 | 13 | 3,200 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 4 | 512 |  | 12 | 13 | 12,800 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 5 | 512 |  | 12 | 13 | 12,800 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 6 | 4,096 |  | 12 | 13 | 102,400 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 8 | 256 |  | 12 | 13 | 6,400 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 9 | 131,072 |  | 12 | 13 | 3,276,800 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 0 | 1,048,576 |  | 12 | 17 | 30,408,704 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 1 | 1,048,576 |  | 12 | 17 | 30,408,704 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 10 | 262,144 |  | 12 | 17 | 7,602,176 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 2 | 1,048,576 |  | 12 | 17 | 30,408,704 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 3 | 1,048,576 |  | 12 | 17 | 30,408,704 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 4 | 2,097,152 |  | 12 | 17 | 60,817,408 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 5 | 2,097,152 |  | 12 | 17 | 60,817,408 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 6 | 2,097,152 |  | 12 | 17 | 60,817,408 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 7 | 2,097,152 |  | 12 | 17 | 60,817,408 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 8 | 2,097,152 |  | 12 | 17 | 60,817,408 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<8> | 21000000 | 9 | 1,048,576 |  | 12 | 17 | 30,408,704 | 
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
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 0 | 1 |  | 532 | 3,164 | 3,696 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 1 | 1 |  | 532 | 3,164 | 3,696 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 10 | 16,384 |  | 532 | 3,164 | 60,555,264 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 2 | 1 |  | 532 | 3,164 | 3,696 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 3 | 524,288 |  | 532 | 3,164 | 1,937,768,448 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 4 | 131,072 |  | 532 | 3,164 | 484,442,112 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 5 | 16,384 |  | 532 | 3,164 | 60,555,264 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 6 | 16,384 |  | 532 | 3,164 | 60,555,264 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 7 | 16,384 |  | 532 | 3,164 | 60,555,264 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 8 | 65,536 |  | 532 | 3,164 | 242,221,056 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 9 | 524,288 |  | 532 | 3,164 | 1,937,768,448 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 0 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 1 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 10 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 2 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 3 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 4 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 5 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 6 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 7 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 8 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21000000 | MemoryMerkleAir<8> | 21000000 | 9 | 2,097,152 |  | 12 | 32 | 92,274,688 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 0 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 1 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 10 | 262,144 |  | 8 | 20 | 7,340,032 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 2 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 3 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 4 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 5 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 6 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 7 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 8 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 9 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 0 | 65,536 |  | 8 | 6 | 917,504 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 1 | 65,536 |  | 8 | 6 | 917,504 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 10 | 512 |  | 8 | 6 | 7,168 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 2 | 65,536 |  | 8 | 6 | 917,504 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 3 | 16,384 |  | 8 | 6 | 229,376 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 4 | 8,192 |  | 8 | 6 | 114,688 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 5 | 4,096 |  | 8 | 6 | 57,344 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 6 | 4,096 |  | 8 | 6 | 57,344 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 7 | 8,192 |  | 8 | 6 | 114,688 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 8 | 4,096 |  | 8 | 6 | 57,344 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 9 | 8,192 |  | 8 | 6 | 114,688 | 
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
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 0 | 524,288 |  | 24 | 32 | 29,360,128 | 
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 1 | 524,288 |  | 24 | 32 | 29,360,128 | 
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 2 | 524,288 |  | 24 | 32 | 29,360,128 | 
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 3 | 1,048,576 |  | 24 | 32 | 58,720,256 | 
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 4 | 2,048 |  | 24 | 32 | 114,688 | 
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 5 | 8 |  | 24 | 32 | 448 | 
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 6 | 64 |  | 24 | 32 | 3,584 | 
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 7 | 16 |  | 24 | 32 | 896 | 
| reth.prove_e2e.block_21000000 | Rv32HintStoreAir | 21000000 | 8 | 64 |  | 24 | 32 | 3,584 | 
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
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 0 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 1 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 10 | 1,048,576 |  | 28 | 36 | 67,108,864 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 2 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 3 | 2,097,152 |  | 28 | 36 | 134,217,728 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 4 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 5 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 6 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 7 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 8 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 9 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 0 | 524,288 |  | 24 | 37 | 31,981,568 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 1 | 524,288 |  | 24 | 37 | 31,981,568 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 10 | 32,768 |  | 24 | 37 | 1,998,848 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 2 | 524,288 |  | 24 | 37 | 31,981,568 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 3 | 131,072 |  | 24 | 37 | 7,995,392 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 4 | 262,144 |  | 24 | 37 | 15,990,784 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 5 | 1,048,576 |  | 24 | 37 | 63,963,136 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 6 | 524,288 |  | 24 | 37 | 31,981,568 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 7 | 524,288 |  | 24 | 37 | 31,981,568 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 8 | 524,288 |  | 24 | 37 | 31,981,568 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 9 | 524,288 |  | 24 | 37 | 31,981,568 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 0 | 1,048,576 |  | 28 | 53 | 84,934,656 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 1 | 1,048,576 |  | 28 | 53 | 84,934,656 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 10 | 131,072 |  | 28 | 53 | 10,616,832 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 2 | 1,048,576 |  | 28 | 53 | 84,934,656 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 3 | 262,144 |  | 28 | 53 | 21,233,664 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 4 | 2,097,152 |  | 28 | 53 | 169,869,312 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 5 | 2,097,152 |  | 28 | 53 | 169,869,312 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 6 | 2,097,152 |  | 28 | 53 | 169,869,312 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 7 | 2,097,152 |  | 28 | 53 | 169,869,312 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 8 | 2,097,152 |  | 28 | 53 | 169,869,312 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 9 | 524,288 |  | 28 | 53 | 42,467,328 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 0 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 1 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 10 | 262,144 |  | 16 | 26 | 11,010,048 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 2 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 3 | 524,288 |  | 16 | 26 | 22,020,096 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 4 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 5 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 6 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 7 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 8 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 9 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 0 | 1,048,576 |  | 20 | 32 | 54,525,952 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 1 | 1,048,576 |  | 20 | 32 | 54,525,952 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 10 | 32,768 |  | 20 | 32 | 1,703,936 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 2 | 1,048,576 |  | 20 | 32 | 54,525,952 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 3 | 262,144 |  | 20 | 32 | 13,631,488 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 4 | 2,097,152 |  | 20 | 32 | 109,051,904 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 5 | 2,097,152 |  | 20 | 32 | 109,051,904 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 6 | 4,194,304 |  | 20 | 32 | 218,103,808 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 7 | 2,097,152 |  | 20 | 32 | 109,051,904 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 8 | 2,097,152 |  | 20 | 32 | 109,051,904 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 9 | 262,144 |  | 20 | 32 | 13,631,488 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 0 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 1 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 10 | 32,768 |  | 16 | 18 | 1,114,112 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 2 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 3 | 262,144 |  | 16 | 18 | 8,912,896 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 4 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 5 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 6 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 7 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 8 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 9 | 262,144 |  | 16 | 18 | 8,912,896 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 4 | 4,096 |  | 100 | 168 | 1,097,728 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 5 | 16,384 |  | 100 | 168 | 4,390,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 6 | 16,384 |  | 100 | 168 | 4,390,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 7 | 16,384 |  | 100 | 168 | 4,390,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 8 | 16,384 |  | 100 | 168 | 4,390,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 9 | 512 |  | 100 | 168 | 137,216 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 4 | 2,048 |  | 36 | 169 | 419,840 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 5 | 4,096 |  | 36 | 169 | 839,680 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 6 | 8,192 |  | 36 | 169 | 1,679,360 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 7 | 4,096 |  | 36 | 169 | 839,680 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 8 | 4,096 |  | 36 | 169 | 839,680 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 4 | 512 |  | 100 | 164 | 135,168 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 5 | 2,048 |  | 100 | 164 | 540,672 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 6 | 1,024 |  | 100 | 164 | 270,336 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 7 | 1,024 |  | 100 | 164 | 270,336 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 8 | 512 |  | 100 | 164 | 135,168 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 4 | 1,024 |  | 84 | 241 | 332,800 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 5 | 2,048 |  | 84 | 241 | 665,600 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 6 | 2,048 |  | 84 | 241 | 665,600 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 7 | 2,048 |  | 84 | 241 | 665,600 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 8 | 2,048 |  | 84 | 241 | 665,600 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 10 | 2 |  | 28 | 124 | 304 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 4 | 8,192 |  | 28 | 124 | 1,245,184 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 5 | 32,768 |  | 28 | 124 | 4,980,736 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 6 | 16,384 |  | 28 | 124 | 2,490,368 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 7 | 16,384 |  | 28 | 124 | 2,490,368 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 8 | 16,384 |  | 28 | 124 | 2,490,368 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 9 | 512 |  | 28 | 124 | 77,824 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 0 | 1 |  | 32 | 166 | 198 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 4 | 131,072 |  | 32 | 166 | 25,952,256 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 5 | 1,024 |  | 32 | 166 | 202,752 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 6 | 4,096 |  | 32 | 166 | 811,008 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 7 | 2,048 |  | 32 | 166 | 405,504 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 8 | 4,096 |  | 32 | 166 | 811,008 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 0 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 1 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 10 | 262,144 |  | 20 | 28 | 12,582,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 2 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 3 | 131,072 |  | 20 | 28 | 6,291,456 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 4 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 5 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 6 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 7 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 8 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 9 | 262,144 |  | 20 | 28 | 12,582,912 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 0 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 1 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 10 | 32,768 |  | 28 | 35 | 2,064,384 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 2 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 3 | 262,144 |  | 28 | 35 | 16,515,072 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 4 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 5 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 6 | 2,097,152 |  | 28 | 35 | 132,120,576 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 7 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 8 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 9 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 0 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 1 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 10 | 1,048,576 |  | 28 | 40 | 71,303,168 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 2 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 3 | 4,194,304 |  | 28 | 40 | 285,212,672 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 4 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 5 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 6 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 7 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 8 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 9 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 4 | 64 |  | 40 | 57 | 6,208 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 5 | 256 |  | 40 | 57 | 24,832 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 6 | 256 |  | 40 | 57 | 24,832 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 7 | 64 |  | 40 | 57 | 6,208 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 8 | 128 |  | 40 | 57 | 12,416 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 9 | 512 |  | 40 | 57 | 49,664 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 1 | 1,024 |  | 40 | 39 | 80,896 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 10 | 32 |  | 40 | 39 | 2,528 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 2 | 1,024 |  | 40 | 39 | 80,896 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 3 | 4,096 |  | 40 | 39 | 323,584 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 4 | 65,536 |  | 40 | 39 | 5,177,344 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 5 | 131,072 |  | 40 | 39 | 10,354,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 6 | 131,072 |  | 40 | 39 | 10,354,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 7 | 131,072 |  | 40 | 39 | 10,354,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 8 | 131,072 |  | 40 | 39 | 10,354,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 9 | 16,384 |  | 40 | 39 | 1,294,336 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 0 | 256 |  | 28 | 31 | 15,104 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 1 | 2,048 |  | 28 | 31 | 120,832 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 10 | 8,192 |  | 28 | 31 | 483,328 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 2 | 2,048 |  | 28 | 31 | 120,832 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 3 | 16,384 |  | 28 | 31 | 966,656 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 4 | 262,144 |  | 28 | 31 | 15,466,496 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 5 | 262,144 |  | 28 | 31 | 15,466,496 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 6 | 262,144 |  | 28 | 31 | 15,466,496 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 7 | 524,288 |  | 28 | 31 | 30,932,992 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 8 | 262,144 |  | 28 | 31 | 15,466,496 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 9 | 65,536 |  | 28 | 31 | 3,866,624 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 0 | 262,144 |  | 16 | 21 | 9,699,328 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 1 | 262,144 |  | 16 | 21 | 9,699,328 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 10 | 131,072 |  | 16 | 21 | 4,849,664 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 2 | 262,144 |  | 16 | 21 | 9,699,328 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 3 | 65,536 |  | 16 | 21 | 2,424,832 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 4 | 262,144 |  | 16 | 21 | 9,699,328 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 5 | 65,536 |  | 16 | 21 | 2,424,832 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 6 | 65,536 |  | 16 | 21 | 2,424,832 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 7 | 131,072 |  | 16 | 21 | 4,849,664 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 8 | 131,072 |  | 16 | 21 | 4,849,664 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 9 | 131,072 |  | 16 | 21 | 4,849,664 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 0 | 1 |  | 416 | 543 | 959 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 65,536 |  | 416 | 543 | 62,849,024 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 5 | 256 |  | 416 | 543 | 245,504 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 6 | 2,048 |  | 416 | 543 | 1,964,032 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 7 | 512 |  | 416 | 543 | 491,008 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 8 | 2,048 |  | 416 | 543 | 1,964,032 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 0 | 1 |  | 160 | 261 | 421 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 1,024 |  | 160 | 261 | 431,104 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 5 | 4 |  | 160 | 261 | 1,684 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 6 | 32 |  | 160 | 261 | 13,472 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 7 | 8 |  | 160 | 261 | 3,368 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 8 | 32 |  | 160 | 261 | 13,472 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 0 | 1 |  | 428 | 619 | 1,047 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 32,768 |  | 428 | 619 | 34,308,096 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 5 | 256 |  | 428 | 619 | 268,032 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 6 | 1,024 |  | 428 | 619 | 1,072,128 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 7 | 512 |  | 428 | 619 | 536,064 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 8 | 1,024 |  | 428 | 619 | 1,072,128 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 0 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 1 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 10 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 2 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 3 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 4 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 5 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 6 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 7 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 8 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21000000 | VmConnectorAir | 21000000 | 9 | 2 | 1 | 8 | 4 | 24 | 

| group | block_number | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | num_segments | main_trace_commit_time_ms | main_cells_used | halo2_total_cells | halo2_keygen_time_ms | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 1,336 | 32,718 | 4,501,474 | 488,719,320 | 31,160 | 1,565 | 11,805 | 5,084 | 5,481 | 1 | 6,810 | 173,934,434 | 5,447,564 | 13,685 | 411 | 222 | 
| halo2_outer | 21000000 |  | 49,456 |  |  |  |  |  |  |  |  |  | 62,033,753 |  |  |  |  | 
| halo2_wrapper | 21000000 |  | 23,215 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
| reth.prove_e2e.block_21000000 | 21000000 |  |  |  |  |  |  |  |  |  | 11 |  |  |  |  |  |  | 

| group | block_number | cell_tracker_span | simple_advice_cells | lookup_advice_cells | fixed_cells |
| --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | VerifierProgram | 601,677 | 154,661 | 190,799 | 
| agg_keygen | 21000000 | VerifierProgram;PoseidonCell | 20,120 |  | 5,800 | 
| agg_keygen | 21000000 | VerifierProgram;stage-c-build-rounds | 335,431 | 737 | 94,619 | 
| agg_keygen | 21000000 | VerifierProgram;stage-c-build-rounds;PoseidonCell | 47,785 |  | 13,775 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs | 161 | 40 | 61 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify | 583,795 | 2,096 | 170,007 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;PoseidonCell | 72,935 |  | 21,025 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;cache-generator-powers | 414,288 | 71,064 | 127,344 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;compute-reduced-opening;single-reduced-opening-eval | 8,516,284 | 378,952 | 1,585,360 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;pre-compute-alpha-pows | 76,596 | 11,168 | 22,344 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch | 83,720 |  | 19,320 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;PoseidonCell | 9,831,192 |  | 2,841,160 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;verify-batch-reduce-fast;PoseidonCell | 8,444,072 | 236,712 | 2,579,668 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query | 999,768 | 163,632 | 287,504 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext | 182,000 |  | 42,000 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;PoseidonCell | 17,682,000 |  | 5,110,000 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;verify-batch-reduce-fast;PoseidonCell | 1,618,176 | 42,280 | 484,960 | 
| agg_keygen | 21000000 | VerifierProgram;stage-e-verify-constraints | 9,600,137 | 1,862,273 | 2,956,714 | 

| group | block_number | idx | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | 21000000 | 0 | 2,895 | 14,323 | 3,904,904 | 318,587,864 | 10,918 | 1,255 | 2,358 | 925 | 3,898 | 1,345 | 148,217,436 | 1,133 | 510 | 
| internal.0 | 21000000 | 1 | 2,899 | 14,580 | 3,904,181 | 318,587,864 | 11,123 | 1,242 | 2,425 | 861 | 4,059 | 1,026 | 148,206,792 | 1,507 | 558 | 
| internal.0 | 21000000 | 2 | 2,897 | 14,484 | 3,904,692 | 318,587,864 | 11,021 | 1,280 | 2,399 | 1,002 | 4,069 | 1,116 | 148,212,006 | 1,150 | 566 | 
| internal.0 | 21000000 | 3 | 2,910 | 14,129 | 3,904,971 | 318,587,864 | 10,682 | 1,269 | 2,338 | 957 | 3,949 | 1,056 | 148,229,873 | 1,108 | 537 | 
| internal.0 | 21000000 | 4 | 2,933 | 14,402 | 3,904,248 | 318,587,864 | 10,934 | 1,277 | 2,333 | 931 | 3,955 | 1,136 | 148,291,304 | 1,299 | 535 | 
| internal.0 | 21000000 | 5 | 1,518 | 8,873 | 1,952,375 | 162,834,136 | 7,108 | 756 | 1,813 | 720 | 2,597 | 811 | 75,283,283 | 406 | 247 | 
| internal.1 | 21000000 | 6 | 2,810 | 13,952 | 3,765,450 | 318,587,864 | 10,624 | 1,256 | 2,243 | 980 | 3,853 | 1,070 | 142,956,844 | 1,216 | 518 | 
| internal.1 | 21000000 | 7 | 2,867 | 13,748 | 3,766,005 | 318,587,864 | 10,329 | 1,265 | 2,227 | 977 | 3,889 | 1,027 | 142,962,394 | 941 | 552 | 
| internal.1 | 21000000 | 8 | 2,742 | 13,684 | 3,704,840 | 318,587,864 | 10,428 | 1,246 | 2,337 | 916 | 3,921 | 1,007 | 141,209,697 | 997 | 514 | 
| internal.2 | 21000000 | 10 | 1,424 | 9,250 | 1,883,277 | 162,834,136 | 7,586 | 773 | 1,845 | 718 | 2,656 | 1,018 | 72,653,613 | 571 | 240 | 
| internal.2 | 21000000 | 9 | 2,863 | 13,768 | 3,765,600 | 318,587,864 | 10,364 | 1,254 | 2,327 | 967 | 3,727 | 1,056 | 142,958,344 | 1,028 | 541 | 
| internal.3 | 21000000 | 11 | 2,826 | 13,940 | 3,704,290 | 318,587,864 | 10,598 | 1,259 | 2,232 | 932 | 3,933 | 1,063 | 141,204,197 | 1,175 | 516 | 
| leaf | 21000000 | 0 | 5,869 | 27,709 | 5,811,277 | 838,477,528 | 20,960 | 2,600 | 3,551 | 1,862 | 7,072 | 1,966 | 277,127,109 | 3,904 | 880 | 
| leaf | 21000000 | 1 | 2,996 | 17,007 | 3,125,252 | 426,387,160 | 13,542 | 1,528 | 2,814 | 1,181 | 5,419 | 1,562 | 144,132,017 | 1,035 | 469 | 
| leaf | 21000000 | 10 | 2,840 | 16,812 | 3,103,850 | 426,394,584 | 13,539 | 1,537 | 2,697 | 1,160 | 5,460 | 1,419 | 143,981,262 | 1,261 | 433 | 
| leaf | 21000000 | 2 | 2,794 | 16,503 | 3,124,941 | 426,387,160 | 13,264 | 1,540 | 2,710 | 1,071 | 5,410 | 1,385 | 144,128,907 | 1,142 | 445 | 
| leaf | 21000000 | 3 | 2,748 | 16,536 | 3,059,437 | 426,387,160 | 13,353 | 1,544 | 2,716 | 1,156 | 5,522 | 1,333 | 141,754,386 | 1,078 | 435 | 
| leaf | 21000000 | 4 | 4,478 | 23,821 | 4,848,990 | 732,571,352 | 18,594 | 2,337 | 3,640 | 1,710 | 6,569 | 1,962 | 225,472,981 | 2,371 | 749 | 
| leaf | 21000000 | 5 | 4,414 | 24,354 | 4,844,000 | 732,571,352 | 19,058 | 2,319 | 3,392 | 1,580 | 6,707 | 1,890 | 225,407,692 | 3,165 | 882 | 
| leaf | 21000000 | 6 | 4,406 | 24,573 | 4,846,960 | 732,571,352 | 19,282 | 2,395 | 3,490 | 1,689 | 6,831 | 1,961 | 225,516,195 | 2,912 | 885 | 
| leaf | 21000000 | 7 | 4,267 | 23,750 | 4,760,888 | 719,464,152 | 18,605 | 2,252 | 3,544 | 1,589 | 6,590 | 1,859 | 222,560,028 | 2,766 | 878 | 
| leaf | 21000000 | 8 | 4,268 | 23,606 | 4,804,171 | 719,464,152 | 18,499 | 2,257 | 3,477 | 1,584 | 6,584 | 1,867 | 224,066,351 | 2,725 | 839 | 
| leaf | 21000000 | 9 | 3,067 | 17,801 | 3,467,715 | 430,188,248 | 14,236 | 1,550 | 2,749 | 1,241 | 5,478 | 2,092 | 158,077,820 | 1,122 | 498 | 
| root | 21000000 | 0 | 1,397 | 31,651 | 1,883,941 | 139,764,120 | 30,029 | 1,271 | 10,884 | 4,723 | 5,543 | 7,042 | 72,676,675 | 561 | 225 | 

| group | block_number | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 0 | 18 | 1,424 |  | 7,747,673 | 1,405 | 85 | 198 | 134 | 707 | 144 | 928,069 | 134 | 1 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 16,650 | 62,839 | 20,485,123 | 1,976,080,644 | 42,428 | 4,290 | 6,211 | 4,601 | 12,019 | 8,565 | 988,882,283 | 6,713 | 3,761 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 16,238 | 58,864 | 20,509,666 | 1,976,246,920 | 36,750 | 4,262 | 4,701 | 3,201 | 11,766 | 5,619 | 985,929,699 | 7,191 | 5,876 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 16,879 | 30,044 | 1,959,808 | 499,288,864 | 12,697 | 1,519 | 2,218 | 1,078 | 5,171 | 1,736 | 289,588,357 | 968 | 468 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 19,354 | 63,071 | 20,512,191 | 1,976,246,920 | 37,848 | 4,206 | 4,396 | 3,207 | 11,821 | 7,548 | 986,781,354 | 6,657 | 5,869 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 10,209 | 62,618 | 5,383,300 | 2,815,176,728 | 50,649 | 11,626 | 3,641 | 2,743 | 15,554 | 10,828 | 1,423,967,380 | 6,248 | 1,760 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 25,797 | 83,412 | 23,489,873 | 2,637,345,368 | 48,239 | 7,290 | 5,746 | 4,380 | 17,450 | 6,749 | 1,360,095,831 | 6,605 | 9,376 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 22,419 | 69,121 | 23,903,872 | 2,094,138,520 | 40,227 | 5,000 | 5,592 | 3,963 | 13,458 | 5,670 | 1,089,340,047 | 6,528 | 6,475 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 25,448 | 75,748 | 24,800,651 | 2,238,667,032 | 42,754 | 5,278 | 6,233 | 3,950 | 14,126 | 5,663 | 1,150,581,966 | 7,488 | 7,546 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 24,941 | 72,797 | 24,267,068 | 2,078,010,200 | 40,210 | 5,063 | 5,461 | 4,371 | 13,247 | 5,243 | 1,108,349,320 | 6,812 | 7,646 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 26,622 | 76,802 | 23,897,711 | 2,246,467,992 | 42,359 | 5,910 | 6,052 | 3,945 | 15,190 | 5,547 | 1,278,187,751 | 5,696 | 7,821 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 26,439 | 93,537 | 14,076,269 | 4,158,290,456 | 61,982 | 13,523 | 4,609 | 3,953 | 19,692 | 12,166 | 1,924,190,592 | 8,027 | 5,116 | 

</details>


Commit: https://github.com/axiom-crypto/openvm-reth-benchmark/commit/00ce959dac0dcddfcc524c7f427ab0574f3b750f

Max Segment Length: 8388508

Instance Type: m7a.48xlarge

Memory Allocator: jemalloc

[Benchmark Workflow](https://github.com/axiom-crypto/openvm-reth-benchmark/actions/runs/13038301685)
