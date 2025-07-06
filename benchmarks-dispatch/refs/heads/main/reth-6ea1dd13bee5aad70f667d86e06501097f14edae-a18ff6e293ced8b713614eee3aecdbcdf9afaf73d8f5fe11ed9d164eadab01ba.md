| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total |  591.78 |  221.92 |
| reth.prove_evm.block_21000000 |  292.44 |  34.14 |
| leaf |  106.98 |  12.05 |
| internal.0 |  47.04 |  12.53 |
| internal.1 |  12.76 |  8.45 |
| internal.2 |  6.37 |  6.37 |
| root |  31.70 |  31.70 |
| halo2_outer |  55.64 |  55.64 |
| halo2_wrapper |  38.86 |  38.86 |
| agg_keygen |  32.90 |  32.05 |


| reth.prove_evm.block_21000000 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  26,585.55 |  292,441 |  34,138 |  6,887 |
| `main_cells_used     ` |  1,091,374,803.45 |  12,005,122,838 |  1,591,081,371 |  210,290,236 |
| `total_cycles        ` |  14,278,165.18 |  157,059,817 |  22,248,987 |  1,152,076 |
| `execute_time_ms     ` |  2,357.91 |  25,937 |  3,752 |  173 |
| `trace_gen_time_ms   ` |  7,516.36 |  82,680 |  12,195 |  2,562 |
| `stark_prove_excluding_trace_time_ms` |  16,711.27 |  183,824 |  22,137 |  4,152 |
| `main_trace_commit_time_ms` |  4,416.73 |  48,584 |  7,733 |  905 |
| `generate_perm_trace_time_ms` |  1,151.45 |  12,666 |  1,410 |  198 |
| `perm_trace_commit_time_ms` |  3,012.82 |  33,141 |  3,520 |  550 |
| `quotient_poly_compute_time_ms` |  2,778 |  30,558 |  5,443 |  366 |
| `quotient_poly_commit_time_ms` |  1,259.82 |  13,858 |  1,538 |  553 |
| `pcs_opening_time_ms ` |  4,080.36 |  44,884 |  4,947 |  1,572 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  9,725.55 |  106,981 |  12,052 |  7,300 |
| `main_cells_used     ` |  216,232,008.36 |  2,378,552,092 |  284,864,993 |  154,398,147 |
| `total_cycles        ` |  2,743,770.45 |  30,181,475 |  3,518,479 |  2,022,979 |
| `execute_time_ms     ` |  1,230.82 |  13,539 |  1,478 |  971 |
| `trace_gen_time_ms   ` |  1,828 |  20,108 |  2,384 |  1,353 |
| `stark_prove_excluding_trace_time_ms` |  6,666.73 |  73,334 |  8,257 |  4,976 |
| `main_trace_commit_time_ms` |  1,133.09 |  12,464 |  1,403 |  780 |
| `generate_perm_trace_time_ms` |  572.45 |  6,297 |  776 |  432 |
| `perm_trace_commit_time_ms` |  1,450.45 |  15,955 |  1,862 |  1,022 |
| `quotient_poly_compute_time_ms` |  666.45 |  7,331 |  838 |  473 |
| `quotient_poly_commit_time_ms` |  720.45 |  7,925 |  856 |  593 |
| `pcs_opening_time_ms ` |  2,117.64 |  23,294 |  2,657 |  1,650 |

| internal.0 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  11,759.75 |  47,039 |  12,533 |  9,993 |
| `main_cells_used     ` |  193,085,236.25 |  772,340,945 |  212,773,600 |  139,492,404 |
| `total_cycles        ` |  3,227,383.50 |  12,909,534 |  3,545,702 |  2,333,327 |
| `execute_time_ms     ` |  2,002.25 |  8,009 |  2,246 |  1,433 |
| `trace_gen_time_ms   ` |  1,546.50 |  6,186 |  1,700 |  1,124 |
| `stark_prove_excluding_trace_time_ms` |  8,211 |  32,844 |  8,587 |  7,436 |
| `main_trace_commit_time_ms` |  1,572.50 |  6,290 |  1,692 |  1,309 |
| `generate_perm_trace_time_ms` |  416.25 |  1,665 |  486 |  378 |
| `perm_trace_commit_time_ms` |  1,217.25 |  4,869 |  1,266 |  1,102 |
| `quotient_poly_compute_time_ms` |  940 |  3,760 |  995 |  790 |
| `quotient_poly_commit_time_ms` |  1,637.25 |  6,549 |  1,693 |  1,537 |
| `pcs_opening_time_ms ` |  2,422.25 |  9,689 |  2,516 |  2,315 |

| internal.1 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  6,377.50 |  12,755 |  8,448 |  4,307 |
| `main_cells_used     ` |  77,149,720 |  154,299,440 |  115,071,446 |  39,227,994 |
| `total_cycles        ` |  1,559,111.50 |  3,118,223 |  2,338,720 |  779,503 |
| `execute_time_ms     ` |  690 |  1,380 |  1,038 |  342 |
| `trace_gen_time_ms   ` |  691 |  1,382 |  1,009 |  373 |
| `stark_prove_excluding_trace_time_ms` |  4,996.50 |  9,993 |  6,401 |  3,592 |
| `main_trace_commit_time_ms` |  840.50 |  1,681 |  1,142 |  539 |
| `generate_perm_trace_time_ms` |  218 |  436 |  307 |  129 |
| `perm_trace_commit_time_ms` |  680 |  1,360 |  926 |  434 |
| `quotient_poly_compute_time_ms` |  499 |  998 |  686 |  312 |
| `quotient_poly_commit_time_ms` |  1,156.50 |  2,313 |  1,321 |  992 |
| `pcs_opening_time_ms ` |  1,595.50 |  3,191 |  2,013 |  1,178 |

| internal.2 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  6,366 |  6,366 |  6,366 |  6,366 |
| `main_cells_used     ` |  75,830,346 |  75,830,346 |  75,830,346 |  75,830,346 |
| `total_cycles        ` |  1,542,952 |  1,542,952 |  1,542,952 |  1,542,952 |
| `execute_time_ms     ` |  649 |  649 |  649 |  649 |
| `trace_gen_time_ms   ` |  682 |  682 |  682 |  682 |
| `stark_prove_excluding_trace_time_ms` |  5,035 |  5,035 |  5,035 |  5,035 |
| `main_trace_commit_time_ms` |  813 |  813 |  813 |  813 |
| `generate_perm_trace_time_ms` |  308 |  308 |  308 |  308 |
| `perm_trace_commit_time_ms` |  661 |  661 |  661 |  661 |
| `quotient_poly_compute_time_ms` |  479 |  479 |  479 |  479 |
| `quotient_poly_commit_time_ms` |  1,160 |  1,160 |  1,160 |  1,160 |
| `pcs_opening_time_ms ` |  1,610 |  1,610 |  1,610 |  1,610 |

| root |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  31,705 |  31,705 |  31,705 |  31,705 |
| `main_cells_used     ` |  38,541,241 |  38,541,241 |  38,541,241 |  38,541,241 |
| `total_cycles        ` |  772,565 |  772,565 |  772,565 |  772,565 |
| `execute_time_ms     ` |  325 |  325 |  325 |  325 |
| `trace_gen_time_ms   ` |  370 |  370 |  370 |  370 |
| `stark_prove_excluding_trace_time_ms` |  31,010 |  31,010 |  31,010 |  31,010 |
| `main_trace_commit_time_ms` |  9,004 |  9,004 |  9,004 |  9,004 |
| `generate_perm_trace_time_ms` |  173 |  173 |  173 |  173 |
| `perm_trace_commit_time_ms` |  5,677 |  5,677 |  5,677 |  5,677 |
| `quotient_poly_compute_time_ms` |  673 |  673 |  673 |  673 |
| `quotient_poly_commit_time_ms` |  11,714 |  11,714 |  11,714 |  11,714 |
| `pcs_opening_time_ms ` |  3,755 |  3,755 |  3,755 |  3,755 |

| halo2_outer |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  55,639 |  55,639 |  55,639 |  55,639 |
| `main_cells_used     ` |  66,137,544 |  66,137,544 |  66,137,544 |  66,137,544 |

| halo2_wrapper |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  38,857 |  38,857 |  38,857 |  38,857 |

| agg_keygen |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  16,448 |  32,896 |  32,051 |  845 |
| `main_cells_used     ` |  43,232,882 |  86,465,764 |  86,455,208 |  10,556 |
| `total_cycles        ` |  1,622,323 |  1,622,323 |  1,622,323 |  1,622,323 |
| `execute_time_ms     ` |  165.50 |  331 |  331 |  0 |
| `trace_gen_time_ms   ` |  200.50 |  401 |  376 |  25 |
| `stark_prove_excluding_trace_time_ms` |  16,082 |  32,164 |  31,344 |  820 |
| `main_trace_commit_time_ms` |  4,531.50 |  9,063 |  8,975 |  88 |
| `generate_perm_trace_time_ms` |  122.50 |  245 |  210 |  35 |
| `perm_trace_commit_time_ms` |  2,890 |  5,780 |  5,697 |  83 |
| `quotient_poly_compute_time_ms` |  360.50 |  721 |  684 |  37 |
| `quotient_poly_commit_time_ms` |  5,977.50 |  11,955 |  11,837 |  118 |
| `pcs_opening_time_ms ` |  2,193 |  4,386 |  3,930 |  456 |



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
| reth.prove_evm.block_21000000 | AccessAdapterAir<2> | 21000000 | 10 | 256 |  | 16 | 11 | 6,912 | 
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
| reth.prove_evm.block_21000000 | AccessAdapterAir<4> | 21000000 | 10 | 128 |  | 16 | 13 | 3,712 | 
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
| reth.prove_evm.block_21000000 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 | 4 | 131,072 |  | 8 | 300 | 40,370,176 | 
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
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 10 | 256 |  | 40 | 37 | 19,712 | 
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
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 | 10 | 2,048 |  | 32 | 32 | 131,072 | 
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
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21000000 | 3 | 16 |  | 48 | 124 | 2,752 | 
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
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 10 | 512 |  | 52 | 36 | 45,056 | 
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
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 0 | 2,048 |  | 72 | 39 | 227,328 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 1 | 16,384 |  | 72 | 39 | 1,818,624 | 
| reth.prove_evm.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 10 | 1 |  | 72 | 39 | 111 | 
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
| agg_keygen | 21000000 | 376 | 32,051 | 1,622,323 | 270,872,042 | 31,344 | 684 | 11,837 | 5,697 | 3,930 | 1 |  | 8,975 | 86,455,208 | 8,037,489 | 20,258 | 210 |  | 331 | 
| halo2_outer | 21000000 |  | 55,639 |  |  |  |  |  |  |  |  |  |  | 66,137,544 |  |  |  |  |  | 
| halo2_wrapper | 21000000 |  | 38,857 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
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
| internal.0 | 21000000 | 0 | 1,675 | 12,246 | 3,499,873 | 472,992,226 | 8,400 | 987 | 1,693 | 1,266 | 2,422 | 1,644 | 208,470,006 | 381 |  | 2,171 | 
| internal.0 | 21000000 | 1 | 1,700 | 12,533 | 3,545,702 | 472,992,226 | 8,587 | 988 | 1,656 | 1,244 | 2,516 | 1,692 | 212,773,600 | 486 |  | 2,246 | 
| internal.0 | 21000000 | 2 | 1,687 | 12,267 | 3,530,632 | 472,992,226 | 8,421 | 995 | 1,663 | 1,257 | 2,436 | 1,645 | 211,604,935 | 420 |  | 2,159 | 
| internal.0 | 21000000 | 3 | 1,124 | 9,993 | 2,333,327 | 346,728,930 | 7,436 | 790 | 1,537 | 1,102 | 2,315 | 1,309 | 139,492,404 | 378 |  | 1,433 | 
| internal.1 | 21000000 | 4 | 1,009 | 8,448 | 2,338,720 | 302,819,810 | 6,401 | 686 | 1,321 | 926 | 2,013 | 1,142 | 115,071,446 | 307 |  | 1,038 | 
| internal.1 | 21000000 | 5 | 373 | 4,307 | 779,503 | 95,656,418 | 3,592 | 312 | 992 | 434 | 1,178 | 539 | 39,227,994 | 129 |  | 342 | 
| internal.2 | 21000000 | 6 | 682 | 6,366 | 1,542,952 | 186,591,714 | 5,035 | 479 | 1,160 | 661 | 1,610 | 813 | 75,830,346 | 308 |  | 649 | 
| leaf | 21000000 | 0 | 1,371 | 7,824 | 2,022,979 | 664,751,594 | 5,388 | 511 | 594 | 1,158 | 1,736 | 903 | 156,403,676 | 477 |  | 1,065 | 
| leaf | 21000000 | 1 | 1,387 | 8,157 | 2,102,289 | 732,909,034 | 5,687 | 562 | 651 | 1,195 | 1,788 | 1,003 | 159,296,466 | 482 |  | 1,083 | 
| leaf | 21000000 | 10 | 1,353 | 7,300 | 2,027,879 | 571,690,474 | 4,976 | 473 | 593 | 1,033 | 1,650 | 780 | 154,398,147 | 440 |  | 971 | 
| leaf | 21000000 | 2 | 1,367 | 7,520 | 2,041,955 | 571,690,474 | 5,136 | 475 | 614 | 1,022 | 1,714 | 799 | 155,641,090 | 503 |  | 1,017 | 
| leaf | 21000000 | 3 | 2,199 | 11,652 | 3,275,926 | 1,080,659,434 | 8,092 | 828 | 856 | 1,804 | 2,538 | 1,316 | 265,245,282 | 745 |  | 1,361 | 
| leaf | 21000000 | 4 | 2,364 | 12,052 | 3,518,479 | 1,100,058,090 | 8,257 | 838 | 856 | 1,849 | 2,657 | 1,398 | 284,500,751 | 653 |  | 1,431 | 
| leaf | 21000000 | 5 | 2,384 | 12,008 | 3,517,379 | 1,100,058,090 | 8,146 | 835 | 836 | 1,862 | 2,557 | 1,396 | 284,864,993 | 655 |  | 1,478 | 
| leaf | 21000000 | 6 | 2,334 | 11,860 | 3,517,540 | 1,100,058,090 | 8,073 | 834 | 849 | 1,844 | 2,538 | 1,395 | 284,710,892 | 606 |  | 1,453 | 
| leaf | 21000000 | 7 | 2,336 | 12,011 | 3,517,503 | 1,100,058,090 | 8,237 | 836 | 812 | 1,786 | 2,619 | 1,403 | 284,511,116 | 776 |  | 1,438 | 
| leaf | 21000000 | 8 | 1,498 | 8,337 | 2,320,001 | 746,278,378 | 5,721 | 569 | 623 | 1,214 | 1,783 | 1,000 | 174,398,005 | 528 |  | 1,118 | 
| leaf | 21000000 | 9 | 1,515 | 8,260 | 2,319,545 | 746,278,378 | 5,621 | 570 | 641 | 1,188 | 1,714 | 1,071 | 174,581,674 | 432 |  | 1,124 | 
| root | 21000000 | 0 | 370 | 31,705 | 772,565 | 80,435,354 | 31,010 | 673 | 11,714 | 5,677 | 3,755 | 9,004 | 38,541,241 | 173 | 3 | 325 | 

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
| agg_keygen | 21000000 | 0 | 25 | 845 |  | 7,747,601 | 820 | 37 | 118 | 83 | 456 | 88 | 10,556 | 35 | 0 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 10,872 | 30,398 | 22,248,987 | 2,755,303,538 | 16,481 | 1,971 | 1,454 | 3,276 | 4,481 | 3,878 | 1,196,838,784 | 1,410 | 3,045 | 
| reth.prove_evm.block_21000000 | 21000000 | 1 | 12,195 | 34,138 | 22,000,527 | 3,363,192,910 | 18,920 | 2,603 | 1,391 | 3,520 | 4,947 | 5,043 | 1,398,456,318 | 1,407 | 3,023 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 2,562 | 6,887 | 1,152,076 | 383,995,179 | 4,152 | 366 | 553 | 550 | 1,572 | 905 | 210,290,236 | 198 | 173 | 
| reth.prove_evm.block_21000000 | 21000000 | 2 | 7,053 | 30,698 | 10,120,301 | 3,817,479,228 | 22,137 | 5,443 | 1,018 | 3,178 | 3,513 | 7,733 | 1,575,750,920 | 1,243 | 1,508 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 5,486 | 29,430 | 7,950,619 | 3,559,333,226 | 21,402 | 5,393 | 1,150 | 3,186 | 3,472 | 7,057 | 1,390,208,779 | 1,131 | 2,542 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 7,573 | 27,175 | 17,815,996 | 2,382,503,722 | 15,850 | 2,064 | 1,538 | 3,313 | 4,560 | 3,240 | 884,176,455 | 1,117 | 3,752 | 
| reth.prove_evm.block_21000000 | 21000000 | 5 | 6,963 | 24,227 | 16,616,823 | 2,226,688,458 | 14,795 | 1,783 | 1,348 | 3,140 | 4,365 | 2,916 | 745,366,903 | 1,227 | 2,469 | 
| reth.prove_evm.block_21000000 | 21000000 | 6 | 7,106 | 24,332 | 16,703,077 | 2,197,377,242 | 14,766 | 1,676 | 1,452 | 3,116 | 4,355 | 2,935 | 762,024,084 | 1,216 | 2,460 | 
| reth.prove_evm.block_21000000 | 21000000 | 7 | 7,392 | 24,712 | 16,924,166 | 2,214,796,138 | 14,845 | 1,759 | 1,305 | 3,125 | 4,410 | 2,985 | 769,265,351 | 1,246 | 2,475 | 
| reth.prove_evm.block_21000000 | 21000000 | 8 | 7,562 | 30,110 | 13,341,393 | 3,447,403,450 | 20,040 | 3,805 | 1,363 | 3,487 | 4,466 | 5,674 | 1,481,663,637 | 1,236 | 2,508 | 
| reth.prove_evm.block_21000000 | 21000000 | 9 | 7,916 | 30,334 | 12,185,852 | 3,435,263,652 | 20,436 | 3,695 | 1,286 | 3,250 | 4,743 | 6,218 | 1,591,081,371 | 1,235 | 1,982 | 

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
| reth.prove_evm.block_21000000 | 21000000 | 0 | 0 | 52,482,060 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 1 | 148,336,640 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 2 | 26,241,030 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 3 | 175,599,620 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 4 | 7,340,032 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 5 | 3,145,728 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 6 | 62,031,872 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 7 |  | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 8 | 49,152 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 0 | 9 | 479,289,366 | 2,013,265,921 | 
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
| reth.prove_evm.block_21000000 | 21000000 | 10 | 0 | 3,041,800 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 1 | 9,157,254 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 2 | 1,520,900 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 3 | 10,227,466 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 4 | 3,407,872 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 5 | 1,310,720 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 6 | 3,178,241 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 7 |  | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 8 | 16,392 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 10 | 9 | 35,399,589 | 2,013,265,921 | 
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
| reth.prove_evm.block_21000000 | 21000000 | 3 | 0 | 23,436,836 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 1 | 115,388,032 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 2 | 11,718,418 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 3 | 144,249,732 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 4 | 4,194,304 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 5 | 2,097,152 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 6 | 97,862,160 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 7 |  | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 8 | 786,432 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 3 | 9 | 403,272,010 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 0 | 45,470,980 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 1 | 139,235,840 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 2 | 22,735,490 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 3 | 186,588,932 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 4 | 2,097,152 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 5 | 1,048,576 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 6 | 60,407,808 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 7 |  | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 8 | 2,131,968 | 2,013,265,921 | 
| reth.prove_evm.block_21000000 | 21000000 | 4 | 9 | 462,862,474 | 2,013,265,921 | 
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


Commit: https://github.com/axiom-crypto/openvm-reth-benchmark/commit/6ea1dd13bee5aad70f667d86e06501097f14edae

Max Segment Length: 8388508

Instance Type: m8g.24xlarge

Memory Allocator: jemalloc

[Benchmark Workflow](https://github.com/axiom-crypto/openvm-reth-benchmark/actions/runs/16098793202)
