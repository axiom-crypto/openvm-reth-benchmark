| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total |  1,202.04 |  288.47 |
| reth.prove_e2e.block_21000000 |  712.65 |  92.91 |
| leaf |  221.97 |  26.53 |
| internal.0 |  76.51 |  13.69 |
| internal.1 |  40.22 |  13.51 |
| internal.2 |  22.33 |  13.48 |
| internal.3 |  13.43 |  13.43 |
| root |  32.91 |  32.91 |
| halo2_outer |  59.17 |  59.17 |
| halo2_wrapper |  22.85 |  22.85 |
| agg_keygen |  34.86 |  33.51 |


| reth.prove_e2e.block_21000000 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  64,786.55 |  712,652 |  92,913 |  29,440 |
| `main_cells_used     ` |  1,143,945,302.27 |  12,583,398,325 |  1,923,804,427 |  289,471,070 |
| `total_cycles        ` |  1,959,722 |  1,959,722 |  1,959,722 |  1,959,722 |
| `execute_time_ms     ` |  5,046.27 |  55,509 |  8,616 |  418 |
| `trace_gen_time_ms   ` |  20,005.82 |  220,064 |  26,066 |  9,793 |
| `stark_prove_excluding_trace_time_ms` |  39,734.45 |  437,079 |  62,485 |  12,058 |
| `main_trace_commit_time_ms` |  7,567.82 |  83,246 |  16,312 |  1,674 |
| `generate_perm_trace_time_ms` |  5,074.73 |  55,822 |  6,595 |  744 |
| `perm_trace_commit_time_ms` |  3,686.27 |  40,549 |  4,528 |  1,029 |
| `quotient_poly_compute_time_ms` |  6,013.64 |  66,150 |  13,188 |  1,454 |
| `quotient_poly_commit_time_ms` |  4,975.82 |  54,734 |  5,975 |  2,475 |
| `pcs_opening_time_ms ` |  12,403.36 |  136,437 |  18,116 |  4,672 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  20,179.27 |  221,972 |  26,526 |  16,028 |
| `main_cells_used     ` |  187,343,178.73 |  2,060,774,966 |  267,839,610 |  137,096,146 |
| `total_cycles        ` |  4,163,130.36 |  45,794,434 |  5,810,820 |  3,059,322 |
| `execute_time_ms     ` |  641.09 |  7,052 |  893 |  388 |
| `trace_gen_time_ms   ` |  3,682.55 |  40,508 |  5,376 |  2,658 |
| `stark_prove_excluding_trace_time_ms` |  15,855.64 |  174,412 |  20,348 |  12,801 |
| `main_trace_commit_time_ms` |  1,737.82 |  19,116 |  2,047 |  1,378 |
| `generate_perm_trace_time_ms` |  1,387.55 |  15,263 |  2,646 |  634 |
| `perm_trace_commit_time_ms` |  1,423.55 |  15,659 |  1,824 |  1,078 |
| `quotient_poly_compute_time_ms` |  1,972.09 |  21,693 |  2,564 |  1,535 |
| `quotient_poly_commit_time_ms` |  3,153.36 |  34,687 |  3,713 |  2,761 |
| `pcs_opening_time_ms ` |  6,176.82 |  67,945 |  7,549 |  5,157 |

| internal.0 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  12,751.67 |  76,510 |  13,690 |  8,882 |
| `main_cells_used     ` |  130,793,560.83 |  784,761,365 |  142,509,176 |  72,401,020 |
| `total_cycles        ` |  3,576,158.50 |  21,456,951 |  3,901,714 |  1,950,639 |
| `execute_time_ms     ` |  449.83 |  2,699 |  507 |  226 |
| `trace_gen_time_ms   ` |  2,582.83 |  15,497 |  2,857 |  1,391 |
| `stark_prove_excluding_trace_time_ms` |  9,719 |  58,314 |  10,483 |  7,265 |
| `main_trace_commit_time_ms` |  1,085 |  6,510 |  1,265 |  938 |
| `generate_perm_trace_time_ms` |  679.17 |  4,075 |  891 |  330 |
| `perm_trace_commit_time_ms` |  908.83 |  5,453 |  1,010 |  723 |
| `quotient_poly_compute_time_ms` |  1,174.83 |  7,049 |  1,268 |  745 |
| `quotient_poly_commit_time_ms` |  2,273.67 |  13,642 |  2,556 |  1,856 |
| `pcs_opening_time_ms ` |  3,592.67 |  21,556 |  3,833 |  2,668 |

| internal.1 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  13,405.67 |  40,217 |  13,508 |  13,254 |
| `main_cells_used     ` |  136,781,142.33 |  410,343,427 |  137,335,106 |  135,675,681 |
| `total_cycles        ` |  3,742,170.67 |  11,226,512 |  3,762,666 |  3,701,454 |
| `execute_time_ms     ` |  474.33 |  1,423 |  488 |  448 |
| `trace_gen_time_ms   ` |  2,712 |  8,136 |  2,757 |  2,667 |
| `stark_prove_excluding_trace_time_ms` |  10,219.33 |  30,658 |  10,353 |  10,010 |
| `main_trace_commit_time_ms` |  1,073.33 |  3,220 |  1,094 |  1,044 |
| `generate_perm_trace_time_ms` |  982.67 |  2,948 |  1,037 |  908 |
| `perm_trace_commit_time_ms` |  901.33 |  2,704 |  928 |  885 |
| `quotient_poly_compute_time_ms` |  1,270.67 |  3,812 |  1,285 |  1,263 |
| `quotient_poly_commit_time_ms` |  2,197.67 |  6,593 |  2,284 |  2,063 |
| `pcs_opening_time_ms ` |  3,789.33 |  11,368 |  3,882 |  3,714 |

| internal.2 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  11,164.50 |  22,329 |  13,479 |  8,850 |
| `main_cells_used     ` |  103,589,600.50 |  207,179,201 |  137,335,178 |  69,844,023 |
| `total_cycles        ` |  2,822,219 |  5,644,438 |  3,762,674 |  1,881,764 |
| `execute_time_ms     ` |  358.50 |  717 |  499 |  218 |
| `trace_gen_time_ms   ` |  2,014 |  4,028 |  2,673 |  1,355 |
| `stark_prove_excluding_trace_time_ms` |  8,792 |  17,584 |  10,307 |  7,277 |
| `main_trace_commit_time_ms` |  1,047 |  2,094 |  1,128 |  966 |
| `generate_perm_trace_time_ms` |  566 |  1,132 |  800 |  332 |
| `perm_trace_commit_time_ms` |  798 |  1,596 |  928 |  668 |
| `quotient_poly_compute_time_ms` |  1,011.50 |  2,023 |  1,269 |  754 |
| `quotient_poly_commit_time_ms` |  2,116.50 |  4,233 |  2,423 |  1,810 |
| `pcs_opening_time_ms ` |  3,249.50 |  6,499 |  3,755 |  2,744 |

| internal.3 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  13,431 |  13,431 |  13,431 |  13,431 |
| `main_cells_used     ` |  135,673,386 |  135,673,386 |  135,673,386 |  135,673,386 |
| `total_cycles        ` |  3,701,199 |  3,701,199 |  3,701,199 |  3,701,199 |
| `execute_time_ms     ` |  452 |  452 |  452 |  452 |
| `trace_gen_time_ms   ` |  2,711 |  2,711 |  2,711 |  2,711 |
| `stark_prove_excluding_trace_time_ms` |  10,268 |  10,268 |  10,268 |  10,268 |
| `main_trace_commit_time_ms` |  1,132 |  1,132 |  1,132 |  1,132 |
| `generate_perm_trace_time_ms` |  961 |  961 |  961 |  961 |
| `perm_trace_commit_time_ms` |  956 |  956 |  956 |  956 |
| `quotient_poly_compute_time_ms` |  1,294 |  1,294 |  1,294 |  1,294 |
| `quotient_poly_commit_time_ms` |  2,234 |  2,234 |  2,234 |  2,234 |
| `pcs_opening_time_ms ` |  3,687 |  3,687 |  3,687 |  3,687 |

| root |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  32,905 |  32,905 |  32,905 |  32,905 |
| `main_cells_used     ` |  69,862,393 |  69,862,393 |  69,862,393 |  69,862,393 |
| `total_cycles        ` |  1,882,047 |  1,882,047 |  1,882,047 |  1,882,047 |
| `execute_time_ms     ` |  219 |  219 |  219 |  219 |
| `trace_gen_time_ms   ` |  1,363 |  1,363 |  1,363 |  1,363 |
| `stark_prove_excluding_trace_time_ms` |  31,323 |  31,323 |  31,323 |  31,323 |
| `main_trace_commit_time_ms` |  7,009 |  7,009 |  7,009 |  7,009 |
| `generate_perm_trace_time_ms` |  288 |  288 |  288 |  288 |
| `perm_trace_commit_time_ms` |  4,991 |  4,991 |  4,991 |  4,991 |
| `quotient_poly_compute_time_ms` |  1,392 |  1,392 |  1,392 |  1,392 |
| `quotient_poly_commit_time_ms` |  11,925 |  11,925 |  11,925 |  11,925 |
| `pcs_opening_time_ms ` |  5,712 |  5,712 |  5,712 |  5,712 |

| halo2_outer |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  59,168 |  59,168 |  59,168 |  59,168 |
| `main_cells_used     ` |  61,886,027 |  61,886,027 |  61,886,027 |  61,886,027 |

| halo2_wrapper |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  22,854 |  22,854 |  22,854 |  22,854 |

| agg_keygen |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  17,432 |  34,864 |  33,514 |  1,350 |
| `main_cells_used     ` |  84,083,811 |  168,167,622 |  167,239,553 |  928,069 |
| `total_cycles        ` |  4,501,337 |  4,501,337 |  4,501,337 |  4,501,337 |
| `execute_time_ms     ` |  99.50 |  199 |  198 |  1 |
| `trace_gen_time_ms   ` |  643.50 |  1,287 |  1,270 |  17 |
| `stark_prove_excluding_trace_time_ms` |  16,689 |  33,378 |  32,046 |  1,332 |
| `main_trace_commit_time_ms` |  3,469.50 |  6,939 |  6,792 |  147 |
| `generate_perm_trace_time_ms` |  220.50 |  441 |  321 |  120 |
| `perm_trace_commit_time_ms` |  2,638 |  5,276 |  5,129 |  147 |
| `quotient_poly_compute_time_ms` |  804.50 |  1,609 |  1,520 |  89 |
| `quotient_poly_commit_time_ms` |  6,300 |  12,600 |  12,436 |  164 |
| `pcs_opening_time_ms ` |  3,252 |  6,504 |  5,843 |  661 |



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
| KeccakVmAir | 21000000 | 4 | 321 | 4,380 | 
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
| 21000000 | 210 | 

| group | air_name | block_number | rows | quotient_deg | prep_cols | perm_cols | main_cols | interactions | constraints | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | AccessAdapterAir<16> | 21000000 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<2> | 21000000 | 524,288 | 8 |  | 12 | 11 | 5 | 12 | 12,058,624 | 
| agg_keygen | AccessAdapterAir<32> | 21000000 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<4> | 21000000 | 262,144 | 8 |  | 12 | 13 | 5 | 12 | 6,553,600 | 
| agg_keygen | AccessAdapterAir<64> | 21000000 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<8> | 21000000 | 512 | 8 |  | 12 | 17 | 5 | 12 | 14,848 | 
| agg_keygen | BitwiseOperationLookupAir<8> | 21000000 |  | 2 |  |  |  | 2 | 4 |  | 
| agg_keygen | FriReducedOpeningAir | 21000000 | 524,288 | 8 |  | 36 | 25 | 31 | 52 | 31,981,568 | 
| agg_keygen | MemoryMerkleAir<8> | 21000000 |  | 2 |  |  |  | 4 | 39 |  | 
| agg_keygen | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 65,536 | 8 |  | 216 | 399 | 176 | 555 | 40,304,640 | 
| agg_keygen | PersistentBoundaryAir<8> | 21000000 |  | 2 |  |  |  | 3 | 6 |  | 
| agg_keygen | PhantomAir | 21000000 | 65,536 | 4 |  | 8 | 6 | 3 | 5 | 917,504 | 
| agg_keygen | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 |  | 2 |  |  |  | 1 | 286 |  | 
| agg_keygen | ProgramAir | 21000000 | 262,144 | 1 |  | 8 | 10 | 1 | 4 | 4,718,592 | 
| agg_keygen | RangeTupleCheckerAir<2> | 21000000 |  | 1 |  |  |  | 1 | 4 |  | 
| agg_keygen | Rv32HintStoreAir | 21000000 |  | 2 |  |  |  | 19 | 26 |  | 
| agg_keygen | VariableRangeCheckerAir | 21000000 | 262,144 | 1 | 2 | 8 | 1 | 1 | 4 | 2,359,296 | 
| agg_keygen | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 4,194,304 | 8 |  | 20 | 29 | 15 | 23 | 205,520,896 | 
| agg_keygen | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 1,048,576 | 8 |  | 16 | 23 | 11 | 22 | 40,894,464 | 
| agg_keygen | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 131,072 | 8 |  | 12 | 9 | 7 | 6 | 2,752,512 | 
| agg_keygen | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 64 | 8 |  | 16 | 23 | 11 | 23 | 2,496 | 
| agg_keygen | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 2,097,152 | 8 |  | 24 | 22 | 15 | 16 | 96,468,992 | 
| agg_keygen | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 131,072 | 8 |  | 24 | 31 | 15 | 16 | 7,208,960 | 
| agg_keygen | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 131,072 | 8 |  | 20 | 38 | 15 | 23 | 7,602,176 | 
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
| internal.0 | FriReducedOpeningAir | 21000000 | 0 | 262,144 |  | 36 | 25 | 15,990,784 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 1 | 262,144 |  | 36 | 25 | 15,990,784 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 2 | 262,144 |  | 36 | 25 | 15,990,784 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 3 | 262,144 |  | 36 | 25 | 15,990,784 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 4 | 262,144 |  | 36 | 25 | 15,990,784 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 5 | 131,072 |  | 36 | 25 | 7,995,392 | 
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
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 0 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 1 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 2 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 3 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 4 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 5 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 0 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 1 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 2 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 3 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 4 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 5 | 524,288 |  | 16 | 23 | 20,447,232 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 0 | 131,072 |  | 12 | 9 | 2,752,512 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 1 | 131,072 |  | 12 | 9 | 2,752,512 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 2 | 131,072 |  | 12 | 9 | 2,752,512 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 3 | 131,072 |  | 12 | 9 | 2,752,512 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 4 | 131,072 |  | 12 | 9 | 2,752,512 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 5 | 65,536 |  | 12 | 9 | 1,376,256 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 0 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 1 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 2 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 3 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 4 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 5 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 0 | 1,048,576 |  | 24 | 22 | 48,234,496 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 1 | 1,048,576 |  | 24 | 22 | 48,234,496 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 2 | 1,048,576 |  | 24 | 22 | 48,234,496 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 3 | 1,048,576 |  | 24 | 22 | 48,234,496 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 4 | 1,048,576 |  | 24 | 22 | 48,234,496 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 5 | 524,288 |  | 24 | 22 | 24,117,248 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 0 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 1 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 2 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 3 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 4 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 5 | 65,536 |  | 24 | 31 | 3,604,480 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 0 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 1 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 2 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 3 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 4 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 5 | 65,536 |  | 20 | 38 | 3,801,088 | 
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
| internal.1 | FriReducedOpeningAir | 21000000 | 6 | 262,144 |  | 36 | 25 | 15,990,784 | 
| internal.1 | FriReducedOpeningAir | 21000000 | 7 | 262,144 |  | 36 | 25 | 15,990,784 | 
| internal.1 | FriReducedOpeningAir | 21000000 | 8 | 262,144 |  | 36 | 25 | 15,990,784 | 
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
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 6 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 7 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 8 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 6 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 7 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 8 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 6 | 131,072 |  | 12 | 9 | 2,752,512 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 7 | 131,072 |  | 12 | 9 | 2,752,512 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 8 | 131,072 |  | 12 | 9 | 2,752,512 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 6 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 7 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 8 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 6 | 1,048,576 |  | 24 | 22 | 48,234,496 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 7 | 1,048,576 |  | 24 | 22 | 48,234,496 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 8 | 1,048,576 |  | 24 | 22 | 48,234,496 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 6 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 7 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 8 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 6 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 7 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 8 | 131,072 |  | 20 | 38 | 7,602,176 | 
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
| internal.2 | FriReducedOpeningAir | 21000000 | 10 | 131,072 |  | 36 | 25 | 7,995,392 | 
| internal.2 | FriReducedOpeningAir | 21000000 | 9 | 262,144 |  | 36 | 25 | 15,990,784 | 
| internal.2 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 10 | 32,768 |  | 216 | 399 | 20,152,320 | 
| internal.2 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 9 | 65,536 |  | 216 | 399 | 40,304,640 | 
| internal.2 | PhantomAir | 21000000 | 10 | 32,768 |  | 8 | 6 | 458,752 | 
| internal.2 | PhantomAir | 21000000 | 9 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.2 | ProgramAir | 21000000 | 10 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.2 | ProgramAir | 21000000 | 9 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.2 | VariableRangeCheckerAir | 21000000 | 10 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.2 | VariableRangeCheckerAir | 21000000 | 9 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.2 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 10 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.2 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 9 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.2 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 10 | 524,288 |  | 16 | 23 | 20,447,232 | 
| internal.2 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 9 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.2 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 10 | 65,536 |  | 12 | 9 | 1,376,256 | 
| internal.2 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 9 | 131,072 |  | 12 | 9 | 2,752,512 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 10 | 64 |  | 16 | 23 | 2,496 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 9 | 64 |  | 16 | 23 | 2,496 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 10 | 524,288 |  | 24 | 22 | 24,117,248 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 9 | 1,048,576 |  | 24 | 22 | 48,234,496 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 10 | 65,536 |  | 24 | 31 | 3,604,480 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 9 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 10 | 65,536 |  | 20 | 38 | 3,801,088 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 9 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.2 | VmConnectorAir | 21000000 | 10 | 2 | 1 | 8 | 4 | 24 | 
| internal.2 | VmConnectorAir | 21000000 | 9 | 2 | 1 | 8 | 4 | 24 | 
| internal.2 | VolatileBoundaryAir | 21000000 | 10 | 524,288 |  | 8 | 11 | 9,961,472 | 
| internal.2 | VolatileBoundaryAir | 21000000 | 9 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| internal.3 | AccessAdapterAir<2> | 21000000 | 11 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.3 | AccessAdapterAir<4> | 21000000 | 11 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.3 | AccessAdapterAir<8> | 21000000 | 11 | 512 |  | 12 | 17 | 14,848 | 
| internal.3 | FriReducedOpeningAir | 21000000 | 11 | 262,144 |  | 36 | 25 | 15,990,784 | 
| internal.3 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 11 | 65,536 |  | 216 | 399 | 40,304,640 | 
| internal.3 | PhantomAir | 21000000 | 11 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.3 | ProgramAir | 21000000 | 11 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.3 | VariableRangeCheckerAir | 21000000 | 11 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.3 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 11 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.3 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 11 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| internal.3 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 11 | 131,072 |  | 12 | 9 | 2,752,512 | 
| internal.3 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 11 | 64 |  | 16 | 23 | 2,496 | 
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 11 | 1,048,576 |  | 24 | 22 | 48,234,496 | 
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 11 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.3 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 11 | 131,072 |  | 20 | 38 | 7,602,176 | 
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
| leaf | FriReducedOpeningAir | 21000000 | 0 | 2,097,152 |  | 36 | 25 | 127,926,272 | 
| leaf | FriReducedOpeningAir | 21000000 | 1 | 524,288 |  | 36 | 25 | 31,981,568 | 
| leaf | FriReducedOpeningAir | 21000000 | 10 | 524,288 |  | 36 | 25 | 31,981,568 | 
| leaf | FriReducedOpeningAir | 21000000 | 2 | 524,288 |  | 36 | 25 | 31,981,568 | 
| leaf | FriReducedOpeningAir | 21000000 | 3 | 524,288 |  | 36 | 25 | 31,981,568 | 
| leaf | FriReducedOpeningAir | 21000000 | 4 | 1,048,576 |  | 36 | 25 | 63,963,136 | 
| leaf | FriReducedOpeningAir | 21000000 | 5 | 1,048,576 |  | 36 | 25 | 63,963,136 | 
| leaf | FriReducedOpeningAir | 21000000 | 6 | 1,048,576 |  | 36 | 25 | 63,963,136 | 
| leaf | FriReducedOpeningAir | 21000000 | 7 | 1,048,576 |  | 36 | 25 | 63,963,136 | 
| leaf | FriReducedOpeningAir | 21000000 | 8 | 1,048,576 |  | 36 | 25 | 63,963,136 | 
| leaf | FriReducedOpeningAir | 21000000 | 9 | 524,288 |  | 36 | 25 | 31,981,568 | 
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
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 0 | 4,194,304 |  | 20 | 29 | 205,520,896 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 1 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 10 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 2 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 3 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 4 | 4,194,304 |  | 20 | 29 | 205,520,896 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 5 | 4,194,304 |  | 20 | 29 | 205,520,896 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 6 | 4,194,304 |  | 20 | 29 | 205,520,896 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 7 | 4,194,304 |  | 20 | 29 | 205,520,896 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 8 | 4,194,304 |  | 20 | 29 | 205,520,896 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 9 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
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
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 0 | 65,536 |  | 12 | 9 | 1,376,256 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 1 | 65,536 |  | 12 | 9 | 1,376,256 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 10 | 65,536 |  | 12 | 9 | 1,376,256 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 2 | 65,536 |  | 12 | 9 | 1,376,256 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 3 | 65,536 |  | 12 | 9 | 1,376,256 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 4 | 65,536 |  | 12 | 9 | 1,376,256 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 5 | 65,536 |  | 12 | 9 | 1,376,256 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 6 | 65,536 |  | 12 | 9 | 1,376,256 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 7 | 65,536 |  | 12 | 9 | 1,376,256 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 8 | 65,536 |  | 12 | 9 | 1,376,256 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 9 | 65,536 |  | 12 | 9 | 1,376,256 | 
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
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 0 | 2,097,152 |  | 24 | 22 | 96,468,992 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 1 | 1,048,576 |  | 24 | 22 | 48,234,496 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 10 | 1,048,576 |  | 24 | 22 | 48,234,496 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 2 | 1,048,576 |  | 24 | 22 | 48,234,496 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 3 | 1,048,576 |  | 24 | 22 | 48,234,496 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 4 | 2,097,152 |  | 24 | 22 | 96,468,992 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 5 | 2,097,152 |  | 24 | 22 | 96,468,992 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 6 | 2,097,152 |  | 24 | 22 | 96,468,992 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 7 | 2,097,152 |  | 24 | 22 | 96,468,992 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 8 | 2,097,152 |  | 24 | 22 | 96,468,992 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 9 | 1,048,576 |  | 24 | 22 | 48,234,496 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 0 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 1 | 65,536 |  | 24 | 31 | 3,604,480 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 10 | 65,536 |  | 24 | 31 | 3,604,480 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 2 | 65,536 |  | 24 | 31 | 3,604,480 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 3 | 65,536 |  | 24 | 31 | 3,604,480 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 4 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 5 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 6 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 7 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 8 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 9 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 0 | 524,288 |  | 20 | 38 | 30,408,704 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 1 | 262,144 |  | 20 | 38 | 15,204,352 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 10 | 262,144 |  | 20 | 38 | 15,204,352 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 2 | 262,144 |  | 20 | 38 | 15,204,352 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 3 | 262,144 |  | 20 | 38 | 15,204,352 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 4 | 524,288 |  | 20 | 38 | 30,408,704 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 5 | 524,288 |  | 20 | 38 | 30,408,704 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 6 | 524,288 |  | 20 | 38 | 30,408,704 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 7 | 524,288 |  | 20 | 38 | 30,408,704 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 8 | 524,288 |  | 20 | 38 | 30,408,704 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 9 | 262,144 |  | 20 | 38 | 15,204,352 | 
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
| root | FriReducedOpeningAir | 21000000 | 0 | 131,072 |  | 20 | 25 | 5,898,240 | 
| root | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 0 | 32,768 |  | 112 | 399 | 16,744,448 | 
| root | PhantomAir | 21000000 | 0 | 32,768 |  | 8 | 6 | 458,752 | 
| root | ProgramAir | 21000000 | 0 | 262,144 |  | 8 | 10 | 4,718,592 | 
| root | VariableRangeCheckerAir | 21000000 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| root | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 0 | 1,048,576 |  | 12 | 29 | 42,991,616 | 
| root | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 0 | 524,288 |  | 12 | 23 | 18,350,080 | 
| root | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 0 | 65,536 |  | 8 | 9 | 1,114,112 | 
| root | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 0 | 64 |  | 12 | 22 | 2,176 | 
| root | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 0 | 524,288 |  | 16 | 22 | 19,922,944 | 
| root | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 0 | 65,536 |  | 16 | 31 | 3,080,192 | 
| root | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 0 | 65,536 |  | 12 | 38 | 3,276,800 | 
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
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 0 | 1 |  | 532 | 3,163 | 3,695 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 1 | 1 |  | 532 | 3,163 | 3,695 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 10 | 16,384 |  | 532 | 3,163 | 60,538,880 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 2 | 1 |  | 532 | 3,163 | 3,695 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 3 | 524,288 |  | 532 | 3,163 | 1,937,244,160 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 4 | 131,072 |  | 532 | 3,163 | 484,311,040 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 5 | 16,384 |  | 532 | 3,163 | 60,538,880 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 6 | 16,384 |  | 532 | 3,163 | 60,538,880 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 7 | 16,384 |  | 532 | 3,163 | 60,538,880 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 8 | 65,536 |  | 532 | 3,163 | 242,155,520 | 
| reth.prove_e2e.block_21000000 | KeccakVmAir | 21000000 | 9 | 524,288 |  | 532 | 3,163 | 1,937,244,160 | 
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
| agg_keygen | 21000000 | 1,270 | 33,514 | 4,501,337 | 476,922,840 | 32,046 | 1,520 | 12,436 | 5,129 | 5,843 | 1 | 6,792 | 167,239,553 | 5,447,564 | 15,140 | 321 | 198 | 
| halo2_outer | 21000000 |  | 59,168 |  |  |  |  |  |  |  |  |  | 61,886,027 |  |  |  |  | 
| halo2_wrapper | 21000000 |  | 22,854 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
| reth.prove_e2e.block_21000000 | 21000000 |  |  |  |  |  |  |  |  |  | 11 |  |  |  |  |  |  | 

| group | block_number | cell_tracker_span | simple_advice_cells | lookup_advice_cells | fixed_cells |
| --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | VerifierProgram | 598,113 | 153,473 | 189,611 | 
| agg_keygen | 21000000 | VerifierProgram;PoseidonCell | 20,120 |  | 5,800 | 
| agg_keygen | 21000000 | VerifierProgram;stage-c-build-rounds | 335,431 | 737 | 94,619 | 
| agg_keygen | 21000000 | VerifierProgram;stage-c-build-rounds;PoseidonCell | 47,785 |  | 13,775 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs | 161 | 40 | 61 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify | 583,795 | 2,096 | 170,007 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;PoseidonCell | 72,935 |  | 21,025 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;cache-generator-powers | 414,288 | 71,064 | 127,344 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;compute-reduced-opening;single-reduced-opening-eval | 8,462,076 | 378,952 | 1,577,352 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;pre-compute-alpha-pows | 76,596 | 11,168 | 22,344 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch | 83,720 |  | 19,320 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;PoseidonCell | 9,831,192 |  | 2,841,160 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;verify-batch-reduce-fast;PoseidonCell | 8,362,788 | 234,864 | 2,555,112 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query | 999,768 | 163,632 | 287,504 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext | 182,000 |  | 42,000 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;PoseidonCell | 17,682,000 |  | 5,110,000 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;verify-batch-reduce-fast;PoseidonCell | 1,618,176 | 42,280 | 484,960 | 
| agg_keygen | 21000000 | VerifierProgram;stage-e-verify-constraints | 9,595,641 | 1,861,135 | 2,955,219 | 

| group | block_number | idx | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | 21000000 | 0 | 2,754 | 13,690 | 3,901,364 | 312,296,408 | 10,483 | 1,262 | 2,556 | 871 | 3,793 | 1,265 | 142,462,592 | 731 | 453 | 
| internal.0 | 21000000 | 1 | 2,850 | 13,487 | 3,900,241 | 312,296,408 | 10,132 | 1,268 | 2,334 | 979 | 3,761 | 1,085 | 142,432,289 | 699 | 505 | 
| internal.0 | 21000000 | 2 | 2,857 | 13,534 | 3,901,665 | 312,296,408 | 10,173 | 1,251 | 2,232 | 1,010 | 3,721 | 1,064 | 142,478,765 | 891 | 504 | 
| internal.0 | 21000000 | 3 | 2,840 | 13,504 | 3,901,714 | 312,296,408 | 10,160 | 1,255 | 2,407 | 894 | 3,833 | 1,090 | 142,477,523 | 676 | 504 | 
| internal.0 | 21000000 | 4 | 2,805 | 13,413 | 3,901,328 | 312,296,408 | 10,101 | 1,268 | 2,257 | 976 | 3,780 | 1,068 | 142,509,176 | 748 | 507 | 
| internal.0 | 21000000 | 5 | 1,391 | 8,882 | 1,950,639 | 159,688,408 | 7,265 | 745 | 1,856 | 723 | 2,668 | 938 | 72,401,020 | 330 | 226 | 
| internal.1 | 21000000 | 6 | 2,712 | 13,455 | 3,762,666 | 312,296,408 | 10,295 | 1,285 | 2,246 | 891 | 3,772 | 1,094 | 137,335,106 | 1,003 | 448 | 
| internal.1 | 21000000 | 7 | 2,667 | 13,508 | 3,762,392 | 312,296,408 | 10,353 | 1,264 | 2,284 | 928 | 3,882 | 1,082 | 137,332,640 | 908 | 488 | 
| internal.1 | 21000000 | 8 | 2,757 | 13,254 | 3,701,454 | 312,296,408 | 10,010 | 1,263 | 2,063 | 885 | 3,714 | 1,044 | 135,675,681 | 1,037 | 487 | 
| internal.2 | 21000000 | 10 | 1,355 | 8,850 | 1,881,764 | 159,688,408 | 7,277 | 754 | 1,810 | 668 | 2,744 | 966 | 69,844,023 | 332 | 218 | 
| internal.2 | 21000000 | 9 | 2,673 | 13,479 | 3,762,674 | 312,296,408 | 10,307 | 1,269 | 2,423 | 928 | 3,755 | 1,128 | 137,335,178 | 800 | 499 | 
| internal.3 | 21000000 | 11 | 2,711 | 13,431 | 3,701,199 | 312,296,408 | 10,268 | 1,294 | 2,234 | 956 | 3,687 | 1,132 | 135,673,386 | 961 | 452 | 
| leaf | 21000000 | 0 | 5,376 | 26,526 | 5,810,820 | 824,387,288 | 20,348 | 2,564 | 3,713 | 1,824 | 7,549 | 2,047 | 267,839,610 | 2,646 | 802 | 
| leaf | 21000000 | 1 | 2,920 | 16,189 | 3,124,942 | 419,833,560 | 12,801 | 1,535 | 2,857 | 1,078 | 5,175 | 1,454 | 139,389,225 | 698 | 468 | 
| leaf | 21000000 | 10 | 2,753 | 16,441 | 3,103,275 | 419,840,984 | 13,299 | 1,539 | 2,785 | 1,279 | 5,309 | 1,514 | 139,216,763 | 869 | 389 | 
| leaf | 21000000 | 2 | 2,763 | 16,028 | 3,125,317 | 419,833,560 | 12,875 | 1,539 | 2,761 | 1,271 | 5,157 | 1,378 | 139,392,600 | 765 | 390 | 
| leaf | 21000000 | 3 | 2,658 | 16,363 | 3,059,322 | 419,833,560 | 13,317 | 1,551 | 2,928 | 1,102 | 5,587 | 1,418 | 137,096,146 | 726 | 388 | 
| leaf | 21000000 | 4 | 4,181 | 23,039 | 4,848,935 | 719,529,688 | 18,158 | 2,290 | 3,406 | 1,604 | 6,945 | 1,973 | 217,841,597 | 1,935 | 700 | 
| leaf | 21000000 | 5 | 4,329 | 22,753 | 4,843,992 | 719,529,688 | 17,551 | 2,307 | 3,310 | 1,533 | 6,798 | 1,780 | 217,781,603 | 1,818 | 873 | 
| leaf | 21000000 | 6 | 4,182 | 22,897 | 4,846,448 | 719,529,688 | 17,822 | 2,320 | 3,369 | 1,649 | 6,870 | 1,847 | 217,882,664 | 1,762 | 893 | 
| leaf | 21000000 | 7 | 4,157 | 22,170 | 4,759,964 | 706,422,488 | 17,168 | 2,231 | 3,351 | 1,609 | 6,657 | 1,804 | 215,059,637 | 1,511 | 845 | 
| leaf | 21000000 | 8 | 4,186 | 22,563 | 4,803,609 | 706,422,488 | 17,564 | 2,237 | 3,303 | 1,621 | 6,605 | 1,895 | 216,500,407 | 1,899 | 813 | 
| leaf | 21000000 | 9 | 3,003 | 17,003 | 3,467,810 | 423,438,040 | 13,509 | 1,580 | 2,904 | 1,089 | 5,293 | 2,006 | 152,774,714 | 634 | 491 | 
| root | 21000000 | 0 | 1,363 | 32,905 | 1,882,047 | 136,618,392 | 31,323 | 1,392 | 11,925 | 4,991 | 5,712 | 7,009 | 69,862,393 | 288 | 219 | 

| group | block_number | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 0 | 17 | 1,350 |  | 7,747,673 | 1,332 | 89 | 164 | 147 | 661 | 147 | 928,069 | 120 | 1 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 15,787 | 60,438 |  | 1,976,080,643 | 41,541 | 4,756 | 5,975 | 4,289 | 10,988 | 8,915 | 988,878,222 | 6,595 | 3,110 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 15,521 | 55,495 |  | 1,976,246,919 | 34,686 | 4,104 | 4,947 | 3,800 | 10,293 | 5,829 | 985,911,040 | 5,703 | 5,288 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 16,964 | 29,440 | 1,959,722 | 499,272,480 | 12,058 | 1,454 | 2,475 | 1,029 | 4,672 | 1,674 | 289,471,070 | 744 | 418 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 16,321 | 54,450 |  | 1,976,246,919 | 32,933 | 4,023 | 4,313 | 3,115 | 10,287 | 5,217 | 986,790,771 | 5,970 | 5,196 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 9,793 | 61,256 |  | 2,814,652,440 | 49,789 | 11,395 | 3,934 | 3,916 | 14,167 | 12,160 | 1,428,059,256 | 4,209 | 1,674 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 24,428 | 77,962 |  | 2,637,214,296 | 44,918 | 7,026 | 5,873 | 3,807 | 15,870 | 7,951 | 1,353,951,817 | 4,378 | 8,616 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 21,437 | 64,928 |  | 2,094,122,136 | 37,808 | 4,754 | 5,563 | 4,055 | 12,397 | 5,255 | 1,090,253,513 | 5,770 | 5,683 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 24,628 | 70,794 |  | 2,238,650,648 | 39,394 | 5,060 | 5,624 | 3,820 | 12,912 | 6,579 | 1,150,194,864 | 5,383 | 6,772 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 23,396 | 68,907 |  | 2,077,993,816 | 38,539 | 4,764 | 5,328 | 4,528 | 12,474 | 6,107 | 1,108,199,294 | 5,323 | 6,972 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 26,066 | 76,069 |  | 2,246,402,456 | 42,928 | 5,626 | 5,694 | 4,234 | 14,261 | 7,247 | 1,277,884,051 | 5,851 | 7,075 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 25,723 | 92,913 |  | 4,157,766,168 | 62,485 | 13,188 | 5,008 | 3,956 | 18,116 | 16,312 | 1,923,804,427 | 5,896 | 4,705 | 

</details>


Commit: https://github.com/axiom-crypto/openvm-reth-benchmark/commit/f684c8e760c33019684b5a7d6232050e9e2962f3

Max Segment Length: 8388508

Instance Type: m7a.48xlarge

Memory Allocator: jemalloc

[Benchmark Workflow](https://github.com/axiom-crypto/openvm-reth-benchmark/actions/runs/13082535221)
