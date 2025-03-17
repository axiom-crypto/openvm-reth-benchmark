| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total |  692.07 |  211.34 |
| reth.prove_e2e.block_21000000 |  350.100 |  43.12 |
| leaf |  118.35 |  13.07 |
| internal.0 |  60.66 |  11.28 |
| internal.1 |  20.32 |  6.79 |
| internal.2 |  11.43 |  6.77 |
| internal.3 |  6.89 |  6.89 |
| root |  32.16 |  32.16 |
| halo2_outer |  53.01 |  53.01 |
| halo2_wrapper |  38.25 |  38.25 |
| agg_keygen |  32.56 |  31.73 |


| reth.prove_e2e.block_21000000 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  31,908.64 |  350,995 |  43,120 |  9,384 |
| `main_cells_used     ` |  1,154,071,693.82 |  12,694,788,632 |  1,939,008,486 |  290,298,651 |
| `total_cycles        ` |  18,539,949.45 |  203,939,444 |  24,818,528 |  1,960,856 |
| `execute_time_ms     ` |  2,865.82 |  31,524 |  6,142 |  280 |
| `trace_gen_time_ms   ` |  9,214.09 |  101,355 |  11,611 |  3,201 |
| `stark_prove_excluding_trace_time_ms` |  19,828.73 |  218,116 |  30,954 |  5,903 |
| `main_trace_commit_time_ms` |  4,599.18 |  50,591 |  9,454 |  1,249 |
| `generate_perm_trace_time_ms` |  1,070.64 |  11,777 |  1,418 |  214 |
| `perm_trace_commit_time_ms` |  4,745.91 |  52,205 |  6,366 |  1,028 |
| `quotient_poly_compute_time_ms` |  3,397 |  37,367 |  7,287 |  789 |
| `quotient_poly_commit_time_ms` |  1,832.91 |  20,162 |  2,310 |  769 |
| `pcs_opening_time_ms ` |  4,169.09 |  45,860 |  5,037 |  1,844 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  10,758.91 |  118,348 |  13,069 |  7,862 |
| `main_cells_used     ` |  221,972,179.27 |  2,441,693,972 |  278,429,262 |  155,902,748 |
| `total_cycles        ` |  2,794,872.91 |  30,743,602 |  3,457,162 |  2,026,273 |
| `execute_time_ms     ` |  1,264.55 |  13,910 |  1,470 |  1,011 |
| `trace_gen_time_ms   ` |  1,859.82 |  20,458 |  2,333 |  1,341 |
| `stark_prove_excluding_trace_time_ms` |  7,634.55 |  83,980 |  9,393 |  5,489 |
| `main_trace_commit_time_ms` |  1,204 |  13,244 |  1,461 |  834 |
| `generate_perm_trace_time_ms` |  484.27 |  5,327 |  614 |  327 |
| `perm_trace_commit_time_ms` |  2,095.18 |  23,047 |  2,627 |  1,357 |
| `quotient_poly_compute_time_ms` |  956.36 |  10,520 |  1,165 |  666 |
| `quotient_poly_commit_time_ms` |  922.09 |  10,143 |  1,108 |  710 |
| `pcs_opening_time_ms ` |  1,966.82 |  21,635 |  2,442 |  1,555 |

| internal.0 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  10,110.17 |  60,661 |  11,278 |  6,892 |
| `main_cells_used     ` |  130,355,028.83 |  782,130,173 |  143,242,441 |  71,078,541 |
| `total_cycles        ` |  2,185,672.83 |  13,114,037 |  2,399,063 |  1,183,618 |
| `execute_time_ms     ` |  1,350.50 |  8,103 |  1,545 |  709 |
| `trace_gen_time_ms   ` |  1,068.33 |  6,410 |  1,171 |  609 |
| `stark_prove_excluding_trace_time_ms` |  7,691.33 |  46,148 |  8,562 |  5,574 |
| `main_trace_commit_time_ms` |  1,379.67 |  8,278 |  1,610 |  907 |
| `generate_perm_trace_time_ms` |  256.17 |  1,537 |  309 |  143 |
| `perm_trace_commit_time_ms` |  1,220.17 |  7,321 |  1,377 |  791 |
| `quotient_poly_compute_time_ms` |  1,003.50 |  6,021 |  1,155 |  621 |
| `quotient_poly_commit_time_ms` |  1,799 |  10,794 |  1,858 |  1,557 |
| `pcs_opening_time_ms ` |  2,027.17 |  12,163 |  2,248 |  1,549 |

| internal.1 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  6,774.33 |  20,323 |  6,794 |  6,740 |
| `main_cells_used     ` |  75,126,256 |  225,378,768 |  75,324,567 |  74,757,444 |
| `total_cycles        ` |  1,531,339.33 |  4,594,018 |  1,533,986 |  1,526,170 |
| `execute_time_ms     ` |  661.33 |  1,984 |  666 |  658 |
| `trace_gen_time_ms   ` |  671.67 |  2,015 |  674 |  670 |
| `stark_prove_excluding_trace_time_ms` |  5,441.33 |  16,324 |  5,463 |  5,404 |
| `main_trace_commit_time_ms` |  855.67 |  2,567 |  864 |  845 |
| `generate_perm_trace_time_ms` |  143.33 |  430 |  155 |  129 |
| `perm_trace_commit_time_ms` |  797.33 |  2,392 |  818 |  784 |
| `quotient_poly_compute_time_ms` |  590.33 |  1,771 |  601 |  584 |
| `quotient_poly_commit_time_ms` |  1,461 |  4,383 |  1,463 |  1,460 |
| `pcs_opening_time_ms ` |  1,587 |  4,761 |  1,623 |  1,561 |

| internal.2 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  5,712.50 |  11,425 |  6,769 |  4,656 |
| `main_cells_used     ` |  55,846,961.50 |  111,693,923 |  73,915,972 |  37,777,951 |
| `total_cycles        ` |  1,139,135 |  2,278,270 |  1,518,602 |  759,668 |
| `execute_time_ms     ` |  475.50 |  951 |  636 |  315 |
| `trace_gen_time_ms   ` |  515 |  1,030 |  665 |  365 |
| `stark_prove_excluding_trace_time_ms` |  4,722 |  9,444 |  5,468 |  3,976 |
| `main_trace_commit_time_ms` |  730.50 |  1,461 |  880 |  581 |
| `generate_perm_trace_time_ms` |  144.50 |  289 |  187 |  102 |
| `perm_trace_commit_time_ms` |  678.50 |  1,357 |  813 |  544 |
| `quotient_poly_compute_time_ms` |  464.50 |  929 |  587 |  342 |
| `quotient_poly_commit_time_ms` |  1,291.50 |  2,583 |  1,414 |  1,169 |
| `pcs_opening_time_ms ` |  1,406 |  2,812 |  1,581 |  1,231 |

| internal.3 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  6,895 |  6,895 |  6,895 |  6,895 |
| `main_cells_used     ` |  73,391,267 |  73,391,267 |  73,391,267 |  73,391,267 |
| `total_cycles        ` |  1,510,871 |  1,510,871 |  1,510,871 |  1,510,871 |
| `execute_time_ms     ` |  615 |  615 |  615 |  615 |
| `trace_gen_time_ms   ` |  661 |  661 |  661 |  661 |
| `stark_prove_excluding_trace_time_ms` |  5,619 |  5,619 |  5,619 |  5,619 |
| `main_trace_commit_time_ms` |  847 |  847 |  847 |  847 |
| `generate_perm_trace_time_ms` |  200 |  200 |  200 |  200 |
| `perm_trace_commit_time_ms` |  796 |  796 |  796 |  796 |
| `quotient_poly_compute_time_ms` |  581 |  581 |  581 |  581 |
| `quotient_poly_commit_time_ms` |  1,570 |  1,570 |  1,570 |  1,570 |
| `pcs_opening_time_ms ` |  1,620 |  1,620 |  1,620 |  1,620 |

| root |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  32,160 |  32,160 |  32,160 |  32,160 |
| `main_cells_used     ` |  37,781,218 |  37,781,218 |  37,781,218 |  37,781,218 |
| `total_cycles        ` |  760,098 |  760,098 |  760,098 |  760,098 |
| `execute_time_ms     ` |  315 |  315 |  315 |  315 |
| `trace_gen_time_ms   ` |  359 |  359 |  359 |  359 |
| `stark_prove_excluding_trace_time_ms` |  31,486 |  31,486 |  31,486 |  31,486 |
| `main_trace_commit_time_ms` |  8,985 |  8,985 |  8,985 |  8,985 |
| `generate_perm_trace_time_ms` |  111 |  111 |  111 |  111 |
| `perm_trace_commit_time_ms` |  5,663 |  5,663 |  5,663 |  5,663 |
| `quotient_poly_compute_time_ms` |  677 |  677 |  677 |  677 |
| `quotient_poly_commit_time_ms` |  11,809 |  11,809 |  11,809 |  11,809 |
| `pcs_opening_time_ms ` |  4,236 |  4,236 |  4,236 |  4,236 |

| halo2_outer |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  53,010 |  53,010 |  53,010 |  53,010 |
| `main_cells_used     ` |  61,870,615 |  61,870,615 |  61,870,615 |  61,870,615 |

| halo2_wrapper |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  38,249 |  38,249 |  38,249 |  38,249 |

| agg_keygen |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  16,281 |  32,562 |  31,728 |  834 |
| `main_cells_used     ` |  45,232,195.50 |  90,464,391 |  89,536,320 |  928,071 |
| `total_cycles        ` |  1,683,753 |  1,683,753 |  1,683,753 |  1,683,753 |
| `execute_time_ms     ` |  163.50 |  327 |  327 |  0 |
| `trace_gen_time_ms   ` |  189.50 |  379 |  356 |  23 |
| `stark_prove_excluding_trace_time_ms` |  15,928 |  31,856 |  31,045 |  811 |
| `main_trace_commit_time_ms` |  4,535 |  9,070 |  8,973 |  97 |
| `generate_perm_trace_time_ms` |  71.50 |  143 |  116 |  27 |
| `perm_trace_commit_time_ms` |  2,890.50 |  5,781 |  5,686 |  95 |
| `quotient_poly_compute_time_ms` |  352.50 |  705 |  671 |  34 |
| `quotient_poly_commit_time_ms` |  5,913 |  11,826 |  11,707 |  119 |
| `pcs_opening_time_ms ` |  2,157 |  4,314 |  3,882 |  432 |



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
| VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 | 2 | 16 | 20 | 
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 2 | 18 | 33 | 
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 2 | 17 | 40 | 
| VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 2 | 25 | 84 | 
| VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 2 | 24 | 31 | 
| VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 2 | 19 | 19 | 
| VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 2 | 12 | 14 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 2 | 411 | 476 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 2 | 156 | 188 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 2 | 422 | 451 | 
| VmConnectorAir | 21000000 | 2 | 5 | 10 | 

| block_number | execute_time_ms |
| --- | --- |
| 21000000 | 319 | 

| group | air_name | block_number | rows | quotient_deg | prep_cols | perm_cols | main_cols | interactions | constraints | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | AccessAdapterAir<16> | 21000000 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<2> | 21000000 | 524,288 | 8 |  | 16 | 11 | 5 | 12 | 14,155,776 | 
| agg_keygen | AccessAdapterAir<32> | 21000000 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<4> | 21000000 | 262,144 | 8 |  | 16 | 13 | 5 | 12 | 7,602,176 | 
| agg_keygen | AccessAdapterAir<64> | 21000000 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<8> | 21000000 | 8,192 | 8 |  | 16 | 17 | 5 | 12 | 270,336 | 
| agg_keygen | BitwiseOperationLookupAir<8> | 21000000 |  | 2 |  |  |  | 2 | 4 |  | 
| agg_keygen | FriReducedOpeningAir | 21000000 | 524,288 | 8 |  | 84 | 27 | 39 | 71 | 58,195,968 | 
| agg_keygen | JalRangeCheckAir | 21000000 | 65,536 | 8 |  | 28 | 12 | 9 | 14 | 2,621,440 | 
| agg_keygen | MemoryMerkleAir<8> | 21000000 |  | 2 |  |  |  | 4 | 39 |  | 
| agg_keygen | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 131,072 | 8 |  | 312 | 398 | 136 | 572 | 93,061,120 | 
| agg_keygen | PersistentBoundaryAir<8> | 21000000 |  | 2 |  |  |  | 3 | 6 |  | 
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
| agg_keygen | VmConnectorAir | 21000000 | 2 | 8 | 1 | 16 | 5 | 5 | 10 | 42 | 
| agg_keygen | VolatileBoundaryAir | 21000000 | 131,072 | 4 |  | 12 | 11 | 4 | 17 | 3,014,656 | 

| group | air_name | block_number | idx | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | AccessAdapterAir<2> | 21000000 | 0 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 1 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 2 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 3 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 4 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21000000 | 5 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 0 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 1 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 2 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 3 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21000000 | 4 | 262,144 |  | 12 | 13 | 6,553,600 | 
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
| internal.0 | JalRangeCheckAir | 21000000 | 0 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | JalRangeCheckAir | 21000000 | 1 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | JalRangeCheckAir | 21000000 | 2 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | JalRangeCheckAir | 21000000 | 3 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | JalRangeCheckAir | 21000000 | 4 | 131,072 |  | 16 | 12 | 3,670,016 | 
| internal.0 | JalRangeCheckAir | 21000000 | 5 | 65,536 |  | 16 | 12 | 1,835,008 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 0 | 131,072 |  | 160 | 398 | 73,138,176 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 1 | 131,072 |  | 160 | 398 | 73,138,176 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 2 | 262,144 |  | 160 | 398 | 146,276,352 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 3 | 262,144 |  | 160 | 398 | 146,276,352 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 4 | 131,072 |  | 160 | 398 | 73,138,176 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 5 | 65,536 |  | 160 | 398 | 36,569,088 | 
| internal.0 | PhantomAir | 21000000 | 0 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 1 | 32,768 |  | 8 | 6 | 458,752 | 
| internal.0 | PhantomAir | 21000000 | 2 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 3 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 4 | 65,536 |  | 8 | 6 | 917,504 | 
| internal.0 | PhantomAir | 21000000 | 5 | 16,384 |  | 8 | 6 | 229,376 | 
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
| internal.0 | VmConnectorAir | 21000000 | 0 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VmConnectorAir | 21000000 | 1 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VmConnectorAir | 21000000 | 2 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VmConnectorAir | 21000000 | 3 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VmConnectorAir | 21000000 | 4 | 2 | 1 | 12 | 5 | 34 | 
| internal.0 | VmConnectorAir | 21000000 | 5 | 2 | 1 | 12 | 5 | 34 | 
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
| internal.1 | JalRangeCheckAir | 21000000 | 6 | 65,536 |  | 16 | 12 | 1,835,008 | 
| internal.1 | JalRangeCheckAir | 21000000 | 7 | 65,536 |  | 16 | 12 | 1,835,008 | 
| internal.1 | JalRangeCheckAir | 21000000 | 8 | 65,536 |  | 16 | 12 | 1,835,008 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 6 | 65,536 |  | 160 | 398 | 36,569,088 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 7 | 65,536 |  | 160 | 398 | 36,569,088 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 8 | 65,536 |  | 160 | 398 | 36,569,088 | 
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
| internal.1 | VmConnectorAir | 21000000 | 6 | 2 | 1 | 12 | 5 | 34 | 
| internal.1 | VmConnectorAir | 21000000 | 7 | 2 | 1 | 12 | 5 | 34 | 
| internal.1 | VmConnectorAir | 21000000 | 8 | 2 | 1 | 12 | 5 | 34 | 
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
| internal.2 | JalRangeCheckAir | 21000000 | 10 | 32,768 |  | 16 | 12 | 917,504 | 
| internal.2 | JalRangeCheckAir | 21000000 | 9 | 65,536 |  | 16 | 12 | 1,835,008 | 
| internal.2 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 10 | 32,768 |  | 160 | 398 | 18,284,544 | 
| internal.2 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 9 | 65,536 |  | 160 | 398 | 36,569,088 | 
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
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 10 | 64 |  | 16 | 23 | 2,496 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 9 | 64 |  | 16 | 23 | 2,496 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 10 | 262,144 |  | 24 | 21 | 11,796,480 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 9 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 10 | 65,536 |  | 24 | 27 | 3,342,336 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 9 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 10 | 65,536 |  | 20 | 38 | 3,801,088 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 9 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.2 | VmConnectorAir | 21000000 | 10 | 2 | 1 | 12 | 5 | 34 | 
| internal.2 | VmConnectorAir | 21000000 | 9 | 2 | 1 | 12 | 5 | 34 | 
| internal.2 | VolatileBoundaryAir | 21000000 | 10 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.2 | VolatileBoundaryAir | 21000000 | 9 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.3 | AccessAdapterAir<2> | 21000000 | 11 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.3 | AccessAdapterAir<4> | 21000000 | 11 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.3 | AccessAdapterAir<8> | 21000000 | 11 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.3 | FriReducedOpeningAir | 21000000 | 11 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.3 | JalRangeCheckAir | 21000000 | 11 | 65,536 |  | 16 | 12 | 1,835,008 | 
| internal.3 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 11 | 65,536 |  | 160 | 398 | 36,569,088 | 
| internal.3 | PhantomAir | 21000000 | 11 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.3 | ProgramAir | 21000000 | 11 | 131,072 |  | 8 | 10 | 2,359,296 | 
| internal.3 | VariableRangeCheckerAir | 21000000 | 11 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.3 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 11 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.3 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 11 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.3 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 11 | 64 |  | 16 | 23 | 2,496 | 
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 11 | 524,288 |  | 24 | 21 | 23,592,960 | 
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 11 | 131,072 |  | 24 | 27 | 6,684,672 | 
| internal.3 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 11 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.3 | VmConnectorAir | 21000000 | 11 | 2 | 1 | 12 | 5 | 34 | 
| internal.3 | VolatileBoundaryAir | 21000000 | 11 | 131,072 |  | 8 | 11 | 2,490,368 | 
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
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 10 | 131,072 |  | 312 | 398 | 93,061,120 | 
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
| leaf | ProgramAir | 21000000 | 0 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| leaf | ProgramAir | 21000000 | 1 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| leaf | ProgramAir | 21000000 | 10 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| leaf | ProgramAir | 21000000 | 2 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| leaf | ProgramAir | 21000000 | 3 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| leaf | ProgramAir | 21000000 | 4 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| leaf | ProgramAir | 21000000 | 5 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| leaf | ProgramAir | 21000000 | 6 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| leaf | ProgramAir | 21000000 | 7 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| leaf | ProgramAir | 21000000 | 8 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| leaf | ProgramAir | 21000000 | 9 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
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
| leaf | VolatileBoundaryAir | 21000000 | 0 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 21000000 | 1 | 524,288 |  | 12 | 11 | 12,058,624 | 
| leaf | VolatileBoundaryAir | 21000000 | 10 | 524,288 |  | 12 | 11 | 12,058,624 | 
| leaf | VolatileBoundaryAir | 21000000 | 2 | 524,288 |  | 12 | 11 | 12,058,624 | 
| leaf | VolatileBoundaryAir | 21000000 | 3 | 524,288 |  | 12 | 11 | 12,058,624 | 
| leaf | VolatileBoundaryAir | 21000000 | 4 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 21000000 | 5 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 21000000 | 6 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 21000000 | 7 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 21000000 | 8 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | VolatileBoundaryAir | 21000000 | 9 | 524,288 |  | 12 | 11 | 12,058,624 | 
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
| agg_keygen | VmConnectorAir | 21000000 | 0 | 2 | 1 | 16 | 5 | 42 | 
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
| agg_keygen | 21000000 | 356 | 31,728 | 1,683,753 | 316,222,954 | 31,045 | 671 | 11,707 | 5,686 | 3,882 | 1 | 8,973 | 89,536,320 | 8,037,489 | 20,052 | 116 | 327 | 
| halo2_outer | 21000000 |  | 53,010 |  |  |  |  |  |  |  |  |  | 61,870,615 |  |  |  |  | 
| halo2_wrapper | 21000000 |  | 38,249 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
| reth.prove_e2e.block_21000000 | 21000000 |  |  |  |  |  |  |  |  |  | 11 |  |  |  |  |  |  | 

| group | block_number | cell_tracker_span | simple_advice_cells | lookup_advice_cells | fixed_cells |
| --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | VerifierProgram | 475,367 | 153,479 | 155,669 | 
| agg_keygen | 21000000 | VerifierProgram;CheckTraceHeightConstraints | 4,789 | 972 | 1,738 | 
| agg_keygen | 21000000 | VerifierProgram;PoseidonCell | 22,050 |  | 6,525 | 
| agg_keygen | 21000000 | VerifierProgram;stage-c-build-rounds | 19,490 | 2,717 | 6,687 | 
| agg_keygen | 21000000 | VerifierProgram;stage-c-build-rounds;PoseidonCell | 46,550 |  | 13,775 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs | 1,354,794 | 209,997 | 477,574 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;PoseidonCell | 3,809,750 |  | 1,127,375 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify | 45,125 | 5,543 | 19,412 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;PoseidonCell | 68,600 |  | 20,300 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;cache-generator-powers | 66,304 | 11,396 | 20,384 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;compute-reduced-opening;single-reduced-opening-eval | 7,900,284 | 327,292 | 1,461,068 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;pre-compute-rounds-context | 76,224 | 11,116 | 22,232 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch | 49,728 |  | 6,216 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;PoseidonCell | 9,264,780 |  | 2,744,280 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;verify-batch-reduce-fast;PoseidonCell | 8,178,352 | 234,192 | 2,553,488 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query | 946,708 | 163,940 | 269,808 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext | 102,144 |  | 12,768 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;PoseidonCell | 15,647,184 |  | 4,634,784 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;verify-batch-reduce-fast;PoseidonCell | 1,548,932 | 55,440 | 476,252 | 
| agg_keygen | 21000000 | VerifierProgram;stage-e-verify-constraints | 9,215,647 | 1,851,728 | 2,839,461 | 

| group | block_number | idx | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | 21000000 | 0 | 1,166 | 10,398 | 2,382,816 | 345,876,962 | 7,831 | 1,036 | 1,854 | 1,234 | 2,070 | 1,384 | 142,101,434 | 248 | 1,401 | 
| internal.0 | 21000000 | 1 | 1,136 | 10,447 | 2,366,688 | 345,418,210 | 7,876 | 1,030 | 1,843 | 1,297 | 2,050 | 1,404 | 140,481,861 | 246 | 1,435 | 
| internal.0 | 21000000 | 2 | 1,159 | 11,056 | 2,398,948 | 419,015,138 | 8,391 | 1,153 | 1,852 | 1,347 | 2,168 | 1,573 | 143,241,061 | 291 | 1,506 | 
| internal.0 | 21000000 | 3 | 1,171 | 11,278 | 2,399,063 | 419,015,138 | 8,562 | 1,155 | 1,858 | 1,377 | 2,248 | 1,610 | 143,242,441 | 309 | 1,545 | 
| internal.0 | 21000000 | 4 | 1,169 | 10,590 | 2,382,904 | 345,876,962 | 7,914 | 1,026 | 1,830 | 1,275 | 2,078 | 1,400 | 141,984,835 | 300 | 1,507 | 
| internal.0 | 21000000 | 5 | 609 | 6,892 | 1,183,618 | 186,866,146 | 5,574 | 621 | 1,557 | 791 | 1,549 | 907 | 71,078,541 | 143 | 709 | 
| internal.1 | 21000000 | 6 | 674 | 6,789 | 1,533,862 | 182,790,626 | 5,457 | 586 | 1,460 | 790 | 1,623 | 845 | 75,324,567 | 146 | 658 | 
| internal.1 | 21000000 | 7 | 670 | 6,740 | 1,533,986 | 182,790,626 | 5,404 | 584 | 1,460 | 784 | 1,577 | 864 | 75,296,757 | 129 | 666 | 
| internal.1 | 21000000 | 8 | 671 | 6,794 | 1,526,170 | 185,280,994 | 5,463 | 601 | 1,463 | 818 | 1,561 | 858 | 74,757,444 | 155 | 660 | 
| internal.2 | 21000000 | 10 | 365 | 4,656 | 759,668 | 95,001,058 | 3,976 | 342 | 1,169 | 544 | 1,231 | 581 | 37,777,951 | 102 | 315 | 
| internal.2 | 21000000 | 9 | 665 | 6,769 | 1,518,602 | 182,790,626 | 5,468 | 587 | 1,414 | 813 | 1,581 | 880 | 73,915,972 | 187 | 636 | 
| internal.3 | 21000000 | 11 | 661 | 6,895 | 1,510,871 | 182,790,626 | 5,619 | 581 | 1,570 | 796 | 1,620 | 847 | 73,391,267 | 200 | 615 | 
| leaf | 21000000 | 0 | 2,060 | 12,115 | 3,026,221 | 1,008,831,978 | 8,702 | 1,084 | 1,025 | 2,489 | 2,142 | 1,354 | 247,999,644 | 602 | 1,353 | 
| leaf | 21000000 | 1 | 1,350 | 8,348 | 2,042,023 | 641,158,634 | 5,921 | 721 | 738 | 1,519 | 1,590 | 949 | 157,515,767 | 398 | 1,077 | 
| leaf | 21000000 | 10 | 1,393 | 8,341 | 2,103,609 | 616,254,954 | 5,937 | 717 | 774 | 1,509 | 1,636 | 878 | 159,968,166 | 416 | 1,011 | 
| leaf | 21000000 | 2 | 1,360 | 8,383 | 2,042,261 | 641,158,634 | 5,961 | 722 | 710 | 1,532 | 1,594 | 985 | 157,519,636 | 411 | 1,062 | 
| leaf | 21000000 | 3 | 1,341 | 7,862 | 2,026,273 | 548,097,514 | 5,489 | 666 | 744 | 1,357 | 1,555 | 834 | 155,902,748 | 327 | 1,032 | 
| leaf | 21000000 | 4 | 2,333 | 12,845 | 3,457,162 | 1,071,746,538 | 9,042 | 1,164 | 1,055 | 2,580 | 2,270 | 1,435 | 278,248,685 | 532 | 1,470 | 
| leaf | 21000000 | 5 | 2,289 | 12,819 | 3,455,991 | 1,071,746,538 | 9,089 | 1,165 | 1,108 | 2,559 | 2,241 | 1,422 | 278,372,798 | 590 | 1,441 | 
| leaf | 21000000 | 6 | 2,313 | 12,827 | 3,456,549 | 1,071,746,538 | 9,060 | 1,156 | 1,068 | 2,584 | 2,274 | 1,436 | 278,429,262 | 536 | 1,454 | 
| leaf | 21000000 | 7 | 2,241 | 13,069 | 3,376,651 | 1,071,746,538 | 9,393 | 1,164 | 1,096 | 2,627 | 2,442 | 1,445 | 275,284,575 | 614 | 1,435 | 
| leaf | 21000000 | 8 | 2,276 | 12,771 | 3,416,746 | 1,071,746,538 | 9,049 | 1,155 | 1,040 | 2,599 | 2,237 | 1,461 | 276,926,914 | 551 | 1,446 | 
| leaf | 21000000 | 9 | 1,502 | 8,968 | 2,340,116 | 722,685,418 | 6,337 | 806 | 785 | 1,692 | 1,654 | 1,045 | 175,525,777 | 350 | 1,129 | 
| root | 21000000 | 0 | 359 | 32,160 | 760,098 | 80,304,282 | 31,486 | 677 | 11,809 | 5,663 | 4,236 | 8,985 | 37,781,218 | 111 | 315 | 

| group | block_number | idx | trace_height_constraint | weighted_sum | threshold |
| --- | --- | --- | --- | --- | --- |
| internal.0 | 21000000 | 0 | 0 | 9,830,532 | 2,013,265,921 | 
| internal.0 | 21000000 | 0 | 1 | 50,356,480 | 2,013,265,921 | 
| internal.0 | 21000000 | 0 | 2 | 4,915,266 | 2,013,265,921 | 
| internal.0 | 21000000 | 0 | 3 | 49,824,004 | 2,013,265,921 | 
| internal.0 | 21000000 | 0 | 4 | 262,144 | 2,013,265,921 | 
| internal.0 | 21000000 | 0 | 5 | 115,581,642 | 2,013,265,921 | 
| internal.0 | 21000000 | 1 | 0 | 9,764,996 | 2,013,265,921 | 
| internal.0 | 21000000 | 1 | 1 | 50,356,480 | 2,013,265,921 | 
| internal.0 | 21000000 | 1 | 2 | 4,882,498 | 2,013,265,921 | 
| internal.0 | 21000000 | 1 | 3 | 49,824,004 | 2,013,265,921 | 
| internal.0 | 21000000 | 1 | 4 | 262,144 | 2,013,265,921 | 
| internal.0 | 21000000 | 1 | 5 | 115,483,338 | 2,013,265,921 | 
| internal.0 | 21000000 | 2 | 0 | 10,354,820 | 2,013,265,921 | 
| internal.0 | 21000000 | 2 | 1 | 58,745,088 | 2,013,265,921 | 
| internal.0 | 21000000 | 2 | 2 | 5,177,410 | 2,013,265,921 | 
| internal.0 | 21000000 | 2 | 3 | 58,212,612 | 2,013,265,921 | 
| internal.0 | 21000000 | 2 | 4 | 524,288 | 2,013,265,921 | 
| internal.0 | 21000000 | 2 | 5 | 133,407,434 | 2,013,265,921 | 
| internal.0 | 21000000 | 3 | 0 | 10,354,820 | 2,013,265,921 | 
| internal.0 | 21000000 | 3 | 1 | 58,745,088 | 2,013,265,921 | 
| internal.0 | 21000000 | 3 | 2 | 5,177,410 | 2,013,265,921 | 
| internal.0 | 21000000 | 3 | 3 | 58,212,612 | 2,013,265,921 | 
| internal.0 | 21000000 | 3 | 4 | 524,288 | 2,013,265,921 | 
| internal.0 | 21000000 | 3 | 5 | 133,407,434 | 2,013,265,921 | 
| internal.0 | 21000000 | 4 | 0 | 9,830,532 | 2,013,265,921 | 
| internal.0 | 21000000 | 4 | 1 | 50,356,480 | 2,013,265,921 | 
| internal.0 | 21000000 | 4 | 2 | 4,915,266 | 2,013,265,921 | 
| internal.0 | 21000000 | 4 | 3 | 49,824,004 | 2,013,265,921 | 
| internal.0 | 21000000 | 4 | 4 | 262,144 | 2,013,265,921 | 
| internal.0 | 21000000 | 4 | 5 | 115,581,642 | 2,013,265,921 | 
| internal.0 | 21000000 | 5 | 0 | 4,882,564 | 2,013,265,921 | 
| internal.0 | 21000000 | 5 | 1 | 26,620,160 | 2,013,265,921 | 
| internal.0 | 21000000 | 5 | 2 | 2,441,282 | 2,013,265,921 | 
| internal.0 | 21000000 | 5 | 3 | 25,960,708 | 2,013,265,921 | 
| internal.0 | 21000000 | 5 | 4 | 131,072 | 2,013,265,921 | 
| internal.0 | 21000000 | 5 | 5 | 60,429,002 | 2,013,265,921 | 
| internal.1 | 21000000 | 6 | 0 | 5,144,708 | 2,013,265,921 | 
| internal.1 | 21000000 | 6 | 1 | 23,748,864 | 2,013,265,921 | 
| internal.1 | 21000000 | 6 | 2 | 2,572,354 | 2,013,265,921 | 
| internal.1 | 21000000 | 6 | 3 | 23,085,316 | 2,013,265,921 | 
| internal.1 | 21000000 | 6 | 4 | 131,072 | 2,013,265,921 | 
| internal.1 | 21000000 | 6 | 5 | 55,075,530 | 2,013,265,921 | 
| internal.1 | 21000000 | 7 | 0 | 5,144,708 | 2,013,265,921 | 
| internal.1 | 21000000 | 7 | 1 | 23,748,864 | 2,013,265,921 | 
| internal.1 | 21000000 | 7 | 2 | 2,572,354 | 2,013,265,921 | 
| internal.1 | 21000000 | 7 | 3 | 23,085,316 | 2,013,265,921 | 
| internal.1 | 21000000 | 7 | 4 | 131,072 | 2,013,265,921 | 
| internal.1 | 21000000 | 7 | 5 | 55,075,530 | 2,013,265,921 | 
| internal.1 | 21000000 | 8 | 0 | 5,144,708 | 2,013,265,921 | 
| internal.1 | 21000000 | 8 | 1 | 24,011,008 | 2,013,265,921 | 
| internal.1 | 21000000 | 8 | 2 | 2,572,354 | 2,013,265,921 | 
| internal.1 | 21000000 | 8 | 3 | 23,347,460 | 2,013,265,921 | 
| internal.1 | 21000000 | 8 | 4 | 131,072 | 2,013,265,921 | 
| internal.1 | 21000000 | 8 | 5 | 55,599,818 | 2,013,265,921 | 
| internal.2 | 21000000 | 10 | 0 | 2,572,420 | 2,013,265,921 | 
| internal.2 | 21000000 | 10 | 1 | 12,005,632 | 2,013,265,921 | 
| internal.2 | 21000000 | 10 | 2 | 1,286,210 | 2,013,265,921 | 
| internal.2 | 21000000 | 10 | 3 | 11,673,860 | 2,013,265,921 | 
| internal.2 | 21000000 | 10 | 4 | 65,536 | 2,013,265,921 | 
| internal.2 | 21000000 | 10 | 5 | 27,996,874 | 2,013,265,921 | 
| internal.2 | 21000000 | 9 | 0 | 5,144,708 | 2,013,265,921 | 
| internal.2 | 21000000 | 9 | 1 | 23,748,864 | 2,013,265,921 | 
| internal.2 | 21000000 | 9 | 2 | 2,572,354 | 2,013,265,921 | 
| internal.2 | 21000000 | 9 | 3 | 23,085,316 | 2,013,265,921 | 
| internal.2 | 21000000 | 9 | 4 | 131,072 | 2,013,265,921 | 
| internal.2 | 21000000 | 9 | 5 | 55,075,530 | 2,013,265,921 | 
| internal.3 | 21000000 | 11 | 0 | 5,144,708 | 2,013,265,921 | 
| internal.3 | 21000000 | 11 | 1 | 23,748,864 | 2,013,265,921 | 
| internal.3 | 21000000 | 11 | 2 | 2,572,354 | 2,013,265,921 | 
| internal.3 | 21000000 | 11 | 3 | 23,085,316 | 2,013,265,921 | 
| internal.3 | 21000000 | 11 | 4 | 131,072 | 2,013,265,921 | 
| internal.3 | 21000000 | 11 | 5 | 55,075,530 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 0 | 18,022,532 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 1 | 123,437,312 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 2 | 9,011,266 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 3 | 121,962,756 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 0 | 5 | 274,268,874 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 0 | 9,896,068 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 1 | 73,318,656 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 2 | 4,948,034 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 3 | 71,860,484 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 1 | 5 | 161,858,250 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 0 | 11,468,932 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 1 | 71,221,504 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 2 | 5,734,466 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 3 | 69,763,332 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 4 | 262,144 | 2,013,265,921 | 
| leaf | 21000000 | 10 | 5 | 159,761,098 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 0 | 9,896,068 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 1 | 73,318,656 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 2 | 4,948,034 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 3 | 71,860,484 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 2 | 5 | 161,858,250 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 0 | 9,371,780 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 1 | 64,930,048 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 2 | 4,685,890 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 3 | 63,471,876 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 4 | 262,144 | 2,013,265,921 | 
| leaf | 21000000 | 3 | 5 | 144,032,458 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 0 | 18,546,820 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 1 | 129,728,768 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 2 | 9,273,410 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 3 | 126,681,348 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 4 | 5 | 286,065,354 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 0 | 18,546,820 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 1 | 129,728,768 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 2 | 9,273,410 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 3 | 126,681,348 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 5 | 5 | 286,065,354 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 0 | 18,546,820 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 1 | 129,728,768 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 2 | 9,273,410 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 3 | 126,681,348 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 6 | 5 | 286,065,354 | 2,013,265,921 | 
| leaf | 21000000 | 7 | 0 | 18,546,820 | 2,013,265,921 | 
| leaf | 21000000 | 7 | 1 | 129,728,768 | 2,013,265,921 | 
| leaf | 21000000 | 7 | 2 | 9,273,410 | 2,013,265,921 | 
| leaf | 21000000 | 7 | 3 | 126,681,348 | 2,013,265,921 | 
| leaf | 21000000 | 7 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 7 | 5 | 286,065,354 | 2,013,265,921 | 
| leaf | 21000000 | 8 | 0 | 18,546,820 | 2,013,265,921 | 
| leaf | 21000000 | 8 | 1 | 129,728,768 | 2,013,265,921 | 
| leaf | 21000000 | 8 | 2 | 9,273,410 | 2,013,265,921 | 
| leaf | 21000000 | 8 | 3 | 126,681,348 | 2,013,265,921 | 
| leaf | 21000000 | 8 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 8 | 5 | 286,065,354 | 2,013,265,921 | 
| leaf | 21000000 | 9 | 0 | 12,517,508 | 2,013,265,921 | 
| leaf | 21000000 | 9 | 1 | 80,658,688 | 2,013,265,921 | 
| leaf | 21000000 | 9 | 2 | 6,258,754 | 2,013,265,921 | 
| leaf | 21000000 | 9 | 3 | 79,200,516 | 2,013,265,921 | 
| leaf | 21000000 | 9 | 4 | 524,288 | 2,013,265,921 | 
| leaf | 21000000 | 9 | 5 | 180,470,474 | 2,013,265,921 | 
| root | 21000000 | 0 | 0 | 2,252,928 | 2,013,265,921 | 
| root | 21000000 | 0 | 1 | 14,557,184 | 2,013,265,921 | 
| root | 21000000 | 0 | 2 | 1,126,464 | 2,013,265,921 | 
| root | 21000000 | 0 | 3 | 14,753,792 | 2,013,265,921 | 
| root | 21000000 | 0 | 4 | 262,144 | 2,013,265,921 | 
| root | 21000000 | 0 | 5 | 33,476,802 | 2,013,265,921 | 

| group | block_number | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 0 | 23 | 834 |  | 7,747,690 | 811 | 34 | 119 | 95 | 432 | 97 | 928,071 | 27 | 0 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 9,346 | 30,653 | 20,422,705 | 2,557,907,183 | 18,532 | 2,489 | 1,948 | 4,883 | 4,243 | 3,808 | 994,421,279 | 1,143 | 2,775 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 9,451 | 30,032 | 20,434,522 | 2,558,016,183 | 17,843 | 2,479 | 1,815 | 4,645 | 4,164 | 3,658 | 991,772,661 | 1,072 | 2,738 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 3,201 | 9,384 | 1,960,856 | 589,214,668 | 5,903 | 789 | 769 | 1,028 | 1,844 | 1,249 | 290,298,651 | 214 | 280 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 9,509 | 29,914 | 20,453,510 | 2,558,327,991 | 17,672 | 2,496 | 1,746 | 4,482 | 4,191 | 3,708 | 993,317,657 | 1,040 | 2,733 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 4,837 | 28,068 | 5,876,483 | 3,341,267,022 | 22,401 | 5,940 | 1,228 | 4,030 | 2,924 | 7,374 | 1,454,792,147 | 895 | 830 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 10,592 | 40,256 | 23,439,171 | 3,351,350,506 | 23,522 | 4,040 | 2,185 | 5,800 | 5,037 | 5,180 | 1,360,022,335 | 1,264 | 6,142 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 10,765 | 33,585 | 23,882,763 | 2,713,921,154 | 19,668 | 2,809 | 2,046 | 5,110 | 4,636 | 3,887 | 1,097,004,355 | 1,163 | 3,152 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 11,611 | 36,336 | 24,818,528 | 2,902,320,874 | 21,289 | 2,994 | 2,310 | 5,613 | 4,911 | 4,147 | 1,161,361,680 | 1,297 | 3,436 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 11,137 | 34,077 | 24,328,072 | 2,696,164,250 | 19,590 | 2,787 | 2,098 | 5,047 | 4,703 | 3,832 | 1,121,209,272 | 1,106 | 3,350 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 11,472 | 35,570 | 24,014,164 | 2,884,954,730 | 20,742 | 3,257 | 2,121 | 5,201 | 4,686 | 4,294 | 1,291,580,109 | 1,165 | 3,356 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 9,434 | 43,120 | 14,308,670 | 4,957,707,962 | 30,954 | 7,287 | 1,896 | 6,366 | 4,521 | 9,454 | 1,939,008,486 | 1,418 | 2,732 | 

| group | block_number | segment | trace_height_constraint | weighted_sum | threshold |
| --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 0 | 0 | 34 | 2,013,265,921 | 
| agg_keygen | 21000000 | 0 | 1 | 89 | 2,013,265,921 | 
| agg_keygen | 21000000 | 0 | 2 | 17 | 2,013,265,921 | 
| agg_keygen | 21000000 | 0 | 3 | 100 | 2,013,265,921 | 
| agg_keygen | 21000000 | 0 | 4 | 193 | 2,013,265,921 | 
| agg_keygen | 21000000 | 0 | 5 | 65 | 2,013,265,921 | 
| agg_keygen | 21000000 | 0 | 6 | 29 | 2,013,265,921 | 
| agg_keygen | 21000000 | 0 | 7 | 20 | 2,013,265,921 | 
| agg_keygen | 21000000 | 0 | 8 | 918,084 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 0 | 49,807,902 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 1 | 141,035,676 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 2 | 24,903,951 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 3 | 166,202,492 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 6 | 56,361,113 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 8 | 1,024 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 9 | 449,191,134 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 0 | 49,810,440 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 1 | 141,042,778 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 2 | 24,905,220 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 3 | 166,208,606 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 6 | 56,361,610 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 8 | 8,192 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 9 | 449,215,822 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 0 | 6,078,538 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 1 | 18,891,032 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 2 | 3,039,269 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 3 | 22,266,116 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 4 | 3,407,872 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 5 | 1,310,720 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 6 | 9,207,842 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 8 | 33,024 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 9 | 68,297,645 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 0 | 49,817,608 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 1 | 141,064,282 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 2 | 24,908,804 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 3 | 166,230,110 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 6 | 56,362,122 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 8 | 24,576 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 9 | 449,286,478 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 0 | 20,094,984 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 1 | 105,373,696 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 2 | 10,047,492 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 3 | 115,073,028 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 6 | 89,198,592 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 8 | 98,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 9 | 350,240,784 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 0 | 52,798,084 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 1 | 170,270,592 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 2 | 26,399,042 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 3 | 235,903,364 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 6 | 77,454,464 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 8 | 1,589,760 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 9 | 574,769,994 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 0 | 53,497,408 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 1 | 158,818,004 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 2 | 26,748,704 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 3 | 188,047,444 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 6 | 63,141,188 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 8 | 2,164,736 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 9 | 502,772,172 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 0 | 58,725,220 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 1 | 170,345,248 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 2 | 29,362,610 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 3 | 203,698,948 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 6 | 65,770,336 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 8 | 2,131,968 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 9 | 540,389,018 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 0 | 53,072,124 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 1 | 157,245,224 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 2 | 26,536,062 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 3 | 186,672,676 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 6 | 61,880,072 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 8 | 3,179,008 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 9 | 498,939,854 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 0 | 52,653,668 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 1 | 160,140,064 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 2 | 26,326,834 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 3 | 190,355,972 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 6 | 68,669,024 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 8 | 2,114,560 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 9 | 510,614,810 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 0 | 44,993,556 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 1 | 177,619,968 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 2 | 22,496,778 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 3 | 198,710,276 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 4 | 7,340,032 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 5 | 3,145,728 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 6 | 120,228,352 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 7 |  | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 8 | 397,312 | 2,013,265,921 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 9 | 580,568,098 | 2,013,265,921 | 

| group | block_number | trace_height_constraint | weighted_sum | threshold |
| --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 0 | 5,963,908 | 2,013,265,921 | 
| agg_keygen | 21000000 | 1 | 32,661,760 | 2,013,265,921 | 
| agg_keygen | 21000000 | 2 | 2,981,954 | 2,013,265,921 | 
| agg_keygen | 21000000 | 3 | 31,998,212 | 2,013,265,921 | 
| agg_keygen | 21000000 | 4 | 262,144 | 2,013,265,921 | 
| agg_keygen | 21000000 | 5 | 74,261,194 | 2,013,265,921 | 

</details>


Commit: https://github.com/axiom-crypto/openvm-reth-benchmark/commit/a81808c3399989877e3aad2cfa07cea4cdb0df7a

Max Segment Length: 8388508

Instance Type: m8g.24xlarge

Memory Allocator: jemalloc

[Benchmark Workflow](https://github.com/axiom-crypto/openvm-reth-benchmark/actions/runs/13898933561)
