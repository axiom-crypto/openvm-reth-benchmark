| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total |  540.05 |  218.90 |
| reth.prove_e2e.block_21882667 |  262.45 |  49.56 |
| leaf |  144.76 |  51.16 |
| internal.0 |  14.76 |  4.11 |
| internal.1 |  7.100 |  4 |
| internal.2 |  3.96 |  3.96 |
| internal.3 |  2.44 |  2.44 |
| root |  21.29 |  21.29 |
| halo2_outer |  47.21 |  47.21 |
| halo2_wrapper |  35.17 |  35.17 |
| agg_keygen |  21.66 |  21.24 |


| reth.prove_e2e.block_21882667 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  37,493.43 |  262,454 |  49,557 |  30,321 |
| `main_cells_used     ` |  1,203,676,353 |  8,425,734,471 |  1,822,227,092 |  851,175,341 |
| `total_cycles        ` |  8,801,130 |  8,801,130 |  8,801,130 |  8,801,130 |
| `execute_time_ms     ` |  3,264.29 |  22,850 |  5,236 |  1,432 |
| `trace_gen_time_ms   ` |  9,096.29 |  63,674 |  10,797 |  6,719 |
| `stark_prove_excluding_trace_time_ms` |  25,132.86 |  175,930 |  33,980 |  21,864 |
| `main_trace_commit_time_ms` |  6,129.43 |  42,906 |  9,382 |  4,467 |
| `generate_perm_trace_time_ms` |  802.71 |  5,619 |  1,044 |  530 |
| `perm_trace_commit_time_ms` |  3,719.43 |  26,036 |  4,734 |  2,619 |
| `quotient_poly_compute_time_ms` |  5,049.86 |  35,349 |  7,949 |  3,692 |
| `quotient_poly_commit_time_ms` |  3,588.14 |  25,117 |  4,216 |  2,519 |
| `pcs_opening_time_ms ` |  5,798.71 |  40,591 |  6,638 |  3,735 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  20,680.29 |  144,762 |  51,164 |  13,923 |
| `main_cells_used     ` |  243,979,240.71 |  1,707,854,685 |  889,319,508 |  89,391,856 |
| `total_cycles        ` |  2,890,262.57 |  20,231,838 |  9,910,466 |  1,043,537 |
| `execute_time_ms     ` |  674.86 |  4,724 |  1,995 |  337 |
| `trace_gen_time_ms   ` |  2,341 |  16,387 |  9,354 |  825 |
| `stark_prove_excluding_trace_time_ms` |  17,664.43 |  123,651 |  39,815 |  12,761 |
| `main_trace_commit_time_ms` |  2,695.57 |  18,869 |  6,333 |  1,847 |
| `generate_perm_trace_time_ms` |  354 |  2,478 |  1,040 |  217 |
| `perm_trace_commit_time_ms` |  2,463.71 |  17,246 |  6,429 |  1,531 |
| `quotient_poly_compute_time_ms` |  1,935.86 |  13,551 |  6,333 |  912 |
| `quotient_poly_commit_time_ms` |  3,100.57 |  21,704 |  7,547 |  2,156 |
| `pcs_opening_time_ms ` |  7,110.71 |  49,775 |  12,129 |  6,094 |

| internal.0 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  3,691 |  14,764 |  4,105 |  2,483 |
| `main_cells_used     ` |  63,469,993.50 |  253,879,974 |  72,338,218 |  37,046,931 |
| `total_cycles        ` |  1,110,531.25 |  4,442,125 |  1,270,699 |  634,940 |
| `execute_time_ms     ` |  433.75 |  1,735 |  499 |  250 |
| `trace_gen_time_ms   ` |  502.75 |  2,011 |  584 |  298 |
| `stark_prove_excluding_trace_time_ms` |  2,754.50 |  11,018 |  3,041 |  1,935 |
| `main_trace_commit_time_ms` |  466.50 |  1,866 |  522 |  307 |
| `generate_perm_trace_time_ms` |  74 |  296 |  84 |  47 |
| `perm_trace_commit_time_ms` |  409.50 |  1,638 |  458 |  272 |
| `quotient_poly_compute_time_ms` |  397.50 |  1,590 |  446 |  253 |
| `quotient_poly_commit_time_ms` |  621.50 |  2,486 |  682 |  468 |
| `pcs_opening_time_ms ` |  781.25 |  3,125 |  852 |  583 |

| internal.1 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  3,998 |  7,996 |  4,001 |  3,995 |
| `main_cells_used     ` |  66,110,989 |  132,221,978 |  66,387,034 |  65,834,944 |
| `total_cycles        ` |  1,210,325.50 |  2,420,651 |  1,214,172 |  1,206,479 |
| `execute_time_ms     ` |  399 |  798 |  404 |  394 |
| `trace_gen_time_ms   ` |  541 |  1,082 |  545 |  537 |
| `stark_prove_excluding_trace_time_ms` |  3,058 |  6,116 |  3,070 |  3,046 |
| `main_trace_commit_time_ms` |  527.50 |  1,055 |  530 |  525 |
| `generate_perm_trace_time_ms` |  84 |  168 |  84 |  84 |
| `perm_trace_commit_time_ms` |  456.50 |  913 |  457 |  456 |
| `quotient_poly_compute_time_ms` |  451 |  902 |  460 |  442 |
| `quotient_poly_commit_time_ms` |  677.50 |  1,355 |  679 |  676 |
| `pcs_opening_time_ms ` |  858 |  1,716 |  860 |  856 |

| internal.2 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  3,955 |  3,955 |  3,955 |  3,955 |
| `main_cells_used     ` |  66,404,487 |  66,404,487 |  66,404,487 |  66,404,487 |
| `total_cycles        ` |  1,214,066 |  1,214,066 |  1,214,066 |  1,214,066 |
| `execute_time_ms     ` |  402 |  402 |  402 |  402 |
| `trace_gen_time_ms   ` |  543 |  543 |  543 |  543 |
| `stark_prove_excluding_trace_time_ms` |  3,010 |  3,010 |  3,010 |  3,010 |
| `main_trace_commit_time_ms` |  512 |  512 |  512 |  512 |
| `generate_perm_trace_time_ms` |  91 |  91 |  91 |  91 |
| `perm_trace_commit_time_ms` |  448 |  448 |  448 |  448 |
| `quotient_poly_compute_time_ms` |  438 |  438 |  438 |  438 |
| `quotient_poly_commit_time_ms` |  677 |  677 |  677 |  677 |
| `pcs_opening_time_ms ` |  839 |  839 |  839 |  839 |

| internal.3 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  2,443 |  2,443 |  2,443 |  2,443 |
| `main_cells_used     ` |  33,968,422 |  33,968,422 |  33,968,422 |  33,968,422 |
| `total_cycles        ` |  607,274 |  607,274 |  607,274 |  607,274 |
| `execute_time_ms     ` |  203 |  203 |  203 |  203 |
| `trace_gen_time_ms   ` |  284 |  284 |  284 |  284 |
| `stark_prove_excluding_trace_time_ms` |  1,956 |  1,956 |  1,956 |  1,956 |
| `main_trace_commit_time_ms` |  308 |  308 |  308 |  308 |
| `generate_perm_trace_time_ms` |  54 |  54 |  54 |  54 |
| `perm_trace_commit_time_ms` |  277 |  277 |  277 |  277 |
| `quotient_poly_compute_time_ms` |  256 |  256 |  256 |  256 |
| `quotient_poly_commit_time_ms` |  476 |  476 |  476 |  476 |
| `pcs_opening_time_ms ` |  581 |  581 |  581 |  581 |

| root |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  21,289 |  21,289 |  21,289 |  21,289 |
| `main_cells_used     ` |  33,387,829 |  33,387,829 |  33,387,829 |  33,387,829 |
| `total_cycles        ` |  600,105 |  600,105 |  600,105 |  600,105 |
| `execute_time_ms     ` |  195 |  195 |  195 |  195 |
| `trace_gen_time_ms   ` |  280 |  280 |  280 |  280 |
| `stark_prove_excluding_trace_time_ms` |  20,814 |  20,814 |  20,814 |  20,814 |
| `main_trace_commit_time_ms` |  6,167 |  6,167 |  6,167 |  6,167 |
| `generate_perm_trace_time_ms` |  45 |  45 |  45 |  45 |
| `perm_trace_commit_time_ms` |  4,317 |  4,317 |  4,317 |  4,317 |
| `quotient_poly_compute_time_ms` |  508 |  508 |  508 |  508 |
| `quotient_poly_commit_time_ms` |  7,405 |  7,405 |  7,405 |  7,405 |
| `pcs_opening_time_ms ` |  2,367 |  2,367 |  2,367 |  2,367 |

| halo2_outer |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  47,211 |  47,211 |  47,211 |  47,211 |
| `main_cells_used     ` |  58,076,647 |  58,076,647 |  58,076,647 |  58,076,647 |

| halo2_wrapper |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  35,174 |  35,174 |  35,174 |  35,174 |

| agg_keygen |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  10,830.50 |  21,661 |  21,238 |  423 |
| `main_cells_used     ` |  41,063,645.50 |  82,127,291 |  81,199,222 |  928,069 |
| `total_cycles        ` |  1,357,892 |  1,357,892 |  1,357,892 |  1,357,892 |
| `execute_time_ms     ` |  98 |  196 |  196 |  0 |
| `trace_gen_time_ms   ` |  156 |  312 |  284 |  28 |
| `stark_prove_excluding_trace_time_ms` |  10,576.50 |  21,153 |  20,758 |  395 |
| `main_trace_commit_time_ms` |  3,124.50 |  6,249 |  6,197 |  52 |
| `generate_perm_trace_time_ms` |  30.50 |  61 |  48 |  13 |
| `perm_trace_commit_time_ms` |  2,183 |  4,366 |  4,320 |  46 |
| `quotient_poly_compute_time_ms` |  267 |  534 |  504 |  30 |
| `quotient_poly_commit_time_ms` |  3,710.50 |  7,421 |  7,364 |  57 |
| `pcs_opening_time_ms ` |  1,256.50 |  2,513 |  2,321 |  192 |



<details>
<summary>Detailed Metrics</summary>

| air_name | block_number | quotient_deg | interactions | constraints |
| --- | --- | --- | --- | --- |
| AccessAdapterAir<16> | 21882667 | 4 | 5 | 11 | 
| AccessAdapterAir<2> | 21882667 | 4 | 5 | 11 | 
| AccessAdapterAir<32> | 21882667 | 4 | 5 | 11 | 
| AccessAdapterAir<4> | 21882667 | 4 | 5 | 11 | 
| AccessAdapterAir<64> | 21882667 | 4 | 5 | 11 | 
| AccessAdapterAir<8> | 21882667 | 4 | 5 | 11 | 
| BitwiseOperationLookupAir<8> | 21882667 | 2 | 2 | 4 | 
| KeccakVmAir | 21882667 | 4 | 321 | 4,380 | 
| MemoryMerkleAir<8> | 21882667 | 4 | 4 | 38 | 
| PersistentBoundaryAir<8> | 21882667 | 4 | 3 | 5 | 
| PhantomAir | 21882667 | 4 | 3 | 4 | 
| Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 2 | 1 | 286 | 
| ProgramAir | 21882667 | 1 | 1 | 4 | 
| RangeTupleCheckerAir<2> | 21882667 | 1 | 1 | 4 | 
| Rv32HintStoreAir | 21882667 | 4 | 19 | 21 | 
| Sha256VmAir | 21882667 | 4 | 34 | 759 | 
| VariableRangeCheckerAir | 21882667 | 1 | 1 | 4 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 4 | 19 | 30 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 4 | 17 | 35 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 4 | 23 | 84 | 
| VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 4 | 11 | 17 | 
| VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 4 | 13 | 32 | 
| VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 4 | 10 | 15 | 
| VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21882667 | 4 | 61 | 103 | 
| VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21882667 | 4 | 31 | 121 | 
| VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21882667 | 4 | 61 | 34 | 
| VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21882667 | 4 | 79 | 2,141 | 
| VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21882667 | 4 | 20 | 50 | 
| VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchLessThanCoreAir<32, 8> | 21882667 | 4 | 22 | 120 | 
| VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21882667 | 4 | 25 | 217 | 
| VmAirWrapper<Rv32IsEqualModAdapterAir<2, 3, 16, 48>, ModularIsEqualCoreAir<48, 4, 8> | 21882667 | 4 | 41 | 321 | 
| VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 4 | 16 | 16 | 
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 4 | 18 | 21 | 
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 4 | 17 | 27 | 
| VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21882667 | 4 | 25 | 72 | 
| VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 4 | 24 | 23 | 
| VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 4 | 19 | 13 | 
| VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 4 | 11 | 12 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 12, 24, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 2,642 | 1,981 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 411 | 378 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 4, 8, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 1,716 | 1,310 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 6, 6, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 824 | 714 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 156 | 150 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 12, 12, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 4,370 | 3,323 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 12, 30, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 1,767 | 1,295 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 12, 36, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 4,662 | 3,503 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 422 | 351 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 3, 3, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 244 | 226 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 36, 36, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 9,235 | 6,932 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 4, 10, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 1,303 | 988 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 4, 12, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 2,903 | 2,221 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 6, 6, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 662 | 535 | 
| VmAirWrapper<Rv32VecHeapTwoReadsAdapterAir<12, 10, 12, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 3,977 | 3,023 | 
| VmAirWrapper<Rv32VecHeapTwoReadsAdapterAir<12, 6, 12, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 901 | 655 | 
| VmAirWrapper<Rv32VecHeapTwoReadsAdapterAir<36, 30, 36, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 8,447 | 6,337 | 
| VmAirWrapper<Rv32VecHeapTwoReadsAdapterAir<4, 2, 4, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 565 | 423 | 
| VmConnectorAir | 21882667 | 4 | 3 | 8 | 

| block_number | execute_time_ms |
| --- | --- |
| 21882667 | 196 | 

| group | air_name | block_number | rows | quotient_deg | prep_cols | perm_cols | main_cols | interactions | constraints | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | AccessAdapterAir<16> | 21882667 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<2> | 21882667 | 524,288 | 8 |  | 12 | 11 | 5 | 12 | 12,058,624 | 
| agg_keygen | AccessAdapterAir<32> | 21882667 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<4> | 21882667 | 262,144 | 8 |  | 12 | 13 | 5 | 12 | 6,553,600 | 
| agg_keygen | AccessAdapterAir<64> | 21882667 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<8> | 21882667 | 512 | 8 |  | 12 | 17 | 5 | 12 | 14,848 | 
| agg_keygen | BitwiseOperationLookupAir<8> | 21882667 |  | 2 |  |  |  | 2 | 4 |  | 
| agg_keygen | FriReducedOpeningAir | 21882667 | 524,288 | 8 |  | 44 | 27 | 39 | 60 | 37,224,448 | 
| agg_keygen | MemoryMerkleAir<8> | 21882667 |  | 2 |  |  |  | 4 | 39 |  | 
| agg_keygen | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 65,536 | 8 |  | 160 | 399 | 136 | 530 | 36,634,624 | 
| agg_keygen | PersistentBoundaryAir<8> | 21882667 |  | 2 |  |  |  | 3 | 6 |  | 
| agg_keygen | PhantomAir | 21882667 | 16,384 | 4 |  | 8 | 6 | 3 | 5 | 229,376 | 
| agg_keygen | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 |  | 2 |  |  |  | 1 | 286 |  | 
| agg_keygen | ProgramAir | 21882667 | 262,144 | 1 |  | 8 | 10 | 1 | 4 | 4,718,592 | 
| agg_keygen | RangeTupleCheckerAir<2> | 21882667 |  | 1 |  |  |  | 1 | 4 |  | 
| agg_keygen | Rv32HintStoreAir | 21882667 |  | 2 |  |  |  | 19 | 26 |  | 
| agg_keygen | VariableRangeCheckerAir | 21882667 | 262,144 | 1 | 2 | 8 | 1 | 1 | 4 | 2,359,296 | 
| agg_keygen | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 1,048,576 | 8 |  | 20 | 29 | 15 | 23 | 51,380,224 | 
| agg_keygen | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 131,072 | 8 |  | 16 | 23 | 11 | 22 | 5,111,808 | 
| agg_keygen | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 32,768 | 8 |  | 12 | 9 | 7 | 6 | 688,128 | 
| agg_keygen | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 64 | 8 |  | 16 | 23 | 11 | 23 | 2,496 | 
| agg_keygen | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 524,288 | 8 |  | 24 | 22 | 15 | 16 | 24,117,248 | 
| agg_keygen | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 131,072 | 8 |  | 24 | 31 | 15 | 16 | 7,208,960 | 
| agg_keygen | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 131,072 | 8 |  | 20 | 38 | 15 | 23 | 7,602,176 | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 |  | 2 |  |  |  | 19 | 36 |  | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 |  | 2 |  |  |  | 17 | 39 |  | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 |  | 2 |  |  |  | 23 | 90 |  | 
| agg_keygen | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 |  | 2 |  |  |  | 11 | 20 |  | 
| agg_keygen | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 |  | 2 |  |  |  | 13 | 35 |  | 
| agg_keygen | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 |  | 2 |  |  |  | 10 | 18 |  | 
| agg_keygen | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 |  | 2 |  |  |  | 16 | 20 |  | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 |  | 2 |  |  |  | 18 | 26 |  | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 |  | 2 |  |  |  | 17 | 33 |  | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21882667 |  | 2 |  |  |  | 25 | 80 |  | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 |  | 2 |  |  |  | 24 | 31 |  | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 |  | 2 |  |  |  | 19 | 19 |  | 
| agg_keygen | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 |  | 2 |  |  |  | 11 | 15 |  | 
| agg_keygen | VmConnectorAir | 21882667 | 2 | 4 | 1 | 8 | 4 | 3 | 9 | 24 | 
| agg_keygen | VolatileBoundaryAir | 21882667 | 131,072 | 4 |  | 8 | 11 | 4 | 16 | 2,490,368 | 

| group | air_name | block_number | idx | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | AccessAdapterAir<2> | 21882667 | 0 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21882667 | 1 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21882667 | 2 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21882667 | 3 | 262,144 |  | 12 | 11 | 6,029,312 | 
| internal.0 | AccessAdapterAir<4> | 21882667 | 0 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21882667 | 1 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21882667 | 2 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21882667 | 3 | 131,072 |  | 12 | 13 | 3,276,800 | 
| internal.0 | AccessAdapterAir<8> | 21882667 | 0 | 512 |  | 12 | 17 | 14,848 | 
| internal.0 | AccessAdapterAir<8> | 21882667 | 1 | 512 |  | 12 | 17 | 14,848 | 
| internal.0 | AccessAdapterAir<8> | 21882667 | 2 | 512 |  | 12 | 17 | 14,848 | 
| internal.0 | AccessAdapterAir<8> | 21882667 | 3 | 256 |  | 12 | 17 | 7,424 | 
| internal.0 | FriReducedOpeningAir | 21882667 | 0 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.0 | FriReducedOpeningAir | 21882667 | 1 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.0 | FriReducedOpeningAir | 21882667 | 2 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.0 | FriReducedOpeningAir | 21882667 | 3 | 131,072 |  | 44 | 27 | 9,306,112 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 0 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 1 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 2 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 3 | 32,768 |  | 160 | 399 | 18,317,312 | 
| internal.0 | PhantomAir | 21882667 | 0 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.0 | PhantomAir | 21882667 | 1 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.0 | PhantomAir | 21882667 | 2 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.0 | PhantomAir | 21882667 | 3 | 8,192 |  | 8 | 6 | 114,688 | 
| internal.0 | ProgramAir | 21882667 | 0 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | ProgramAir | 21882667 | 1 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | ProgramAir | 21882667 | 2 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | ProgramAir | 21882667 | 3 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | VariableRangeCheckerAir | 21882667 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21882667 | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21882667 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21882667 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 0 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 1 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 2 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 3 | 524,288 |  | 20 | 29 | 25,690,112 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 0 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 1 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 2 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 3 | 65,536 |  | 16 | 23 | 2,555,904 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 0 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 1 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 2 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 3 | 16,384 |  | 12 | 9 | 344,064 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 0 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 1 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 2 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 3 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 0 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 1 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 2 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 3 | 131,072 |  | 24 | 22 | 6,029,312 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 0 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 1 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 2 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 3 | 65,536 |  | 24 | 31 | 3,604,480 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 0 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 1 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 2 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 3 | 65,536 |  | 20 | 38 | 3,801,088 | 
| internal.0 | VmConnectorAir | 21882667 | 0 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21882667 | 1 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21882667 | 2 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21882667 | 3 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VolatileBoundaryAir | 21882667 | 0 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.0 | VolatileBoundaryAir | 21882667 | 1 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.0 | VolatileBoundaryAir | 21882667 | 2 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.0 | VolatileBoundaryAir | 21882667 | 3 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.1 | AccessAdapterAir<2> | 21882667 | 4 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.1 | AccessAdapterAir<2> | 21882667 | 5 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.1 | AccessAdapterAir<4> | 21882667 | 4 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.1 | AccessAdapterAir<4> | 21882667 | 5 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.1 | AccessAdapterAir<8> | 21882667 | 4 | 512 |  | 12 | 17 | 14,848 | 
| internal.1 | AccessAdapterAir<8> | 21882667 | 5 | 512 |  | 12 | 17 | 14,848 | 
| internal.1 | FriReducedOpeningAir | 21882667 | 4 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.1 | FriReducedOpeningAir | 21882667 | 5 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 4 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 5 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.1 | PhantomAir | 21882667 | 4 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.1 | PhantomAir | 21882667 | 5 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.1 | ProgramAir | 21882667 | 4 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.1 | ProgramAir | 21882667 | 5 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.1 | VariableRangeCheckerAir | 21882667 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 21882667 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 4 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 5 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 4 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 5 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 4 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 5 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 4 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 5 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 4 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 5 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 4 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 5 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 4 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 5 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.1 | VmConnectorAir | 21882667 | 4 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VmConnectorAir | 21882667 | 5 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VolatileBoundaryAir | 21882667 | 4 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.1 | VolatileBoundaryAir | 21882667 | 5 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.2 | AccessAdapterAir<2> | 21882667 | 6 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.2 | AccessAdapterAir<4> | 21882667 | 6 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.2 | AccessAdapterAir<8> | 21882667 | 6 | 512 |  | 12 | 17 | 14,848 | 
| internal.2 | FriReducedOpeningAir | 21882667 | 6 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.2 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 6 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.2 | PhantomAir | 21882667 | 6 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.2 | ProgramAir | 21882667 | 6 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.2 | VariableRangeCheckerAir | 21882667 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.2 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 6 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.2 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 6 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.2 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 6 | 16,384 |  | 12 | 9 | 344,064 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 6 | 64 |  | 16 | 23 | 2,496 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 6 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 6 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 6 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.2 | VmConnectorAir | 21882667 | 6 | 2 | 1 | 8 | 4 | 24 | 
| internal.2 | VolatileBoundaryAir | 21882667 | 6 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.3 | AccessAdapterAir<2> | 21882667 | 7 | 262,144 |  | 12 | 11 | 6,029,312 | 
| internal.3 | AccessAdapterAir<4> | 21882667 | 7 | 131,072 |  | 12 | 13 | 3,276,800 | 
| internal.3 | AccessAdapterAir<8> | 21882667 | 7 | 256 |  | 12 | 17 | 7,424 | 
| internal.3 | FriReducedOpeningAir | 21882667 | 7 | 131,072 |  | 44 | 27 | 9,306,112 | 
| internal.3 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 7 | 32,768 |  | 160 | 399 | 18,317,312 | 
| internal.3 | PhantomAir | 21882667 | 7 | 8,192 |  | 8 | 6 | 114,688 | 
| internal.3 | ProgramAir | 21882667 | 7 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.3 | VariableRangeCheckerAir | 21882667 | 7 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.3 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 7 | 524,288 |  | 20 | 29 | 25,690,112 | 
| internal.3 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 7 | 65,536 |  | 16 | 23 | 2,555,904 | 
| internal.3 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 7 | 16,384 |  | 12 | 9 | 344,064 | 
| internal.3 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 7 | 64 |  | 16 | 23 | 2,496 | 
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 7 | 131,072 |  | 24 | 22 | 6,029,312 | 
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 7 | 65,536 |  | 24 | 31 | 3,604,480 | 
| internal.3 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 7 | 65,536 |  | 20 | 38 | 3,801,088 | 
| internal.3 | VmConnectorAir | 21882667 | 7 | 2 | 1 | 8 | 4 | 24 | 
| internal.3 | VolatileBoundaryAir | 21882667 | 7 | 131,072 |  | 8 | 11 | 2,490,368 | 
| leaf | AccessAdapterAir<2> | 21882667 | 0 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 1 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 2 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 3 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 4 | 16,777,216 |  | 12 | 11 | 385,875,968 | 
| leaf | AccessAdapterAir<2> | 21882667 | 5 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 6 | 2,097,152 |  | 12 | 11 | 48,234,496 | 
| leaf | AccessAdapterAir<4> | 21882667 | 0 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 1 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 2 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 3 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 4 | 8,388,608 |  | 12 | 13 | 209,715,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 5 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 6 | 1,048,576 |  | 12 | 13 | 26,214,400 | 
| leaf | AccessAdapterAir<8> | 21882667 | 0 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 1 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 2 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 3 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 4 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 5 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 6 | 512 |  | 12 | 17 | 14,848 | 
| leaf | FriReducedOpeningAir | 21882667 | 0 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| leaf | FriReducedOpeningAir | 21882667 | 1 | 524,288 |  | 44 | 27 | 37,224,448 | 
| leaf | FriReducedOpeningAir | 21882667 | 2 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| leaf | FriReducedOpeningAir | 21882667 | 3 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| leaf | FriReducedOpeningAir | 21882667 | 4 | 8,388,608 |  | 44 | 27 | 595,591,168 | 
| leaf | FriReducedOpeningAir | 21882667 | 5 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| leaf | FriReducedOpeningAir | 21882667 | 6 | 2,097,152 |  | 44 | 27 | 148,897,792 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 0 | 131,072 |  | 160 | 399 | 73,269,248 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 1 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 2 | 131,072 |  | 160 | 399 | 73,269,248 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 3 | 131,072 |  | 160 | 399 | 73,269,248 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 4 | 524,288 |  | 160 | 399 | 293,076,992 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 5 | 131,072 |  | 160 | 399 | 73,269,248 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 6 | 131,072 |  | 160 | 399 | 73,269,248 | 
| leaf | PhantomAir | 21882667 | 0 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 1 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 2 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 3 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 4 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 5 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 6 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | ProgramAir | 21882667 | 0 | 16,777,216 |  | 8 | 10 | 301,989,888 | 
| leaf | ProgramAir | 21882667 | 1 | 16,777,216 |  | 8 | 10 | 301,989,888 | 
| leaf | ProgramAir | 21882667 | 2 | 16,777,216 |  | 8 | 10 | 301,989,888 | 
| leaf | ProgramAir | 21882667 | 3 | 16,777,216 |  | 8 | 10 | 301,989,888 | 
| leaf | ProgramAir | 21882667 | 4 | 16,777,216 |  | 8 | 10 | 301,989,888 | 
| leaf | ProgramAir | 21882667 | 5 | 16,777,216 |  | 8 | 10 | 301,989,888 | 
| leaf | ProgramAir | 21882667 | 6 | 16,777,216 |  | 8 | 10 | 301,989,888 | 
| leaf | VariableRangeCheckerAir | 21882667 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 0 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 1 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 2 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 3 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 4 | 8,388,608 |  | 20 | 29 | 411,041,792 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 5 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 6 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 0 | 262,144 |  | 16 | 23 | 10,223,616 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 1 | 131,072 |  | 16 | 23 | 5,111,808 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 2 | 262,144 |  | 16 | 23 | 10,223,616 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 3 | 262,144 |  | 16 | 23 | 10,223,616 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 4 | 1,048,576 |  | 16 | 23 | 40,894,464 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 5 | 262,144 |  | 16 | 23 | 10,223,616 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 6 | 262,144 |  | 16 | 23 | 10,223,616 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 0 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 1 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 2 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 3 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 4 | 32,768 |  | 12 | 9 | 688,128 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 5 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 6 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 0 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 1 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 2 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 3 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 4 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 5 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 6 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 0 | 262,144 |  | 24 | 22 | 12,058,624 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 1 | 262,144 |  | 24 | 22 | 12,058,624 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 2 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 3 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 4 | 1,048,576 |  | 24 | 22 | 48,234,496 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 5 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 6 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 0 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 1 | 65,536 |  | 24 | 31 | 3,604,480 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 2 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 3 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 4 | 262,144 |  | 24 | 31 | 14,417,920 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 5 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 6 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 0 | 262,144 |  | 20 | 38 | 15,204,352 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 1 | 262,144 |  | 20 | 38 | 15,204,352 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 2 | 524,288 |  | 20 | 38 | 30,408,704 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 3 | 524,288 |  | 20 | 38 | 30,408,704 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 4 | 4,194,304 |  | 20 | 38 | 243,269,632 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 5 | 524,288 |  | 20 | 38 | 30,408,704 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 6 | 524,288 |  | 20 | 38 | 30,408,704 | 
| leaf | VmConnectorAir | 21882667 | 0 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 1 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 2 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 3 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 4 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 5 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 6 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VolatileBoundaryAir | 21882667 | 0 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21882667 | 1 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 21882667 | 2 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21882667 | 3 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21882667 | 4 | 8,388,608 |  | 8 | 11 | 159,383,552 | 
| leaf | VolatileBoundaryAir | 21882667 | 5 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21882667 | 6 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| root | AccessAdapterAir<2> | 21882667 | 0 | 262,144 |  | 8 | 11 | 4,980,736 | 
| root | AccessAdapterAir<4> | 21882667 | 0 | 131,072 |  | 8 | 13 | 2,752,512 | 
| root | AccessAdapterAir<8> | 21882667 | 0 | 256 |  | 8 | 17 | 6,400 | 
| root | FriReducedOpeningAir | 21882667 | 0 | 131,072 |  | 24 | 27 | 6,684,672 | 
| root | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 0 | 32,768 |  | 84 | 399 | 15,826,944 | 
| root | PhantomAir | 21882667 | 0 | 8,192 |  | 8 | 6 | 114,688 | 
| root | ProgramAir | 21882667 | 0 | 262,144 |  | 8 | 10 | 4,718,592 | 
| root | VariableRangeCheckerAir | 21882667 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| root | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 0 | 524,288 |  | 12 | 29 | 21,495,808 | 
| root | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 0 | 65,536 |  | 12 | 23 | 2,293,760 | 
| root | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 0 | 8,192 |  | 8 | 9 | 139,264 | 
| root | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 0 | 64 |  | 12 | 22 | 2,176 | 
| root | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 0 | 131,072 |  | 16 | 22 | 4,980,736 | 
| root | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 0 | 65,536 |  | 16 | 31 | 3,080,192 | 
| root | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 0 | 65,536 |  | 12 | 38 | 3,276,800 | 
| root | VmConnectorAir | 21882667 | 0 | 2 | 1 | 8 | 4 | 24 | 
| root | VolatileBoundaryAir | 21882667 | 0 | 131,072 |  | 8 | 11 | 2,490,368 | 

| group | air_name | block_number | segment | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | AccessAdapterAir<16> | 21882667 | 0 | 1 |  | 16 | 25 | 41 | 
| agg_keygen | AccessAdapterAir<2> | 21882667 | 0 | 1 |  | 16 | 11 | 27 | 
| agg_keygen | AccessAdapterAir<32> | 21882667 | 0 | 1 |  | 16 | 41 | 57 | 
| agg_keygen | AccessAdapterAir<4> | 21882667 | 0 | 1 |  | 16 | 13 | 29 | 
| agg_keygen | AccessAdapterAir<64> | 21882667 | 0 | 1 |  | 16 | 73 | 89 | 
| agg_keygen | AccessAdapterAir<8> | 21882667 | 0 | 1 |  | 16 | 17 | 33 | 
| agg_keygen | BitwiseOperationLookupAir<8> | 21882667 | 0 | 65,536 | 3 | 8 | 2 | 655,360 | 
| agg_keygen | MemoryMerkleAir<8> | 21882667 | 0 | 64 |  | 16 | 32 | 3,072 | 
| agg_keygen | PersistentBoundaryAir<8> | 21882667 | 0 | 1 |  | 12 | 20 | 32 | 
| agg_keygen | PhantomAir | 21882667 | 0 | 1 |  | 12 | 6 | 18 | 
| agg_keygen | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 0 | 32 |  | 8 | 300 | 9,856 | 
| agg_keygen | ProgramAir | 21882667 | 0 | 1 |  | 8 | 10 | 18 | 
| agg_keygen | RangeTupleCheckerAir<2> | 21882667 | 0 | 524,288 | 2 | 8 | 1 | 4,718,592 | 
| agg_keygen | Rv32HintStoreAir | 21882667 | 0 | 1 |  | 44 | 32 | 76 | 
| agg_keygen | VariableRangeCheckerAir | 21882667 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 0 | 1 |  | 52 | 36 | 88 | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 0 | 1 |  | 40 | 37 | 77 | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 0 | 1 |  | 52 | 53 | 105 | 
| agg_keygen | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 0 | 1 |  | 28 | 26 | 54 | 
| agg_keygen | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 0 | 1 |  | 32 | 32 | 64 | 
| agg_keygen | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 0 | 1 |  | 28 | 18 | 46 | 
| agg_keygen | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 0 | 1 |  | 36 | 28 | 64 | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 0 | 1 |  | 48 | 35 | 83 | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 0 | 1 |  | 52 | 40 | 92 | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21882667 | 0 | 1 |  | 72 | 57 | 129 | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 0 | 1 |  | 72 | 39 | 111 | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 0 | 1 |  | 52 | 31 | 83 | 
| agg_keygen | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 0 | 1 |  | 28 | 21 | 49 | 
| agg_keygen | VmConnectorAir | 21882667 | 0 | 2 | 1 | 12 | 4 | 32 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<16> | 21882667 | 0 | 64 |  | 12 | 25 | 2,368 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<16> | 21882667 | 2 | 524,288 |  | 12 | 25 | 19,398,656 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<16> | 21882667 | 3 | 524,288 |  | 12 | 25 | 19,398,656 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<16> | 21882667 | 4 | 524,288 |  | 12 | 25 | 19,398,656 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<16> | 21882667 | 5 | 262,144 |  | 12 | 25 | 9,699,328 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<16> | 21882667 | 6 | 512 |  | 12 | 25 | 18,944 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<2> | 21882667 | 3 | 8,192 |  | 12 | 11 | 188,416 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<2> | 21882667 | 5 | 65,536 |  | 12 | 11 | 1,507,328 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<2> | 21882667 | 6 | 262,144 |  | 12 | 11 | 6,029,312 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<32> | 21882667 | 0 | 32 |  | 12 | 41 | 1,696 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<32> | 21882667 | 2 | 262,144 |  | 12 | 41 | 13,893,632 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<32> | 21882667 | 3 | 262,144 |  | 12 | 41 | 13,893,632 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<32> | 21882667 | 4 | 131,072 |  | 12 | 41 | 6,946,816 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<32> | 21882667 | 5 | 131,072 |  | 12 | 41 | 6,946,816 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<32> | 21882667 | 6 | 256 |  | 12 | 41 | 13,568 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<4> | 21882667 | 0 | 64 |  | 12 | 13 | 1,600 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<4> | 21882667 | 3 | 8,192 |  | 12 | 13 | 204,800 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<4> | 21882667 | 4 | 16,384 |  | 12 | 13 | 409,600 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<4> | 21882667 | 5 | 32,768 |  | 12 | 13 | 819,200 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<4> | 21882667 | 6 | 131,072 |  | 12 | 13 | 3,276,800 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 0 | 1,048,576 |  | 12 | 17 | 30,408,704 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 1 | 1,048,576 |  | 12 | 17 | 30,408,704 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 2 | 2,097,152 |  | 12 | 17 | 60,817,408 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 3 | 2,097,152 |  | 12 | 17 | 60,817,408 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 4 | 2,097,152 |  | 12 | 17 | 60,817,408 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 5 | 2,097,152 |  | 12 | 17 | 60,817,408 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 6 | 1,048,576 |  | 12 | 17 | 30,408,704 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 0 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 1 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 2 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 3 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 4 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 5 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 6 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 0 | 1 |  | 532 | 3,163 | 3,695 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 1 | 1 |  | 532 | 3,163 | 3,695 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 2 | 262,144 |  | 532 | 3,163 | 968,622,080 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 3 | 16,384 |  | 532 | 3,163 | 60,538,880 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 4 | 16,384 |  | 532 | 3,163 | 60,538,880 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 5 | 131,072 |  | 532 | 3,163 | 484,311,040 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 6 | 262,144 |  | 532 | 3,163 | 968,622,080 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 0 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 1 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 2 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 3 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 4 | 524,288 |  | 12 | 32 | 23,068,672 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 5 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 6 | 2,097,152 |  | 12 | 32 | 92,274,688 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 0 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 1 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 2 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 3 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 4 | 524,288 |  | 8 | 20 | 14,680,064 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 5 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 6 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 0 | 65,536 |  | 8 | 6 | 917,504 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 1 | 65,536 |  | 8 | 6 | 917,504 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 2 | 32,768 |  | 8 | 6 | 458,752 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 3 | 4,096 |  | 8 | 6 | 57,344 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 4 | 2,048 |  | 8 | 6 | 28,672 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 5 | 8,192 |  | 8 | 6 | 114,688 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 6 | 4,096 |  | 8 | 6 | 57,344 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 0 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 1 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 2 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 3 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 4 | 262,144 |  | 8 | 300 | 80,740,352 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 5 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 6 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 0 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 1 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 2 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 3 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 4 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 5 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 6 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 0 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 1 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 2 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 3 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 4 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 5 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 6 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | Rv32HintStoreAir | 21882667 | 0 | 524,288 |  | 24 | 32 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | Rv32HintStoreAir | 21882667 | 1 | 524,288 |  | 24 | 32 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | Rv32HintStoreAir | 21882667 | 2 | 524,288 |  | 24 | 32 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | Rv32HintStoreAir | 21882667 | 3 | 128 |  | 24 | 32 | 7,168 | 
| reth.prove_e2e.block_21882667 | Rv32HintStoreAir | 21882667 | 4 | 512 |  | 24 | 32 | 28,672 | 
| reth.prove_e2e.block_21882667 | Rv32HintStoreAir | 21882667 | 5 | 128 |  | 24 | 32 | 7,168 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 0 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 1 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 2 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 3 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 4 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 5 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 6 | 4,194,304 |  | 28 | 36 | 268,435,456 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 0 | 524,288 |  | 24 | 37 | 31,981,568 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 1 | 524,288 |  | 24 | 37 | 31,981,568 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 2 | 524,288 |  | 24 | 37 | 31,981,568 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 3 | 524,288 |  | 24 | 37 | 31,981,568 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 4 | 1,048,576 |  | 24 | 37 | 63,963,136 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 5 | 524,288 |  | 24 | 37 | 31,981,568 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 6 | 262,144 |  | 24 | 37 | 15,990,784 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 0 | 1,048,576 |  | 28 | 53 | 84,934,656 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 1 | 1,048,576 |  | 28 | 53 | 84,934,656 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 2 | 2,097,152 |  | 28 | 53 | 169,869,312 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 3 | 2,097,152 |  | 28 | 53 | 169,869,312 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 4 | 1,048,576 |  | 28 | 53 | 84,934,656 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 5 | 2,097,152 |  | 28 | 53 | 169,869,312 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 6 | 262,144 |  | 28 | 53 | 21,233,664 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 0 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 1 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 2 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 3 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 4 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 5 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 6 | 1,048,576 |  | 16 | 26 | 44,040,192 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 0 | 1,048,576 |  | 20 | 32 | 54,525,952 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 1 | 1,048,576 |  | 20 | 32 | 54,525,952 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 2 | 2,097,152 |  | 20 | 32 | 109,051,904 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 3 | 2,097,152 |  | 20 | 32 | 109,051,904 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 4 | 2,097,152 |  | 20 | 32 | 109,051,904 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 5 | 2,097,152 |  | 20 | 32 | 109,051,904 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 6 | 131,072 |  | 20 | 32 | 6,815,744 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 0 | 1,048,576 |  | 16 | 18 | 35,651,584 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 1 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 2 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 3 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 4 | 262,144 |  | 16 | 18 | 8,912,896 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 5 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 6 | 131,072 |  | 16 | 18 | 4,456,448 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21882667 | 2 | 2,048 |  | 100 | 168 | 548,864 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21882667 | 3 | 16,384 |  | 100 | 168 | 4,390,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21882667 | 4 | 16,384 |  | 100 | 168 | 4,390,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21882667 | 5 | 16,384 |  | 100 | 168 | 4,390,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21882667 | 6 | 1 |  | 100 | 168 | 268 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21882667 | 2 | 1,024 |  | 36 | 169 | 209,920 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21882667 | 3 | 8,192 |  | 36 | 169 | 1,679,360 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21882667 | 4 | 4,096 |  | 36 | 169 | 839,680 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21882667 | 5 | 4,096 |  | 36 | 169 | 839,680 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21882667 | 2 | 256 |  | 100 | 164 | 67,584 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21882667 | 3 | 4,096 |  | 100 | 164 | 1,081,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21882667 | 4 | 2,048 |  | 100 | 164 | 540,672 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21882667 | 5 | 1,024 |  | 100 | 164 | 270,336 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21882667 | 2 | 256 |  | 84 | 241 | 83,200 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21882667 | 3 | 4,096 |  | 84 | 241 | 1,331,200 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21882667 | 4 | 4,096 |  | 84 | 241 | 1,331,200 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21882667 | 5 | 2,048 |  | 84 | 241 | 665,600 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21882667 | 2 | 4,096 |  | 28 | 124 | 622,592 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21882667 | 3 | 32,768 |  | 28 | 124 | 4,980,736 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21882667 | 4 | 16,384 |  | 28 | 124 | 2,490,368 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21882667 | 5 | 16,384 |  | 28 | 124 | 2,490,368 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21882667 | 6 | 128 |  | 28 | 124 | 19,456 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21882667 | 0 | 1 |  | 32 | 166 | 198 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21882667 | 2 | 131,072 |  | 32 | 166 | 25,952,256 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21882667 | 3 | 8,192 |  | 32 | 166 | 1,622,016 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21882667 | 4 | 1 |  | 32 | 166 | 198 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21882667 | 5 | 16,384 |  | 32 | 166 | 3,244,032 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21882667 | 6 | 1 |  | 32 | 166 | 198 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 3, 16, 48>, ModularIsEqualCoreAir<48, 4, 8> | 21882667 | 4 | 1,024 |  | 48 | 242 | 296,960 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 3, 16, 48>, ModularIsEqualCoreAir<48, 4, 8> | 21882667 | 6 | 1 |  | 48 | 242 | 290 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 0 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 1 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 2 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 3 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 4 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 5 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 6 | 262,144 |  | 20 | 28 | 12,582,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 0 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 1 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 2 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 3 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 4 | 524,288 |  | 28 | 35 | 33,030,144 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 5 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 6 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 0 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 1 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 2 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 3 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 4 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 5 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 6 | 4,194,304 |  | 28 | 40 | 285,212,672 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21882667 | 2 | 512 |  | 40 | 57 | 49,664 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21882667 | 3 | 256 |  | 40 | 57 | 24,832 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21882667 | 4 | 2,048 |  | 40 | 57 | 198,656 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21882667 | 5 | 256 |  | 40 | 57 | 24,832 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21882667 | 6 | 64 |  | 40 | 57 | 6,208 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 1 | 1,024 |  | 40 | 39 | 80,896 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 2 | 65,536 |  | 40 | 39 | 5,177,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 3 | 131,072 |  | 40 | 39 | 10,354,688 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 4 | 32,768 |  | 40 | 39 | 2,588,672 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 5 | 65,536 |  | 40 | 39 | 5,177,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 6 | 2,048 |  | 40 | 39 | 161,792 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 0 | 256 |  | 28 | 31 | 15,104 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 1 | 4,096 |  | 28 | 31 | 241,664 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 2 | 262,144 |  | 28 | 31 | 15,466,496 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 3 | 524,288 |  | 28 | 31 | 30,932,992 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 4 | 131,072 |  | 28 | 31 | 7,733,248 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 5 | 262,144 |  | 28 | 31 | 15,466,496 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 6 | 16,384 |  | 28 | 31 | 966,656 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 0 | 262,144 |  | 16 | 21 | 9,699,328 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 1 | 262,144 |  | 16 | 21 | 9,699,328 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 2 | 262,144 |  | 16 | 21 | 9,699,328 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 3 | 131,072 |  | 16 | 21 | 4,849,664 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 4 | 32,768 |  | 16 | 21 | 1,212,416 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 5 | 131,072 |  | 16 | 21 | 4,849,664 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 6 | 131,072 |  | 16 | 21 | 4,849,664 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 12, 24, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 128 |  | 2,648 | 3,189 | 747,136 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 0 | 1 |  | 416 | 543 | 959 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 2 | 65,536 |  | 416 | 543 | 62,849,024 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 3 | 4,096 |  | 416 | 543 | 3,928,064 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 5 | 4,096 |  | 416 | 543 | 3,928,064 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 6, 6, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 256 |  | 828 | 1,012 | 471,040 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 6, 6, 16, 16>, FieldExpressionCoreAir> | 21882667 | 6 | 1 |  | 828 | 1,012 | 1,840 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21882667 | 0 | 1 |  | 160 | 261 | 421 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21882667 | 2 | 1,024 |  | 160 | 261 | 431,104 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21882667 | 3 | 64 |  | 160 | 261 | 26,944 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21882667 | 5 | 64 |  | 160 | 261 | 26,944 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21882667 | 6 | 1 |  | 160 | 261 | 421 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 12, 30, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 128 |  | 1,772 | 2,587 | 557,952 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 12, 36, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 8 |  | 4,668 | 5,572 | 81,920 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 0 | 1 |  | 428 | 619 | 1,047 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 2 | 32,768 |  | 428 | 619 | 34,308,096 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 3 | 2,048 |  | 428 | 619 | 2,144,256 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 5 | 2,048 |  | 428 | 619 | 2,144,256 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 3, 3, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 64 |  | 248 | 391 | 28,992 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 3, 3, 16, 16>, FieldExpressionCoreAir> | 21882667 | 6 | 1 |  | 248 | 391 | 639 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 36, 36, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 128 |  | 9,240 | 10,865 | 2,573,440 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 6, 6, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 2,048 |  | 668 | 943 | 2,516,992 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 6, 6, 16, 16>, FieldExpressionCoreAir> | 21882667 | 6 | 1 |  | 668 | 943 | 1,611 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapTwoReadsAdapterAir<12, 6, 12, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 256 |  | 908 | 1,361 | 580,864 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapTwoReadsAdapterAir<36, 30, 36, 16, 16>, FieldExpressionCoreAir> | 21882667 | 4 | 128 |  | 8,452 | 9,987 | 2,360,192 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 0 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 1 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 2 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 3 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 4 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 5 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 6 | 2 | 1 | 8 | 4 | 24 | 

| group | block_number | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | num_segments | main_trace_commit_time_ms | main_cells_used | halo2_total_cells | halo2_keygen_time_ms | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 21882667 | 284 | 21,238 | 1,357,892 | 196,035,544 | 20,758 | 504 | 7,364 | 4,320 | 2,321 | 1 | 6,197 | 81,199,222 | 7,911,006 | 17,845 | 48 | 196 | 
| halo2_outer | 21882667 |  | 47,211 |  |  |  |  |  |  |  |  |  | 58,076,647 |  |  |  |  | 
| halo2_wrapper | 21882667 |  | 35,174 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
| reth.prove_e2e.block_21882667 | 21882667 |  |  |  |  |  |  |  |  |  | 7 |  |  |  |  |  |  | 

| group | block_number | cell_tracker_span | simple_advice_cells | lookup_advice_cells | fixed_cells |
| --- | --- | --- | --- | --- | --- |
| agg_keygen | 21882667 | VerifierProgram | 589,276 | 150,761 | 186,899 | 
| agg_keygen | 21882667 | VerifierProgram;PoseidonCell | 19,600 |  | 5,800 | 
| agg_keygen | 21882667 | VerifierProgram;stage-c-build-rounds | 335,431 | 737 | 94,619 | 
| agg_keygen | 21882667 | VerifierProgram;stage-c-build-rounds;PoseidonCell | 46,550 |  | 13,775 | 
| agg_keygen | 21882667 | VerifierProgram;stage-d-verify-pcs | 637 | 96 | 285 | 
| agg_keygen | 21882667 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify | 561,881 | 2,122 | 163,227 | 
| agg_keygen | 21882667 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;PoseidonCell | 68,600 |  | 20,300 | 
| agg_keygen | 21882667 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;cache-generator-powers | 65,212 | 11,088 | 19,880 | 
| agg_keygen | 21882667 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;compute-reduced-opening;single-reduced-opening-eval | 8,364,748 | 382,564 | 1,564,304 | 
| agg_keygen | 21882667 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;pre-compute-rounds-context | 76,596 | 11,168 | 22,344 | 
| agg_keygen | 21882667 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch | 81,536 |  | 18,816 | 
| agg_keygen | 21882667 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;PoseidonCell | 9,371,488 |  | 2,779,840 | 
| agg_keygen | 21882667 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;verify-batch-reduce-fast;PoseidonCell | 7,954,968 | 231,168 | 2,485,672 | 
| agg_keygen | 21882667 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query | 962,444 | 159,376 | 276,472 | 
| agg_keygen | 21882667 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext | 165,984 |  | 38,304 | 
| agg_keygen | 21882667 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;PoseidonCell | 15,711,024 |  | 4,660,320 | 
| agg_keygen | 21882667 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;verify-batch-reduce-fast;PoseidonCell | 1,513,820 | 44,016 | 464,828 | 
| agg_keygen | 21882667 | VerifierProgram;stage-e-verify-constraints | 9,360,016 | 1,833,739 | 2,888,110 | 

| group | block_number | idx | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | 21882667 | 0 | 570 | 4,077 | 1,267,893 | 167,723,992 | 3,015 | 445 | 670 | 455 | 839 | 518 | 72,270,358 | 84 | 492 | 
| internal.0 | 21882667 | 1 | 559 | 4,099 | 1,268,593 | 167,723,992 | 3,041 | 446 | 682 | 453 | 852 | 522 | 72,224,467 | 82 | 499 | 
| internal.0 | 21882667 | 2 | 584 | 4,105 | 1,270,699 | 167,723,992 | 3,027 | 446 | 666 | 458 | 851 | 519 | 72,338,218 | 83 | 494 | 
| internal.0 | 21882667 | 3 | 298 | 2,483 | 634,940 | 88,647,384 | 1,935 | 253 | 468 | 272 | 583 | 307 | 37,046,931 | 47 | 250 | 
| internal.1 | 21882667 | 4 | 545 | 3,995 | 1,214,172 | 167,723,992 | 3,046 | 442 | 679 | 456 | 856 | 525 | 66,387,034 | 84 | 404 | 
| internal.1 | 21882667 | 5 | 537 | 4,001 | 1,206,479 | 170,214,360 | 3,070 | 460 | 676 | 457 | 860 | 530 | 65,834,944 | 84 | 394 | 
| internal.2 | 21882667 | 6 | 543 | 3,955 | 1,214,066 | 167,379,928 | 3,010 | 438 | 677 | 448 | 839 | 512 | 66,404,487 | 91 | 402 | 
| internal.3 | 21882667 | 7 | 284 | 2,443 | 607,274 | 88,647,384 | 1,956 | 256 | 476 | 277 | 581 | 308 | 33,968,422 | 54 | 203 | 
| leaf | 21882667 | 0 | 1,112 | 15,142 | 1,583,381 | 605,759,192 | 13,595 | 1,133 | 2,317 | 1,723 | 6,167 | 2,011 | 127,879,991 | 240 | 435 | 
| leaf | 21882667 | 1 | 825 | 13,923 | 1,043,537 | 513,222,360 | 12,761 | 912 | 2,156 | 1,531 | 6,094 | 1,847 | 89,391,856 | 217 | 337 | 
| leaf | 21882667 | 2 | 1,177 | 15,526 | 1,804,419 | 633,022,168 | 13,879 | 1,179 | 2,347 | 1,804 | 6,232 | 2,072 | 141,231,806 | 241 | 470 | 
| leaf | 21882667 | 3 | 1,237 | 15,512 | 1,846,818 | 633,022,168 | 13,800 | 1,192 | 2,300 | 1,772 | 6,230 | 2,082 | 143,076,449 | 219 | 475 | 
| leaf | 21882667 | 4 | 9,354 | 51,164 | 9,910,466 | 2,706,663,128 | 39,815 | 6,333 | 7,547 | 6,429 | 12,129 | 6,333 | 889,319,508 | 1,040 | 1,995 | 
| leaf | 21882667 | 5 | 1,262 | 15,574 | 1,846,837 | 633,022,168 | 13,835 | 1,184 | 2,317 | 1,792 | 6,231 | 2,080 | 143,128,543 | 227 | 477 | 
| leaf | 21882667 | 6 | 1,420 | 17,921 | 2,196,380 | 796,083,160 | 15,966 | 1,618 | 2,720 | 2,195 | 6,692 | 2,444 | 173,826,532 | 294 | 535 | 
| root | 21882667 | 0 | 280 | 21,289 | 600,105 | 75,202,968 | 20,814 | 508 | 7,405 | 4,317 | 2,367 | 6,167 | 33,387,829 | 45 | 195 | 

| group | block_number | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 21882667 | 0 | 28 | 423 |  | 7,747,673 | 395 | 30 | 57 | 46 | 192 | 52 | 928,069 | 13 | 0 | 
| reth.prove_e2e.block_21882667 | 21882667 | 0 | 9,171 | 35,480 |  | 2,003,332,777 | 23,415 | 3,794 | 3,720 | 3,664 | 5,997 | 5,456 | 990,668,127 | 772 | 2,894 | 
| reth.prove_e2e.block_21882667 | 21882667 | 1 | 8,706 | 33,494 |  | 1,985,804,935 | 21,864 | 3,692 | 3,315 | 3,467 | 5,713 | 4,920 | 988,941,870 | 749 | 2,924 | 
| reth.prove_e2e.block_21882667 | 21882667 | 2 | 10,341 | 49,557 |  | 3,303,039,768 | 33,980 | 7,949 | 4,216 | 4,734 | 6,638 | 9,382 | 1,822,227,092 | 1,044 | 5,236 | 
| reth.prove_e2e.block_21882667 | 21882667 | 3 | 10,119 | 38,180 |  | 2,115,555,608 | 24,291 | 4,354 | 3,840 | 3,857 | 6,286 | 5,113 | 1,105,381,926 | 824 | 3,770 | 
| reth.prove_e2e.block_21882667 | 21882667 | 4 | 7,821 | 33,447 |  | 1,861,371,518 | 22,658 | 4,051 | 3,608 | 3,596 | 5,923 | 4,467 | 851,175,341 | 789 | 2,968 | 
| reth.prove_e2e.block_21882667 | 21882667 | 5 | 10,797 | 41,975 |  | 2,500,843,800 | 27,552 | 5,772 | 3,899 | 4,099 | 6,299 | 6,559 | 1,408,472,058 | 911 | 3,626 | 
| reth.prove_e2e.block_21882667 | 21882667 | 6 | 6,719 | 30,321 | 8,801,130 | 2,224,626,429 | 22,170 | 5,737 | 2,519 | 2,619 | 3,735 | 7,009 | 1,258,868,057 | 530 | 1,432 | 

</details>


Commit: https://github.com/axiom-crypto/openvm-reth-benchmark/actions/runs/13426162043

Max Segment Length: 8388508

Instance Type: m8g.24xlarge

Memory Allocator: https://github.com/axiom-crypto/openvm-reth-benchmark/commit/66cdc7f9c026d0994eacfe9c07501c8457a4dae4

[Benchmark Workflow]()
