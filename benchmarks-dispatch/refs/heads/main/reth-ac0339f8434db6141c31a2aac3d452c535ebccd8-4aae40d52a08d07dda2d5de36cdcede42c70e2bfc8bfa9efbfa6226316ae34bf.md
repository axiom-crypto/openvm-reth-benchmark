| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total |  502.85 |  179.21 |
| reth.prove_e2e.block_22000107 |  261.02 |  43.01 |
| leaf |  79.94 |  10.18 |
| internal.0 |  34.14 |  7.72 |
| internal.1 |  11.09 |  4.25 |
| internal.2 |  6.82 |  4.20 |
| internal.3 |  4.21 |  4.21 |
| root |  22.91 |  22.91 |
| halo2_outer |  47.60 |  47.60 |
| halo2_wrapper |  35.13 |  35.13 |
| agg_keygen |  23.29 |  22.86 |


| reth.prove_e2e.block_22000107 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  29,002.67 |  261,024 |  43,009 |  23,928 |
| `main_cells_used     ` |  1,246,805,443.56 |  11,221,248,992 |  2,277,774,551 |  845,833,748 |
| `total_cycles        ` |  20,631,037 |  185,679,333 |  23,376,955 |  12,870,127 |
| `execute_time_ms     ` |  3,099.56 |  27,896 |  5,002 |  1,769 |
| `trace_gen_time_ms   ` |  9,102.44 |  81,922 |  12,425 |  7,081 |
| `stark_prove_excluding_trace_time_ms` |  16,800.67 |  151,206 |  26,844 |  13,770 |
| `main_trace_commit_time_ms` |  3,887.22 |  34,985 |  8,234 |  2,484 |
| `generate_perm_trace_time_ms` |  906.33 |  8,157 |  1,186 |  772 |
| `perm_trace_commit_time_ms` |  4,251.78 |  38,266 |  5,851 |  3,736 |
| `quotient_poly_compute_time_ms` |  3,076.56 |  27,689 |  6,187 |  2,105 |
| `quotient_poly_commit_time_ms` |  1,337.33 |  12,036 |  1,486 |  1,190 |
| `pcs_opening_time_ms ` |  3,327.44 |  29,947 |  3,887 |  2,993 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  8,882.33 |  79,941 |  10,181 |  6,210 |
| `main_cells_used     ` |  240,606,924.78 |  2,165,462,323 |  279,760,082 |  158,012,099 |
| `total_cycles        ` |  3,012,499.78 |  27,112,498 |  3,458,727 |  2,040,031 |
| `execute_time_ms     ` |  988 |  8,892 |  1,111 |  737 |
| `trace_gen_time_ms   ` |  1,824.33 |  16,419 |  2,156 |  1,259 |
| `stark_prove_excluding_trace_time_ms` |  6,070 |  54,630 |  6,934 |  4,214 |
| `main_trace_commit_time_ms` |  936.67 |  8,430 |  1,066 |  655 |
| `generate_perm_trace_time_ms` |  355.22 |  3,197 |  410 |  237 |
| `perm_trace_commit_time_ms` |  1,800.56 |  16,205 |  2,083 |  1,212 |
| `quotient_poly_compute_time_ms` |  890.22 |  8,012 |  1,017 |  621 |
| `quotient_poly_commit_time_ms` |  652.78 |  5,875 |  749 |  464 |
| `pcs_opening_time_ms ` |  1,429.56 |  12,866 |  1,663 |  1,021 |

| internal.0 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  6,828 |  34,140 |  7,723 |  4,271 |
| `main_cells_used     ` |  128,320,454.40 |  641,602,272 |  143,302,521 |  70,912,770 |
| `total_cycles        ` |  2,146,253.60 |  10,731,268 |  2,395,460 |  1,181,959 |
| `execute_time_ms     ` |  844 |  4,220 |  956 |  454 |
| `trace_gen_time_ms   ` |  960.20 |  4,801 |  1,086 |  554 |
| `stark_prove_excluding_trace_time_ms` |  5,023.80 |  25,119 |  5,698 |  3,263 |
| `main_trace_commit_time_ms` |  941.40 |  4,707 |  1,139 |  541 |
| `generate_perm_trace_time_ms` |  179.60 |  898 |  218 |  112 |
| `perm_trace_commit_time_ms` |  836.60 |  4,183 |  965 |  503 |
| `quotient_poly_compute_time_ms` |  832.20 |  4,161 |  965 |  514 |
| `quotient_poly_commit_time_ms` |  990.20 |  4,951 |  1,079 |  718 |
| `pcs_opening_time_ms ` |  1,239.40 |  6,197 |  1,345 |  871 |

| internal.1 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  3,695 |  11,085 |  4,252 |  2,583 |
| `main_cells_used     ` |  62,716,203.33 |  188,148,610 |  75,228,045 |  37,724,070 |
| `total_cycles        ` |  1,272,797.33 |  3,818,392 |  1,530,363 |  757,736 |
| `execute_time_ms     ` |  361.67 |  1,085 |  437 |  211 |
| `trace_gen_time_ms   ` |  504.33 |  1,513 |  601 |  318 |
| `stark_prove_excluding_trace_time_ms` |  2,829 |  8,487 |  3,219 |  2,054 |
| `main_trace_commit_time_ms` |  464.67 |  1,394 |  537 |  325 |
| `generate_perm_trace_time_ms` |  86.67 |  260 |  107 |  53 |
| `perm_trace_commit_time_ms` |  426 |  1,278 |  497 |  295 |
| `quotient_poly_compute_time_ms` |  419.67 |  1,259 |  493 |  277 |
| `quotient_poly_commit_time_ms` |  638.67 |  1,916 |  714 |  494 |
| `pcs_opening_time_ms ` |  788.67 |  2,366 |  887 |  606 |

| internal.2 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  3,410 |  6,820 |  4,201 |  2,619 |
| `main_cells_used     ` |  55,441,141 |  110,882,282 |  73,817,354 |  37,064,928 |
| `total_cycles        ` |  1,132,564 |  2,265,128 |  1,515,046 |  750,082 |
| `execute_time_ms     ` |  309.50 |  619 |  415 |  204 |
| `trace_gen_time_ms   ` |  454 |  908 |  598 |  310 |
| `stark_prove_excluding_trace_time_ms` |  2,646.50 |  5,293 |  3,188 |  2,105 |
| `main_trace_commit_time_ms` |  425 |  850 |  533 |  317 |
| `generate_perm_trace_time_ms` |  78 |  156 |  94 |  62 |
| `perm_trace_commit_time_ms` |  404.50 |  809 |  496 |  313 |
| `quotient_poly_compute_time_ms` |  384.50 |  769 |  490 |  279 |
| `quotient_poly_commit_time_ms` |  595 |  1,190 |  700 |  490 |
| `pcs_opening_time_ms ` |  755 |  1,510 |  871 |  639 |

| internal.3 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  4,206 |  4,206 |  4,206 |  4,206 |
| `main_cells_used     ` |  73,292,109 |  73,292,109 |  73,292,109 |  73,292,109 |
| `total_cycles        ` |  1,507,270 |  1,507,270 |  1,507,270 |  1,507,270 |
| `execute_time_ms     ` |  409 |  409 |  409 |  409 |
| `trace_gen_time_ms   ` |  607 |  607 |  607 |  607 |
| `stark_prove_excluding_trace_time_ms` |  3,190 |  3,190 |  3,190 |  3,190 |
| `main_trace_commit_time_ms` |  531 |  531 |  531 |  531 |
| `generate_perm_trace_time_ms` |  95 |  95 |  95 |  95 |
| `perm_trace_commit_time_ms` |  488 |  488 |  488 |  488 |
| `quotient_poly_compute_time_ms` |  493 |  493 |  493 |  493 |
| `quotient_poly_commit_time_ms` |  705 |  705 |  705 |  705 |
| `pcs_opening_time_ms ` |  874 |  874 |  874 |  874 |

| root |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  22,909 |  22,909 |  22,909 |  22,909 |
| `main_cells_used     ` |  37,731,915 |  37,731,915 |  37,731,915 |  37,731,915 |
| `total_cycles        ` |  758,315 |  758,315 |  758,315 |  758,315 |
| `execute_time_ms     ` |  210 |  210 |  210 |  210 |
| `trace_gen_time_ms   ` |  322 |  322 |  322 |  322 |
| `stark_prove_excluding_trace_time_ms` |  22,377 |  22,377 |  22,377 |  22,377 |
| `main_trace_commit_time_ms` |  6,868 |  6,868 |  6,868 |  6,868 |
| `generate_perm_trace_time_ms` |  52 |  52 |  52 |  52 |
| `perm_trace_commit_time_ms` |  4,286 |  4,286 |  4,286 |  4,286 |
| `quotient_poly_compute_time_ms` |  560 |  560 |  560 |  560 |
| `quotient_poly_commit_time_ms` |  8,196 |  8,196 |  8,196 |  8,196 |
| `pcs_opening_time_ms ` |  2,409 |  2,409 |  2,409 |  2,409 |

| halo2_outer |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  47,598 |  47,598 |  47,598 |  47,598 |
| `main_cells_used     ` |  61,870,615 |  61,870,615 |  61,870,615 |  61,870,615 |

| halo2_wrapper |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  35,127 |  35,127 |  35,127 |  35,127 |

| agg_keygen |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  11,645.50 |  23,291 |  22,859 |  432 |
| `main_cells_used     ` |  44,006,085.50 |  88,012,171 |  87,084,100 |  928,071 |
| `total_cycles        ` |  1,636,535 |  1,636,535 |  1,636,535 |  1,636,535 |
| `execute_time_ms     ` |  104 |  208 |  208 |  0 |
| `trace_gen_time_ms   ` |  166.50 |  333 |  306 |  27 |
| `stark_prove_excluding_trace_time_ms` |  11,375 |  22,750 |  22,345 |  405 |
| `main_trace_commit_time_ms` |  3,463 |  6,926 |  6,875 |  51 |
| `generate_perm_trace_time_ms` |  32 |  64 |  50 |  14 |
| `perm_trace_commit_time_ms` |  2,159 |  4,318 |  4,268 |  50 |
| `quotient_poly_compute_time_ms` |  296.50 |  593 |  564 |  29 |
| `quotient_poly_commit_time_ms` |  4,078 |  8,156 |  8,099 |  57 |
| `pcs_opening_time_ms ` |  1,341.50 |  2,683 |  2,483 |  200 |



<details>
<summary>Detailed Metrics</summary>

| air_name | block_number | quotient_deg | interactions | constraints |
| --- | --- | --- | --- | --- |
| AccessAdapterAir<16> | 22000107 | 2 | 5 | 12 | 
| AccessAdapterAir<2> | 22000107 | 2 | 5 | 12 | 
| AccessAdapterAir<32> | 22000107 | 2 | 5 | 12 | 
| AccessAdapterAir<4> | 22000107 | 2 | 5 | 12 | 
| AccessAdapterAir<8> | 22000107 | 2 | 5 | 12 | 
| BitwiseOperationLookupAir<8> | 22000107 | 2 | 2 | 4 | 
| KeccakVmAir | 22000107 | 2 | 321 | 4,513 | 
| MemoryMerkleAir<8> | 22000107 | 2 | 4 | 39 | 
| PersistentBoundaryAir<8> | 22000107 | 2 | 3 | 7 | 
| PhantomAir | 22000107 | 2 | 3 | 5 | 
| Poseidon2PeripheryAir<BabyBearParameters>, 1> | 22000107 | 2 | 1 | 286 | 
| ProgramAir | 22000107 | 1 | 1 | 4 | 
| RangeTupleCheckerAir<2> | 22000107 | 1 | 1 | 4 | 
| Rv32HintStoreAir | 22000107 | 2 | 18 | 28 | 
| Sha256VmAir | 22000107 | 2 | 50 | 663 | 
| VariableRangeCheckerAir | 22000107 | 1 | 1 | 4 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 22000107 | 2 | 20 | 37 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 22000107 | 2 | 18 | 40 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 22000107 | 2 | 24 | 91 | 
| VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 22000107 | 2 | 11 | 20 | 
| VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 22000107 | 2 | 13 | 35 | 
| VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 22000107 | 2 | 10 | 18 | 
| VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 22000107 | 2 | 61 | 126 | 
| VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 22000107 | 2 | 31 | 129 | 
| VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 22000107 | 2 | 61 | 57 | 
| VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 22000107 | 2 | 79 | 2,161 | 
| VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 22000107 | 2 | 20 | 55 | 
| VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchLessThanCoreAir<32, 8> | 22000107 | 2 | 22 | 126 | 
| VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 22000107 | 2 | 25 | 225 | 
| VmAirWrapper<Rv32IsEqualModAdapterAir<2, 3, 16, 48>, ModularIsEqualCoreAir<48, 4, 8> | 22000107 | 2 | 41 | 333 | 
| VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 22000107 | 2 | 16 | 20 | 
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 22000107 | 2 | 18 | 33 | 
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 22000107 | 2 | 17 | 40 | 
| VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 22000107 | 2 | 25 | 84 | 
| VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 22000107 | 2 | 24 | 31 | 
| VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 22000107 | 2 | 19 | 19 | 
| VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 22000107 | 2 | 12 | 14 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 22000107 | 2 | 415 | 480 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 6, 6, 16, 16>, FieldExpressionCoreAir> | 22000107 | 2 | 832 | 921 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 22000107 | 2 | 158 | 190 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 22000107 | 2 | 428 | 457 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 3, 3, 16, 16>, FieldExpressionCoreAir> | 22000107 | 2 | 246 | 288 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 6, 6, 16, 16>, FieldExpressionCoreAir> | 22000107 | 2 | 668 | 701 | 
| VmConnectorAir | 22000107 | 2 | 5 | 10 | 

| block_number | execute_time_ms |
| --- | --- |
| 22000107 | 209 | 

| group | air_name | block_number | rows | quotient_deg | prep_cols | perm_cols | main_cols | interactions | constraints | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | AccessAdapterAir<16> | 22000107 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<2> | 22000107 | 524,288 | 8 |  | 16 | 11 | 5 | 12 | 14,155,776 | 
| agg_keygen | AccessAdapterAir<32> | 22000107 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<4> | 22000107 | 262,144 | 8 |  | 16 | 13 | 5 | 12 | 7,602,176 | 
| agg_keygen | AccessAdapterAir<8> | 22000107 | 8,192 | 8 |  | 16 | 17 | 5 | 12 | 270,336 | 
| agg_keygen | BitwiseOperationLookupAir<8> | 22000107 |  | 2 |  |  |  | 2 | 4 |  | 
| agg_keygen | FriReducedOpeningAir | 22000107 | 524,288 | 8 |  | 84 | 27 | 39 | 71 | 58,195,968 | 
| agg_keygen | JalRangeCheckAir | 22000107 | 65,536 | 8 |  | 28 | 12 | 9 | 14 | 2,621,440 | 
| agg_keygen | MemoryMerkleAir<8> | 22000107 |  | 2 |  |  |  | 4 | 39 |  | 
| agg_keygen | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 65,536 | 8 |  | 312 | 398 | 136 | 572 | 46,530,560 | 
| agg_keygen | PersistentBoundaryAir<8> | 22000107 |  | 2 |  |  |  | 3 | 7 |  | 
| agg_keygen | PhantomAir | 22000107 | 32,768 | 4 |  | 12 | 6 | 3 | 5 | 589,824 | 
| agg_keygen | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 22000107 |  | 2 |  |  |  | 1 | 286 |  | 
| agg_keygen | ProgramAir | 22000107 | 131,072 | 1 |  | 8 | 10 | 1 | 4 | 2,359,296 | 
| agg_keygen | RangeTupleCheckerAir<2> | 22000107 |  | 1 |  |  |  | 1 | 4 |  | 
| agg_keygen | Rv32HintStoreAir | 22000107 |  | 2 |  |  |  | 18 | 28 |  | 
| agg_keygen | VariableRangeCheckerAir | 22000107 | 262,144 | 1 | 2 | 8 | 1 | 1 | 4 | 2,359,296 | 
| agg_keygen | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 1,048,576 | 8 |  | 36 | 29 | 15 | 27 | 68,157,440 | 
| agg_keygen | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 262,144 | 8 |  | 28 | 23 | 11 | 25 | 13,369,344 | 
| agg_keygen | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 64 | 8 |  | 28 | 27 | 11 | 30 | 3,520 | 
| agg_keygen | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 524,288 | 8 |  | 40 | 21 | 15 | 20 | 31,981,568 | 
| agg_keygen | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 131,072 | 8 |  | 40 | 27 | 15 | 20 | 8,781,824 | 
| agg_keygen | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 131,072 | 8 |  | 36 | 38 | 15 | 27 | 9,699,328 | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 22000107 |  | 2 |  |  |  | 20 | 37 |  | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 22000107 |  | 2 |  |  |  | 18 | 40 |  | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 22000107 |  | 2 |  |  |  | 24 | 91 |  | 
| agg_keygen | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 22000107 |  | 2 |  |  |  | 11 | 20 |  | 
| agg_keygen | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 22000107 |  | 2 |  |  |  | 13 | 35 |  | 
| agg_keygen | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 22000107 |  | 2 |  |  |  | 10 | 18 |  | 
| agg_keygen | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 22000107 |  | 2 |  |  |  | 16 | 20 |  | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 22000107 |  | 2 |  |  |  | 18 | 33 |  | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 22000107 |  | 2 |  |  |  | 17 | 40 |  | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 22000107 |  | 2 |  |  |  | 25 | 84 |  | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 22000107 |  | 2 |  |  |  | 24 | 31 |  | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 22000107 |  | 2 |  |  |  | 19 | 19 |  | 
| agg_keygen | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 22000107 |  | 2 |  |  |  | 12 | 14 |  | 
| agg_keygen | VmConnectorAir | 22000107 | 2 | 8 | 1 | 16 | 5 | 5 | 10 | 42 | 
| agg_keygen | VolatileBoundaryAir | 22000107 | 131,072 | 4 |  | 12 | 11 | 4 | 17 | 3,014,656 | 

| group | air_name | block_number | idx | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | AccessAdapterAir<2> | 22000107 | 0 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 22000107 | 1 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 22000107 | 2 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 22000107 | 3 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 22000107 | 4 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<4> | 22000107 | 0 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 22000107 | 1 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 22000107 | 2 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 22000107 | 3 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 22000107 | 4 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<8> | 22000107 | 0 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.0 | AccessAdapterAir<8> | 22000107 | 1 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.0 | AccessAdapterAir<8> | 22000107 | 2 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.0 | AccessAdapterAir<8> | 22000107 | 3 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.0 | AccessAdapterAir<8> | 22000107 | 4 | 4,096 |  | 12 | 17 | 118,784 | 
| internal.0 | FriReducedOpeningAir | 22000107 | 0 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | FriReducedOpeningAir | 22000107 | 1 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | FriReducedOpeningAir | 22000107 | 2 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | FriReducedOpeningAir | 22000107 | 3 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| internal.0 | FriReducedOpeningAir | 22000107 | 4 | 524,288 |  | 44 | 27 | 37,224,448 | 
| internal.0 | JalRangeCheckAir | 22000107 | 0 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | JalRangeCheckAir | 22000107 | 1 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | JalRangeCheckAir | 22000107 | 2 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | JalRangeCheckAir | 22000107 | 3 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | JalRangeCheckAir | 22000107 | 4 | 65,536 |  | 16 | 12 | 1,835,008 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 0 | 131,072 |  | 160 | 398 | 73,138,176 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 1 | 131,072 |  | 160 | 398 | 73,138,176 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 2 | 262,144 |  | 160 | 398 | 146,276,352 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 3 | 262,144 |  | 160 | 398 | 146,276,352 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 4 | 65,536 |  | 160 | 398 | 36,569,088 | 
| internal.0 | PhantomAir | 22000107 | 0 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 22000107 | 1 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 22000107 | 2 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 22000107 | 3 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 22000107 | 4 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.0 | ProgramAir | 22000107 | 0 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 22000107 | 1 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 22000107 | 2 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 22000107 | 3 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | ProgramAir | 22000107 | 4 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 22000107 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 22000107 | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 22000107 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 22000107 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 22000107 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 0 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 1 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 2 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 3 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 4 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 0 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 1 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 2 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 3 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 4 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 0 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 1 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 2 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 3 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 4 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 0 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 1 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 2 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 3 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 4 | 262,144 |  | 24 | 21 | 11,796,480 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 0 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 1 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 2 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 3 | 262,144 |  | 24 | 27 | 13,369,344 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 4 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 0 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 1 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 2 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 3 | 262,144 |  | 20 | 38 | 15,204,352 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 4 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmConnectorAir | 22000107 | 0 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VmConnectorAir | 22000107 | 1 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VmConnectorAir | 22000107 | 2 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VmConnectorAir | 22000107 | 3 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VmConnectorAir | 22000107 | 4 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VolatileBoundaryAir | 22000107 | 0 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.0 | VolatileBoundaryAir | 22000107 | 1 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.0 | VolatileBoundaryAir | 22000107 | 2 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.0 | VolatileBoundaryAir | 22000107 | 3 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.0 | VolatileBoundaryAir | 22000107 | 4 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.1 | AccessAdapterAir<2> | 22000107 | 5 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.1 | AccessAdapterAir<2> | 22000107 | 6 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.1 | AccessAdapterAir<2> | 22000107 | 7 | 262,144 |  | 12 | 11 | 6,029,312 | 
| internal.1 | AccessAdapterAir<4> | 22000107 | 5 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.1 | AccessAdapterAir<4> | 22000107 | 6 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.1 | AccessAdapterAir<4> | 22000107 | 7 | 131,072 |  | 12 | 13 | 3,276,800 | 
| internal.1 | AccessAdapterAir<8> | 22000107 | 5 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.1 | AccessAdapterAir<8> | 22000107 | 6 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.1 | AccessAdapterAir<8> | 22000107 | 7 | 4,096 |  | 12 | 17 | 118,784 | 
| internal.1 | FriReducedOpeningAir | 22000107 | 5 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.1 | FriReducedOpeningAir | 22000107 | 6 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.1 | FriReducedOpeningAir | 22000107 | 7 | 131,072 |  | 44 | 27 | 9,306,112 | 
| internal.1 | JalRangeCheckAir | 22000107 | 5 | 65,536 |  | 16 | 12 | 1,835,008 | 
| internal.1 | JalRangeCheckAir | 22000107 | 6 | 65,536 |  | 16 | 12 | 1,835,008 | 
| internal.1 | JalRangeCheckAir | 22000107 | 7 | 32,768 |  | 16 | 12 | 917,504 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 5 | 65,536 |  | 160 | 398 | 36,569,088 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 6 | 65,536 |  | 160 | 398 | 36,569,088 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 7 | 32,768 |  | 160 | 398 | 18,284,544 | 
| internal.1 | PhantomAir | 22000107 | 5 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.1 | PhantomAir | 22000107 | 6 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.1 | PhantomAir | 22000107 | 7 | 8,192 |  | 8 | 6 | 114,688 | 
| internal.1 | ProgramAir | 22000107 | 5 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.1 | ProgramAir | 22000107 | 6 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.1 | ProgramAir | 22000107 | 7 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 22000107 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 22000107 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 22000107 | 7 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 5 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 6 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 7 | 524,288 |  | 20 | 29 | 25,690,112 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 5 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 6 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 7 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 5 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 6 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 7 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 5 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 6 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 7 | 262,144 |  | 24 | 21 | 11,796,480 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 5 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 6 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 7 | 65,536 |  | 24 | 27 | 3,342,336 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 5 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 6 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 7 | 65,536 |  | 20 | 38 | 3,801,088 | 
| internal.1 | VmConnectorAir | 22000107 | 5 | 2 | 1 | 12 | 5 | 34 | 
| internal.1 | VmConnectorAir | 22000107 | 6 | 2 | 1 | 12 | 5 | 34 | 
| internal.1 | VmConnectorAir | 22000107 | 7 | 2 | 1 | 12 | 5 | 34 | 
| internal.1 | VolatileBoundaryAir | 22000107 | 5 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.1 | VolatileBoundaryAir | 22000107 | 6 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.1 | VolatileBoundaryAir | 22000107 | 7 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.2 | AccessAdapterAir<2> | 22000107 | 8 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.2 | AccessAdapterAir<2> | 22000107 | 9 | 262,144 |  | 12 | 11 | 6,029,312 | 
| internal.2 | AccessAdapterAir<4> | 22000107 | 8 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.2 | AccessAdapterAir<4> | 22000107 | 9 | 131,072 |  | 12 | 13 | 3,276,800 | 
| internal.2 | AccessAdapterAir<8> | 22000107 | 8 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.2 | AccessAdapterAir<8> | 22000107 | 9 | 4,096 |  | 12 | 17 | 118,784 | 
| internal.2 | FriReducedOpeningAir | 22000107 | 8 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.2 | FriReducedOpeningAir | 22000107 | 9 | 131,072 |  | 44 | 27 | 9,306,112 | 
| internal.2 | JalRangeCheckAir | 22000107 | 8 | 65,536 |  | 16 | 12 | 1,835,008 | 
| internal.2 | JalRangeCheckAir | 22000107 | 9 | 32,768 |  | 16 | 12 | 917,504 | 
| internal.2 | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 8 | 65,536 |  | 160 | 398 | 36,569,088 | 
| internal.2 | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 9 | 32,768 |  | 160 | 398 | 18,284,544 | 
| internal.2 | PhantomAir | 22000107 | 8 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.2 | PhantomAir | 22000107 | 9 | 8,192 |  | 8 | 6 | 114,688 | 
| internal.2 | ProgramAir | 22000107 | 8 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.2 | ProgramAir | 22000107 | 9 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.2 | VariableRangeCheckerAir | 22000107 | 8 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.2 | VariableRangeCheckerAir | 22000107 | 9 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.2 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 8 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.2 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 9 | 524,288 |  | 20 | 29 | 25,690,112 | 
| internal.2 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 8 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.2 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 9 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 8 | 64 |  | 16 | 23 | 2,496 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 9 | 64 |  | 16 | 23 | 2,496 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 8 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 9 | 262,144 |  | 24 | 21 | 11,796,480 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 8 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 9 | 65,536 |  | 24 | 27 | 3,342,336 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 8 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 9 | 65,536 |  | 20 | 38 | 3,801,088 | 
| internal.2 | VmConnectorAir | 22000107 | 8 | 2 | 1 | 12 | 5 | 34 | 
| internal.2 | VmConnectorAir | 22000107 | 9 | 2 | 1 | 12 | 5 | 34 | 
| internal.2 | VolatileBoundaryAir | 22000107 | 8 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.2 | VolatileBoundaryAir | 22000107 | 9 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.3 | AccessAdapterAir<2> | 22000107 | 10 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.3 | AccessAdapterAir<4> | 22000107 | 10 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.3 | AccessAdapterAir<8> | 22000107 | 10 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.3 | FriReducedOpeningAir | 22000107 | 10 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.3 | JalRangeCheckAir | 22000107 | 10 | 65,536 |  | 16 | 12 | 1,835,008 | 
| internal.3 | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 10 | 65,536 |  | 160 | 398 | 36,569,088 | 
| internal.3 | PhantomAir | 22000107 | 10 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.3 | ProgramAir | 22000107 | 10 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.3 | VariableRangeCheckerAir | 22000107 | 10 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.3 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 10 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.3 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 10 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.3 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 10 | 64 |  | 16 | 23 | 2,496 | 
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 10 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 10 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.3 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 10 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.3 | VmConnectorAir | 22000107 | 10 | 2 | 1 | 12 | 5 | 34 | 
| internal.3 | VolatileBoundaryAir | 22000107 | 10 | 131,072 |  | 8 | 11 | 2,490,368 | 
| leaf | AccessAdapterAir<2> | 22000107 | 0 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<2> | 22000107 | 1 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<2> | 22000107 | 2 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<2> | 22000107 | 3 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 22000107 | 4 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 22000107 | 5 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 22000107 | 6 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 22000107 | 7 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<2> | 22000107 | 8 | 1,048,576 |  | 16 | 11 | 28,311,552 | 
| leaf | AccessAdapterAir<4> | 22000107 | 0 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 22000107 | 1 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 22000107 | 2 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<4> | 22000107 | 3 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 22000107 | 4 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 22000107 | 5 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 22000107 | 6 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 22000107 | 7 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<4> | 22000107 | 8 | 524,288 |  | 16 | 13 | 15,204,352 | 
| leaf | AccessAdapterAir<8> | 22000107 | 0 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 22000107 | 1 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 22000107 | 2 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | AccessAdapterAir<8> | 22000107 | 3 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 22000107 | 4 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 22000107 | 5 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 22000107 | 6 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 22000107 | 7 | 32,768 |  | 16 | 17 | 1,081,344 | 
| leaf | AccessAdapterAir<8> | 22000107 | 8 | 16,384 |  | 16 | 17 | 540,672 | 
| leaf | FriReducedOpeningAir | 22000107 | 0 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 22000107 | 1 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | FriReducedOpeningAir | 22000107 | 2 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | FriReducedOpeningAir | 22000107 | 3 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 22000107 | 4 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 22000107 | 5 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 22000107 | 6 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 22000107 | 7 | 4,194,304 |  | 84 | 27 | 465,567,744 | 
| leaf | FriReducedOpeningAir | 22000107 | 8 | 2,097,152 |  | 84 | 27 | 232,783,872 | 
| leaf | JalRangeCheckAir | 22000107 | 0 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 22000107 | 1 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 22000107 | 2 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 22000107 | 3 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 22000107 | 4 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 22000107 | 5 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 22000107 | 6 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 22000107 | 7 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | JalRangeCheckAir | 22000107 | 8 | 65,536 |  | 28 | 12 | 2,621,440 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 0 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 1 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 2 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 3 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 4 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 5 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 6 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 7 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 8 | 262,144 |  | 312 | 398 | 186,122,240 | 
| leaf | PhantomAir | 22000107 | 0 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 22000107 | 1 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 22000107 | 2 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 22000107 | 3 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 22000107 | 4 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 22000107 | 5 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 22000107 | 6 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 22000107 | 7 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | PhantomAir | 22000107 | 8 | 32,768 |  | 12 | 6 | 589,824 | 
| leaf | ProgramAir | 22000107 | 0 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 22000107 | 1 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 22000107 | 2 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 22000107 | 3 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 22000107 | 4 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 22000107 | 5 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 22000107 | 6 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 22000107 | 7 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | ProgramAir | 22000107 | 8 | 2,097,152 |  | 8 | 10 | 37,748,736 | 
| leaf | VariableRangeCheckerAir | 22000107 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 22000107 | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 22000107 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 22000107 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 22000107 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 22000107 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 22000107 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 22000107 | 7 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 22000107 | 8 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 0 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 1 | 1,048,576 |  | 36 | 29 | 68,157,440 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 2 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 3 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 4 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 5 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 6 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 7 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 8 | 2,097,152 |  | 36 | 29 | 136,314,880 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 0 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 1 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 2 | 262,144 |  | 28 | 23 | 13,369,344 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 3 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 4 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 5 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 6 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 7 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 8 | 524,288 |  | 28 | 23 | 26,738,688 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 0 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 1 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 2 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 3 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 4 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 5 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 6 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 7 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 8 | 64 |  | 28 | 27 | 3,520 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 0 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 1 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 2 | 524,288 |  | 40 | 21 | 31,981,568 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 3 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 4 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 5 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 6 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 7 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 8 | 1,048,576 |  | 40 | 21 | 63,963,136 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 0 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 1 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 2 | 131,072 |  | 40 | 27 | 8,781,824 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 3 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 4 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 5 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 6 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 7 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 8 | 262,144 |  | 40 | 27 | 17,563,648 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 0 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 1 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 2 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 3 | 524,288 |  | 36 | 38 | 38,797,312 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 4 | 524,288 |  | 36 | 38 | 38,797,312 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 5 | 524,288 |  | 36 | 38 | 38,797,312 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 6 | 524,288 |  | 36 | 38 | 38,797,312 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 7 | 524,288 |  | 36 | 38 | 38,797,312 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 8 | 262,144 |  | 36 | 38 | 19,398,656 | 
| leaf | VmConnectorAir | 22000107 | 0 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 22000107 | 1 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 22000107 | 2 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 22000107 | 3 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 22000107 | 4 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 22000107 | 5 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 22000107 | 6 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 22000107 | 7 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VmConnectorAir | 22000107 | 8 | 2 | 1 | 16 | 5 | 42 | 
| leaf | VolatileBoundaryAir | 22000107 | 0 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 22000107 | 1 | 524,288 |  | 12 | 11 | 12,058,624 | 
| leaf | VolatileBoundaryAir | 22000107 | 2 | 524,288 |  | 12 | 11 | 12,058,624 | 
| leaf | VolatileBoundaryAir | 22000107 | 3 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 22000107 | 4 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 22000107 | 5 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 22000107 | 6 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 22000107 | 7 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 22000107 | 8 | 524,288 |  | 12 | 11 | 12,058,624 | 
| root | AccessAdapterAir<2> | 22000107 | 0 | 262,144 |  | 8 | 11 | 4,980,736 | 
| root | AccessAdapterAir<4> | 22000107 | 0 | 131,072 |  | 8 | 13 | 2,752,512 | 
| root | AccessAdapterAir<8> | 22000107 | 0 | 4,096 |  | 8 | 17 | 102,400 | 
| root | FriReducedOpeningAir | 22000107 | 0 | 131,072 |  | 24 | 27 | 6,684,672 | 
| root | JalRangeCheckAir | 22000107 | 0 | 32,768 |  | 12 | 12 | 786,432 | 
| root | NativePoseidon2Air<BabyBearParameters>, 1> | 22000107 | 0 | 32,768 |  | 84 | 398 | 15,794,176 | 
| root | PhantomAir | 22000107 | 0 | 8,192 |  | 8 | 6 | 114,688 | 
| root | ProgramAir | 22000107 | 0 | 131,072 |  | 8 | 10 | 2,359,296 | 
| root | VariableRangeCheckerAir | 22000107 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| root | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 22000107 | 0 | 524,288 |  | 12 | 29 | 21,495,808 | 
| root | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 22000107 | 0 | 131,072 |  | 12 | 23 | 4,587,520 | 
| root | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 22000107 | 0 | 64 |  | 12 | 22 | 2,176 | 
| root | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 22000107 | 0 | 262,144 |  | 16 | 21 | 9,699,328 | 
| root | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 22000107 | 0 | 65,536 |  | 16 | 27 | 2,818,048 | 
| root | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 22000107 | 0 | 65,536 |  | 12 | 38 | 3,276,800 | 
| root | VmConnectorAir | 22000107 | 0 | 2 | 1 | 8 | 5 | 26 | 
| root | VolatileBoundaryAir | 22000107 | 0 | 131,072 |  | 8 | 11 | 2,490,368 | 

| group | air_name | block_number | segment | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | AccessAdapterAir<16> | 22000107 | 0 | 1 |  | 16 | 25 | 41 | 
| agg_keygen | AccessAdapterAir<2> | 22000107 | 0 | 1 |  | 16 | 11 | 27 | 
| agg_keygen | AccessAdapterAir<32> | 22000107 | 0 | 1 |  | 16 | 41 | 57 | 
| agg_keygen | AccessAdapterAir<4> | 22000107 | 0 | 1 |  | 16 | 13 | 29 | 
| agg_keygen | AccessAdapterAir<8> | 22000107 | 0 | 1 |  | 16 | 17 | 33 | 
| agg_keygen | BitwiseOperationLookupAir<8> | 22000107 | 0 | 65,536 | 3 | 8 | 2 | 655,360 | 
| agg_keygen | MemoryMerkleAir<8> | 22000107 | 0 | 64 |  | 16 | 32 | 3,072 | 
| agg_keygen | PersistentBoundaryAir<8> | 22000107 | 0 | 1 |  | 12 | 20 | 32 | 
| agg_keygen | PhantomAir | 22000107 | 0 | 1 |  | 12 | 6 | 18 | 
| agg_keygen | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 22000107 | 0 | 32 |  | 8 | 300 | 9,856 | 
| agg_keygen | ProgramAir | 22000107 | 0 | 1 |  | 8 | 10 | 18 | 
| agg_keygen | RangeTupleCheckerAir<2> | 22000107 | 0 | 524,288 | 2 | 8 | 1 | 4,718,592 | 
| agg_keygen | Rv32HintStoreAir | 22000107 | 0 | 1 |  | 44 | 32 | 76 | 
| agg_keygen | VariableRangeCheckerAir | 22000107 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 22000107 | 0 | 1 |  | 52 | 36 | 88 | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 22000107 | 0 | 1 |  | 40 | 37 | 77 | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 22000107 | 0 | 1 |  | 52 | 53 | 105 | 
| agg_keygen | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 22000107 | 0 | 1 |  | 28 | 26 | 54 | 
| agg_keygen | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 22000107 | 0 | 1 |  | 32 | 32 | 64 | 
| agg_keygen | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 22000107 | 0 | 1 |  | 28 | 18 | 46 | 
| agg_keygen | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 22000107 | 0 | 1 |  | 36 | 28 | 64 | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 22000107 | 0 | 1 |  | 52 | 36 | 88 | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 22000107 | 0 | 1 |  | 52 | 41 | 93 | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 22000107 | 0 | 1 |  | 72 | 59 | 131 | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 22000107 | 0 | 1 |  | 72 | 39 | 111 | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 22000107 | 0 | 1 |  | 52 | 31 | 83 | 
| agg_keygen | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 22000107 | 0 | 1 |  | 28 | 20 | 48 | 
| agg_keygen | VmConnectorAir | 22000107 | 0 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<16> | 22000107 | 0 | 64 |  | 16 | 25 | 2,624 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<16> | 22000107 | 2 | 8 |  | 16 | 25 | 328 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<16> | 22000107 | 3 | 524,288 |  | 16 | 25 | 21,495,808 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<16> | 22000107 | 4 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<16> | 22000107 | 5 | 524,288 |  | 16 | 25 | 21,495,808 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<16> | 22000107 | 6 | 524,288 |  | 16 | 25 | 21,495,808 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<16> | 22000107 | 7 | 262,144 |  | 16 | 25 | 10,747,904 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<16> | 22000107 | 8 | 131,072 |  | 16 | 25 | 5,373,952 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<2> | 22000107 | 3 | 512 |  | 16 | 11 | 13,824 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<2> | 22000107 | 4 | 512 |  | 16 | 11 | 13,824 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<2> | 22000107 | 5 | 4,096 |  | 16 | 11 | 110,592 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<2> | 22000107 | 6 | 512 |  | 16 | 11 | 13,824 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<2> | 22000107 | 7 | 4,096 |  | 16 | 11 | 110,592 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<2> | 22000107 | 8 | 262,144 |  | 16 | 11 | 7,077,888 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<32> | 22000107 | 0 | 32 |  | 16 | 41 | 1,824 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<32> | 22000107 | 2 | 4 |  | 16 | 41 | 228 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<32> | 22000107 | 3 | 262,144 |  | 16 | 41 | 14,942,208 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<32> | 22000107 | 4 | 262,144 |  | 16 | 41 | 14,942,208 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<32> | 22000107 | 5 | 262,144 |  | 16 | 41 | 14,942,208 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<32> | 22000107 | 6 | 262,144 |  | 16 | 41 | 14,942,208 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<32> | 22000107 | 7 | 131,072 |  | 16 | 41 | 7,471,104 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<32> | 22000107 | 8 | 65,536 |  | 16 | 41 | 3,735,552 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<4> | 22000107 | 0 | 64 |  | 16 | 13 | 1,856 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<4> | 22000107 | 3 | 256 |  | 16 | 13 | 7,424 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<4> | 22000107 | 4 | 1,024 |  | 16 | 13 | 29,696 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<4> | 22000107 | 5 | 2,048 |  | 16 | 13 | 59,392 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<4> | 22000107 | 6 | 256 |  | 16 | 13 | 7,424 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<4> | 22000107 | 7 | 4,096 |  | 16 | 13 | 118,784 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<4> | 22000107 | 8 | 131,072 |  | 16 | 13 | 3,801,088 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<8> | 22000107 | 0 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<8> | 22000107 | 1 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<8> | 22000107 | 2 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<8> | 22000107 | 3 | 2,097,152 |  | 16 | 17 | 69,206,016 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<8> | 22000107 | 4 | 2,097,152 |  | 16 | 17 | 69,206,016 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<8> | 22000107 | 5 | 1,048,576 |  | 16 | 17 | 34,603,008 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<8> | 22000107 | 6 | 2,097,152 |  | 16 | 17 | 69,206,016 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<8> | 22000107 | 7 | 2,097,152 |  | 16 | 17 | 69,206,016 | 
| reth.prove_e2e.block_22000107 | AccessAdapterAir<8> | 22000107 | 8 | 2,097,152 |  | 16 | 17 | 69,206,016 | 
| reth.prove_e2e.block_22000107 | BitwiseOperationLookupAir<8> | 22000107 | 0 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_22000107 | BitwiseOperationLookupAir<8> | 22000107 | 1 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_22000107 | BitwiseOperationLookupAir<8> | 22000107 | 2 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_22000107 | BitwiseOperationLookupAir<8> | 22000107 | 3 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_22000107 | BitwiseOperationLookupAir<8> | 22000107 | 4 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_22000107 | BitwiseOperationLookupAir<8> | 22000107 | 5 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_22000107 | BitwiseOperationLookupAir<8> | 22000107 | 6 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_22000107 | BitwiseOperationLookupAir<8> | 22000107 | 7 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_22000107 | BitwiseOperationLookupAir<8> | 22000107 | 8 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_22000107 | KeccakVmAir | 22000107 | 0 | 1 |  | 1,056 | 3,163 | 4,219 | 
| reth.prove_e2e.block_22000107 | KeccakVmAir | 22000107 | 1 | 1 |  | 1,056 | 3,163 | 4,219 | 
| reth.prove_e2e.block_22000107 | KeccakVmAir | 22000107 | 2 | 524,288 |  | 1,056 | 3,163 | 2,211,971,072 | 
| reth.prove_e2e.block_22000107 | KeccakVmAir | 22000107 | 3 | 65,536 |  | 1,056 | 3,163 | 276,496,384 | 
| reth.prove_e2e.block_22000107 | KeccakVmAir | 22000107 | 4 | 8,192 |  | 1,056 | 3,163 | 34,562,048 | 
| reth.prove_e2e.block_22000107 | KeccakVmAir | 22000107 | 5 | 8,192 |  | 1,056 | 3,163 | 34,562,048 | 
| reth.prove_e2e.block_22000107 | KeccakVmAir | 22000107 | 6 | 8,192 |  | 1,056 | 3,163 | 34,562,048 | 
| reth.prove_e2e.block_22000107 | KeccakVmAir | 22000107 | 7 | 16,384 |  | 1,056 | 3,163 | 69,124,096 | 
| reth.prove_e2e.block_22000107 | KeccakVmAir | 22000107 | 8 | 524,288 |  | 1,056 | 3,163 | 2,211,971,072 | 
| reth.prove_e2e.block_22000107 | MemoryMerkleAir<8> | 22000107 | 0 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_22000107 | MemoryMerkleAir<8> | 22000107 | 1 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_22000107 | MemoryMerkleAir<8> | 22000107 | 2 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_22000107 | MemoryMerkleAir<8> | 22000107 | 3 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_22000107 | MemoryMerkleAir<8> | 22000107 | 4 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_22000107 | MemoryMerkleAir<8> | 22000107 | 5 | 524,288 |  | 16 | 32 | 25,165,824 | 
| reth.prove_e2e.block_22000107 | MemoryMerkleAir<8> | 22000107 | 6 | 524,288 |  | 16 | 32 | 25,165,824 | 
| reth.prove_e2e.block_22000107 | MemoryMerkleAir<8> | 22000107 | 7 | 1,048,576 |  | 16 | 32 | 50,331,648 | 
| reth.prove_e2e.block_22000107 | MemoryMerkleAir<8> | 22000107 | 8 | 2,097,152 |  | 16 | 32 | 100,663,296 | 
| reth.prove_e2e.block_22000107 | PersistentBoundaryAir<8> | 22000107 | 0 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_22000107 | PersistentBoundaryAir<8> | 22000107 | 1 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_22000107 | PersistentBoundaryAir<8> | 22000107 | 2 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_22000107 | PersistentBoundaryAir<8> | 22000107 | 3 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_22000107 | PersistentBoundaryAir<8> | 22000107 | 4 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_22000107 | PersistentBoundaryAir<8> | 22000107 | 5 | 524,288 |  | 12 | 20 | 16,777,216 | 
| reth.prove_e2e.block_22000107 | PersistentBoundaryAir<8> | 22000107 | 6 | 524,288 |  | 12 | 20 | 16,777,216 | 
| reth.prove_e2e.block_22000107 | PersistentBoundaryAir<8> | 22000107 | 7 | 1,048,576 |  | 12 | 20 | 33,554,432 | 
| reth.prove_e2e.block_22000107 | PersistentBoundaryAir<8> | 22000107 | 8 | 2,097,152 |  | 12 | 20 | 67,108,864 | 
| reth.prove_e2e.block_22000107 | PhantomAir | 22000107 | 0 | 4 |  | 12 | 6 | 72 | 
| reth.prove_e2e.block_22000107 | PhantomAir | 22000107 | 1 | 1 |  | 12 | 6 | 18 | 
| reth.prove_e2e.block_22000107 | PhantomAir | 22000107 | 2 | 2 |  | 12 | 6 | 36 | 
| reth.prove_e2e.block_22000107 | PhantomAir | 22000107 | 3 | 256 |  | 12 | 6 | 4,608 | 
| reth.prove_e2e.block_22000107 | PhantomAir | 22000107 | 4 | 4 |  | 12 | 6 | 72 | 
| reth.prove_e2e.block_22000107 | PhantomAir | 22000107 | 5 | 128 |  | 12 | 6 | 2,304 | 
| reth.prove_e2e.block_22000107 | PhantomAir | 22000107 | 6 | 32 |  | 12 | 6 | 576 | 
| reth.prove_e2e.block_22000107 | PhantomAir | 22000107 | 7 | 128 |  | 12 | 6 | 2,304 | 
| reth.prove_e2e.block_22000107 | PhantomAir | 22000107 | 8 | 8 |  | 12 | 6 | 144 | 
| reth.prove_e2e.block_22000107 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 22000107 | 0 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_e2e.block_22000107 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 22000107 | 1 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_e2e.block_22000107 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 22000107 | 2 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_e2e.block_22000107 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 22000107 | 3 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_22000107 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 22000107 | 4 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_22000107 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 22000107 | 5 | 262,144 |  | 8 | 300 | 80,740,352 | 
| reth.prove_e2e.block_22000107 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 22000107 | 6 | 262,144 |  | 8 | 300 | 80,740,352 | 
| reth.prove_e2e.block_22000107 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 22000107 | 7 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_22000107 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 22000107 | 8 | 2,097,152 |  | 8 | 300 | 645,922,816 | 
| reth.prove_e2e.block_22000107 | ProgramAir | 22000107 | 0 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | ProgramAir | 22000107 | 1 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | ProgramAir | 22000107 | 2 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | ProgramAir | 22000107 | 3 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | ProgramAir | 22000107 | 4 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | ProgramAir | 22000107 | 5 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | ProgramAir | 22000107 | 6 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | ProgramAir | 22000107 | 7 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | ProgramAir | 22000107 | 8 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | RangeTupleCheckerAir<2> | 22000107 | 0 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | RangeTupleCheckerAir<2> | 22000107 | 1 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | RangeTupleCheckerAir<2> | 22000107 | 2 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | RangeTupleCheckerAir<2> | 22000107 | 3 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | RangeTupleCheckerAir<2> | 22000107 | 4 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | RangeTupleCheckerAir<2> | 22000107 | 5 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | RangeTupleCheckerAir<2> | 22000107 | 6 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | RangeTupleCheckerAir<2> | 22000107 | 7 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | RangeTupleCheckerAir<2> | 22000107 | 8 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_22000107 | Rv32HintStoreAir | 22000107 | 0 | 524,288 |  | 44 | 32 | 39,845,888 | 
| reth.prove_e2e.block_22000107 | Rv32HintStoreAir | 22000107 | 1 | 524,288 |  | 44 | 32 | 39,845,888 | 
| reth.prove_e2e.block_22000107 | Rv32HintStoreAir | 22000107 | 2 | 1,048,576 |  | 44 | 32 | 79,691,776 | 
| reth.prove_e2e.block_22000107 | Rv32HintStoreAir | 22000107 | 3 | 2,048 |  | 44 | 32 | 155,648 | 
| reth.prove_e2e.block_22000107 | Rv32HintStoreAir | 22000107 | 4 | 16 |  | 44 | 32 | 1,216 | 
| reth.prove_e2e.block_22000107 | Rv32HintStoreAir | 22000107 | 5 | 256 |  | 44 | 32 | 19,456 | 
| reth.prove_e2e.block_22000107 | Rv32HintStoreAir | 22000107 | 6 | 64 |  | 44 | 32 | 4,864 | 
| reth.prove_e2e.block_22000107 | Rv32HintStoreAir | 22000107 | 7 | 128 |  | 44 | 32 | 9,728 | 
| reth.prove_e2e.block_22000107 | VariableRangeCheckerAir | 22000107 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_22000107 | VariableRangeCheckerAir | 22000107 | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_22000107 | VariableRangeCheckerAir | 22000107 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_22000107 | VariableRangeCheckerAir | 22000107 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_22000107 | VariableRangeCheckerAir | 22000107 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_22000107 | VariableRangeCheckerAir | 22000107 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_22000107 | VariableRangeCheckerAir | 22000107 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_22000107 | VariableRangeCheckerAir | 22000107 | 7 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_22000107 | VariableRangeCheckerAir | 22000107 | 8 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 22000107 | 0 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 22000107 | 1 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 22000107 | 2 | 4,194,304 |  | 52 | 36 | 369,098,752 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 22000107 | 3 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 22000107 | 4 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 22000107 | 5 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 22000107 | 6 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 22000107 | 7 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 22000107 | 8 | 8,388,608 |  | 52 | 36 | 738,197,504 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 22000107 | 0 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 22000107 | 1 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 22000107 | 2 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 22000107 | 3 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 22000107 | 4 | 1,048,576 |  | 40 | 37 | 80,740,352 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 22000107 | 5 | 1,048,576 |  | 40 | 37 | 80,740,352 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 22000107 | 6 | 1,048,576 |  | 40 | 37 | 80,740,352 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 22000107 | 7 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 22000107 | 8 | 524,288 |  | 40 | 37 | 40,370,176 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 22000107 | 0 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 22000107 | 1 | 1,048,576 |  | 52 | 53 | 110,100,480 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 22000107 | 2 | 524,288 |  | 52 | 53 | 55,050,240 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 22000107 | 3 | 2,097,152 |  | 52 | 53 | 220,200,960 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 22000107 | 4 | 2,097,152 |  | 52 | 53 | 220,200,960 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 22000107 | 5 | 2,097,152 |  | 52 | 53 | 220,200,960 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 22000107 | 6 | 2,097,152 |  | 52 | 53 | 220,200,960 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 22000107 | 7 | 2,097,152 |  | 52 | 53 | 220,200,960 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 22000107 | 8 | 2,097,152 |  | 52 | 53 | 220,200,960 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 22000107 | 0 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 22000107 | 1 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 22000107 | 2 | 1,048,576 |  | 28 | 26 | 56,623,104 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 22000107 | 3 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 22000107 | 4 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 22000107 | 5 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 22000107 | 6 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 22000107 | 7 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 22000107 | 8 | 2,097,152 |  | 28 | 26 | 113,246,208 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 22000107 | 0 | 1,048,576 |  | 32 | 32 | 67,108,864 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 22000107 | 1 | 1,048,576 |  | 32 | 32 | 67,108,864 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 22000107 | 2 | 524,288 |  | 32 | 32 | 33,554,432 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 22000107 | 3 | 2,097,152 |  | 32 | 32 | 134,217,728 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 22000107 | 4 | 2,097,152 |  | 32 | 32 | 134,217,728 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 22000107 | 5 | 1,048,576 |  | 32 | 32 | 67,108,864 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 22000107 | 6 | 2,097,152 |  | 32 | 32 | 134,217,728 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 22000107 | 7 | 2,097,152 |  | 32 | 32 | 134,217,728 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 22000107 | 8 | 1,048,576 |  | 32 | 32 | 67,108,864 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 22000107 | 0 | 1,048,576 |  | 28 | 18 | 48,234,496 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 22000107 | 1 | 1,048,576 |  | 28 | 18 | 48,234,496 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 22000107 | 2 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 22000107 | 3 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 22000107 | 4 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 22000107 | 5 | 262,144 |  | 28 | 18 | 12,058,624 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 22000107 | 6 | 262,144 |  | 28 | 18 | 12,058,624 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 22000107 | 7 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 22000107 | 8 | 524,288 |  | 28 | 18 | 24,117,248 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 22000107 | 3 | 8,192 |  | 192 | 168 | 2,949,120 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 22000107 | 4 | 16,384 |  | 192 | 168 | 5,898,240 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 22000107 | 5 | 16,384 |  | 192 | 168 | 5,898,240 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 22000107 | 6 | 16,384 |  | 192 | 168 | 5,898,240 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 22000107 | 7 | 16,384 |  | 192 | 168 | 5,898,240 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 22000107 | 8 | 8,192 |  | 192 | 168 | 2,949,120 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 22000107 | 3 | 4,096 |  | 68 | 169 | 970,752 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 22000107 | 4 | 8,192 |  | 68 | 169 | 1,941,504 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 22000107 | 5 | 16,384 |  | 68 | 169 | 3,883,008 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 22000107 | 6 | 16,384 |  | 68 | 169 | 3,883,008 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 22000107 | 7 | 8,192 |  | 68 | 169 | 1,941,504 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 22000107 | 8 | 2,048 |  | 68 | 169 | 485,376 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 22000107 | 3 | 1,024 |  | 192 | 164 | 364,544 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 22000107 | 4 | 2,048 |  | 192 | 164 | 729,088 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 22000107 | 5 | 1,024 |  | 192 | 164 | 364,544 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 22000107 | 6 | 512 |  | 192 | 164 | 182,272 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 22000107 | 7 | 1,024 |  | 192 | 164 | 364,544 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 22000107 | 8 | 1,024 |  | 192 | 164 | 364,544 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 22000107 | 3 | 4,096 |  | 164 | 241 | 1,658,880 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 22000107 | 4 | 4,096 |  | 164 | 241 | 1,658,880 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 22000107 | 5 | 2,048 |  | 164 | 241 | 829,440 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 22000107 | 6 | 1,024 |  | 164 | 241 | 414,720 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 22000107 | 7 | 2,048 |  | 164 | 241 | 829,440 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 22000107 | 8 | 1,024 |  | 164 | 241 | 414,720 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 22000107 | 2 | 1 |  | 48 | 124 | 172 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 22000107 | 3 | 16,384 |  | 48 | 124 | 2,818,048 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 22000107 | 4 | 32,768 |  | 48 | 124 | 5,636,096 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 22000107 | 5 | 32,768 |  | 48 | 124 | 5,636,096 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 22000107 | 6 | 32,768 |  | 48 | 124 | 5,636,096 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 22000107 | 7 | 16,384 |  | 48 | 124 | 2,818,048 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 22000107 | 8 | 8,192 |  | 48 | 124 | 1,409,024 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 22000107 | 0 | 1 |  | 56 | 166 | 222 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 22000107 | 3 | 131,072 |  | 56 | 166 | 29,097,984 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 22000107 | 4 | 1,024 |  | 56 | 166 | 227,328 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 22000107 | 5 | 16,384 |  | 56 | 166 | 3,637,248 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 22000107 | 6 | 8,192 |  | 56 | 166 | 1,818,624 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 22000107 | 7 | 16,384 |  | 56 | 166 | 3,637,248 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 22000107 | 0 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 22000107 | 1 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 22000107 | 2 | 262,144 |  | 36 | 28 | 16,777,216 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 22000107 | 3 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 22000107 | 4 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 22000107 | 5 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 22000107 | 6 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 22000107 | 7 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 22000107 | 8 | 524,288 |  | 36 | 28 | 33,554,432 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 22000107 | 0 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 22000107 | 1 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 22000107 | 2 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 22000107 | 3 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 22000107 | 4 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 22000107 | 5 | 524,288 |  | 52 | 36 | 46,137,344 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 22000107 | 6 | 524,288 |  | 52 | 36 | 46,137,344 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 22000107 | 7 | 1,048,576 |  | 52 | 36 | 92,274,688 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 22000107 | 8 | 2,097,152 |  | 52 | 36 | 184,549,376 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 22000107 | 0 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 22000107 | 1 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 22000107 | 2 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 22000107 | 3 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 22000107 | 4 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 22000107 | 5 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 22000107 | 6 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 22000107 | 7 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 22000107 | 8 | 8,388,608 |  | 52 | 41 | 780,140,544 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 22000107 | 3 | 256 |  | 72 | 59 | 33,536 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 22000107 | 4 | 512 |  | 72 | 59 | 67,072 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 22000107 | 5 | 128 |  | 72 | 59 | 16,768 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 22000107 | 6 | 128 |  | 72 | 59 | 16,768 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 22000107 | 7 | 256 |  | 72 | 59 | 33,536 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 22000107 | 8 | 512 |  | 72 | 59 | 67,072 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 22000107 | 1 | 1,024 |  | 72 | 39 | 113,664 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 22000107 | 2 | 8,192 |  | 72 | 39 | 909,312 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 22000107 | 3 | 65,536 |  | 72 | 39 | 7,274,496 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 22000107 | 4 | 131,072 |  | 72 | 39 | 14,548,992 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 22000107 | 5 | 65,536 |  | 72 | 39 | 7,274,496 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 22000107 | 6 | 65,536 |  | 72 | 39 | 7,274,496 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 22000107 | 7 | 65,536 |  | 72 | 39 | 7,274,496 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 22000107 | 8 | 65,536 |  | 72 | 39 | 7,274,496 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 22000107 | 0 | 128 |  | 52 | 31 | 10,624 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 22000107 | 1 | 2,048 |  | 52 | 31 | 169,984 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 22000107 | 2 | 16,384 |  | 52 | 31 | 1,359,872 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 22000107 | 3 | 262,144 |  | 52 | 31 | 21,757,952 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 22000107 | 4 | 262,144 |  | 52 | 31 | 21,757,952 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 22000107 | 5 | 131,072 |  | 52 | 31 | 10,878,976 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 22000107 | 6 | 131,072 |  | 52 | 31 | 10,878,976 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 22000107 | 7 | 262,144 |  | 52 | 31 | 21,757,952 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 22000107 | 8 | 262,144 |  | 52 | 31 | 21,757,952 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 22000107 | 0 | 262,144 |  | 28 | 20 | 12,582,912 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 22000107 | 1 | 262,144 |  | 28 | 20 | 12,582,912 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 22000107 | 2 | 131,072 |  | 28 | 20 | 6,291,456 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 22000107 | 3 | 131,072 |  | 28 | 20 | 6,291,456 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 22000107 | 4 | 65,536 |  | 28 | 20 | 3,145,728 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 22000107 | 5 | 65,536 |  | 28 | 20 | 3,145,728 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 22000107 | 6 | 32,768 |  | 28 | 20 | 1,572,864 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 22000107 | 7 | 65,536 |  | 28 | 20 | 3,145,728 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 22000107 | 8 | 262,144 |  | 28 | 20 | 12,582,912 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 22000107 | 0 | 1 |  | 836 | 547 | 1,383 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 22000107 | 3 | 32,768 |  | 836 | 547 | 45,318,144 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 22000107 | 4 | 256 |  | 836 | 547 | 354,048 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 22000107 | 5 | 8,192 |  | 836 | 547 | 11,329,536 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 22000107 | 6 | 2,048 |  | 836 | 547 | 2,832,384 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 22000107 | 7 | 4,096 |  | 836 | 547 | 5,664,768 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 22000107 | 0 | 1 |  | 320 | 263 | 583 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 22000107 | 3 | 512 |  | 320 | 263 | 298,496 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 22000107 | 4 | 4 |  | 320 | 263 | 2,332 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 22000107 | 5 | 128 |  | 320 | 263 | 74,624 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 22000107 | 6 | 32 |  | 320 | 263 | 18,656 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 22000107 | 7 | 64 |  | 320 | 263 | 37,312 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 22000107 | 0 | 1 |  | 860 | 625 | 1,485 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 22000107 | 3 | 32,768 |  | 860 | 625 | 48,660,480 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 22000107 | 4 | 256 |  | 860 | 625 | 380,160 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 22000107 | 5 | 4,096 |  | 860 | 625 | 6,082,560 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 22000107 | 6 | 1,024 |  | 860 | 625 | 1,520,640 | 
| reth.prove_e2e.block_22000107 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 22000107 | 7 | 2,048 |  | 860 | 625 | 3,041,280 | 
| reth.prove_e2e.block_22000107 | VmConnectorAir | 22000107 | 0 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_22000107 | VmConnectorAir | 22000107 | 1 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_22000107 | VmConnectorAir | 22000107 | 2 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_22000107 | VmConnectorAir | 22000107 | 3 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_22000107 | VmConnectorAir | 22000107 | 4 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_22000107 | VmConnectorAir | 22000107 | 5 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_22000107 | VmConnectorAir | 22000107 | 6 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_22000107 | VmConnectorAir | 22000107 | 7 | 2 | 1 | 16 | 5 | 42 | 
| reth.prove_e2e.block_22000107 | VmConnectorAir | 22000107 | 8 | 2 | 1 | 16 | 5 | 42 | 

| group | block_number | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | num_segments | main_trace_commit_time_ms | main_cells_used | halo2_total_cells | halo2_keygen_time_ms | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 22000107 | 306 | 22,859 | 1,636,535 | 269,692,394 | 22,345 | 564 | 8,099 | 4,268 | 2,483 | 1 | 6,875 | 87,084,100 | 8,037,489 | 17,828 | 50 | 208 | 
| halo2_outer | 22000107 |  | 47,598 |  |  |  |  |  |  |  |  |  | 61,870,615 |  |  |  |  | 
| halo2_wrapper | 22000107 |  | 35,127 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
| reth.prove_e2e.block_22000107 | 22000107 |  |  |  |  |  |  |  |  |  | 9 |  |  |  |  |  |  | 

| group | block_number | cell_tracker_span | simple_advice_cells | lookup_advice_cells | fixed_cells |
| --- | --- | --- | --- | --- | --- |
| agg_keygen | 22000107 | VerifierProgram | 475,367 | 153,479 | 155,669 | 
| agg_keygen | 22000107 | VerifierProgram;CheckTraceHeightConstraints | 4,789 | 972 | 1,738 | 
| agg_keygen | 22000107 | VerifierProgram;PoseidonCell | 22,050 |  | 6,525 | 
| agg_keygen | 22000107 | VerifierProgram;stage-c-build-rounds | 19,490 | 2,717 | 6,687 | 
| agg_keygen | 22000107 | VerifierProgram;stage-c-build-rounds;PoseidonCell | 46,550 |  | 13,775 | 
| agg_keygen | 22000107 | VerifierProgram;stage-d-verify-pcs | 1,354,794 | 209,997 | 477,574 | 
| agg_keygen | 22000107 | VerifierProgram;stage-d-verify-pcs;PoseidonCell | 3,809,750 |  | 1,127,375 | 
| agg_keygen | 22000107 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify | 45,125 | 5,543 | 19,412 | 
| agg_keygen | 22000107 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;PoseidonCell | 68,600 |  | 20,300 | 
| agg_keygen | 22000107 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;cache-generator-powers | 66,304 | 11,396 | 20,384 | 
| agg_keygen | 22000107 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;compute-reduced-opening;single-reduced-opening-eval | 7,900,284 | 327,292 | 1,461,068 | 
| agg_keygen | 22000107 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;pre-compute-rounds-context | 76,224 | 11,116 | 22,232 | 
| agg_keygen | 22000107 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch | 49,728 |  | 6,216 | 
| agg_keygen | 22000107 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;PoseidonCell | 9,264,780 |  | 2,744,280 | 
| agg_keygen | 22000107 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;verify-batch-reduce-fast;PoseidonCell | 8,178,352 | 234,192 | 2,553,488 | 
| agg_keygen | 22000107 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query | 946,708 | 163,940 | 269,808 | 
| agg_keygen | 22000107 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext | 102,144 |  | 12,768 | 
| agg_keygen | 22000107 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;PoseidonCell | 15,647,184 |  | 4,634,784 | 
| agg_keygen | 22000107 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;verify-batch-reduce-fast;PoseidonCell | 1,548,932 | 55,440 | 476,252 | 
| agg_keygen | 22000107 | VerifierProgram;stage-e-verify-constraints | 9,215,647 | 1,851,728 | 2,839,461 | 

| group | block_number | idx | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | 22000107 | 0 | 1,070 | 7,238 | 2,379,246 | 345,876,962 | 5,239 | 865 | 1,039 | 884 | 1,320 | 952 | 142,042,663 | 176 | 929 | 
| internal.0 | 22000107 | 1 | 1,054 | 7,224 | 2,379,272 | 345,876,962 | 5,238 | 859 | 1,044 | 872 | 1,320 | 957 | 142,043,345 | 181 | 932 | 
| internal.0 | 22000107 | 2 | 1,086 | 7,723 | 2,395,331 | 419,015,138 | 5,681 | 965 | 1,079 | 959 | 1,345 | 1,118 | 143,300,973 | 211 | 956 | 
| internal.0 | 22000107 | 3 | 1,037 | 7,684 | 2,395,460 | 419,015,138 | 5,698 | 958 | 1,071 | 965 | 1,341 | 1,139 | 143,302,521 | 218 | 949 | 
| internal.0 | 22000107 | 4 | 554 | 4,271 | 1,181,959 | 186,866,146 | 3,263 | 514 | 718 | 503 | 871 | 541 | 70,912,770 | 112 | 454 | 
| internal.1 | 22000107 | 5 | 594 | 4,250 | 1,530,363 | 182,790,626 | 3,219 | 493 | 708 | 486 | 887 | 532 | 75,228,045 | 107 | 437 | 
| internal.1 | 22000107 | 6 | 601 | 4,252 | 1,530,293 | 182,790,626 | 3,214 | 489 | 714 | 497 | 873 | 537 | 75,196,495 | 100 | 437 | 
| internal.1 | 22000107 | 7 | 318 | 2,583 | 757,736 | 95,001,058 | 2,054 | 277 | 494 | 295 | 606 | 325 | 37,724,070 | 53 | 211 | 
| internal.2 | 22000107 | 8 | 598 | 4,201 | 1,515,046 | 182,790,626 | 3,188 | 490 | 700 | 496 | 871 | 533 | 73,817,354 | 94 | 415 | 
| internal.2 | 22000107 | 9 | 310 | 2,619 | 750,082 | 95,001,058 | 2,105 | 279 | 490 | 313 | 639 | 317 | 37,064,928 | 62 | 204 | 
| internal.3 | 22000107 | 10 | 607 | 4,206 | 1,507,270 | 182,790,626 | 3,190 | 493 | 705 | 488 | 874 | 531 | 73,292,109 | 95 | 409 | 
| leaf | 22000107 | 0 | 1,877 | 9,385 | 3,028,468 | 1,027,706,346 | 6,508 | 928 | 683 | 2,001 | 1,517 | 983 | 249,082,853 | 391 | 1,000 | 
| leaf | 22000107 | 1 | 1,259 | 6,210 | 2,040,031 | 660,033,002 | 4,214 | 621 | 464 | 1,212 | 1,021 | 655 | 158,012,099 | 237 | 737 | 
| leaf | 22000107 | 2 | 1,283 | 6,580 | 2,176,080 | 728,190,442 | 4,530 | 669 | 491 | 1,292 | 1,078 | 738 | 165,875,373 | 257 | 767 | 
| leaf | 22000107 | 3 | 2,070 | 10,072 | 3,458,727 | 1,090,620,906 | 6,900 | 1,010 | 738 | 2,067 | 1,620 | 1,054 | 279,349,841 | 406 | 1,102 | 
| leaf | 22000107 | 4 | 2,080 | 10,086 | 3,457,549 | 1,090,620,906 | 6,897 | 1,010 | 738 | 2,071 | 1,599 | 1,064 | 279,594,845 | 410 | 1,109 | 
| leaf | 22000107 | 5 | 2,156 | 10,181 | 3,458,221 | 1,090,620,906 | 6,915 | 1,017 | 740 | 2,023 | 1,663 | 1,058 | 279,415,308 | 408 | 1,110 | 
| leaf | 22000107 | 6 | 2,103 | 10,147 | 3,457,735 | 1,090,620,906 | 6,933 | 1,016 | 745 | 2,076 | 1,626 | 1,056 | 279,760,082 | 408 | 1,111 | 
| leaf | 22000107 | 7 | 2,099 | 10,143 | 3,458,199 | 1,090,620,906 | 6,934 | 1,013 | 749 | 2,083 | 1,617 | 1,066 | 279,494,397 | 401 | 1,110 | 
| leaf | 22000107 | 8 | 1,492 | 7,137 | 2,577,488 | 782,323,178 | 4,799 | 728 | 527 | 1,380 | 1,125 | 756 | 194,877,525 | 279 | 846 | 
| root | 22000107 | 0 | 322 | 22,909 | 758,315 | 80,304,282 | 22,377 | 560 | 8,196 | 4,286 | 2,409 | 6,868 | 37,731,915 | 52 | 210 | 

| group | block_number | idx | trace_height_constraint | weighted_sum | threshold |
| --- | --- | --- | --- | --- | --- |
| internal.0 | 22000107 | 0 | 0 | 9,830,532 | 2,013,265,921 | 
| internal.0 | 22000107 | 0 | 1 | 50,356,480 | 2,013,265,921 | 
| internal.0 | 22000107 | 0 | 2 | 4,915,266 | 2,013,265,921 | 
| internal.0 | 22000107 | 0 | 3 | 49,824,004 | 2,013,265,921 | 
| internal.0 | 22000107 | 0 | 4 | 262,144 | 2,013,265,921 | 
| internal.0 | 22000107 | 0 | 5 | 115,581,642 | 2,013,265,921 | 
| internal.0 | 22000107 | 1 | 0 | 9,830,532 | 2,013,265,921 | 
| internal.0 | 22000107 | 1 | 1 | 50,356,480 | 2,013,265,921 | 
| internal.0 | 22000107 | 1 | 2 | 4,915,266 | 2,013,265,921 | 
| internal.0 | 22000107 | 1 | 3 | 49,824,004 | 2,013,265,921 | 
| internal.0 | 22000107 | 1 | 4 | 262,144 | 2,013,265,921 | 
| internal.0 | 22000107 | 1 | 5 | 115,581,642 | 2,013,265,921 | 
| internal.0 | 22000107 | 2 | 0 | 10,354,820 | 2,013,265,921 | 
| internal.0 | 22000107 | 2 | 1 | 58,745,088 | 2,013,265,921 | 
| internal.0 | 22000107 | 2 | 2 | 5,177,410 | 2,013,265,921 | 
| internal.0 | 22000107 | 2 | 3 | 58,212,612 | 2,013,265,921 | 
| internal.0 | 22000107 | 2 | 4 | 524,288 | 2,013,265,921 | 
| internal.0 | 22000107 | 2 | 5 | 133,407,434 | 2,013,265,921 | 
| internal.0 | 22000107 | 3 | 0 | 10,354,820 | 2,013,265,921 | 
| internal.0 | 22000107 | 3 | 1 | 58,745,088 | 2,013,265,921 | 
| internal.0 | 22000107 | 3 | 2 | 5,177,410 | 2,013,265,921 | 
| internal.0 | 22000107 | 3 | 3 | 58,212,612 | 2,013,265,921 | 
| internal.0 | 22000107 | 3 | 4 | 524,288 | 2,013,265,921 | 
| internal.0 | 22000107 | 3 | 5 | 133,407,434 | 2,013,265,921 | 
| internal.0 | 22000107 | 4 | 0 | 4,882,564 | 2,013,265,921 | 
| internal.0 | 22000107 | 4 | 1 | 26,620,160 | 2,013,265,921 | 
| internal.0 | 22000107 | 4 | 2 | 2,441,282 | 2,013,265,921 | 
| internal.0 | 22000107 | 4 | 3 | 25,960,708 | 2,013,265,921 | 
| internal.0 | 22000107 | 4 | 4 | 131,072 | 2,013,265,921 | 
| internal.0 | 22000107 | 4 | 5 | 60,429,002 | 2,013,265,921 | 
| internal.1 | 22000107 | 5 | 0 | 5,144,708 | 2,013,265,921 | 
| internal.1 | 22000107 | 5 | 1 | 23,748,864 | 2,013,265,921 | 
| internal.1 | 22000107 | 5 | 2 | 2,572,354 | 2,013,265,921 | 
| internal.1 | 22000107 | 5 | 3 | 23,085,316 | 2,013,265,921 | 
| internal.1 | 22000107 | 5 | 4 | 131,072 | 2,013,265,921 | 
| internal.1 | 22000107 | 5 | 5 | 55,075,530 | 2,013,265,921 | 
| internal.1 | 22000107 | 6 | 0 | 5,144,708 | 2,013,265,921 | 
| internal.1 | 22000107 | 6 | 1 | 23,748,864 | 2,013,265,921 | 
| internal.1 | 22000107 | 6 | 2 | 2,572,354 | 2,013,265,921 | 
| internal.1 | 22000107 | 6 | 3 | 23,085,316 | 2,013,265,921 | 
| internal.1 | 22000107 | 6 | 4 | 131,072 | 2,013,265,921 | 
| internal.1 | 22000107 | 6 | 5 | 55,075,530 | 2,013,265,921 | 
| internal.1 | 22000107 | 7 | 0 | 2,572,420 | 2,013,265,921 | 
| internal.1 | 22000107 | 7 | 1 | 12,005,632 | 2,013,265,921 | 
| internal.1 | 22000107 | 7 | 2 | 1,286,210 | 2,013,265,921 | 
| internal.1 | 22000107 | 7 | 3 | 11,673,860 | 2,013,265,921 | 
| internal.1 | 22000107 | 7 | 4 | 65,536 | 2,013,265,921 | 
| internal.1 | 22000107 | 7 | 5 | 27,996,874 | 2,013,265,921 | 
| internal.2 | 22000107 | 8 | 0 | 5,144,708 | 2,013,265,921 | 
| internal.2 | 22000107 | 8 | 1 | 23,748,864 | 2,013,265,921 | 
| internal.2 | 22000107 | 8 | 2 | 2,572,354 | 2,013,265,921 | 
| internal.2 | 22000107 | 8 | 3 | 23,085,316 | 2,013,265,921 | 
| internal.2 | 22000107 | 8 | 4 | 131,072 | 2,013,265,921 | 
| internal.2 | 22000107 | 8 | 5 | 55,075,530 | 2,013,265,921 | 
| internal.2 | 22000107 | 9 | 0 | 2,572,420 | 2,013,265,921 | 
| internal.2 | 22000107 | 9 | 1 | 12,005,632 | 2,013,265,921 | 
| internal.2 | 22000107 | 9 | 2 | 1,286,210 | 2,013,265,921 | 
| internal.2 | 22000107 | 9 | 3 | 11,673,860 | 2,013,265,921 | 
| internal.2 | 22000107 | 9 | 4 | 65,536 | 2,013,265,921 | 
| internal.2 | 22000107 | 9 | 5 | 27,996,874 | 2,013,265,921 | 
| internal.3 | 22000107 | 10 | 0 | 5,144,708 | 2,013,265,921 | 
| internal.3 | 22000107 | 10 | 1 | 23,748,864 | 2,013,265,921 | 
| internal.3 | 22000107 | 10 | 2 | 2,572,354 | 2,013,265,921 | 
| internal.3 | 22000107 | 10 | 3 | 23,085,316 | 2,013,265,921 | 
| internal.3 | 22000107 | 10 | 4 | 131,072 | 2,013,265,921 | 
| internal.3 | 22000107 | 10 | 5 | 55,075,530 | 2,013,265,921 | 
| leaf | 22000107 | 0 | 0 | 18,022,532 | 2,013,265,921 | 
| leaf | 22000107 | 0 | 1 | 123,437,312 | 2,013,265,921 | 
| leaf | 22000107 | 0 | 2 | 9,011,266 | 2,013,265,921 | 
| leaf | 22000107 | 0 | 3 | 121,962,756 | 2,013,265,921 | 
| leaf | 22000107 | 0 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 22000107 | 0 | 5 | 275,317,450 | 2,013,265,921 | 
| leaf | 22000107 | 1 | 0 | 9,896,068 | 2,013,265,921 | 
| leaf | 22000107 | 1 | 1 | 73,318,656 | 2,013,265,921 | 
| leaf | 22000107 | 1 | 2 | 4,948,034 | 2,013,265,921 | 
| leaf | 22000107 | 1 | 3 | 71,860,484 | 2,013,265,921 | 
| leaf | 22000107 | 1 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 22000107 | 1 | 5 | 162,906,826 | 2,013,265,921 | 
| leaf | 22000107 | 2 | 0 | 11,993,220 | 2,013,265,921 | 
| leaf | 22000107 | 2 | 1 | 79,610,112 | 2,013,265,921 | 
| leaf | 22000107 | 2 | 2 | 5,996,610 | 2,013,265,921 | 
| leaf | 22000107 | 2 | 3 | 78,151,940 | 2,013,265,921 | 
| leaf | 22000107 | 2 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 22000107 | 2 | 5 | 178,635,466 | 2,013,265,921 | 
| leaf | 22000107 | 3 | 0 | 18,546,820 | 2,013,265,921 | 
| leaf | 22000107 | 3 | 1 | 129,728,768 | 2,013,265,921 | 
| leaf | 22000107 | 3 | 2 | 9,273,410 | 2,013,265,921 | 
| leaf | 22000107 | 3 | 3 | 126,681,348 | 2,013,265,921 | 
| leaf | 22000107 | 3 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 22000107 | 3 | 5 | 287,113,930 | 2,013,265,921 | 
| leaf | 22000107 | 4 | 0 | 18,546,820 | 2,013,265,921 | 
| leaf | 22000107 | 4 | 1 | 129,728,768 | 2,013,265,921 | 
| leaf | 22000107 | 4 | 2 | 9,273,410 | 2,013,265,921 | 
| leaf | 22000107 | 4 | 3 | 126,681,348 | 2,013,265,921 | 
| leaf | 22000107 | 4 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 22000107 | 4 | 5 | 287,113,930 | 2,013,265,921 | 
| leaf | 22000107 | 5 | 0 | 18,546,820 | 2,013,265,921 | 
| leaf | 22000107 | 5 | 1 | 129,728,768 | 2,013,265,921 | 
| leaf | 22000107 | 5 | 2 | 9,273,410 | 2,013,265,921 | 
| leaf | 22000107 | 5 | 3 | 126,681,348 | 2,013,265,921 | 
| leaf | 22000107 | 5 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 22000107 | 5 | 5 | 287,113,930 | 2,013,265,921 | 
| leaf | 22000107 | 6 | 0 | 18,546,820 | 2,013,265,921 | 
| leaf | 22000107 | 6 | 1 | 129,728,768 | 2,013,265,921 | 
| leaf | 22000107 | 6 | 2 | 9,273,410 | 2,013,265,921 | 
| leaf | 22000107 | 6 | 3 | 126,681,348 | 2,013,265,921 | 
| leaf | 22000107 | 6 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 22000107 | 6 | 5 | 287,113,930 | 2,013,265,921 | 
| leaf | 22000107 | 7 | 0 | 18,546,820 | 2,013,265,921 | 
| leaf | 22000107 | 7 | 1 | 129,728,768 | 2,013,265,921 | 
| leaf | 22000107 | 7 | 2 | 9,273,410 | 2,013,265,921 | 
| leaf | 22000107 | 7 | 3 | 126,681,348 | 2,013,265,921 | 
| leaf | 22000107 | 7 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 22000107 | 7 | 5 | 287,113,930 | 2,013,265,921 | 
| leaf | 22000107 | 8 | 0 | 13,828,228 | 2,013,265,921 | 
| leaf | 22000107 | 8 | 1 | 84,590,848 | 2,013,265,921 | 
| leaf | 22000107 | 8 | 2 | 6,914,114 | 2,013,265,921 | 
| leaf | 22000107 | 8 | 3 | 83,132,676 | 2,013,265,921 | 
| leaf | 22000107 | 8 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 22000107 | 8 | 5 | 191,349,450 | 2,013,265,921 | 
| root | 22000107 | 0 | 0 | 2,252,928 | 2,013,265,921 | 
| root | 22000107 | 0 | 1 | 14,557,184 | 2,013,265,921 | 
| root | 22000107 | 0 | 2 | 1,126,464 | 2,013,265,921 | 
| root | 22000107 | 0 | 3 | 14,753,792 | 2,013,265,921 | 
| root | 22000107 | 0 | 4 | 262,144 | 2,013,265,921 | 
| root | 22000107 | 0 | 5 | 33,476,802 | 2,013,265,921 | 

| group | block_number | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 22000107 | 0 | 27 | 432 |  | 7,747,601 | 405 | 29 | 57 | 50 | 200 | 51 | 928,071 | 14 | 0 | 
| reth.prove_e2e.block_22000107 | 22000107 | 0 | 8,741 | 25,800 | 20,430,482 | 2,557,896,601 | 14,333 | 2,134 | 1,358 | 3,912 | 3,136 | 2,892 | 995,795,440 | 886 | 2,726 | 
| reth.prove_e2e.block_22000107 | 22000107 | 1 | 8,565 | 25,051 | 20,449,268 | 2,558,158,007 | 13,770 | 2,123 | 1,255 | 3,736 | 3,096 | 2,779 | 992,741,022 | 772 | 2,716 | 
| reth.prove_e2e.block_22000107 | 22000107 | 2 | 7,081 | 30,951 | 12,870,127 | 4,250,444,582 | 22,101 | 5,435 | 1,190 | 4,571 | 2,993 | 6,999 | 1,769,086,820 | 904 | 1,769 | 
| reth.prove_e2e.block_22000107 | 22000107 | 3 | 9,636 | 30,963 | 23,324,839 | 3,012,571,690 | 16,325 | 2,801 | 1,405 | 4,360 | 3,469 | 3,327 | 1,236,717,752 | 948 | 5,002 | 
| reth.prove_e2e.block_22000107 | 22000107 | 4 | 9,836 | 27,908 | 23,376,955 | 2,688,672,678 | 14,928 | 2,355 | 1,369 | 4,062 | 3,432 | 2,824 | 1,071,497,594 | 869 | 3,144 | 
| reth.prove_e2e.block_22000107 | 22000107 | 5 | 7,337 | 23,928 | 18,962,952 | 2,419,754,282 | 13,822 | 2,105 | 1,292 | 3,852 | 3,203 | 2,484 | 845,833,748 | 868 | 2,769 | 
| reth.prove_e2e.block_22000107 | 22000107 | 6 | 8,227 | 25,190 | 20,143,642 | 2,504,140,170 | 14,201 | 2,199 | 1,299 | 3,906 | 3,326 | 2,585 | 921,584,730 | 871 | 2,762 | 
| reth.prove_e2e.block_22000107 | 22000107 | 7 | 10,074 | 28,224 | 23,309,891 | 2,675,722,602 | 14,882 | 2,350 | 1,382 | 4,016 | 3,405 | 2,861 | 1,110,217,335 | 853 | 3,268 | 
| reth.prove_e2e.block_22000107 | 22000107 | 8 | 12,425 | 43,009 | 22,811,177 | 5,404,414,650 | 26,844 | 6,187 | 1,486 | 5,851 | 3,887 | 8,234 | 2,277,774,551 | 1,186 | 3,740 | 

| group | block_number | segment | trace_height_constraint | weighted_sum | threshold |
| --- | --- | --- | --- | --- | --- |
| agg_keygen | 22000107 | 0 | 0 | 34 | 2,013,265,921 | 
| agg_keygen | 22000107 | 0 | 1 | 86 | 2,013,265,921 | 
| agg_keygen | 22000107 | 0 | 2 | 17 | 2,013,265,921 | 
| agg_keygen | 22000107 | 0 | 3 | 98 | 2,013,265,921 | 
| agg_keygen | 22000107 | 0 | 4 | 193 | 2,013,265,921 | 
| agg_keygen | 22000107 | 0 | 5 | 65 | 2,013,265,921 | 
| agg_keygen | 22000107 | 0 | 6 | 29 | 2,013,265,921 | 
| agg_keygen | 22000107 | 0 | 7 | 20 | 2,013,265,921 | 
| agg_keygen | 22000107 | 0 | 8 | 918,079 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 0 | 0 | 49,807,646 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 0 | 1 | 141,034,908 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 0 | 2 | 24,903,823 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 0 | 3 | 166,201,738 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 0 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 0 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 0 | 6 | 56,361,113 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 0 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 0 | 8 | 512 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 0 | 9 | 449,188,716 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 1 | 0 | 49,813,512 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 1 | 1 | 141,051,994 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 1 | 2 | 24,906,756 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 1 | 3 | 166,217,822 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 1 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 1 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 1 | 6 | 56,362,122 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 1 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 1 | 8 | 16,384 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 1 | 9 | 449,247,566 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 2 | 0 | 37,535,754 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 2 | 1 | 154,550,316 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 2 | 2 | 18,767,877 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 2 | 3 | 176,046,116 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 2 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 2 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 2 | 6 | 103,686,145 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 2 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 2 | 8 | 131,072 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 2 | 9 | 501,596,256 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 3 | 0 | 52,897,284 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 3 | 1 | 163,096,832 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 3 | 2 | 26,448,642 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 3 | 3 | 217,283,076 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 3 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 3 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 3 | 6 | 68,721,664 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 3 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 3 | 8 | 1,607,680 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 3 | 9 | 540,409,866 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 4 | 0 | 53,493,824 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 4 | 1 | 158,549,204 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 4 | 2 | 26,746,912 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 4 | 3 | 187,717,728 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 4 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 4 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 4 | 6 | 62,066,500 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 4 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 4 | 8 | 2,166,784 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 4 | 9 | 501,095,640 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 5 | 0 | 49,495,684 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 5 | 1 | 147,017,600 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 5 | 2 | 24,747,842 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 5 | 3 | 180,437,124 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 5 | 4 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 5 | 5 | 1,048,576 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 5 | 6 | 59,156,096 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 5 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 5 | 8 | 1,082,368 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 5 | 9 | 468,883,530 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 6 | 0 | 51,488,356 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 6 | 1 | 154,042,656 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 6 | 2 | 25,744,178 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 6 | 3 | 182,796,900 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 6 | 4 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 6 | 5 | 1,048,576 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 6 | 6 | 61,039,200 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 6 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 6 | 8 | 1,065,984 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 6 | 9 | 483,124,090 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 7 | 0 | 52,333,892 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 7 | 1 | 155,439,168 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 7 | 2 | 26,166,946 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 7 | 3 | 186,857,668 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 7 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 7 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 7 | 6 | 61,539,520 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 7 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 7 | 8 | 1,607,680 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 7 | 9 | 494,299,562 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 8 | 0 | 53,650,452 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 8 | 1 | 204,753,920 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 8 | 2 | 26,825,226 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 8 | 3 | 235,655,172 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 8 | 4 | 8,388,608 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 8 | 5 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 8 | 6 | 130,144,256 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 8 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 8 | 8 | 1,609,728 | 2,013,265,921 | 
| reth.prove_e2e.block_22000107 | 22000107 | 8 | 9 | 670,857,762 | 2,013,265,921 | 

| group | block_number | trace_height_constraint | weighted_sum | threshold |
| --- | --- | --- | --- | --- |
| agg_keygen | 22000107 | 0 | 5,701,764 | 2,013,265,921 | 
| agg_keygen | 22000107 | 1 | 28,467,456 | 2,013,265,921 | 
| agg_keygen | 22000107 | 2 | 2,850,882 | 2,013,265,921 | 
| agg_keygen | 22000107 | 3 | 27,803,908 | 2,013,265,921 | 
| agg_keygen | 22000107 | 4 | 262,144 | 2,013,265,921 | 
| agg_keygen | 22000107 | 5 | 65,348,298 | 2,013,265,921 | 

</details>


Commit: https://github.com/axiom-crypto/openvm-reth-benchmark/actions/runs/13976877596

Max Segment Length: 8388508

Instance Type: m8g.24xlarge

Memory Allocator: https://github.com/axiom-crypto/openvm-reth-benchmark/commit/ac0339f8434db6141c31a2aac3d452c535ebccd8

[Benchmark Workflow]()
