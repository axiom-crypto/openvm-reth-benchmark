| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total |  861.41 |  182.50 |
| reth.prove_e2e.block_21882667 |  564.27 |  49.27 |
| leaf |  119.40 |  8.51 |
| internal.0 |  34.21 |  4.01 |
| internal.1 |  18.09 |  3.94 |
| internal.2 |  10.23 |  3.95 |
| internal.3 |  6.34 |  3.95 |
| internal.4 |  3.93 |  3.93 |
| internal.5 |  2.42 |  2.42 |
| root |  21.07 |  21.07 |
| halo2_outer |  46.23 |  46.23 |
| halo2_wrapper |  35.22 |  35.22 |
| agg_keygen |  21.42 |  20.99 |


| reth.prove_e2e.block_21882667 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  33,192.35 |  564,270 |  49,274 |  28,158 |
| `main_cells_used     ` |  943,399,187.71 |  16,037,786,191 |  1,822,411,712 |  702,989,602 |
| `total_cycles        ` |  9,371,511 |  9,371,511 |  9,371,511 |  9,371,511 |
| `execute_time_ms     ` |  3,306.71 |  56,214 |  5,156 |  1,486 |
| `trace_gen_time_ms   ` |  7,822.41 |  132,981 |  10,626 |  6,549 |
| `stark_prove_excluding_trace_time_ms` |  22,063.24 |  375,075 |  33,685 |  18,377 |
| `main_trace_commit_time_ms` |  4,751.18 |  80,770 |  9,293 |  3,401 |
| `generate_perm_trace_time_ms` |  787.88 |  13,394 |  983 |  543 |
| `perm_trace_commit_time_ms` |  3,661.47 |  62,245 |  4,641 |  2,680 |
| `quotient_poly_compute_time_ms` |  4,043.53 |  68,740 |  7,977 |  3,013 |
| `quotient_poly_commit_time_ms` |  3,276.82 |  55,706 |  4,211 |  2,471 |
| `pcs_opening_time_ms ` |  5,532.41 |  94,051 |  6,565 |  3,822 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  7,023.76 |  119,404 |  8,512 |  6,358 |
| `main_cells_used     ` |  95,560,224.12 |  1,624,523,810 |  136,429,314 |  80,374,541 |
| `total_cycles        ` |  1,227,912.47 |  20,874,512 |  1,846,702 |  989,305 |
| `execute_time_ms     ` |  367.76 |  6,252 |  474 |  327 |
| `trace_gen_time_ms   ` |  833.47 |  14,169 |  1,172 |  705 |
| `stark_prove_excluding_trace_time_ms` |  5,822.53 |  98,983 |  6,868 |  5,323 |
| `main_trace_commit_time_ms` |  918.88 |  15,621 |  1,143 |  826 |
| `generate_perm_trace_time_ms` |  122.12 |  2,076 |  173 |  94 |
| `perm_trace_commit_time_ms` |  826.82 |  14,056 |  1,018 |  730 |
| `quotient_poly_compute_time_ms` |  711 |  12,087 |  961 |  591 |
| `quotient_poly_commit_time_ms` |  1,170.18 |  19,893 |  1,350 |  1,070 |
| `pcs_opening_time_ms ` |  2,069.53 |  35,182 |  2,263 |  1,978 |

| internal.0 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  3,800.89 |  34,208 |  4,006 |  2,408 |
| `main_cells_used     ` |  65,603,410.56 |  590,430,695 |  69,391,704 |  35,535,431 |
| `total_cycles        ` |  1,171,735.11 |  10,545,616 |  1,241,551 |  620,748 |
| `execute_time_ms     ` |  415.22 |  3,737 |  443 |  222 |
| `trace_gen_time_ms   ` |  515.11 |  4,636 |  552 |  282 |
| `stark_prove_excluding_trace_time_ms` |  2,870.56 |  25,835 |  3,017 |  1,904 |
| `main_trace_commit_time_ms` |  492 |  4,428 |  523 |  301 |
| `generate_perm_trace_time_ms` |  79.44 |  715 |  87 |  45 |
| `perm_trace_commit_time_ms` |  429.11 |  3,862 |  457 |  268 |
| `quotient_poly_compute_time_ms` |  420.89 |  3,788 |  450 |  252 |
| `quotient_poly_commit_time_ms` |  637.78 |  5,740 |  681 |  469 |
| `pcs_opening_time_ms ` |  807.56 |  7,268 |  849 |  565 |

| internal.1 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  3,618.20 |  18,091 |  3,943 |  2,391 |
| `main_cells_used     ` |  59,782,702 |  298,913,510 |  66,387,124 |  33,365,842 |
| `total_cycles        ` |  1,091,259.40 |  5,456,297 |  1,214,182 |  599,661 |
| `execute_time_ms     ` |  362.80 |  1,814 |  408 |  195 |
| `trace_gen_time_ms   ` |  473.80 |  2,369 |  528 |  286 |
| `stark_prove_excluding_trace_time_ms` |  2,781.60 |  13,908 |  3,014 |  1,910 |
| `main_trace_commit_time_ms` |  472.60 |  2,363 |  518 |  304 |
| `generate_perm_trace_time_ms` |  80.20 |  401 |  105 |  46 |
| `perm_trace_commit_time_ms` |  415.60 |  2,078 |  464 |  266 |
| `quotient_poly_compute_time_ms` |  404 |  2,020 |  444 |  254 |
| `quotient_poly_commit_time_ms` |  620.60 |  3,103 |  666 |  474 |
| `pcs_opening_time_ms ` |  784.60 |  3,923 |  842 |  562 |

| internal.2 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  3,410 |  10,230 |  3,950 |  2,361 |
| `main_cells_used     ` |  55,356,449.33 |  166,069,348 |  66,386,701 |  33,296,333 |
| `total_cycles        ` |  1,009,265.33 |  3,027,796 |  1,214,135 |  599,569 |
| `execute_time_ms     ` |  333 |  999 |  402 |  195 |
| `trace_gen_time_ms   ` |  447.33 |  1,342 |  535 |  276 |
| `stark_prove_excluding_trace_time_ms` |  2,629.67 |  7,889 |  3,017 |  1,890 |
| `main_trace_commit_time_ms` |  445.33 |  1,336 |  517 |  307 |
| `generate_perm_trace_time_ms` |  71.33 |  214 |  85 |  45 |
| `perm_trace_commit_time_ms` |  392.33 |  1,177 |  458 |  268 |
| `quotient_poly_compute_time_ms` |  380 |  1,140 |  443 |  256 |
| `quotient_poly_commit_time_ms` |  591.67 |  1,775 |  665 |  454 |
| `pcs_opening_time_ms ` |  745 |  2,235 |  844 |  556 |

| internal.3 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  3,169 |  6,338 |  3,946 |  2,392 |
| `main_cells_used     ` |  49,841,719.50 |  99,683,439 |  66,386,989 |  33,296,450 |
| `total_cycles        ` |  906,874.50 |  1,813,749 |  1,214,167 |  599,582 |
| `execute_time_ms     ` |  296 |  592 |  398 |  194 |
| `trace_gen_time_ms   ` |  407 |  814 |  530 |  284 |
| `stark_prove_excluding_trace_time_ms` |  2,466 |  4,932 |  3,018 |  1,914 |
| `main_trace_commit_time_ms` |  411.50 |  823 |  524 |  299 |
| `generate_perm_trace_time_ms` |  64 |  128 |  82 |  46 |
| `perm_trace_commit_time_ms` |  358.50 |  717 |  450 |  267 |
| `quotient_poly_compute_time_ms` |  345 |  690 |  436 |  254 |
| `quotient_poly_commit_time_ms` |  581.50 |  1,163 |  675 |  488 |
| `pcs_opening_time_ms ` |  701 |  1,402 |  846 |  556 |

| internal.4 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  3,932 |  3,932 |  3,932 |  3,932 |
| `main_cells_used     ` |  65,764,778 |  65,764,778 |  65,764,778 |  65,764,778 |
| `total_cycles        ` |  1,206,314 |  1,206,314 |  1,206,314 |  1,206,314 |
| `execute_time_ms     ` |  392 |  392 |  392 |  392 |
| `trace_gen_time_ms   ` |  527 |  527 |  527 |  527 |
| `stark_prove_excluding_trace_time_ms` |  3,013 |  3,013 |  3,013 |  3,013 |
| `main_trace_commit_time_ms` |  523 |  523 |  523 |  523 |
| `generate_perm_trace_time_ms` |  83 |  83 |  83 |  83 |
| `perm_trace_commit_time_ms` |  448 |  448 |  448 |  448 |
| `quotient_poly_compute_time_ms` |  449 |  449 |  449 |  449 |
| `quotient_poly_commit_time_ms` |  668 |  668 |  668 |  668 |
| `pcs_opening_time_ms ` |  838 |  838 |  838 |  838 |

| internal.5 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  2,417 |  2,417 |  2,417 |  2,417 |
| `main_cells_used     ` |  33,988,134 |  33,988,134 |  33,988,134 |  33,988,134 |
| `total_cycles        ` |  607,419 |  607,419 |  607,419 |  607,419 |
| `execute_time_ms     ` |  201 |  201 |  201 |  201 |
| `trace_gen_time_ms   ` |  284 |  284 |  284 |  284 |
| `stark_prove_excluding_trace_time_ms` |  1,932 |  1,932 |  1,932 |  1,932 |
| `main_trace_commit_time_ms` |  298 |  298 |  298 |  298 |
| `generate_perm_trace_time_ms` |  55 |  55 |  55 |  55 |
| `perm_trace_commit_time_ms` |  283 |  283 |  283 |  283 |
| `quotient_poly_compute_time_ms` |  255 |  255 |  255 |  255 |
| `quotient_poly_commit_time_ms` |  475 |  475 |  475 |  475 |
| `pcs_opening_time_ms ` |  562 |  562 |  562 |  562 |

| root |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  21,074 |  21,074 |  21,074 |  21,074 |
| `main_cells_used     ` |  33,388,351 |  33,388,351 |  33,388,351 |  33,388,351 |
| `total_cycles        ` |  600,163 |  600,163 |  600,163 |  600,163 |
| `execute_time_ms     ` |  194 |  194 |  194 |  194 |
| `trace_gen_time_ms   ` |  283 |  283 |  283 |  283 |
| `stark_prove_excluding_trace_time_ms` |  20,597 |  20,597 |  20,597 |  20,597 |
| `main_trace_commit_time_ms` |  6,164 |  6,164 |  6,164 |  6,164 |
| `generate_perm_trace_time_ms` |  43 |  43 |  43 |  43 |
| `perm_trace_commit_time_ms` |  4,318 |  4,318 |  4,318 |  4,318 |
| `quotient_poly_compute_time_ms` |  505 |  505 |  505 |  505 |
| `quotient_poly_commit_time_ms` |  7,232 |  7,232 |  7,232 |  7,232 |
| `pcs_opening_time_ms ` |  2,329 |  2,329 |  2,329 |  2,329 |

| halo2_outer |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  46,229 |  46,229 |  46,229 |  46,229 |
| `main_cells_used     ` |  58,076,647 |  58,076,647 |  58,076,647 |  58,076,647 |

| halo2_wrapper |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  35,216 |  35,216 |  35,216 |  35,216 |

| agg_keygen |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  10,712 |  21,424 |  20,993 |  431 |
| `main_cells_used     ` |  41,063,569 |  82,127,138 |  81,199,069 |  928,069 |
| `total_cycles        ` |  1,357,875 |  1,357,875 |  1,357,875 |  1,357,875 |
| `execute_time_ms     ` |  97 |  194 |  194 |  0 |
| `trace_gen_time_ms   ` |  151 |  302 |  274 |  28 |
| `stark_prove_excluding_trace_time_ms` |  10,464 |  20,928 |  20,525 |  403 |
| `main_trace_commit_time_ms` |  3,106.50 |  6,213 |  6,160 |  53 |
| `generate_perm_trace_time_ms` |  28.50 |  57 |  44 |  13 |
| `perm_trace_commit_time_ms` |  2,230 |  4,460 |  4,411 |  49 |
| `quotient_poly_compute_time_ms` |  266.50 |  533 |  503 |  30 |
| `quotient_poly_commit_time_ms` |  3,581 |  7,162 |  7,101 |  61 |
| `pcs_opening_time_ms ` |  1,248 |  2,496 |  2,303 |  193 |



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
| VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 4 | 16 | 16 | 
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 4 | 18 | 21 | 
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 4 | 17 | 27 | 
| VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21882667 | 4 | 25 | 72 | 
| VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 4 | 24 | 23 | 
| VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 4 | 19 | 13 | 
| VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 4 | 11 | 12 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 411 | 378 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 4, 8, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 1,716 | 1,310 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 156 | 150 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 12, 12, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 4,370 | 3,323 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 422 | 351 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 4, 10, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 1,303 | 988 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 4, 12, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 2,903 | 2,221 | 
| VmAirWrapper<Rv32VecHeapTwoReadsAdapterAir<12, 10, 12, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 3,977 | 3,023 | 
| VmAirWrapper<Rv32VecHeapTwoReadsAdapterAir<4, 2, 4, 32, 32>, FieldExpressionCoreAir> | 21882667 | 4 | 565 | 423 | 
| VmConnectorAir | 21882667 | 4 | 3 | 8 | 

| block_number | execute_time_ms |
| --- | --- |
| 21882667 | 195 | 

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
| internal.0 | AccessAdapterAir<2> | 21882667 | 3 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21882667 | 4 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21882667 | 5 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21882667 | 6 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21882667 | 7 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.0 | AccessAdapterAir<2> | 21882667 | 8 | 262,144 |  | 12 | 11 | 6,029,312 | 
| internal.0 | AccessAdapterAir<4> | 21882667 | 0 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21882667 | 1 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21882667 | 2 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21882667 | 3 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21882667 | 4 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21882667 | 5 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21882667 | 6 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21882667 | 7 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.0 | AccessAdapterAir<4> | 21882667 | 8 | 131,072 |  | 12 | 13 | 3,276,800 | 
| internal.0 | AccessAdapterAir<8> | 21882667 | 0 | 512 |  | 12 | 17 | 14,848 | 
| internal.0 | AccessAdapterAir<8> | 21882667 | 1 | 512 |  | 12 | 17 | 14,848 | 
| internal.0 | AccessAdapterAir<8> | 21882667 | 2 | 512 |  | 12 | 17 | 14,848 | 
| internal.0 | AccessAdapterAir<8> | 21882667 | 3 | 512 |  | 12 | 17 | 14,848 | 
| internal.0 | AccessAdapterAir<8> | 21882667 | 4 | 512 |  | 12 | 17 | 14,848 | 
| internal.0 | AccessAdapterAir<8> | 21882667 | 5 | 512 |  | 12 | 17 | 14,848 | 
| internal.0 | AccessAdapterAir<8> | 21882667 | 6 | 512 |  | 12 | 17 | 14,848 | 
| internal.0 | AccessAdapterAir<8> | 21882667 | 7 | 512 |  | 12 | 17 | 14,848 | 
| internal.0 | AccessAdapterAir<8> | 21882667 | 8 | 256 |  | 12 | 17 | 7,424 | 
| internal.0 | FriReducedOpeningAir | 21882667 | 0 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.0 | FriReducedOpeningAir | 21882667 | 1 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.0 | FriReducedOpeningAir | 21882667 | 2 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.0 | FriReducedOpeningAir | 21882667 | 3 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.0 | FriReducedOpeningAir | 21882667 | 4 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.0 | FriReducedOpeningAir | 21882667 | 5 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.0 | FriReducedOpeningAir | 21882667 | 6 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.0 | FriReducedOpeningAir | 21882667 | 7 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.0 | FriReducedOpeningAir | 21882667 | 8 | 131,072 |  | 44 | 27 | 9,306,112 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 0 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 1 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 2 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 3 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 4 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 5 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 6 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 7 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 8 | 32,768 |  | 160 | 399 | 18,317,312 | 
| internal.0 | PhantomAir | 21882667 | 0 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.0 | PhantomAir | 21882667 | 1 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.0 | PhantomAir | 21882667 | 2 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.0 | PhantomAir | 21882667 | 3 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.0 | PhantomAir | 21882667 | 4 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.0 | PhantomAir | 21882667 | 5 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.0 | PhantomAir | 21882667 | 6 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.0 | PhantomAir | 21882667 | 7 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.0 | PhantomAir | 21882667 | 8 | 8,192 |  | 8 | 6 | 114,688 | 
| internal.0 | ProgramAir | 21882667 | 0 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | ProgramAir | 21882667 | 1 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | ProgramAir | 21882667 | 2 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | ProgramAir | 21882667 | 3 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | ProgramAir | 21882667 | 4 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | ProgramAir | 21882667 | 5 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | ProgramAir | 21882667 | 6 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | ProgramAir | 21882667 | 7 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | ProgramAir | 21882667 | 8 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.0 | VariableRangeCheckerAir | 21882667 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21882667 | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21882667 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21882667 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21882667 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21882667 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21882667 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21882667 | 7 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VariableRangeCheckerAir | 21882667 | 8 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 0 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 1 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 2 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 3 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 4 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 5 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 6 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 7 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 8 | 524,288 |  | 20 | 29 | 25,690,112 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 0 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 1 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 2 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 3 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 4 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 5 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 6 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 7 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 8 | 65,536 |  | 16 | 23 | 2,555,904 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 0 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 1 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 2 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 3 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 4 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 5 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 6 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 7 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 8 | 16,384 |  | 12 | 9 | 344,064 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 0 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 1 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 2 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 3 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 4 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 5 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 6 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 7 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 8 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 0 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 1 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 2 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 3 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 4 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 5 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 6 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 7 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 8 | 131,072 |  | 24 | 22 | 6,029,312 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 0 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 1 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 2 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 3 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 4 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 5 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 6 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 7 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 8 | 65,536 |  | 24 | 31 | 3,604,480 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 0 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 1 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 2 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 3 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 4 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 5 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 6 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 7 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 8 | 65,536 |  | 20 | 38 | 3,801,088 | 
| internal.0 | VmConnectorAir | 21882667 | 0 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21882667 | 1 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21882667 | 2 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21882667 | 3 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21882667 | 4 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21882667 | 5 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21882667 | 6 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21882667 | 7 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VmConnectorAir | 21882667 | 8 | 2 | 1 | 8 | 4 | 24 | 
| internal.0 | VolatileBoundaryAir | 21882667 | 0 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.0 | VolatileBoundaryAir | 21882667 | 1 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.0 | VolatileBoundaryAir | 21882667 | 2 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.0 | VolatileBoundaryAir | 21882667 | 3 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.0 | VolatileBoundaryAir | 21882667 | 4 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.0 | VolatileBoundaryAir | 21882667 | 5 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.0 | VolatileBoundaryAir | 21882667 | 6 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.0 | VolatileBoundaryAir | 21882667 | 7 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.0 | VolatileBoundaryAir | 21882667 | 8 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.1 | AccessAdapterAir<2> | 21882667 | 10 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.1 | AccessAdapterAir<2> | 21882667 | 11 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.1 | AccessAdapterAir<2> | 21882667 | 12 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.1 | AccessAdapterAir<2> | 21882667 | 13 | 262,144 |  | 12 | 11 | 6,029,312 | 
| internal.1 | AccessAdapterAir<2> | 21882667 | 9 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.1 | AccessAdapterAir<4> | 21882667 | 10 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.1 | AccessAdapterAir<4> | 21882667 | 11 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.1 | AccessAdapterAir<4> | 21882667 | 12 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.1 | AccessAdapterAir<4> | 21882667 | 13 | 131,072 |  | 12 | 13 | 3,276,800 | 
| internal.1 | AccessAdapterAir<4> | 21882667 | 9 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.1 | AccessAdapterAir<8> | 21882667 | 10 | 512 |  | 12 | 17 | 14,848 | 
| internal.1 | AccessAdapterAir<8> | 21882667 | 11 | 512 |  | 12 | 17 | 14,848 | 
| internal.1 | AccessAdapterAir<8> | 21882667 | 12 | 512 |  | 12 | 17 | 14,848 | 
| internal.1 | AccessAdapterAir<8> | 21882667 | 13 | 256 |  | 12 | 17 | 7,424 | 
| internal.1 | AccessAdapterAir<8> | 21882667 | 9 | 512 |  | 12 | 17 | 14,848 | 
| internal.1 | FriReducedOpeningAir | 21882667 | 10 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.1 | FriReducedOpeningAir | 21882667 | 11 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.1 | FriReducedOpeningAir | 21882667 | 12 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.1 | FriReducedOpeningAir | 21882667 | 13 | 131,072 |  | 44 | 27 | 9,306,112 | 
| internal.1 | FriReducedOpeningAir | 21882667 | 9 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 10 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 11 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 12 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 13 | 32,768 |  | 160 | 399 | 18,317,312 | 
| internal.1 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 9 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.1 | PhantomAir | 21882667 | 10 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.1 | PhantomAir | 21882667 | 11 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.1 | PhantomAir | 21882667 | 12 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.1 | PhantomAir | 21882667 | 13 | 8,192 |  | 8 | 6 | 114,688 | 
| internal.1 | PhantomAir | 21882667 | 9 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.1 | ProgramAir | 21882667 | 10 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.1 | ProgramAir | 21882667 | 11 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.1 | ProgramAir | 21882667 | 12 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.1 | ProgramAir | 21882667 | 13 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.1 | ProgramAir | 21882667 | 9 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.1 | VariableRangeCheckerAir | 21882667 | 10 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 21882667 | 11 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 21882667 | 12 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 21882667 | 13 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VariableRangeCheckerAir | 21882667 | 9 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 10 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 11 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 12 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 13 | 524,288 |  | 20 | 29 | 25,690,112 | 
| internal.1 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 9 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 10 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 11 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 12 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 13 | 65,536 |  | 16 | 23 | 2,555,904 | 
| internal.1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 9 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 10 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 11 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 12 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 13 | 8,192 |  | 12 | 9 | 172,032 | 
| internal.1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 9 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 10 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 11 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 12 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 13 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 9 | 64 |  | 16 | 23 | 2,496 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 10 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 11 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 12 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 13 | 131,072 |  | 24 | 22 | 6,029,312 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 9 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 10 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 11 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 12 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 13 | 65,536 |  | 24 | 31 | 3,604,480 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 9 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 10 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 11 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 12 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 13 | 65,536 |  | 20 | 38 | 3,801,088 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 9 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.1 | VmConnectorAir | 21882667 | 10 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VmConnectorAir | 21882667 | 11 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VmConnectorAir | 21882667 | 12 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VmConnectorAir | 21882667 | 13 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VmConnectorAir | 21882667 | 9 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VolatileBoundaryAir | 21882667 | 10 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.1 | VolatileBoundaryAir | 21882667 | 11 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.1 | VolatileBoundaryAir | 21882667 | 12 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.1 | VolatileBoundaryAir | 21882667 | 13 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.1 | VolatileBoundaryAir | 21882667 | 9 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.2 | AccessAdapterAir<2> | 21882667 | 14 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.2 | AccessAdapterAir<2> | 21882667 | 15 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.2 | AccessAdapterAir<2> | 21882667 | 16 | 262,144 |  | 12 | 11 | 6,029,312 | 
| internal.2 | AccessAdapterAir<4> | 21882667 | 14 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.2 | AccessAdapterAir<4> | 21882667 | 15 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.2 | AccessAdapterAir<4> | 21882667 | 16 | 131,072 |  | 12 | 13 | 3,276,800 | 
| internal.2 | AccessAdapterAir<8> | 21882667 | 14 | 512 |  | 12 | 17 | 14,848 | 
| internal.2 | AccessAdapterAir<8> | 21882667 | 15 | 512 |  | 12 | 17 | 14,848 | 
| internal.2 | AccessAdapterAir<8> | 21882667 | 16 | 256 |  | 12 | 17 | 7,424 | 
| internal.2 | FriReducedOpeningAir | 21882667 | 14 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.2 | FriReducedOpeningAir | 21882667 | 15 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.2 | FriReducedOpeningAir | 21882667 | 16 | 131,072 |  | 44 | 27 | 9,306,112 | 
| internal.2 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 14 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.2 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 15 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.2 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 16 | 32,768 |  | 160 | 399 | 18,317,312 | 
| internal.2 | PhantomAir | 21882667 | 14 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.2 | PhantomAir | 21882667 | 15 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.2 | PhantomAir | 21882667 | 16 | 8,192 |  | 8 | 6 | 114,688 | 
| internal.2 | ProgramAir | 21882667 | 14 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.2 | ProgramAir | 21882667 | 15 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.2 | ProgramAir | 21882667 | 16 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.2 | VariableRangeCheckerAir | 21882667 | 14 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.2 | VariableRangeCheckerAir | 21882667 | 15 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.2 | VariableRangeCheckerAir | 21882667 | 16 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.2 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 14 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.2 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 15 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.2 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 16 | 524,288 |  | 20 | 29 | 25,690,112 | 
| internal.2 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 14 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.2 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 15 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.2 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 16 | 65,536 |  | 16 | 23 | 2,555,904 | 
| internal.2 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 14 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.2 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 15 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.2 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 16 | 8,192 |  | 12 | 9 | 172,032 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 14 | 64 |  | 16 | 23 | 2,496 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 15 | 64 |  | 16 | 23 | 2,496 | 
| internal.2 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 16 | 64 |  | 16 | 23 | 2,496 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 14 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 15 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 16 | 131,072 |  | 24 | 22 | 6,029,312 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 14 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 15 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 16 | 65,536 |  | 24 | 31 | 3,604,480 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 14 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 15 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 16 | 65,536 |  | 20 | 38 | 3,801,088 | 
| internal.2 | VmConnectorAir | 21882667 | 14 | 2 | 1 | 8 | 4 | 24 | 
| internal.2 | VmConnectorAir | 21882667 | 15 | 2 | 1 | 8 | 4 | 24 | 
| internal.2 | VmConnectorAir | 21882667 | 16 | 2 | 1 | 8 | 4 | 24 | 
| internal.2 | VolatileBoundaryAir | 21882667 | 14 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.2 | VolatileBoundaryAir | 21882667 | 15 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.2 | VolatileBoundaryAir | 21882667 | 16 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.3 | AccessAdapterAir<2> | 21882667 | 17 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.3 | AccessAdapterAir<2> | 21882667 | 18 | 262,144 |  | 12 | 11 | 6,029,312 | 
| internal.3 | AccessAdapterAir<4> | 21882667 | 17 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.3 | AccessAdapterAir<4> | 21882667 | 18 | 131,072 |  | 12 | 13 | 3,276,800 | 
| internal.3 | AccessAdapterAir<8> | 21882667 | 17 | 512 |  | 12 | 17 | 14,848 | 
| internal.3 | AccessAdapterAir<8> | 21882667 | 18 | 256 |  | 12 | 17 | 7,424 | 
| internal.3 | FriReducedOpeningAir | 21882667 | 17 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.3 | FriReducedOpeningAir | 21882667 | 18 | 131,072 |  | 44 | 27 | 9,306,112 | 
| internal.3 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 17 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.3 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 18 | 32,768 |  | 160 | 399 | 18,317,312 | 
| internal.3 | PhantomAir | 21882667 | 17 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.3 | PhantomAir | 21882667 | 18 | 8,192 |  | 8 | 6 | 114,688 | 
| internal.3 | ProgramAir | 21882667 | 17 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.3 | ProgramAir | 21882667 | 18 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.3 | VariableRangeCheckerAir | 21882667 | 17 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.3 | VariableRangeCheckerAir | 21882667 | 18 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.3 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 17 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.3 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 18 | 524,288 |  | 20 | 29 | 25,690,112 | 
| internal.3 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 17 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.3 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 18 | 65,536 |  | 16 | 23 | 2,555,904 | 
| internal.3 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 17 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.3 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 18 | 8,192 |  | 12 | 9 | 172,032 | 
| internal.3 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 17 | 64 |  | 16 | 23 | 2,496 | 
| internal.3 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 18 | 64 |  | 16 | 23 | 2,496 | 
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 17 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 18 | 131,072 |  | 24 | 22 | 6,029,312 | 
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 17 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 18 | 65,536 |  | 24 | 31 | 3,604,480 | 
| internal.3 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 17 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.3 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 18 | 65,536 |  | 20 | 38 | 3,801,088 | 
| internal.3 | VmConnectorAir | 21882667 | 17 | 2 | 1 | 8 | 4 | 24 | 
| internal.3 | VmConnectorAir | 21882667 | 18 | 2 | 1 | 8 | 4 | 24 | 
| internal.3 | VolatileBoundaryAir | 21882667 | 17 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.3 | VolatileBoundaryAir | 21882667 | 18 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.4 | AccessAdapterAir<2> | 21882667 | 19 | 524,288 |  | 12 | 11 | 12,058,624 | 
| internal.4 | AccessAdapterAir<4> | 21882667 | 19 | 262,144 |  | 12 | 13 | 6,553,600 | 
| internal.4 | AccessAdapterAir<8> | 21882667 | 19 | 512 |  | 12 | 17 | 14,848 | 
| internal.4 | FriReducedOpeningAir | 21882667 | 19 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.4 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 19 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.4 | PhantomAir | 21882667 | 19 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.4 | ProgramAir | 21882667 | 19 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.4 | VariableRangeCheckerAir | 21882667 | 19 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.4 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 19 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.4 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 19 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.4 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 19 | 16,384 |  | 12 | 9 | 344,064 | 
| internal.4 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 19 | 64 |  | 16 | 23 | 2,496 | 
| internal.4 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 19 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.4 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 19 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.4 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 19 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.4 | VmConnectorAir | 21882667 | 19 | 2 | 1 | 8 | 4 | 24 | 
| internal.4 | VolatileBoundaryAir | 21882667 | 19 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.5 | AccessAdapterAir<2> | 21882667 | 20 | 262,144 |  | 12 | 11 | 6,029,312 | 
| internal.5 | AccessAdapterAir<4> | 21882667 | 20 | 131,072 |  | 12 | 13 | 3,276,800 | 
| internal.5 | AccessAdapterAir<8> | 21882667 | 20 | 256 |  | 12 | 17 | 7,424 | 
| internal.5 | FriReducedOpeningAir | 21882667 | 20 | 131,072 |  | 44 | 27 | 9,306,112 | 
| internal.5 | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 20 | 32,768 |  | 160 | 399 | 18,317,312 | 
| internal.5 | PhantomAir | 21882667 | 20 | 8,192 |  | 8 | 6 | 114,688 | 
| internal.5 | ProgramAir | 21882667 | 20 | 262,144 |  | 8 | 10 | 4,718,592 | 
| internal.5 | VariableRangeCheckerAir | 21882667 | 20 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| internal.5 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 20 | 524,288 |  | 20 | 29 | 25,690,112 | 
| internal.5 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 20 | 65,536 |  | 16 | 23 | 2,555,904 | 
| internal.5 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 20 | 16,384 |  | 12 | 9 | 344,064 | 
| internal.5 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 20 | 64 |  | 16 | 23 | 2,496 | 
| internal.5 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 20 | 131,072 |  | 24 | 22 | 6,029,312 | 
| internal.5 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 20 | 65,536 |  | 24 | 31 | 3,604,480 | 
| internal.5 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 20 | 65,536 |  | 20 | 38 | 3,801,088 | 
| internal.5 | VmConnectorAir | 21882667 | 20 | 2 | 1 | 8 | 4 | 24 | 
| internal.5 | VolatileBoundaryAir | 21882667 | 20 | 131,072 |  | 8 | 11 | 2,490,368 | 
| leaf | AccessAdapterAir<2> | 21882667 | 0 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 1 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 10 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 11 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 12 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 13 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 14 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 15 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 16 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 2 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 3 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 4 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 5 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 6 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 7 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 8 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<2> | 21882667 | 9 | 1,048,576 |  | 12 | 11 | 24,117,248 | 
| leaf | AccessAdapterAir<4> | 21882667 | 0 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 1 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 10 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 11 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 12 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 13 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 14 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 15 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 16 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 2 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 3 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 4 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 5 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 6 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 7 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 8 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<4> | 21882667 | 9 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<8> | 21882667 | 0 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 1 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 10 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 11 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 12 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 13 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 14 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 15 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 16 | 512 |  | 12 | 17 | 14,848 | 
| leaf | AccessAdapterAir<8> | 21882667 | 2 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 3 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 4 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 5 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 6 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 7 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 8 | 256 |  | 12 | 17 | 7,424 | 
| leaf | AccessAdapterAir<8> | 21882667 | 9 | 256 |  | 12 | 17 | 7,424 | 
| leaf | FriReducedOpeningAir | 21882667 | 0 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| leaf | FriReducedOpeningAir | 21882667 | 1 | 524,288 |  | 44 | 27 | 37,224,448 | 
| leaf | FriReducedOpeningAir | 21882667 | 10 | 524,288 |  | 44 | 27 | 37,224,448 | 
| leaf | FriReducedOpeningAir | 21882667 | 11 | 524,288 |  | 44 | 27 | 37,224,448 | 
| leaf | FriReducedOpeningAir | 21882667 | 12 | 524,288 |  | 44 | 27 | 37,224,448 | 
| leaf | FriReducedOpeningAir | 21882667 | 13 | 524,288 |  | 44 | 27 | 37,224,448 | 
| leaf | FriReducedOpeningAir | 21882667 | 14 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| leaf | FriReducedOpeningAir | 21882667 | 15 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| leaf | FriReducedOpeningAir | 21882667 | 16 | 524,288 |  | 44 | 27 | 37,224,448 | 
| leaf | FriReducedOpeningAir | 21882667 | 2 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| leaf | FriReducedOpeningAir | 21882667 | 3 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| leaf | FriReducedOpeningAir | 21882667 | 4 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| leaf | FriReducedOpeningAir | 21882667 | 5 | 524,288 |  | 44 | 27 | 37,224,448 | 
| leaf | FriReducedOpeningAir | 21882667 | 6 | 524,288 |  | 44 | 27 | 37,224,448 | 
| leaf | FriReducedOpeningAir | 21882667 | 7 | 524,288 |  | 44 | 27 | 37,224,448 | 
| leaf | FriReducedOpeningAir | 21882667 | 8 | 524,288 |  | 44 | 27 | 37,224,448 | 
| leaf | FriReducedOpeningAir | 21882667 | 9 | 524,288 |  | 44 | 27 | 37,224,448 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 0 | 131,072 |  | 160 | 399 | 73,269,248 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 1 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 10 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 11 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 12 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 13 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 14 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 15 | 131,072 |  | 160 | 399 | 73,269,248 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 16 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 2 | 131,072 |  | 160 | 399 | 73,269,248 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 3 | 131,072 |  | 160 | 399 | 73,269,248 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 4 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 5 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 6 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 7 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 8 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21882667 | 9 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | PhantomAir | 21882667 | 0 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 1 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 10 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 11 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 12 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 13 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 14 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 15 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 16 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 2 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 3 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 4 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 5 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 6 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 7 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 8 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21882667 | 9 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | ProgramAir | 21882667 | 0 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21882667 | 1 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21882667 | 10 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21882667 | 11 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21882667 | 12 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21882667 | 13 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21882667 | 14 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21882667 | 15 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21882667 | 16 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21882667 | 2 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21882667 | 3 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21882667 | 4 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21882667 | 5 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21882667 | 6 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21882667 | 7 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21882667 | 8 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | ProgramAir | 21882667 | 9 | 4,194,304 |  | 8 | 10 | 75,497,472 | 
| leaf | VariableRangeCheckerAir | 21882667 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 10 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 11 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 12 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 13 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 14 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 15 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 16 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 7 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 8 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 21882667 | 9 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 0 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 1 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 10 | 524,288 |  | 20 | 29 | 25,690,112 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 11 | 524,288 |  | 20 | 29 | 25,690,112 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 12 | 524,288 |  | 20 | 29 | 25,690,112 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 13 | 524,288 |  | 20 | 29 | 25,690,112 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 14 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 15 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 16 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 2 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 3 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 4 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 5 | 524,288 |  | 20 | 29 | 25,690,112 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 6 | 524,288 |  | 20 | 29 | 25,690,112 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 7 | 524,288 |  | 20 | 29 | 25,690,112 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 8 | 524,288 |  | 20 | 29 | 25,690,112 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21882667 | 9 | 524,288 |  | 20 | 29 | 25,690,112 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 0 | 262,144 |  | 16 | 23 | 10,223,616 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 1 | 131,072 |  | 16 | 23 | 5,111,808 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 10 | 131,072 |  | 16 | 23 | 5,111,808 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 11 | 131,072 |  | 16 | 23 | 5,111,808 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 12 | 131,072 |  | 16 | 23 | 5,111,808 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 13 | 131,072 |  | 16 | 23 | 5,111,808 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 14 | 131,072 |  | 16 | 23 | 5,111,808 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 15 | 262,144 |  | 16 | 23 | 10,223,616 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 16 | 131,072 |  | 16 | 23 | 5,111,808 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 2 | 262,144 |  | 16 | 23 | 10,223,616 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 3 | 262,144 |  | 16 | 23 | 10,223,616 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 4 | 131,072 |  | 16 | 23 | 5,111,808 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 5 | 131,072 |  | 16 | 23 | 5,111,808 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 6 | 131,072 |  | 16 | 23 | 5,111,808 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 7 | 131,072 |  | 16 | 23 | 5,111,808 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 8 | 131,072 |  | 16 | 23 | 5,111,808 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21882667 | 9 | 131,072 |  | 16 | 23 | 5,111,808 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 0 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 1 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 10 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 11 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 12 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 13 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 14 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 15 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 16 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 2 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 3 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 4 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 5 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 6 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 7 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 8 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21882667 | 9 | 16,384 |  | 12 | 9 | 344,064 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 0 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 1 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 10 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 11 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 12 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 13 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 14 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 15 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 16 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 2 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 3 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 4 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 5 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 6 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 7 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 8 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21882667 | 9 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 0 | 262,144 |  | 24 | 22 | 12,058,624 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 1 | 262,144 |  | 24 | 22 | 12,058,624 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 10 | 262,144 |  | 24 | 22 | 12,058,624 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 11 | 262,144 |  | 24 | 22 | 12,058,624 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 12 | 262,144 |  | 24 | 22 | 12,058,624 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 13 | 262,144 |  | 24 | 22 | 12,058,624 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 14 | 262,144 |  | 24 | 22 | 12,058,624 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 15 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 16 | 262,144 |  | 24 | 22 | 12,058,624 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 2 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 3 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 4 | 262,144 |  | 24 | 22 | 12,058,624 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 5 | 262,144 |  | 24 | 22 | 12,058,624 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 6 | 262,144 |  | 24 | 22 | 12,058,624 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 7 | 262,144 |  | 24 | 22 | 12,058,624 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 8 | 262,144 |  | 24 | 22 | 12,058,624 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21882667 | 9 | 262,144 |  | 24 | 22 | 12,058,624 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 0 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 1 | 65,536 |  | 24 | 31 | 3,604,480 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 10 | 65,536 |  | 24 | 31 | 3,604,480 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 11 | 65,536 |  | 24 | 31 | 3,604,480 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 12 | 65,536 |  | 24 | 31 | 3,604,480 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 13 | 65,536 |  | 24 | 31 | 3,604,480 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 14 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 15 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 16 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 2 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 3 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 4 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 5 | 65,536 |  | 24 | 31 | 3,604,480 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 6 | 65,536 |  | 24 | 31 | 3,604,480 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 7 | 65,536 |  | 24 | 31 | 3,604,480 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 8 | 65,536 |  | 24 | 31 | 3,604,480 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21882667 | 9 | 65,536 |  | 24 | 31 | 3,604,480 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 0 | 262,144 |  | 20 | 38 | 15,204,352 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 1 | 262,144 |  | 20 | 38 | 15,204,352 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 10 | 131,072 |  | 20 | 38 | 7,602,176 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 11 | 131,072 |  | 20 | 38 | 7,602,176 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 12 | 131,072 |  | 20 | 38 | 7,602,176 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 13 | 131,072 |  | 20 | 38 | 7,602,176 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 14 | 262,144 |  | 20 | 38 | 15,204,352 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 15 | 524,288 |  | 20 | 38 | 30,408,704 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 16 | 262,144 |  | 20 | 38 | 15,204,352 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 2 | 524,288 |  | 20 | 38 | 30,408,704 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 3 | 524,288 |  | 20 | 38 | 30,408,704 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 4 | 262,144 |  | 20 | 38 | 15,204,352 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 5 | 131,072 |  | 20 | 38 | 7,602,176 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 6 | 131,072 |  | 20 | 38 | 7,602,176 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 7 | 131,072 |  | 20 | 38 | 7,602,176 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 8 | 131,072 |  | 20 | 38 | 7,602,176 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21882667 | 9 | 131,072 |  | 20 | 38 | 7,602,176 | 
| leaf | VmConnectorAir | 21882667 | 0 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 1 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 10 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 11 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 12 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 13 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 14 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 15 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 16 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 2 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 3 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 4 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 5 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 6 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 7 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 8 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VmConnectorAir | 21882667 | 9 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VolatileBoundaryAir | 21882667 | 0 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21882667 | 1 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 21882667 | 10 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 21882667 | 11 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 21882667 | 12 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 21882667 | 13 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 21882667 | 14 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 21882667 | 15 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21882667 | 16 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 21882667 | 2 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21882667 | 3 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21882667 | 4 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 21882667 | 5 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 21882667 | 6 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 21882667 | 7 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 21882667 | 8 | 524,288 |  | 8 | 11 | 9,961,472 | 
| leaf | VolatileBoundaryAir | 21882667 | 9 | 524,288 |  | 8 | 11 | 9,961,472 | 
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
| reth.prove_e2e.block_21882667 | AccessAdapterAir<16> | 21882667 | 14 | 65,536 |  | 12 | 25 | 2,424,832 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<16> | 21882667 | 15 | 262,144 |  | 12 | 25 | 9,699,328 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<16> | 21882667 | 16 | 1,024 |  | 12 | 25 | 37,888 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<16> | 21882667 | 2 | 524,288 |  | 12 | 25 | 19,398,656 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<16> | 21882667 | 3 | 524,288 |  | 12 | 25 | 19,398,656 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<16> | 21882667 | 4 | 262,144 |  | 12 | 25 | 9,699,328 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<2> | 21882667 | 15 | 32,768 |  | 12 | 11 | 753,664 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<2> | 21882667 | 16 | 262,144 |  | 12 | 11 | 6,029,312 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<2> | 21882667 | 3 | 8,192 |  | 12 | 11 | 188,416 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<32> | 21882667 | 0 | 32 |  | 12 | 41 | 1,696 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<32> | 21882667 | 14 | 32,768 |  | 12 | 41 | 1,736,704 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<32> | 21882667 | 15 | 131,072 |  | 12 | 41 | 6,946,816 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<32> | 21882667 | 16 | 512 |  | 12 | 41 | 27,136 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<32> | 21882667 | 2 | 262,144 |  | 12 | 41 | 13,893,632 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<32> | 21882667 | 3 | 262,144 |  | 12 | 41 | 13,893,632 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<32> | 21882667 | 4 | 131,072 |  | 12 | 41 | 6,946,816 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<4> | 21882667 | 0 | 64 |  | 12 | 13 | 1,600 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<4> | 21882667 | 15 | 16,384 |  | 12 | 13 | 409,600 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<4> | 21882667 | 16 | 131,072 |  | 12 | 13 | 3,276,800 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<4> | 21882667 | 3 | 8,192 |  | 12 | 13 | 204,800 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<4> | 21882667 | 4 | 16,384 |  | 12 | 13 | 409,600 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 0 | 1,048,576 |  | 12 | 17 | 30,408,704 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 1 | 1,048,576 |  | 12 | 17 | 30,408,704 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 10 | 16,384 |  | 12 | 17 | 475,136 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 11 | 8,192 |  | 12 | 17 | 237,568 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 12 | 8,192 |  | 12 | 17 | 237,568 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 13 | 4,096 |  | 12 | 17 | 118,784 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 14 | 524,288 |  | 12 | 17 | 15,204,352 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 15 | 2,097,152 |  | 12 | 17 | 60,817,408 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 16 | 1,048,576 |  | 12 | 17 | 30,408,704 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 2 | 2,097,152 |  | 12 | 17 | 60,817,408 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 3 | 2,097,152 |  | 12 | 17 | 60,817,408 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 4 | 524,288 |  | 12 | 17 | 15,204,352 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 5 | 2,048 |  | 12 | 17 | 59,392 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 6 | 2,048 |  | 12 | 17 | 59,392 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 7 | 2,048 |  | 12 | 17 | 59,392 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 8 | 4,096 |  | 12 | 17 | 118,784 | 
| reth.prove_e2e.block_21882667 | AccessAdapterAir<8> | 21882667 | 9 | 8,192 |  | 12 | 17 | 237,568 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 0 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 1 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 10 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 11 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 12 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 13 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 14 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 15 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 16 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 2 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 3 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 4 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 5 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 6 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 7 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 8 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | BitwiseOperationLookupAir<8> | 21882667 | 9 | 65,536 | 3 | 8 | 2 | 655,360 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 0 | 1 |  | 532 | 3,163 | 3,695 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 1 | 1 |  | 532 | 3,163 | 3,695 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 10 | 1 |  | 532 | 3,163 | 3,695 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 11 | 1 |  | 532 | 3,163 | 3,695 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 12 | 1 |  | 532 | 3,163 | 3,695 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 13 | 1 |  | 532 | 3,163 | 3,695 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 14 | 4,096 |  | 532 | 3,163 | 15,134,720 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 15 | 131,072 |  | 532 | 3,163 | 484,311,040 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 16 | 262,144 |  | 532 | 3,163 | 968,622,080 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 2 | 262,144 |  | 532 | 3,163 | 968,622,080 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 3 | 16,384 |  | 532 | 3,163 | 60,538,880 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 4 | 8,192 |  | 532 | 3,163 | 30,269,440 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 5 | 1 |  | 532 | 3,163 | 3,695 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 6 | 1 |  | 532 | 3,163 | 3,695 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 7 | 1 |  | 532 | 3,163 | 3,695 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 8 | 1 |  | 532 | 3,163 | 3,695 | 
| reth.prove_e2e.block_21882667 | KeccakVmAir | 21882667 | 9 | 1 |  | 532 | 3,163 | 3,695 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 0 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 1 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 10 | 16,384 |  | 12 | 32 | 720,896 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 11 | 8,192 |  | 12 | 32 | 360,448 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 12 | 8,192 |  | 12 | 32 | 360,448 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 13 | 4,096 |  | 12 | 32 | 180,224 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 14 | 524,288 |  | 12 | 32 | 23,068,672 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 15 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 16 | 2,097,152 |  | 12 | 32 | 92,274,688 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 2 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 3 | 1,048,576 |  | 12 | 32 | 46,137,344 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 4 | 65,536 |  | 12 | 32 | 2,883,584 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 5 | 2,048 |  | 12 | 32 | 90,112 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 6 | 2,048 |  | 12 | 32 | 90,112 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 7 | 2,048 |  | 12 | 32 | 90,112 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 8 | 4,096 |  | 12 | 32 | 180,224 | 
| reth.prove_e2e.block_21882667 | MemoryMerkleAir<8> | 21882667 | 9 | 8,192 |  | 12 | 32 | 360,448 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 0 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 1 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 10 | 16,384 |  | 8 | 20 | 458,752 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 11 | 8,192 |  | 8 | 20 | 229,376 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 12 | 8,192 |  | 8 | 20 | 229,376 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 13 | 4,096 |  | 8 | 20 | 114,688 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 14 | 524,288 |  | 8 | 20 | 14,680,064 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 15 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 16 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 2 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 3 | 1,048,576 |  | 8 | 20 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 4 | 32,768 |  | 8 | 20 | 917,504 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 5 | 2,048 |  | 8 | 20 | 57,344 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 6 | 2,048 |  | 8 | 20 | 57,344 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 7 | 2,048 |  | 8 | 20 | 57,344 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 8 | 4,096 |  | 8 | 20 | 114,688 | 
| reth.prove_e2e.block_21882667 | PersistentBoundaryAir<8> | 21882667 | 9 | 8,192 |  | 8 | 20 | 229,376 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 0 | 65,536 |  | 8 | 6 | 917,504 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 1 | 65,536 |  | 8 | 6 | 917,504 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 10 | 1 |  | 8 | 6 | 14 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 11 | 1 |  | 8 | 6 | 14 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 12 | 1 |  | 8 | 6 | 14 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 13 | 1 |  | 8 | 6 | 14 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 14 | 1,024 |  | 8 | 6 | 14,336 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 15 | 8,192 |  | 8 | 6 | 114,688 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 16 | 4,096 |  | 8 | 6 | 57,344 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 2 | 32,768 |  | 8 | 6 | 458,752 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 3 | 4,096 |  | 8 | 6 | 57,344 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 4 | 128 |  | 8 | 6 | 1,792 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 5 | 1 |  | 8 | 6 | 14 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 6 | 1 |  | 8 | 6 | 14 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 7 | 1 |  | 8 | 6 | 14 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 8 | 1 |  | 8 | 6 | 14 | 
| reth.prove_e2e.block_21882667 | PhantomAir | 21882667 | 9 | 1 |  | 8 | 6 | 14 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 0 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 1 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 10 | 16,384 |  | 8 | 300 | 5,046,272 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 11 | 16,384 |  | 8 | 300 | 5,046,272 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 12 | 16,384 |  | 8 | 300 | 5,046,272 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 13 | 4,096 |  | 8 | 300 | 1,261,568 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 14 | 262,144 |  | 8 | 300 | 80,740,352 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 15 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 16 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 2 | 1,048,576 |  | 8 | 300 | 322,961,408 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 3 | 524,288 |  | 8 | 300 | 161,480,704 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 4 | 32,768 |  | 8 | 300 | 10,092,544 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 5 | 4,096 |  | 8 | 300 | 1,261,568 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 6 | 4,096 |  | 8 | 300 | 1,261,568 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 7 | 4,096 |  | 8 | 300 | 1,261,568 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 8 | 4,096 |  | 8 | 300 | 1,261,568 | 
| reth.prove_e2e.block_21882667 | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21882667 | 9 | 8,192 |  | 8 | 300 | 2,523,136 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 0 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 1 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 10 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 11 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 12 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 13 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 14 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 15 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 16 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 2 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 3 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 4 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 5 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 6 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 7 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 8 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | ProgramAir | 21882667 | 9 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 0 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 1 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 10 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 11 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 12 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 13 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 14 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 15 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 16 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 2 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 3 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 4 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 5 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 6 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 7 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 8 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | RangeTupleCheckerAir<2> | 21882667 | 9 | 2,097,152 | 2 | 8 | 1 | 18,874,368 | 
| reth.prove_e2e.block_21882667 | Rv32HintStoreAir | 21882667 | 0 | 524,288 |  | 24 | 32 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | Rv32HintStoreAir | 21882667 | 1 | 524,288 |  | 24 | 32 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | Rv32HintStoreAir | 21882667 | 15 | 128 |  | 24 | 32 | 7,168 | 
| reth.prove_e2e.block_21882667 | Rv32HintStoreAir | 21882667 | 2 | 524,288 |  | 24 | 32 | 29,360,128 | 
| reth.prove_e2e.block_21882667 | Rv32HintStoreAir | 21882667 | 3 | 128 |  | 24 | 32 | 7,168 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 1 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 10 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 11 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 12 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 13 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 14 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 15 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 16 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 2 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 3 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 4 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 5 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 6 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 7 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 8 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VariableRangeCheckerAir | 21882667 | 9 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 0 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 1 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 10 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 11 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 12 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 13 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 14 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 15 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 16 | 4,194,304 |  | 28 | 36 | 268,435,456 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 2 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 3 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 4 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 5 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 6 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 7 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 8 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21882667 | 9 | 8,388,608 |  | 28 | 36 | 536,870,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 0 | 524,288 |  | 24 | 37 | 31,981,568 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 1 | 524,288 |  | 24 | 37 | 31,981,568 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 10 | 4,194,304 |  | 24 | 37 | 255,852,544 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 11 | 4,194,304 |  | 24 | 37 | 255,852,544 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 12 | 4,194,304 |  | 24 | 37 | 255,852,544 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 13 | 4,194,304 |  | 24 | 37 | 255,852,544 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 14 | 4,194,304 |  | 24 | 37 | 255,852,544 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 15 | 524,288 |  | 24 | 37 | 31,981,568 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 16 | 262,144 |  | 24 | 37 | 15,990,784 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 2 | 524,288 |  | 24 | 37 | 31,981,568 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 3 | 524,288 |  | 24 | 37 | 31,981,568 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 4 | 4,194,304 |  | 24 | 37 | 255,852,544 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 5 | 4,194,304 |  | 24 | 37 | 255,852,544 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 6 | 4,194,304 |  | 24 | 37 | 255,852,544 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 7 | 4,194,304 |  | 24 | 37 | 255,852,544 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 8 | 4,194,304 |  | 24 | 37 | 255,852,544 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21882667 | 9 | 4,194,304 |  | 24 | 37 | 255,852,544 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 0 | 1,048,576 |  | 28 | 53 | 84,934,656 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 1 | 1,048,576 |  | 28 | 53 | 84,934,656 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 10 | 131,072 |  | 28 | 53 | 10,616,832 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 11 | 65,536 |  | 28 | 53 | 5,308,416 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 12 | 131,072 |  | 28 | 53 | 10,616,832 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 13 | 262,144 |  | 28 | 53 | 21,233,664 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 14 | 1,048,576 |  | 28 | 53 | 84,934,656 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 15 | 2,097,152 |  | 28 | 53 | 169,869,312 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 16 | 262,144 |  | 28 | 53 | 21,233,664 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 2 | 2,097,152 |  | 28 | 53 | 169,869,312 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 3 | 2,097,152 |  | 28 | 53 | 169,869,312 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 4 | 1,048,576 |  | 28 | 53 | 84,934,656 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 5 | 65,536 |  | 28 | 53 | 5,308,416 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 6 | 65,536 |  | 28 | 53 | 5,308,416 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 7 | 65,536 |  | 28 | 53 | 5,308,416 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 8 | 131,072 |  | 28 | 53 | 10,616,832 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21882667 | 9 | 262,144 |  | 28 | 53 | 21,233,664 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 0 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 1 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 10 | 1,048,576 |  | 16 | 26 | 44,040,192 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 11 | 1,048,576 |  | 16 | 26 | 44,040,192 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 12 | 1,048,576 |  | 16 | 26 | 44,040,192 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 13 | 1,048,576 |  | 16 | 26 | 44,040,192 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 14 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 15 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 16 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 2 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 3 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 4 | 2,097,152 |  | 16 | 26 | 88,080,384 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 5 | 1,048,576 |  | 16 | 26 | 44,040,192 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 6 | 1,048,576 |  | 16 | 26 | 44,040,192 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 7 | 1,048,576 |  | 16 | 26 | 44,040,192 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 8 | 1,048,576 |  | 16 | 26 | 44,040,192 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21882667 | 9 | 1,048,576 |  | 16 | 26 | 44,040,192 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 0 | 1,048,576 |  | 20 | 32 | 54,525,952 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 1 | 1,048,576 |  | 20 | 32 | 54,525,952 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 10 | 32,768 |  | 20 | 32 | 1,703,936 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 11 | 65,536 |  | 20 | 32 | 3,407,872 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 12 | 65,536 |  | 20 | 32 | 3,407,872 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 13 | 65,536 |  | 20 | 32 | 3,407,872 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 14 | 1,048,576 |  | 20 | 32 | 54,525,952 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 15 | 2,097,152 |  | 20 | 32 | 109,051,904 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 16 | 262,144 |  | 20 | 32 | 13,631,488 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 2 | 2,097,152 |  | 20 | 32 | 109,051,904 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 3 | 2,097,152 |  | 20 | 32 | 109,051,904 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 4 | 262,144 |  | 20 | 32 | 13,631,488 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 5 | 65,536 |  | 20 | 32 | 3,407,872 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 6 | 65,536 |  | 20 | 32 | 3,407,872 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 7 | 65,536 |  | 20 | 32 | 3,407,872 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 8 | 32,768 |  | 20 | 32 | 1,703,936 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21882667 | 9 | 16,384 |  | 20 | 32 | 851,968 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 0 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 1 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 10 | 262,144 |  | 16 | 18 | 8,912,896 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 11 | 262,144 |  | 16 | 18 | 8,912,896 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 12 | 262,144 |  | 16 | 18 | 8,912,896 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 13 | 262,144 |  | 16 | 18 | 8,912,896 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 14 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 15 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 16 | 131,072 |  | 16 | 18 | 4,456,448 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 2 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 3 | 524,288 |  | 16 | 18 | 17,825,792 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 4 | 131,072 |  | 16 | 18 | 4,456,448 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 5 | 262,144 |  | 16 | 18 | 8,912,896 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 6 | 262,144 |  | 16 | 18 | 8,912,896 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 7 | 262,144 |  | 16 | 18 | 8,912,896 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 8 | 262,144 |  | 16 | 18 | 8,912,896 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21882667 | 9 | 262,144 |  | 16 | 18 | 8,912,896 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21882667 | 14 | 4,096 |  | 100 | 168 | 1,097,728 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21882667 | 15 | 16,384 |  | 100 | 168 | 4,390,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21882667 | 16 | 1 |  | 100 | 168 | 268 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21882667 | 2 | 2,048 |  | 100 | 168 | 548,864 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21882667 | 3 | 16,384 |  | 100 | 168 | 4,390,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, BaseAluCoreAir<32, 8> | 21882667 | 4 | 16,384 |  | 100 | 168 | 4,390,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21882667 | 14 | 1,024 |  | 36 | 169 | 209,920 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21882667 | 15 | 4,096 |  | 36 | 169 | 839,680 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21882667 | 2 | 1,024 |  | 36 | 169 | 209,920 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21882667 | 3 | 8,192 |  | 36 | 169 | 1,679,360 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, LessThanCoreAir<32, 8> | 21882667 | 4 | 4,096 |  | 36 | 169 | 839,680 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21882667 | 14 | 512 |  | 100 | 164 | 135,168 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21882667 | 15 | 1,024 |  | 100 | 164 | 270,336 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21882667 | 2 | 256 |  | 100 | 164 | 67,584 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21882667 | 3 | 4,096 |  | 100 | 164 | 1,081,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, MultiplicationCoreAir<32, 8> | 21882667 | 4 | 2,048 |  | 100 | 164 | 540,672 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21882667 | 14 | 1,024 |  | 84 | 241 | 332,800 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21882667 | 15 | 2,048 |  | 84 | 241 | 665,600 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21882667 | 2 | 256 |  | 84 | 241 | 83,200 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21882667 | 3 | 4,096 |  | 84 | 241 | 1,331,200 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapAdapterAir<2, 32, 32>, ShiftCoreAir<32, 8> | 21882667 | 4 | 2,048 |  | 84 | 241 | 665,600 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21882667 | 14 | 8,192 |  | 28 | 124 | 1,245,184 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21882667 | 15 | 16,384 |  | 28 | 124 | 2,490,368 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21882667 | 16 | 256 |  | 28 | 124 | 38,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21882667 | 2 | 4,096 |  | 28 | 124 | 622,592 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21882667 | 3 | 32,768 |  | 28 | 124 | 4,980,736 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32HeapBranchAdapterAir<2, 32>, BranchEqualCoreAir<32> | 21882667 | 4 | 16,384 |  | 28 | 124 | 2,490,368 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21882667 | 0 | 1 |  | 32 | 166 | 198 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21882667 | 15 | 16,384 |  | 32 | 166 | 3,244,032 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21882667 | 2 | 131,072 |  | 32 | 166 | 25,952,256 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 21882667 | 3 | 8,192 |  | 32 | 166 | 1,622,016 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 0 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 1 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 10 | 32,768 |  | 20 | 28 | 1,572,864 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 11 | 32,768 |  | 20 | 28 | 1,572,864 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 12 | 32,768 |  | 20 | 28 | 1,572,864 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 13 | 32,768 |  | 20 | 28 | 1,572,864 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 14 | 131,072 |  | 20 | 28 | 6,291,456 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 15 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 16 | 262,144 |  | 20 | 28 | 12,582,912 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 2 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 3 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 4 | 524,288 |  | 20 | 28 | 25,165,824 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 5 | 65,536 |  | 20 | 28 | 3,145,728 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 6 | 65,536 |  | 20 | 28 | 3,145,728 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 7 | 65,536 |  | 20 | 28 | 3,145,728 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 8 | 32,768 |  | 20 | 28 | 1,572,864 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21882667 | 9 | 32,768 |  | 20 | 28 | 1,572,864 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 0 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 1 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 14 | 524,288 |  | 28 | 35 | 33,030,144 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 15 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 16 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 2 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 3 | 1,048,576 |  | 28 | 35 | 66,060,288 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21882667 | 4 | 16,384 |  | 28 | 35 | 1,032,192 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 0 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 1 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 10 | 4,194,304 |  | 28 | 40 | 285,212,672 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 11 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 12 | 4,194,304 |  | 28 | 40 | 285,212,672 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 13 | 4,194,304 |  | 28 | 40 | 285,212,672 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 14 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 15 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 16 | 4,194,304 |  | 28 | 40 | 285,212,672 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 2 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 3 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 4 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 5 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 6 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 7 | 8,388,608 |  | 28 | 40 | 570,425,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 8 | 4,194,304 |  | 28 | 40 | 285,212,672 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21882667 | 9 | 4,194,304 |  | 28 | 40 | 285,212,672 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21882667 | 14 | 64 |  | 40 | 57 | 6,208 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21882667 | 15 | 256 |  | 40 | 57 | 24,832 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21882667 | 16 | 128 |  | 40 | 57 | 12,416 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21882667 | 2 | 512 |  | 40 | 57 | 49,664 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21882667 | 3 | 256 |  | 40 | 57 | 24,832 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21882667 | 4 | 1,024 |  | 40 | 57 | 99,328 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 1 | 1,024 |  | 40 | 39 | 80,896 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 10 | 2,097,152 |  | 40 | 39 | 165,675,008 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 11 | 2,097,152 |  | 40 | 39 | 165,675,008 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 12 | 2,097,152 |  | 40 | 39 | 165,675,008 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 13 | 2,097,152 |  | 40 | 39 | 165,675,008 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 14 | 1,048,576 |  | 40 | 39 | 82,837,504 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 15 | 131,072 |  | 40 | 39 | 10,354,688 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 16 | 2,048 |  | 40 | 39 | 161,792 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 2 | 65,536 |  | 40 | 39 | 5,177,344 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 3 | 131,072 |  | 40 | 39 | 10,354,688 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 4 | 2,097,152 |  | 40 | 39 | 165,675,008 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 5 | 2,097,152 |  | 40 | 39 | 165,675,008 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 6 | 2,097,152 |  | 40 | 39 | 165,675,008 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 7 | 2,097,152 |  | 40 | 39 | 165,675,008 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 8 | 2,097,152 |  | 40 | 39 | 165,675,008 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21882667 | 9 | 2,097,152 |  | 40 | 39 | 165,675,008 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 0 | 256 |  | 28 | 31 | 15,104 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 1 | 4,096 |  | 28 | 31 | 241,664 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 10 | 2,097,152 |  | 28 | 31 | 123,731,968 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 11 | 2,097,152 |  | 28 | 31 | 123,731,968 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 12 | 2,097,152 |  | 28 | 31 | 123,731,968 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 13 | 2,097,152 |  | 28 | 31 | 123,731,968 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 14 | 1,048,576 |  | 28 | 31 | 61,865,984 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 15 | 262,144 |  | 28 | 31 | 15,466,496 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 16 | 32,768 |  | 28 | 31 | 1,933,312 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 2 | 262,144 |  | 28 | 31 | 15,466,496 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 3 | 524,288 |  | 28 | 31 | 30,932,992 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 4 | 2,097,152 |  | 28 | 31 | 123,731,968 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 5 | 2,097,152 |  | 28 | 31 | 123,731,968 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 6 | 2,097,152 |  | 28 | 31 | 123,731,968 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 7 | 2,097,152 |  | 28 | 31 | 123,731,968 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 8 | 2,097,152 |  | 28 | 31 | 123,731,968 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21882667 | 9 | 2,097,152 |  | 28 | 31 | 123,731,968 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 0 | 262,144 |  | 16 | 21 | 9,699,328 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 1 | 262,144 |  | 16 | 21 | 9,699,328 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 10 | 16,384 |  | 16 | 21 | 606,208 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 11 | 16,384 |  | 16 | 21 | 606,208 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 12 | 16,384 |  | 16 | 21 | 606,208 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 13 | 16,384 |  | 16 | 21 | 606,208 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 14 | 32,768 |  | 16 | 21 | 1,212,416 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 15 | 131,072 |  | 16 | 21 | 4,849,664 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 16 | 131,072 |  | 16 | 21 | 4,849,664 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 2 | 262,144 |  | 16 | 21 | 9,699,328 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 3 | 131,072 |  | 16 | 21 | 4,849,664 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 4 | 16,384 |  | 16 | 21 | 606,208 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 5 | 32,768 |  | 16 | 21 | 1,212,416 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 6 | 32,768 |  | 16 | 21 | 1,212,416 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 7 | 32,768 |  | 16 | 21 | 1,212,416 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 8 | 16,384 |  | 16 | 21 | 606,208 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21882667 | 9 | 16,384 |  | 16 | 21 | 606,208 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 0 | 1 |  | 416 | 543 | 959 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 15 | 4,096 |  | 416 | 543 | 3,928,064 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 2 | 65,536 |  | 416 | 543 | 62,849,024 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 3 | 4,096 |  | 416 | 543 | 3,928,064 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21882667 | 0 | 1 |  | 160 | 261 | 421 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21882667 | 15 | 64 |  | 160 | 261 | 26,944 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21882667 | 2 | 1,024 |  | 160 | 261 | 431,104 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21882667 | 3 | 64 |  | 160 | 261 | 26,944 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 0 | 1 |  | 428 | 619 | 1,047 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 15 | 2,048 |  | 428 | 619 | 2,144,256 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 2 | 32,768 |  | 428 | 619 | 34,308,096 | 
| reth.prove_e2e.block_21882667 | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21882667 | 3 | 2,048 |  | 428 | 619 | 2,144,256 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 0 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 1 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 10 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 11 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 12 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 13 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 14 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 15 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 16 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 2 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 3 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 4 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 5 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 6 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 7 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 8 | 2 | 1 | 8 | 4 | 24 | 
| reth.prove_e2e.block_21882667 | VmConnectorAir | 21882667 | 9 | 2 | 1 | 8 | 4 | 24 | 

| group | block_number | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | num_segments | main_trace_commit_time_ms | main_cells_used | halo2_total_cells | halo2_keygen_time_ms | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 21882667 | 274 | 20,993 | 1,357,875 | 196,035,544 | 20,525 | 503 | 7,101 | 4,411 | 2,303 | 1 | 6,160 | 81,199,069 | 7,911,006 | 17,702 | 44 | 194 | 
| halo2_outer | 21882667 |  | 46,229 |  |  |  |  |  |  |  |  |  | 58,076,647 |  |  |  |  | 
| halo2_wrapper | 21882667 |  | 35,216 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
| reth.prove_e2e.block_21882667 | 21882667 |  |  |  |  |  |  |  |  |  | 17 |  |  |  |  |  |  | 

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
| internal.0 | 21882667 | 0 | 550 | 3,961 | 1,240,820 | 167,723,992 | 2,975 | 439 | 660 | 445 | 831 | 512 | 69,336,067 | 85 | 436 | 
| internal.0 | 21882667 | 1 | 550 | 3,960 | 1,241,551 | 167,723,992 | 2,971 | 437 | 648 | 446 | 839 | 516 | 69,290,455 | 81 | 439 | 
| internal.0 | 21882667 | 2 | 543 | 3,982 | 1,240,470 | 167,723,992 | 3,000 | 441 | 671 | 452 | 833 | 513 | 69,391,704 | 86 | 439 | 
| internal.0 | 21882667 | 3 | 550 | 4,006 | 1,240,191 | 167,723,992 | 3,017 | 445 | 681 | 451 | 837 | 516 | 69,382,597 | 84 | 439 | 
| internal.0 | 21882667 | 4 | 552 | 3,977 | 1,240,227 | 167,723,992 | 2,987 | 450 | 645 | 457 | 834 | 516 | 69,382,921 | 81 | 438 | 
| internal.0 | 21882667 | 5 | 530 | 3,960 | 1,240,205 | 167,723,992 | 2,991 | 445 | 661 | 447 | 834 | 514 | 69,382,723 | 87 | 439 | 
| internal.0 | 21882667 | 6 | 539 | 3,979 | 1,240,142 | 167,723,992 | 2,998 | 442 | 658 | 449 | 846 | 517 | 69,382,156 | 81 | 442 | 
| internal.0 | 21882667 | 7 | 540 | 3,975 | 1,241,262 | 167,723,992 | 2,992 | 437 | 647 | 447 | 849 | 523 | 69,346,641 | 85 | 443 | 
| internal.0 | 21882667 | 8 | 282 | 2,408 | 620,748 | 88,647,384 | 1,904 | 252 | 469 | 268 | 565 | 301 | 35,535,431 | 45 | 222 | 
| internal.1 | 21882667 | 10 | 511 | 3,904 | 1,214,146 | 167,723,992 | 2,990 | 442 | 654 | 455 | 842 | 512 | 66,386,800 | 81 | 403 | 
| internal.1 | 21882667 | 11 | 523 | 3,943 | 1,214,167 | 167,723,992 | 3,012 | 444 | 657 | 464 | 842 | 516 | 66,386,989 | 84 | 408 | 
| internal.1 | 21882667 | 12 | 521 | 3,938 | 1,214,141 | 167,723,992 | 3,014 | 442 | 666 | 445 | 839 | 513 | 66,386,755 | 105 | 403 | 
| internal.1 | 21882667 | 13 | 286 | 2,391 | 599,661 | 88,475,352 | 1,910 | 254 | 474 | 266 | 562 | 304 | 33,365,842 | 46 | 195 | 
| internal.1 | 21882667 | 9 | 528 | 3,915 | 1,214,182 | 167,723,992 | 2,982 | 438 | 652 | 448 | 838 | 518 | 66,387,124 | 85 | 405 | 
| internal.2 | 21882667 | 14 | 535 | 3,919 | 1,214,092 | 167,723,992 | 2,982 | 441 | 656 | 451 | 835 | 512 | 66,386,314 | 84 | 402 | 
| internal.2 | 21882667 | 15 | 531 | 3,950 | 1,214,135 | 167,723,992 | 3,017 | 443 | 665 | 458 | 844 | 517 | 66,386,701 | 85 | 402 | 
| internal.2 | 21882667 | 16 | 276 | 2,361 | 599,569 | 88,475,352 | 1,890 | 256 | 454 | 268 | 556 | 307 | 33,296,333 | 45 | 195 | 
| internal.3 | 21882667 | 17 | 530 | 3,946 | 1,214,167 | 167,723,992 | 3,018 | 436 | 675 | 450 | 846 | 524 | 66,386,989 | 82 | 398 | 
| internal.3 | 21882667 | 18 | 284 | 2,392 | 599,582 | 88,475,352 | 1,914 | 254 | 488 | 267 | 556 | 299 | 33,296,450 | 46 | 194 | 
| internal.4 | 21882667 | 19 | 527 | 3,932 | 1,206,314 | 169,870,296 | 3,013 | 449 | 668 | 448 | 838 | 523 | 65,764,778 | 83 | 392 | 
| internal.5 | 21882667 | 20 | 284 | 2,417 | 607,419 | 88,647,384 | 1,932 | 255 | 475 | 283 | 562 | 298 | 33,988,134 | 55 | 201 | 
| leaf | 21882667 | 0 | 1,000 | 7,934 | 1,583,227 | 379,266,776 | 6,499 | 883 | 1,225 | 989 | 2,184 | 1,062 | 121,197,349 | 152 | 435 | 
| leaf | 21882667 | 1 | 759 | 6,821 | 1,043,431 | 286,729,944 | 5,724 | 679 | 1,160 | 804 | 2,067 | 890 | 82,711,295 | 119 | 338 | 
| leaf | 21882667 | 10 | 711 | 6,445 | 989,811 | 253,437,656 | 5,407 | 604 | 1,142 | 740 | 1,993 | 827 | 80,406,525 | 98 | 327 | 
| leaf | 21882667 | 11 | 721 | 6,392 | 989,689 | 253,437,656 | 5,343 | 599 | 1,075 | 748 | 1,986 | 831 | 80,385,371 | 100 | 328 | 
| leaf | 21882667 | 12 | 736 | 6,448 | 989,763 | 253,437,656 | 5,383 | 596 | 1,120 | 748 | 1,983 | 831 | 80,486,585 | 100 | 329 | 
| leaf | 21882667 | 13 | 712 | 6,377 | 989,520 | 253,437,656 | 5,336 | 603 | 1,070 | 736 | 1,997 | 828 | 80,413,290 | 98 | 329 | 
| leaf | 21882667 | 14 | 868 | 7,348 | 1,310,156 | 327,558,872 | 6,101 | 787 | 1,195 | 896 | 2,124 | 954 | 98,913,811 | 142 | 379 | 
| leaf | 21882667 | 15 | 1,133 | 8,454 | 1,846,702 | 406,529,752 | 6,847 | 935 | 1,349 | 1,018 | 2,248 | 1,128 | 136,429,314 | 165 | 474 | 
| leaf | 21882667 | 16 | 793 | 6,874 | 1,201,833 | 290,341,848 | 5,735 | 683 | 1,181 | 788 | 2,068 | 891 | 90,784,473 | 121 | 346 | 
| leaf | 21882667 | 2 | 1,111 | 8,449 | 1,804,268 | 406,529,752 | 6,868 | 956 | 1,324 | 1,005 | 2,263 | 1,143 | 134,550,840 | 173 | 470 | 
| leaf | 21882667 | 3 | 1,172 | 8,512 | 1,846,645 | 406,529,752 | 6,867 | 961 | 1,350 | 1,016 | 2,249 | 1,125 | 136,395,285 | 163 | 473 | 
| leaf | 21882667 | 4 | 871 | 7,332 | 1,332,561 | 327,558,872 | 6,079 | 792 | 1,176 | 899 | 2,111 | 957 | 99,850,896 | 140 | 382 | 
| leaf | 21882667 | 5 | 733 | 6,412 | 989,305 | 253,437,656 | 5,352 | 591 | 1,111 | 730 | 1,981 | 835 | 80,374,541 | 99 | 327 | 
| leaf | 21882667 | 6 | 711 | 6,452 | 989,321 | 253,437,656 | 5,414 | 604 | 1,131 | 743 | 1,986 | 835 | 80,374,685 | 110 | 327 | 
| leaf | 21882667 | 7 | 726 | 6,406 | 989,318 | 253,437,656 | 5,350 | 600 | 1,102 | 730 | 1,983 | 832 | 80,374,658 | 100 | 330 | 
| leaf | 21882667 | 8 | 705 | 6,358 | 989,343 | 253,437,656 | 5,323 | 614 | 1,074 | 733 | 1,978 | 826 | 80,458,673 | 94 | 330 | 
| leaf | 21882667 | 9 | 707 | 6,390 | 989,619 | 253,437,656 | 5,355 | 600 | 1,108 | 733 | 1,981 | 826 | 80,416,219 | 102 | 328 | 
| root | 21882667 | 0 | 283 | 21,074 | 600,163 | 75,202,968 | 20,597 | 505 | 7,232 | 4,318 | 2,329 | 6,164 | 33,388,351 | 43 | 194 | 

| group | block_number | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 21882667 | 0 | 28 | 431 |  | 7,747,673 | 403 | 30 | 61 | 49 | 193 | 53 | 928,069 | 13 | 0 | 
| reth.prove_e2e.block_21882667 | 21882667 | 0 | 8,762 | 34,712 |  | 1,985,506,985 | 23,191 | 3,708 | 3,762 | 3,553 | 5,796 | 5,566 | 989,325,067 | 793 | 2,759 | 
| reth.prove_e2e.block_21882667 | 21882667 | 1 | 8,645 | 33,329 |  | 1,985,804,935 | 21,855 | 3,686 | 3,427 | 3,402 | 5,675 | 4,925 | 989,095,497 | 732 | 2,829 | 
| reth.prove_e2e.block_21882667 | 21882667 | 10 | 6,747 | 28,452 |  | 1,482,264,213 | 18,468 | 3,029 | 2,818 | 3,385 | 5,108 | 3,426 | 737,675,040 | 694 | 3,237 | 
| reth.prove_e2e.block_21882667 | 21882667 | 11 | 7,093 | 31,378 |  | 1,763,045,013 | 20,840 | 3,520 | 3,168 | 3,766 | 5,588 | 3,965 | 767,323,190 | 825 | 3,445 | 
| reth.prove_e2e.block_21882667 | 21882667 | 12 | 7,005 | 28,735 |  | 1,483,140,757 | 18,447 | 3,021 | 2,873 | 3,362 | 5,053 | 3,443 | 740,899,192 | 686 | 3,283 | 
| reth.prove_e2e.block_21882667 | 21882667 | 13 | 6,610 | 28,158 |  | 1,489,559,189 | 18,394 | 3,049 | 2,790 | 3,372 | 5,075 | 3,401 | 714,472,454 | 698 | 3,154 | 
| reth.prove_e2e.block_21882667 | 21882667 | 14 | 7,861 | 34,467 |  | 1,990,547,544 | 23,303 | 4,046 | 3,669 | 3,985 | 6,086 | 4,682 | 860,594,196 | 826 | 3,303 | 
| reth.prove_e2e.block_21882667 | 21882667 | 15 | 10,626 | 41,523 |  | 2,504,857,880 | 27,373 | 5,789 | 3,883 | 4,067 | 6,209 | 6,540 | 1,382,590,080 | 871 | 3,524 | 
| reth.prove_e2e.block_21882667 | 21882667 | 16 | 6,892 | 30,741 | 9,371,511 | 2,276,499,364 | 22,363 | 5,848 | 2,471 | 2,680 | 3,822 | 6,991 | 1,308,901,038 | 543 | 1,486 | 
| reth.prove_e2e.block_21882667 | 21882667 | 2 | 10,433 | 49,274 |  | 3,303,039,768 | 33,685 | 7,977 | 4,211 | 4,641 | 6,565 | 9,293 | 1,822,411,712 | 983 | 5,156 | 
| reth.prove_e2e.block_21882667 | 21882667 | 3 | 9,781 | 37,705 |  | 2,115,555,608 | 24,298 | 4,338 | 3,879 | 3,868 | 6,203 | 5,137 | 1,105,560,360 | 859 | 3,626 | 
| reth.prove_e2e.block_21882667 | 21882667 | 4 | 8,107 | 35,457 |  | 1,996,677,912 | 23,725 | 4,122 | 3,791 | 4,146 | 6,056 | 4,696 | 918,974,856 | 905 | 3,625 | 
| reth.prove_e2e.block_21882667 | 21882667 | 5 | 6,900 | 31,121 |  | 1,760,818,837 | 20,786 | 3,534 | 3,136 | 3,747 | 5,581 | 3,955 | 756,341,463 | 823 | 3,435 | 
| reth.prove_e2e.block_21882667 | 21882667 | 6 | 7,050 | 31,312 |  | 1,760,818,837 | 20,856 | 3,533 | 3,179 | 3,750 | 5,560 | 3,959 | 756,342,702 | 866 | 3,406 | 
| reth.prove_e2e.block_21882667 | 21882667 | 7 | 7,100 | 31,194 |  | 1,760,818,837 | 20,663 | 3,504 | 3,059 | 3,733 | 5,545 | 3,951 | 756,305,045 | 862 | 3,431 | 
| reth.prove_e2e.block_21882667 | 21882667 | 8 | 6,820 | 28,508 |  | 1,477,238,421 | 18,377 | 3,013 | 2,818 | 3,364 | 5,071 | 3,404 | 727,984,697 | 698 | 3,311 | 
| reth.prove_e2e.block_21882667 | 21882667 | 9 | 6,549 | 28,204 |  | 1,488,678,549 | 18,451 | 3,023 | 2,772 | 3,424 | 5,058 | 3,436 | 702,989,602 | 730 | 3,204 | 

</details>


Commit: https://github.com/axiom-crypto/openvm-reth-benchmark/actions/runs/13465068918

Max Segment Length: 8388508

Instance Type: m8g.24xlarge

Memory Allocator: https://github.com/axiom-crypto/openvm-reth-benchmark/commit/d8a4152cf8ee045cab3c8363b0d8c5c0683ae2c9

[Benchmark Workflow]()
