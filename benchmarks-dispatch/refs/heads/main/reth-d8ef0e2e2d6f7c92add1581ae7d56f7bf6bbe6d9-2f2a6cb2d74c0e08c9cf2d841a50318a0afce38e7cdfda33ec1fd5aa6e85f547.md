| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total |  588.75 |  218.98 |
| reth.prove_evm.block_21000000 |  292.63 |  34.03 |
| leaf |  106.44 |  12.07 |
| internal.0 |  46.98 |  12.43 |
| internal.1 |  13.21 |  8.73 |
| internal.2 |  6.58 |  6.58 |
| root |  31.56 |  31.56 |
| halo2_outer |  52.52 |  52.52 |
| halo2_wrapper |  38.85 |  38.85 |
| agg_keygen |  32.81 |  31.99 |


| reth.prove_evm.block_21000000 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  26,602.36 |  292,626 |  34,032 |  6,922 |
| `main_cells_used     ` |  1,091,468,028.09 |  12,006,148,309 |  1,591,332,740 |  210,978,086 |
| `total_cycles        ` |  14,278,232.73 |  157,060,560 |  22,213,839 |  1,161,202 |
| `execute_time_ms     ` |  2,359.91 |  25,959 |  3,749 |  176 |
| `trace_gen_time_ms   ` |  7,549.18 |  83,041 |  12,021 |  2,578 |
| `stark_prove_excluding_trace_time_ms` |  16,693.27 |  183,626 |  22,049 |  4,168 |
| `main_trace_commit_time_ms` |  4,405.55 |  48,461 |  7,684 |  920 |
| `generate_perm_trace_time_ms` |  1,148.45 |  12,633 |  1,411 |  222 |
| `perm_trace_commit_time_ms` |  3,038.27 |  33,421 |  3,588 |  560 |
| `quotient_poly_compute_time_ms` |  2,758 |  30,338 |  5,428 |  374 |
| `quotient_poly_commit_time_ms` |  1,256.82 |  13,825 |  1,528 |  530 |
| `pcs_opening_time_ms ` |  4,071.45 |  44,786 |  4,876 |  1,545 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  9,676.09 |  106,437 |  12,075 |  7,266 |
| `main_cells_used     ` |  216,236,447 |  2,378,600,917 |  284,865,257 |  154,403,715 |
| `total_cycles        ` |  2,743,813.27 |  30,181,946 |  3,518,565 |  2,022,895 |
| `execute_time_ms     ` |  1,220.18 |  13,422 |  1,453 |  968 |
| `trace_gen_time_ms   ` |  1,829.91 |  20,129 |  2,368 |  1,335 |
| `stark_prove_excluding_trace_time_ms` |  6,626 |  72,886 |  8,280 |  4,941 |
| `main_trace_commit_time_ms` |  1,115.45 |  12,270 |  1,382 |  761 |
| `generate_perm_trace_time_ms` |  560.36 |  6,164 |  797 |  370 |
| `perm_trace_commit_time_ms` |  1,441.27 |  15,854 |  1,852 |  1,017 |
| `quotient_poly_compute_time_ms` |  666.45 |  7,331 |  835 |  474 |
| `quotient_poly_commit_time_ms` |  708.27 |  7,791 |  886 |  595 |
| `pcs_opening_time_ms ` |  2,128.09 |  23,409 |  2,646 |  1,620 |

| internal.0 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  11,744 |  46,976 |  12,428 |  9,898 |
| `main_cells_used     ` |  193,085,221.25 |  772,340,885 |  212,774,464 |  139,493,352 |
| `total_cycles        ` |  3,227,382.25 |  12,909,529 |  3,545,774 |  2,333,406 |
| `execute_time_ms     ` |  1,974.50 |  7,898 |  2,268 |  1,384 |
| `trace_gen_time_ms   ` |  1,549.25 |  6,197 |  1,704 |  1,141 |
| `stark_prove_excluding_trace_time_ms` |  8,220.25 |  32,881 |  8,543 |  7,373 |
| `main_trace_commit_time_ms` |  1,558.50 |  6,234 |  1,666 |  1,291 |
| `generate_perm_trace_time_ms` |  430 |  1,720 |  492 |  386 |
| `perm_trace_commit_time_ms` |  1,193.75 |  4,775 |  1,235 |  1,091 |
| `quotient_poly_compute_time_ms` |  937.25 |  3,749 |  1,005 |  779 |
| `quotient_poly_commit_time_ms` |  1,638.50 |  6,554 |  1,719 |  1,482 |
| `pcs_opening_time_ms ` |  2,457 |  9,828 |  2,545 |  2,325 |

| internal.1 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  6,603 |  13,206 |  8,735 |  4,471 |
| `main_cells_used     ` |  77,149,798 |  154,299,596 |  115,072,118 |  39,227,478 |
| `total_cycles        ` |  1,559,118 |  3,118,236 |  2,338,776 |  779,460 |
| `execute_time_ms     ` |  692 |  1,384 |  1,043 |  341 |
| `trace_gen_time_ms   ` |  689 |  1,378 |  1,007 |  371 |
| `stark_prove_excluding_trace_time_ms` |  5,222 |  10,444 |  6,685 |  3,759 |
| `main_trace_commit_time_ms` |  867.50 |  1,735 |  1,198 |  537 |
| `generate_perm_trace_time_ms` |  314.50 |  629 |  425 |  204 |
| `perm_trace_commit_time_ms` |  700.50 |  1,401 |  938 |  463 |
| `quotient_poly_compute_time_ms` |  496 |  992 |  686 |  306 |
| `quotient_poly_commit_time_ms` |  1,206 |  2,412 |  1,393 |  1,019 |
| `pcs_opening_time_ms ` |  1,632.50 |  3,265 |  2,040 |  1,225 |

| internal.2 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  6,581 |  6,581 |  6,581 |  6,581 |
| `main_cells_used     ` |  75,829,710 |  75,829,710 |  75,829,710 |  75,829,710 |
| `total_cycles        ` |  1,542,899 |  1,542,899 |  1,542,899 |  1,542,899 |
| `execute_time_ms     ` |  662 |  662 |  662 |  662 |
| `trace_gen_time_ms   ` |  676 |  676 |  676 |  676 |
| `stark_prove_excluding_trace_time_ms` |  5,243 |  5,243 |  5,243 |  5,243 |
| `main_trace_commit_time_ms` |  844 |  844 |  844 |  844 |
| `generate_perm_trace_time_ms` |  308 |  308 |  308 |  308 |
| `perm_trace_commit_time_ms` |  685 |  685 |  685 |  685 |
| `quotient_poly_compute_time_ms` |  478 |  478 |  478 |  478 |
| `quotient_poly_commit_time_ms` |  1,252 |  1,252 |  1,252 |  1,252 |
| `pcs_opening_time_ms ` |  1,671 |  1,671 |  1,671 |  1,671 |

| root |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  31,556 |  31,556 |  31,556 |  31,556 |
| `main_cells_used     ` |  38,540,365 |  38,540,365 |  38,540,365 |  38,540,365 |
| `total_cycles        ` |  772,492 |  772,492 |  772,492 |  772,492 |
| `execute_time_ms     ` |  328 |  328 |  328 |  328 |
| `trace_gen_time_ms   ` |  359 |  359 |  359 |  359 |
| `stark_prove_excluding_trace_time_ms` |  30,869 |  30,869 |  30,869 |  30,869 |
| `main_trace_commit_time_ms` |  8,974 |  8,974 |  8,974 |  8,974 |
| `generate_perm_trace_time_ms` |  204 |  204 |  204 |  204 |
| `perm_trace_commit_time_ms` |  5,644 |  5,644 |  5,644 |  5,644 |
| `quotient_poly_compute_time_ms` |  660 |  660 |  660 |  660 |
| `quotient_poly_commit_time_ms` |  11,662 |  11,662 |  11,662 |  11,662 |
| `pcs_opening_time_ms ` |  3,717 |  3,717 |  3,717 |  3,717 |

| halo2_outer |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  52,515 |  52,515 |  52,515 |  52,515 |
| `main_cells_used     ` |  66,137,544 |  66,137,544 |  66,137,544 |  66,137,544 |

| halo2_wrapper |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  38,850 |  38,850 |  38,850 |  38,850 |

| agg_keygen |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  16,406.50 |  32,813 |  31,992 |  821 |
| `main_cells_used     ` |  43,232,786 |  86,465,572 |  86,455,016 |  10,556 |
| `total_cycles        ` |  1,622,307 |  1,622,307 |  1,622,307 |  1,622,307 |
| `execute_time_ms     ` |  166.50 |  333 |  333 |  0 |
| `trace_gen_time_ms   ` |  195.50 |  391 |  366 |  25 |
| `stark_prove_excluding_trace_time_ms` |  16,044.50 |  32,089 |  31,293 |  796 |
| `main_trace_commit_time_ms` |  4,524 |  9,048 |  8,966 |  82 |
| `generate_perm_trace_time_ms` |  123 |  246 |  207 |  39 |
| `perm_trace_commit_time_ms` |  2,871 |  5,742 |  5,642 |  100 |
| `quotient_poly_compute_time_ms` |  356 |  712 |  677 |  35 |
| `quotient_poly_commit_time_ms` |  5,934 |  11,868 |  11,762 |  106 |
| `pcs_opening_time_ms ` |  2,225 |  4,450 |  4,021 |  429 |



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
| 21000000 | 336 | 

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
| internal.1 | JalRangeCheckAir | 21000000 | 4 | 131,072 |  | 16 | 12 | 3,670,016 | 
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
| leaf | AccessAdapterAir<2> | 21000000 | 3 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 21000000 | 4 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 21000000 | 5 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 21000000 | 6 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 21000000 | 7 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 21000000 | 8 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<2> | 21000000 | 9 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<4> | 21000000 | 0 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 21000000 | 1 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 21000000 | 10 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 21000000 | 2 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 21000000 | 3 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 21000000 | 4 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 21000000 | 5 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 21000000 | 6 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 21000000 | 7 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 21000000 | 8 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 21000000 | 9 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<8> | 21000000 | 0 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 21000000 | 1 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 21000000 | 10 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 21000000 | 2 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 21000000 | 3 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 21000000 | 4 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 21000000 | 5 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 21000000 | 6 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 21000000 | 7 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 21000000 | 8 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 21000000 | 9 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | FriReducedOpeningAir | 21000000 | 0 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | FriReducedOpeningAir | 21000000 | 1 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | FriReducedOpeningAir | 21000000 | 10 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | FriReducedOpeningAir | 21000000 | 2 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | FriReducedOpeningAir | 21000000 | 3 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 21000000 | 4 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 21000000 | 5 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 21000000 | 6 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 21000000 | 7 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 21000000 | 8 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
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
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 10 | 131,072 |  | 312 | 398 | 93,061,120 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 2 | 131,072 |  | 312 | 398 | 93,061,120 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 3 | 262,144 |  | 312 | 398 | 186,122,240 | 
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
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 0 | 1,048,576 |  | 36 | 29 | 68,157,440 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 1 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 10 | 1,048,576 |  | 36 | 29 | 68,157,440 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 2 | 1,048,576 |  | 36 | 29 | 68,157,440 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 3 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 4 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 5 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 6 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 7 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 8 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 9 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 0 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 1 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 10 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 2 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 3 | 524,288 |  | 28 | 23 | 26,738,688 | 
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
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 0 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 1 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 10 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 2 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 3 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 4 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 5 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 6 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 7 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 8 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 9 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 0 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 1 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 10 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 2 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 3 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 4 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 5 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 6 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 7 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 8 | 131,072 |  | 40 | 27 | 8,781,824 | 
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
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 8 | 262,144 |  | 36 | 38 | 19,398,656 | 
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
| leaf | VolatileBoundaryAir | 21000000 | 0 | 524,288 |  | 20 | 12 | 16,777,216 | 
| leaf | VolatileBoundaryAir | 21000000 | 1 | 524,288 |  | 20 | 12 | 16,777,216 | 
| leaf | VolatileBoundaryAir | 21000000 | 10 | 524,288 |  | 20 | 12 | 16,777,216 | 
| leaf | VolatileBoundaryAir | 21000000 | 2 | 524,288 |  | 20 | 12 | 16,777,216 | 
| leaf | VolatileBoundaryAir | 21000000 | 3 | 1,048,576 |  | 20 | 12 | 33,554,432 | 
| leaf | VolatileBoundaryAir | 21000000 | 4 | 1,048,576 |  | 20 | 12 | 33,554,432 | 
| leaf | VolatileBoundaryAir | 21000000 | 5 | 1,048,576 |  | 20 | 12 | 33,554,432 | 
| leaf | VolatileBoundaryAir | 21000000 | 6 | 1,048,576 |  | 20 | 12 | 33,554,432 | 
| leaf | VolatileBoundaryAir | 21000000 | 7 | 1,048,576 |  | 20 | 12 | 33,554,432 | 
| leaf | VolatileBoundaryAir | 21000000 | 8 | 524,288 |  | 20 | 12 | 16,777,216 | 
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
| reth.prove_evm.block_21000000 | AccessAdapterAir<16> | 21000000 | 3 | 65,536 |  | 16 | 25 | 2,686,976 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<16> | 21000000 | 4 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<16> | 21000000 | 5 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<16> | 21000000 | 6 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<16> | 21000000 | 7 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<16> | 21000000 | 8 | 32,768 |  | 16 | 25 | 1,343,488 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<16> | 21000000 | 9 | 1,024 |  | 16 | 25 | 41,984 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<2> | 21000000 | 1 | 32,768 |  | 16 | 11 | 884,736 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<2> | 21000000 | 10 | 1,024 |  | 16 | 11 | 27,648 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<2> | 21000000 | 2 | 262,144 |  | 16 | 11 | 7,077,888 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<2> | 21000000 | 3 | 16,384 |  | 16 | 11 | 442,368 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<2> | 21000000 | 4 | 32,768 |  | 16 | 11 | 884,736 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<2> | 21000000 | 5 | 2,048 |  | 16 | 11 | 55,296 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<2> | 21000000 | 6 | 4,096 |  | 16 | 11 | 110,592 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<2> | 21000000 | 7 | 1,024 |  | 16 | 11 | 27,648 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<2> | 21000000 | 8 | 262,144 |  | 16 | 11 | 7,077,888 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<2> | 21000000 | 9 | 262,144 |  | 16 | 11 | 7,077,888 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<32> | 21000000 | 3 | 32,768 |  | 16 | 41 | 1,867,776 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<32> | 21000000 | 4 | 131,072 |  | 16 | 41 | 7,471,104 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<32> | 21000000 | 5 | 131,072 |  | 16 | 41 | 7,471,104 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<32> | 21000000 | 6 | 131,072 |  | 16 | 41 | 7,471,104 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<32> | 21000000 | 7 | 131,072 |  | 16 | 41 | 7,471,104 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<32> | 21000000 | 8 | 16,384 |  | 16 | 41 | 933,888 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<32> | 21000000 | 9 | 512 |  | 16 | 41 | 29,184 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<4> | 21000000 | 1 | 16,384 |  | 16 | 13 | 475,136 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<4> | 21000000 | 10 | 512 |  | 16 | 13 | 14,848 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<4> | 21000000 | 2 | 131,072 |  | 16 | 13 | 3,801,088 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<4> | 21000000 | 3 | 8,192 |  | 16 | 13 | 237,568 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<4> | 21000000 | 4 | 16,384 |  | 16 | 13 | 475,136 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<4> | 21000000 | 5 | 1,024 |  | 16 | 13 | 29,696 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<4> | 21000000 | 6 | 4,096 |  | 16 | 13 | 118,784 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<4> | 21000000 | 7 | 1,024 |  | 16 | 13 | 29,696 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<4> | 21000000 | 8 | 131,072 |  | 16 | 13 | 3,801,088 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<4> | 21000000 | 9 | 131,072 |  | 16 | 13 | 3,801,088 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<8> | 21000000 | 0 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<8> | 21000000 | 1 | 2,097,152 |  | 16 | 17 | 69,206,016 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<8> | 21000000 | 10 | 262,144 |  | 16 | 17 | 8,650,752 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<8> | 21000000 | 2 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<8> | 21000000 | 3 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<8> | 21000000 | 4 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<8> | 21000000 | 5 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<8> | 21000000 | 6 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<8> | 21000000 | 7 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<8> | 21000000 | 8 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_evm.block_21000000 | AccessAdapterAir<8> | 21000000 | 9 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_evm.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 0 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_evm.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 1 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_evm.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 10 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_evm.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 2 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_evm.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 3 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_evm.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 4 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_evm.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 5 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_evm.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 6 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_evm.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 7 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_evm.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 8 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_evm.block_21000000 | BitwiseOperationLookupAir<8> | 21000000 | 9 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_evm.block_21000000 | KeccakVmAir | 21000000 | 0 | 16,384 |  | 1,056 | 3,163 | 69,124,096 | 
| reth.prove_evm.block_21000000 | KeccakVmAir | 21000000 | 1 | 65,536 |  | 1,056 | 3,163 | 276,496,384 | 
| reth.prove_evm.block_21000000 | KeccakVmAir | 21000000 | 10 | 512 |  | 1,056 | 3,163 | 2,160,128 | 
| reth.prove_evm.block_21000000 | KeccakVmAir | 21000000 | 2 | 524,288 |  | 1,056 | 3,163 | 2,211,971,072 | 
| reth.prove_evm.block_21000000 | KeccakVmAir | 21000000 | 3 | 524,288 |  | 1,056 | 3,163 | 2,211,971,072 | 
| reth.prove_evm.block_21000000 | KeccakVmAir | 21000000 | 4 | 32,768 |  | 1,056 | 3,163 | 138,248,192 | 
| reth.prove_evm.block_21000000 | KeccakVmAir | 21000000 | 5 | 16,384 |  | 1,056 | 3,163 | 69,124,096 | 
| reth.prove_evm.block_21000000 | KeccakVmAir | 21000000 | 6 | 16,384 |  | 1,056 | 3,163 | 69,124,096 | 
| reth.prove_evm.block_21000000 | KeccakVmAir | 21000000 | 7 | 16,384 |  | 1,056 | 3,163 | 69,124,096 | 
| reth.prove_evm.block_21000000 | KeccakVmAir | 21000000 | 8 | 262,144 |  | 1,056 | 3,163 | 1,105,985,536 | 
| reth.prove_evm.block_21000000 | KeccakVmAir | 21000000 | 9 | 262,144 |  | 1,056 | 3,163 | 1,105,985,536 | 
| reth.prove_evm.block_21000000 | MemoryMerkleAir<8> | 21000000 | 0 | 2,097,152 |  | 16 | 32 | 100,663,296 | 
| reth.prove_evm.block_21000000 | MemoryMerkleAir<8> | 21000000 | 1 | 2,097,152 |  | 16 | 32 | 100,663,296 | 
| reth.prove_evm.block_21000000 | MemoryMerkleAir<8> | 21000000 | 10 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_evm.block_21000000 | MemoryMerkleAir<8> | 21000000 | 2 | 2,097,152 |  | 16 | 32 | 100,663,296 | 
| reth.prove_evm.block_21000000 | MemoryMerkleAir<8> | 21000000 | 3 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_evm.block_21000000 | MemoryMerkleAir<8> | 21000000 | 4 | 524,288 |  | 16 | 32 | 25,165,824 | 
| reth.prove_evm.block_21000000 | MemoryMerkleAir<8> | 21000000 | 5 | 262,144 |  | 16 | 32 | 12,582,912 | 
| reth.prove_evm.block_21000000 | MemoryMerkleAir<8> | 21000000 | 6 | 524,288 |  | 16 | 32 | 25,165,824 | 
| reth.prove_evm.block_21000000 | MemoryMerkleAir<8> | 21000000 | 7 | 524,288 |  | 16 | 32 | 25,165,824 | 
| reth.prove_evm.block_21000000 | MemoryMerkleAir<8> | 21000000 | 8 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_evm.block_21000000 | MemoryMerkleAir<8> | 21000000 | 9 | 2,097,152 |  | 16 | 32 | 100,663,296 | 
| reth.prove_evm.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 0 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_evm.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 1 | 2,097,152 |  | 12 | 20 | 67,108,864 | 
| reth.prove_evm.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 10 | 262,144 |  | 12 | 20 | 8,388,608 | 
| reth.prove_evm.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 2 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_evm.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 3 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_evm.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 4 | 524,288 |  | 12 | 20 | 16,777,216 | 
| reth.prove_evm.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 5 | 262,144 |  | 12 | 20 | 8,388,608 | 
| reth.prove_evm.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 6 | 262,144 |  | 12 | 20 | 8,388,608 | 
| reth.prove_evm.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 7 | 524,288 |  | 12 | 20 | 16,777,216 | 
| reth.prove_evm.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 8 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_evm.block_21000000 | PersistentBoundaryAir<8> | 21000000 | 9 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_evm.block_21000000 | PhantomAir | 21000000 | 0 | 4 |  | 12 | 6 | 72 | 
| reth.prove_evm.block_21000000 | PhantomAir | 21000000 | 1 | 2 |  | 12 | 6 | 36 | 
| reth.prove_evm.block_21000000 | PhantomAir | 21000000 | 10 | 1 |  | 12 | 6 | 18 | 
| reth.prove_evm.block_21000000 | PhantomAir | 21000000 | 2 | 1 |  | 12 | 6 | 18 | 
| reth.prove_evm.block_21000000 | PhantomAir | 21000000 | 3 | 128 |  | 12 | 6 | 2,304 | 
| reth.prove_evm.block_21000000 | PhantomAir | 21000000 | 4 | 128 |  | 12 | 6 | 2,304 | 
| reth.prove_evm.block_21000000 | PhantomAir | 21000000 | 5 | 4 |  | 12 | 6 | 72 | 
| reth.prove_evm.block_21000000 | PhantomAir | 21000000 | 6 | 8 |  | 12 | 6 | 144 | 
| reth.prove_evm.block_21000000 | PhantomAir | 21000000 | 7 | 8 |  | 12 | 6 | 144 | 
| reth.prove_evm.block_21000000 | PhantomAir | 21000000 | 8 | 8 |  | 12 | 6 | 144 | 
| reth.prove_evm.block_21000000 | PhantomAir | 21000000 | 9 | 1 |  | 12 | 6 | 18 | 
| reth.prove_evm.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 0 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_evm.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 1 | 2,097,152 |  | 8 | 300 | 645,922,816 | 
| reth.prove_evm.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 10 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_evm.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 2 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_evm.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 3 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_evm.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 4 | 262,144 |  | 8 | 300 | 80,740,352 | 
| reth.prove_evm.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 5 | 131,072 |  | 8 | 300 | 40,370,176 | 
| reth.prove_evm.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 6 | 131,072 |  | 8 | 300 | 40,370,176 | 
| reth.prove_evm.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 7 | 131,072 |  | 8 | 300 | 40,370,176 | 
| reth.prove_evm.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 8 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_evm.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 9 | 2,097,152 |  | 8 | 300 | 645,922,816 | 
| reth.prove_evm.block_21000000 | ProgramAir | 21000000 | 0 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_evm.block_21000000 | ProgramAir | 21000000 | 1 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_evm.block_21000000 | ProgramAir | 21000000 | 10 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_evm.block_21000000 | ProgramAir | 21000000 | 2 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_evm.block_21000000 | ProgramAir | 21000000 | 3 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_evm.block_21000000 | ProgramAir | 21000000 | 4 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_evm.block_21000000 | ProgramAir | 21000000 | 5 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_evm.block_21000000 | ProgramAir | 21000000 | 6 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_evm.block_21000000 | ProgramAir | 21000000 | 7 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_evm.block_21000000 | ProgramAir | 21000000 | 8 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_evm.block_21000000 | ProgramAir | 21000000 | 9 | 524,288 |  | 8 | 10 | 9,437,184 | 
| reth.prove_evm.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 0 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_evm.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 1 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_evm.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 10 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_evm.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 2 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_evm.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 3 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_evm.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 4 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_evm.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 5 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_evm.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 6 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_evm.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 7 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_evm.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 8 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_evm.block_21000000 | RangeTupleCheckerAir<2> | 21000000 | 9 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_evm.block_21000000 | Rv32HintStoreAir | 21000000 | 0 | 524,288 |  | 44 | 32 | 39,845,888 | 
| reth.prove_evm.block_21000000 | Rv32HintStoreAir | 21000000 | 1 | 1,048,576 |  | 44 | 32 | 79,691,776 | 
| reth.prove_evm.block_21000000 | Rv32HintStoreAir | 21000000 | 3 | 1,024 |  | 44 | 32 | 77,824 | 
| reth.prove_evm.block_21000000 | Rv32HintStoreAir | 21000000 | 4 | 1,024 |  | 44 | 32 | 77,824 | 
| reth.prove_evm.block_21000000 | Rv32HintStoreAir | 21000000 | 5 | 32 |  | 44 | 32 | 2,432 | 
| reth.prove_evm.block_21000000 | Rv32HintStoreAir | 21000000 | 6 | 64 |  | 44 | 32 | 4,864 | 
| reth.prove_evm.block_21000000 | Rv32HintStoreAir | 21000000 | 7 | 64 |  | 44 | 32 | 4,864 | 
| reth.prove_evm.block_21000000 | VariableRangeCheckerAir | 21000000 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_evm.block_21000000 | VariableRangeCheckerAir | 21000000 | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_evm.block_21000000 | VariableRangeCheckerAir | 21000000 | 10 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_evm.block_21000000 | VariableRangeCheckerAir | 21000000 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_evm.block_21000000 | VariableRangeCheckerAir | 21000000 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_evm.block_21000000 | VariableRangeCheckerAir | 21000000 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_evm.block_21000000 | VariableRangeCheckerAir | 21000000 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_evm.block_21000000 | VariableRangeCheckerAir | 21000000 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_evm.block_21000000 | VariableRangeCheckerAir | 21000000 | 7 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_evm.block_21000000 | VariableRangeCheckerAir | 21000000 | 8 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_evm.block_21000000 | VariableRangeCheckerAir | 21000000 | 9 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 0 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 1 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 10 | 524,288 |  | 52 | 36 | 46,137,344 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 2 | 4,194,304 |  | 52 | 36 | 369,098,752 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 3 | 4,194,304 |  | 52 | 36 | 369,098,752 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 4 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 5 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 6 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 7 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 8 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 9 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 0 | 1,048,576 |  | 40 | 37 | 80,740,352 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 1 | 1,048,576 |  | 40 | 37 | 80,740,352 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 10 | 512 |  | 40 | 37 | 39,424 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 2 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 3 | 131,072 |  | 40 | 37 | 10,092,544 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 4 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 5 | 1,048,576 |  | 40 | 37 | 80,740,352 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 6 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 7 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 8 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 9 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 0 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 1 | 524,288 |  | 52 | 53 | 55,050,240 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 10 | 4,096 |  | 52 | 53 | 430,080 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 2 | 131,072 |  | 52 | 53 | 13,762,560 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 3 | 524,288 |  | 52 | 53 | 55,050,240 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 4 | 2,097,152 |  | 52 | 53 | 220,200,960 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 5 | 2,097,152 |  | 52 | 53 | 220,200,960 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 6 | 2,097,152 |  | 52 | 53 | 220,200,960 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 7 | 2,097,152 |  | 52 | 53 | 220,200,960 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 8 | 524,288 |  | 52 | 53 | 55,050,240 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 9 | 262,144 |  | 52 | 53 | 27,525,120 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 0 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 1 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 10 | 262,144 |  | 28 | 26 | 14,155,776 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 2 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 3 | 1,048,576 |  | 28 | 26 | 56,623,104 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 4 | 1,048,576 |  | 28 | 26 | 56,623,104 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 5 | 1,048,576 |  | 28 | 26 | 56,623,104 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 6 | 1,048,576 |  | 28 | 26 | 56,623,104 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 7 | 1,048,576 |  | 28 | 26 | 56,623,104 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 8 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 | 9 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 0 | 1,048,576 |  | 32 | 32 | 67,108,864 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 1 | 2,097,152 |  | 32 | 32 | 134,217,728 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 10 | 4,096 |  | 32 | 32 | 262,144 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 2 | 262,144 |  | 32 | 32 | 16,777,216 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 3 | 262,144 |  | 32 | 32 | 16,777,216 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 4 | 524,288 |  | 32 | 32 | 33,554,432 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 5 | 524,288 |  | 32 | 32 | 33,554,432 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 6 | 524,288 |  | 32 | 32 | 33,554,432 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 7 | 524,288 |  | 32 | 32 | 33,554,432 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 8 | 524,288 |  | 32 | 32 | 33,554,432 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 9 | 262,144 |  | 32 | 32 | 16,777,216 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 0 | 1,048,576 |  | 28 | 18 | 48,234,496 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 1 | 1,048,576 |  | 28 | 18 | 48,234,496 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 10 | 2,048 |  | 28 | 18 | 94,208 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 2 | 262,144 |  | 28 | 18 | 12,058,624 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 3 | 262,144 |  | 28 | 18 | 12,058,624 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 4 | 262,144 |  | 28 | 18 | 12,058,624 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 5 | 131,072 |  | 28 | 18 | 6,029,312 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 6 | 131,072 |  | 28 | 18 | 6,029,312 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 7 | 262,144 |  | 28 | 18 | 12,058,624 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 8 | 262,144 |  | 28 | 18 | 12,058,624 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 9 | 262,144 |  | 28 | 18 | 12,058,624 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 3 | 256 |  | 192 | 168 | 92,160 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 4 | 16,384 |  | 192 | 168 | 5,898,240 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 5 | 16,384 |  | 192 | 168 | 5,898,240 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 6 | 16,384 |  | 192 | 168 | 5,898,240 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 7 | 16,384 |  | 192 | 168 | 5,898,240 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 8 | 2,048 |  | 192 | 168 | 737,280 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21000000 | 9 | 1 |  | 192 | 168 | 360 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 3 | 128 |  | 68 | 169 | 30,336 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 4 | 4,096 |  | 68 | 169 | 970,752 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 5 | 8,192 |  | 68 | 169 | 1,941,504 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 6 | 4,096 |  | 68 | 169 | 970,752 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21000000 | 7 | 4,096 |  | 68 | 169 | 970,752 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 4 | 1,024 |  | 192 | 164 | 364,544 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 5 | 2,048 |  | 192 | 164 | 729,088 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 6 | 512 |  | 192 | 164 | 182,272 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21000000 | 7 | 512 |  | 192 | 164 | 182,272 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 4 | 2,048 |  | 164 | 241 | 829,440 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 5 | 2,048 |  | 164 | 241 | 829,440 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 6 | 2,048 |  | 164 | 241 | 829,440 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21000000 | 7 | 2,048 |  | 164 | 241 | 829,440 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 3 | 64 |  | 48 | 124 | 11,008 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 4 | 16,384 |  | 48 | 124 | 2,818,048 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 5 | 32,768 |  | 48 | 124 | 5,636,096 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 6 | 16,384 |  | 48 | 124 | 2,818,048 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 7 | 16,384 |  | 48 | 124 | 2,818,048 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 8 | 1,024 |  | 48 | 124 | 176,128 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 9 | 256 |  | 48 | 124 | 44,032 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 3 | 65,536 |  | 56 | 166 | 14,548,992 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 4 | 65,536 |  | 56 | 166 | 14,548,992 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 5 | 2,048 |  | 56 | 166 | 454,656 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 6 | 4,096 |  | 56 | 166 | 909,312 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21000000 | 7 | 4,096 |  | 56 | 166 | 909,312 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 0 | 1,048,576 |  | 36 | 28 | 67,108,864 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 1 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 10 | 131,072 |  | 36 | 28 | 8,388,608 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 2 | 262,144 |  | 36 | 28 | 16,777,216 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 3 | 131,072 |  | 36 | 28 | 8,388,608 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 4 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 5 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 6 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 7 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 8 | 262,144 |  | 36 | 28 | 16,777,216 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 9 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 0 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 1 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 10 | 1,024 |  | 52 | 36 | 90,112 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 2 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 3 | 131,072 |  | 52 | 36 | 11,534,336 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 4 | 262,144 |  | 52 | 36 | 23,068,672 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 5 | 65,536 |  | 52 | 36 | 5,767,168 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 6 | 65,536 |  | 52 | 36 | 5,767,168 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 7 | 65,536 |  | 52 | 36 | 5,767,168 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 8 | 524,288 |  | 52 | 36 | 46,137,344 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 9 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 0 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 1 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 10 | 524,288 |  | 52 | 41 | 48,758,784 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 2 | 4,194,304 |  | 52 | 41 | 390,070,272 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 3 | 4,194,304 |  | 52 | 41 | 390,070,272 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 4 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 5 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 6 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 7 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 8 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 9 | 4,194,304 |  | 52 | 41 | 390,070,272 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 4 | 256 |  | 72 | 59 | 33,536 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 5 | 256 |  | 72 | 59 | 33,536 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 6 | 256 |  | 72 | 59 | 33,536 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 7 | 512 |  | 72 | 59 | 67,072 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 8 | 256 |  | 72 | 59 | 33,536 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 9 | 256 |  | 72 | 59 | 33,536 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 0 | 1,024 |  | 72 | 39 | 113,664 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 1 | 16,384 |  | 72 | 39 | 1,818,624 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 10 | 4 |  | 72 | 39 | 444 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 2 | 1,024 |  | 72 | 39 | 113,664 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 3 | 65,536 |  | 72 | 39 | 7,274,496 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 4 | 131,072 |  | 72 | 39 | 14,548,992 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 5 | 131,072 |  | 72 | 39 | 14,548,992 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 6 | 131,072 |  | 72 | 39 | 14,548,992 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 7 | 131,072 |  | 72 | 39 | 14,548,992 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 8 | 65,536 |  | 72 | 39 | 7,274,496 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 9 | 4,096 |  | 72 | 39 | 454,656 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 0 | 8,192 |  | 52 | 31 | 679,936 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 1 | 16,384 |  | 52 | 31 | 1,359,872 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 10 | 4,096 |  | 52 | 31 | 339,968 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 2 | 8,192 |  | 52 | 31 | 679,936 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 3 | 65,536 |  | 52 | 31 | 5,439,488 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 4 | 262,144 |  | 52 | 31 | 21,757,952 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 5 | 262,144 |  | 52 | 31 | 21,757,952 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 6 | 262,144 |  | 52 | 31 | 21,757,952 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 7 | 262,144 |  | 52 | 31 | 21,757,952 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 8 | 65,536 |  | 52 | 31 | 5,439,488 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 9 | 16,384 |  | 52 | 31 | 1,359,872 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 0 | 524,288 |  | 28 | 20 | 25,165,824 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 1 | 262,144 |  | 28 | 20 | 12,582,912 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 10 | 65,536 |  | 28 | 20 | 3,145,728 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 2 | 131,072 |  | 28 | 20 | 6,291,456 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 3 | 65,536 |  | 28 | 20 | 3,145,728 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 4 | 131,072 |  | 28 | 20 | 6,291,456 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 5 | 65,536 |  | 28 | 20 | 3,145,728 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 6 | 65,536 |  | 28 | 20 | 3,145,728 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 7 | 131,072 |  | 28 | 20 | 6,291,456 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 8 | 131,072 |  | 28 | 20 | 6,291,456 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 9 | 131,072 |  | 28 | 20 | 6,291,456 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 3 | 32,768 |  | 836 | 547 | 45,318,144 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 32,768 |  | 836 | 547 | 45,318,144 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 5 | 1,024 |  | 836 | 547 | 1,416,192 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 6 | 2,048 |  | 836 | 547 | 2,832,384 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 7 | 2,048 |  | 836 | 547 | 2,832,384 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 3 | 512 |  | 320 | 263 | 298,496 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 512 |  | 320 | 263 | 298,496 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 5 | 16 |  | 320 | 263 | 9,328 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 6 | 32 |  | 320 | 263 | 18,656 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 7 | 32 |  | 320 | 263 | 18,656 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 3 | 16,384 |  | 860 | 625 | 24,330,240 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 16,384 |  | 860 | 625 | 24,330,240 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 5 | 512 |  | 860 | 625 | 760,320 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 6 | 1,024 |  | 860 | 625 | 1,520,640 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 7 | 1,024 |  | 860 | 625 | 1,520,640 | 
| reth.prove_evm.block_21000000 | VmConnectorAir | 21000000 | 0 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_evm.block_21000000 | VmConnectorAir | 21000000 | 1 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_evm.block_21000000 | VmConnectorAir | 21000000 | 10 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_evm.block_21000000 | VmConnectorAir | 21000000 | 2 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_evm.block_21000000 | VmConnectorAir | 21000000 | 3 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_evm.block_21000000 | VmConnectorAir | 21000000 | 4 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_evm.block_21000000 | VmConnectorAir | 21000000 | 5 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_evm.block_21000000 | VmConnectorAir | 21000000 | 6 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_evm.block_21000000 | VmConnectorAir | 21000000 | 7 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_evm.block_21000000 | VmConnectorAir | 21000000 | 8 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_evm.block_21000000 | VmConnectorAir | 21000000 | 9 | 2 | 1 | 16 | 5 | 42 | 

| group | block_number | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | num_segments | num_children | main_trace_commit_time_ms | main_cells_used | halo2_total_cells | halo2_keygen_time_ms | generate_perm_trace_time_ms | fri.log_blowup | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 366 | 31,992 | 1,622,307 | 270,872,042 | 31,293 | 677 | 11,762 | 5,642 | 4,021 | 1 |  | 8,966 | 86,455,016 | 8,037,489 | 19,077 | 207 |  | 333 | 
| halo2_outer | 21000000 |  | 52,515 |  |  |  |  |  |  |  |  |  |  | 66,137,544 |  |  |  |  |  | 
| halo2_wrapper | 21000000 |  | 38,850 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
| internal.0 | 21000000 |  |  |  |  |  |  |  |  |  |  | 3 |  |  |  |  |  | 2 |  | 
| internal.1 | 21000000 |  |  |  |  |  |  |  |  |  |  | 3 |  |  |  |  |  | 2 |  | 
| internal.2 | 21000000 |  |  |  |  |  |  |  |  |  |  | 3 |  |  |  |  |  | 2 |  | 
| leaf | 21000000 |  |  |  |  |  |  |  |  |  |  | 1 |  |  |  |  |  | 1 |  | 
| reth.prove_evm.block_21000000 | 21000000 |  |  |  |  |  |  |  |  |  | 11 |  |  |  |  |  |  | 1 |  | 

| group | block_number | cell_tracker_span | simple_advice_cells | lookup_advice_cells | fixed_cells |
| --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | VerifierProgram | 510,122 | 164,348 | 167,072 | 
| agg_keygen | 21000000 | VerifierProgram;CheckTraceHeightConstraints | 4,789 | 972 | 1,738 | 
| agg_keygen | 21000000 | VerifierProgram;PoseidonCell | 29,400 |  | 8,700 | 
| agg_keygen | 21000000 | VerifierProgram;stage-c-build-rounds | 19,535 | 2,717 | 6,699 | 
| agg_keygen | 21000000 | VerifierProgram;stage-c-build-rounds;PoseidonCell | 46,550 |  | 13,775 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs | 1,364,910 | 211,561 | 481,090 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;PoseidonCell | 3,839,150 |  | 1,136,075 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify | 42,110 | 4,540 | 18,340 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;PoseidonCell | 56,350 |  | 16,675 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;cache-generator-powers | 71,040 | 12,210 | 21,840 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;compute-reduced-opening;single-reduced-opening-eval | 8,565,510 | 359,310 | 1,587,990 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;pre-compute-rounds-context | 76,224 | 11,116 | 22,232 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch | 53,280 |  | 6,660 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;PoseidonCell | 9,926,550 |  | 2,940,300 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;verify-batch-reduce-fast;PoseidonCell | 8,854,140 | 253,980 | 2,764,710 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query | 1,107,690 | 190,530 | 313,530 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext | 109,440 |  | 13,680 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;PoseidonCell | 16,764,840 |  | 4,965,840 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;verify-batch-reduce-fast;PoseidonCell | 1,680,810 | 65,940 | 516,690 | 
| agg_keygen | 21000000 | VerifierProgram;stage-e-verify-constraints | 9,770,542 | 1,967,337 | 3,013,652 | 

| group | block_number | idx | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | fri.log_blowup | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | 21000000 | 0 | 1,704 | 12,336 | 3,499,736 | 472,992,226 | 8,543 | 982 | 1,719 | 1,235 | 2,444 | 1,666 | 208,468,362 | 492 |  | 2,089 | 
| internal.0 | 21000000 | 1 | 1,682 | 12,428 | 3,545,774 | 472,992,226 | 8,478 | 1,005 | 1,679 | 1,223 | 2,545 | 1,634 | 212,774,464 | 386 |  | 2,268 | 
| internal.0 | 21000000 | 2 | 1,670 | 12,314 | 3,530,613 | 472,992,226 | 8,487 | 983 | 1,674 | 1,226 | 2,514 | 1,643 | 211,604,707 | 443 |  | 2,157 | 
| internal.0 | 21000000 | 3 | 1,141 | 9,898 | 2,333,406 | 346,728,930 | 7,373 | 779 | 1,482 | 1,091 | 2,325 | 1,291 | 139,493,352 | 399 |  | 1,384 | 
| internal.1 | 21000000 | 4 | 1,007 | 8,735 | 2,338,776 | 302,819,810 | 6,685 | 686 | 1,393 | 938 | 2,040 | 1,198 | 115,072,118 | 425 |  | 1,043 | 
| internal.1 | 21000000 | 5 | 371 | 4,471 | 779,460 | 95,656,418 | 3,759 | 306 | 1,019 | 463 | 1,225 | 537 | 39,227,478 | 204 |  | 341 | 
| internal.2 | 21000000 | 6 | 676 | 6,581 | 1,542,899 | 186,591,714 | 5,243 | 478 | 1,252 | 685 | 1,671 | 844 | 75,829,710 | 308 |  | 662 | 
| leaf | 21000000 | 0 | 1,379 | 7,705 | 2,022,895 | 664,751,594 | 5,258 | 515 | 599 | 1,117 | 1,695 | 886 | 156,402,336 | 441 |  | 1,068 | 
| leaf | 21000000 | 1 | 1,378 | 7,964 | 2,102,450 | 732,909,034 | 5,517 | 558 | 595 | 1,153 | 1,783 | 966 | 159,298,398 | 455 |  | 1,069 | 
| leaf | 21000000 | 10 | 1,335 | 7,266 | 2,028,113 | 571,690,474 | 4,963 | 480 | 604 | 1,017 | 1,620 | 797 | 154,403,715 | 440 |  | 968 | 
| leaf | 21000000 | 2 | 1,353 | 7,304 | 2,041,949 | 571,690,474 | 4,941 | 474 | 599 | 1,054 | 1,677 | 761 | 155,641,018 | 370 |  | 1,010 | 
| leaf | 21000000 | 3 | 2,229 | 11,752 | 3,275,885 | 1,080,659,434 | 8,169 | 835 | 806 | 1,815 | 2,564 | 1,346 | 265,245,386 | 797 |  | 1,354 | 
| leaf | 21000000 | 4 | 2,352 | 11,971 | 3,518,565 | 1,100,058,090 | 8,169 | 829 | 854 | 1,832 | 2,600 | 1,382 | 284,541,932 | 667 |  | 1,450 | 
| leaf | 21000000 | 5 | 2,368 | 11,959 | 3,517,401 | 1,100,058,090 | 8,161 | 830 | 813 | 1,831 | 2,646 | 1,357 | 284,865,257 | 675 |  | 1,430 | 
| leaf | 21000000 | 6 | 2,342 | 12,075 | 3,517,587 | 1,100,058,090 | 8,280 | 835 | 886 | 1,852 | 2,625 | 1,381 | 284,711,456 | 694 |  | 1,453 | 
| leaf | 21000000 | 7 | 2,354 | 11,880 | 3,517,565 | 1,100,058,090 | 8,091 | 826 | 817 | 1,809 | 2,590 | 1,375 | 284,511,860 | 668 |  | 1,435 | 
| leaf | 21000000 | 8 | 1,507 | 8,257 | 2,320,064 | 746,278,378 | 5,662 | 577 | 602 | 1,167 | 1,820 | 1,021 | 174,398,761 | 468 |  | 1,088 | 
| leaf | 21000000 | 9 | 1,532 | 8,304 | 2,319,472 | 746,278,378 | 5,675 | 572 | 616 | 1,207 | 1,789 | 998 | 174,580,798 | 489 |  | 1,097 | 
| root | 21000000 | 0 | 359 | 31,556 | 772,492 | 80,435,354 | 30,869 | 660 | 11,662 | 5,644 | 3,717 | 8,974 | 38,540,365 | 204 | 3 | 328 | 

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
| internal.1 | 21000000 | 4 | 0 | 8,454,276 | 2,013,265,921 | 
| internal.1 | 21000000 | 4 | 1 | 40,132,864 | 2,013,265,921 | 
| internal.1 | 21000000 | 4 | 2 | 4,227,138 | 2,013,265,921 | 
| internal.1 | 21000000 | 4 | 3 | 40,386,820 | 2,013,265,921 | 
| internal.1 | 21000000 | 4 | 4 | 262,144 | 2,013,265,921 | 
| internal.1 | 21000000 | 4 | 5 | 93,856,458 | 2,013,265,921 | 
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
| leaf | 21000000 | 0 | 0 | 9,896,068 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 1 | 73,318,656 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 2 | 4,948,034 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 3 | 73,433,348 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 5 | 164,479,690 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 0 | 11,993,220 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 1 | 79,610,112 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 2 | 5,996,610 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 3 | 79,724,804 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 5 | 180,208,330 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 0 | 9,371,780 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 1 | 64,930,048 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 2 | 4,685,890 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 3 | 65,044,740 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 4 | 262,144 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 5 | 146,653,898 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 0 | 9,371,780 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 1 | 64,930,048 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 2 | 4,685,890 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 3 | 65,044,740 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 4 | 262,144 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 5 | 146,653,898 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 0 | 18,022,532 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 1 | 128,155,904 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 2 | 9,011,266 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 3 | 128,254,212 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 5 | 286,327,498 | 2,013,265,921 | 
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
| leaf | 21000000 | 8 | 0 | 12,517,508 | 2,013,265,921 | 
| leaf | 21000000 | 8 | 1 | 80,658,688 | 2,013,265,921 | 
| leaf | 21000000 | 8 | 2 | 6,258,754 | 2,013,265,921 | 
| leaf | 21000000 | 8 | 3 | 80,773,380 | 2,013,265,921 | 
| leaf | 21000000 | 8 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 8 | 5 | 183,091,914 | 2,013,265,921 | 
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
| agg_keygen | 21000000 | 0 | 25 | 821 |  | 7,747,601 | 796 | 35 | 106 | 100 | 429 | 82 | 10,556 | 39 | 0 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 10,787 | 30,362 | 22,213,839 | 2,755,189,874 | 16,527 | 2,016 | 1,477 | 3,326 | 4,476 | 3,901 | 1,193,859,829 | 1,317 | 3,048 | 
| reth.prove_evm.block_21000000 | 21000000 | 1 | 12,021 | 34,032 | 22,034,665 | 3,363,192,910 | 18,959 | 2,635 | 1,410 | 3,588 | 4,876 | 5,027 | 1,401,279,115 | 1,411 | 3,052 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 2,578 | 6,922 | 1,161,202 | 384,223,224 | 4,168 | 374 | 530 | 560 | 1,545 | 920 | 210,978,086 | 222 | 176 | 
| reth.prove_evm.block_21000000 | 21000000 | 2 | 6,980 | 30,540 | 10,118,483 | 3,817,479,228 | 22,049 | 5,428 | 1,076 | 3,207 | 3,494 | 7,684 | 1,575,592,495 | 1,149 | 1,511 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 5,456 | 29,525 | 7,954,659 | 3,559,341,482 | 21,520 | 5,346 | 1,167 | 3,358 | 3,537 | 7,011 | 1,390,234,523 | 1,086 | 2,549 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 7,494 | 27,341 | 17,818,723 | 2,422,873,898 | 16,098 | 1,948 | 1,528 | 3,294 | 4,650 | 3,337 | 884,519,320 | 1,321 | 3,749 | 
| reth.prove_evm.block_21000000 | 21000000 | 5 | 7,140 | 24,367 | 16,614,298 | 2,226,688,458 | 14,813 | 1,689 | 1,387 | 3,119 | 4,390 | 2,891 | 745,227,341 | 1,318 | 2,414 | 
| reth.prove_evm.block_21000000 | 21000000 | 6 | 7,264 | 24,283 | 16,707,117 | 2,197,377,242 | 14,553 | 1,707 | 1,320 | 3,102 | 4,359 | 2,913 | 761,922,861 | 1,138 | 2,466 | 
| reth.prove_evm.block_21000000 | 21000000 | 7 | 7,543 | 24,802 | 16,902,653 | 2,214,796,138 | 14,774 | 1,695 | 1,372 | 3,100 | 4,355 | 2,978 | 768,071,087 | 1,255 | 2,485 | 
| reth.prove_evm.block_21000000 | 21000000 | 8 | 7,738 | 30,198 | 13,446,938 | 3,447,403,450 | 19,924 | 3,778 | 1,330 | 3,508 | 4,395 | 5,606 | 1,483,130,912 | 1,298 | 2,536 | 
| reth.prove_evm.block_21000000 | 21000000 | 9 | 8,040 | 30,254 | 12,087,983 | 3,435,263,652 | 20,241 | 3,722 | 1,228 | 3,259 | 4,709 | 6,193 | 1,591,332,740 | 1,118 | 1,973 | 

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
| reth.prove_evm.block_21000000 | 21000000 | 0 | 0 | 52,480,012 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 1 | 148,330,496 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 2 | 26,240,006 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 3 | 175,593,476 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 4 | 7,340,032 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 5 | 3,145,728 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 6 | 62,030,848 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 7 |  | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 8 | 40,960 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 9 | 479,264,790 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 1 | 0 | 53,149,704 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 1 | 1 | 158,810,112 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 1 | 2 | 26,574,852 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 1 | 3 | 179,208,196 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 1 | 4 | 8,388,608 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 1 | 5 | 4,194,304 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 1 | 6 | 68,567,040 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 1 | 7 |  | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 1 | 8 | 196,608 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 1 | 9 | 504,201,232 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 0 | 3,047,438 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 1 | 9,173,528 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 2 | 1,523,719 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 3 | 10,244,124 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 4 | 3,407,872 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 5 | 1,310,720 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 6 | 3,183,108 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 7 |  | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 8 | 16,416 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 9 | 35,445,869 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 2 | 0 | 27,281,414 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 2 | 1 | 124,442,624 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 2 | 2 | 13,640,707 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 2 | 3 | 135,190,532 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 2 | 4 | 7,340,032 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 2 | 5 | 3,145,728 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 2 | 6 | 97,649,664 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 2 | 7 |  | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 2 | 8 | 40,960 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 2 | 9 | 412,794,893 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 0 | 23,436,932 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 1 | 115,388,416 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 2 | 11,718,466 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 3 | 144,250,116 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 6 | 97,862,208 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 7 |  | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 8 | 786,432 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 9 | 403,272,970 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 0 | 45,470,980 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 1 | 139,235,840 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 2 | 22,735,490 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 3 | 186,588,932 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 4 | 2,097,152 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 5 | 1,048,576 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 6 | 60,407,808 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 7 |  | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 8 | 2,131,968 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 9 | 462,993,546 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 5 | 0 | 45,514,460 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 5 | 1 | 137,794,016 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 5 | 2 | 22,757,230 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 5 | 3 | 166,309,900 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 5 | 4 | 1,048,576 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 5 | 5 | 524,288 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 5 | 6 | 58,835,696 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 5 | 7 |  | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 5 | 8 | 2,164,736 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 5 | 9 | 438,094,630 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 6 | 0 | 44,429,268 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 6 | 1 | 134,508,160 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 6 | 2 | 22,214,634 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 6 | 3 | 163,618,532 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 6 | 4 | 1,835,008 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 6 | 5 | 786,432 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 6 | 6 | 57,233,408 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 6 | 7 |  | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 6 | 8 | 2,115,584 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 6 | 9 | 429,886,754 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 7 | 0 | 44,822,964 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 7 | 1 | 135,146,432 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 7 | 2 | 22,411,482 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 7 | 3 | 163,999,764 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 7 | 4 | 2,097,152 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 7 | 5 | 1,048,576 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 7 | 6 | 57,954,784 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 7 | 7 |  | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 7 | 8 | 2,117,632 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 7 | 9 | 432,744,514 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 8 | 0 | 44,046,868 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 8 | 1 | 152,356,352 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 8 | 2 | 22,023,434 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 8 | 3 | 171,836,932 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 8 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 8 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 8 | 6 | 84,678,144 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 8 | 7 |  | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 8 | 8 | 788,480 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 8 | 9 | 486,084,898 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 9 | 0 | 35,955,720 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 9 | 1 | 127,926,284 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 9 | 2 | 17,977,860 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 9 | 3 | 140,376,592 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 9 | 4 | 7,340,032 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 9 | 5 | 3,145,728 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 9 | 6 | 83,235,618 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 9 | 7 |  | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 9 | 8 | 100,352 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 9 | 9 | 421,169,994 | 2,013,265,921 | 

| group | block_number | trace_height_constraint | weighted_sum | threshold |
| --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 0 | 5,701,764 | 2,013,265,921 | 
| agg_keygen | 21000000 | 1 | 28,467,456 | 2,013,265,921 | 
| agg_keygen | 21000000 | 2 | 2,850,882 | 2,013,265,921 | 
| agg_keygen | 21000000 | 3 | 28,197,124 | 2,013,265,921 | 
| agg_keygen | 21000000 | 4 | 262,144 | 2,013,265,921 | 
| agg_keygen | 21000000 | 5 | 65,741,514 | 2,013,265,921 | 

</details>


Commit: https://github.com/axiom-crypto/openvm-reth-benchmark/commit/d8ef0e2e2d6f7c92add1581ae7d56f7bf6bbe6d9

Max Segment Length: 8388508

Instance Type: m8g.24xlarge

Memory Allocator: jemalloc

[Benchmark Workflow](https://github.com/axiom-crypto/openvm-reth-benchmark/actions/runs/16370061883)
