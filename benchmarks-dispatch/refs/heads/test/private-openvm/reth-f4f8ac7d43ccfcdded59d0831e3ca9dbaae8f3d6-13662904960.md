| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total |  690.92 |  233.89 |
| reth.prove_e2e.block_21000000 |  389.77 |  50.44 |
| leaf |  97.03 |  10.30 |
| internal.0 |  24.20 |  4.35 |
| internal.1 |  12.82 |  4.30 |
| internal.2 |  6.87 |  4.28 |
| internal.3 |  4.25 |  4.25 |
| root |  23.09 |  23.09 |
| halo2_outer |  59.02 |  59.02 |
| halo2_wrapper |  73.85 |  73.85 |
| agg_keygen |  23.39 |  22.96 |


| reth.prove_e2e.block_21000000 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  35,433.73 |  389,771 |  50,435 |  9,511 |
| `main_cells_used     ` |  1,152,956,770.18 |  12,682,524,472 |  1,933,574,592 |  290,300,354 |
| `total_cycles        ` |  18,520,715.91 |  203,727,875 |  24,792,369 |  1,960,882 |
| `execute_time_ms     ` |  3,006 |  33,066 |  6,271 |  280 |
| `trace_gen_time_ms   ` |  8,741 |  96,151 |  10,901 |  3,136 |
| `stark_prove_excluding_trace_time_ms` |  23,686.73 |  260,554 |  38,349 |  6,095 |
| `main_trace_commit_time_ms` |  6,206.45 |  68,271 |  13,339 |  1,504 |
| `generate_perm_trace_time_ms` |  762.09 |  8,383 |  1,054 |  131 |
| `perm_trace_commit_time_ms` |  3,544.45 |  38,989 |  4,551 |  825 |
| `quotient_poly_compute_time_ms` |  5,187.55 |  57,063 |  10,828 |  1,021 |
| `quotient_poly_commit_time_ms` |  3,279.18 |  36,071 |  4,120 |  1,051 |
| `pcs_opening_time_ms ` |  4,695.09 |  51,646 |  5,618 |  1,555 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  8,820.82 |  97,029 |  10,304 |  7,141 |
| `main_cells_used     ` |  138,864,084.45 |  1,527,504,929 |  172,314,571 |  100,043,559 |
| `total_cycles        ` |  2,298,498.27 |  25,283,481 |  2,926,134 |  1,578,555 |
| `execute_time_ms     ` |  495.91 |  5,455 |  593 |  379 |
| `trace_gen_time_ms   ` |  1,225.45 |  13,480 |  1,536 |  892 |
| `stark_prove_excluding_trace_time_ms` |  7,099.45 |  78,094 |  8,180 |  5,836 |
| `main_trace_commit_time_ms` |  1,185.91 |  13,045 |  1,415 |  929 |
| `generate_perm_trace_time_ms` |  169 |  1,859 |  211 |  120 |
| `perm_trace_commit_time_ms` |  1,079.45 |  11,874 |  1,289 |  841 |
| `quotient_poly_compute_time_ms` |  1,020.82 |  11,229 |  1,241 |  773 |
| `quotient_poly_commit_time_ms` |  1,482.18 |  16,304 |  1,722 |  1,197 |
| `pcs_opening_time_ms ` |  2,158.45 |  23,743 |  2,357 |  1,955 |

| internal.0 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  4,034 |  24,204 |  4,350 |  2,591 |
| `main_cells_used     ` |  71,484,007.67 |  428,904,046 |  77,857,816 |  39,802,333 |
| `total_cycles        ` |  1,375,164.67 |  8,250,988 |  1,500,927 |  749,983 |
| `execute_time_ms     ` |  427.67 |  2,566 |  470 |  232 |
| `trace_gen_time_ms   ` |  569.67 |  3,418 |  627 |  330 |
| `stark_prove_excluding_trace_time_ms` |  3,036.67 |  18,220 |  3,268 |  2,029 |
| `main_trace_commit_time_ms` |  515.83 |  3,095 |  559 |  320 |
| `generate_perm_trace_time_ms` |  88.50 |  531 |  111 |  50 |
| `perm_trace_commit_time_ms` |  454.17 |  2,725 |  493 |  287 |
| `quotient_poly_compute_time_ms` |  458.67 |  2,752 |  498 |  278 |
| `quotient_poly_commit_time_ms` |  691.67 |  4,150 |  754 |  490 |
| `pcs_opening_time_ms ` |  824 |  4,944 |  873 |  600 |

| internal.1 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  4,274.33 |  12,823 |  4,302 |  4,252 |
| `main_cells_used     ` |  74,659,518.33 |  223,978,555 |  74,850,404 |  74,278,008 |
| `total_cycles        ` |  1,469,672.67 |  4,409,018 |  1,472,317 |  1,464,413 |
| `execute_time_ms     ` |  427.67 |  1,283 |  432 |  420 |
| `trace_gen_time_ms   ` |  608 |  1,824 |  611 |  603 |
| `stark_prove_excluding_trace_time_ms` |  3,238.67 |  9,716 |  3,260 |  3,221 |
| `main_trace_commit_time_ms` |  555.33 |  1,666 |  560 |  552 |
| `generate_perm_trace_time_ms` |  100 |  300 |  106 |  93 |
| `perm_trace_commit_time_ms` |  490 |  1,470 |  495 |  485 |
| `quotient_poly_compute_time_ms` |  496 |  1,488 |  499 |  494 |
| `quotient_poly_commit_time_ms` |  722.33 |  2,167 |  751 |  705 |
| `pcs_opening_time_ms ` |  870.67 |  2,612 |  878 |  863 |

| internal.2 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  3,437 |  6,874 |  4,281 |  2,593 |
| `main_cells_used     ` |  56,583,081.50 |  113,166,163 |  74,849,990 |  38,316,173 |
| `total_cycles        ` |  1,104,372.50 |  2,208,745 |  1,472,271 |  736,474 |
| `execute_time_ms     ` |  322.50 |  645 |  429 |  216 |
| `trace_gen_time_ms   ` |  464.50 |  929 |  617 |  312 |
| `stark_prove_excluding_trace_time_ms` |  2,650 |  5,300 |  3,235 |  2,065 |
| `main_trace_commit_time_ms` |  441.50 |  883 |  554 |  329 |
| `generate_perm_trace_time_ms` |  75 |  150 |  92 |  58 |
| `perm_trace_commit_time_ms` |  389 |  778 |  486 |  292 |
| `quotient_poly_compute_time_ms` |  383 |  766 |  493 |  273 |
| `quotient_poly_commit_time_ms` |  627.50 |  1,255 |  736 |  519 |
| `pcs_opening_time_ms ` |  730.50 |  1,461 |  871 |  590 |

| internal.3 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  4,248 |  4,248 |  4,248 |  4,248 |
| `main_cells_used     ` |  74,278,638 |  74,278,638 |  74,278,638 |  74,278,638 |
| `total_cycles        ` |  1,464,483 |  1,464,483 |  1,464,483 |  1,464,483 |
| `execute_time_ms     ` |  421 |  421 |  421 |  421 |
| `trace_gen_time_ms   ` |  592 |  592 |  592 |  592 |
| `stark_prove_excluding_trace_time_ms` |  3,235 |  3,235 |  3,235 |  3,235 |
| `main_trace_commit_time_ms` |  554 |  554 |  554 |  554 |
| `generate_perm_trace_time_ms` |  91 |  91 |  91 |  91 |
| `perm_trace_commit_time_ms` |  497 |  497 |  497 |  497 |
| `quotient_poly_compute_time_ms` |  496 |  496 |  496 |  496 |
| `quotient_poly_commit_time_ms` |  721 |  721 |  721 |  721 |
| `pcs_opening_time_ms ` |  872 |  872 |  872 |  872 |

| root |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  23,094 |  23,094 |  23,094 |  23,094 |
| `main_cells_used     ` |  38,338,736 |  38,338,736 |  38,338,736 |  38,338,736 |
| `total_cycles        ` |  736,982 |  736,982 |  736,982 |  736,982 |
| `execute_time_ms     ` |  214 |  214 |  214 |  214 |
| `trace_gen_time_ms   ` |  327 |  327 |  327 |  327 |
| `stark_prove_excluding_trace_time_ms` |  22,553 |  22,553 |  22,553 |  22,553 |
| `main_trace_commit_time_ms` |  6,871 |  6,871 |  6,871 |  6,871 |
| `generate_perm_trace_time_ms` |  61 |  61 |  61 |  61 |
| `perm_trace_commit_time_ms` |  4,489 |  4,489 |  4,489 |  4,489 |
| `quotient_poly_compute_time_ms` |  555 |  555 |  555 |  555 |
| `quotient_poly_commit_time_ms` |  8,150 |  8,150 |  8,150 |  8,150 |
| `pcs_opening_time_ms ` |  2,422 |  2,422 |  2,422 |  2,422 |

| halo2_outer |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  59,025 |  59,025 |  59,025 |  59,025 |
| `main_cells_used     ` |  87,097,606 |  87,097,606 |  87,097,606 |  87,097,606 |

| halo2_wrapper |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  73,849 |  73,849 |  73,849 |  73,849 |

| agg_keygen |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  11,696.50 |  23,393 |  22,960 |  433 |
| `main_cells_used     ` |  44,549,689.50 |  89,099,379 |  88,171,310 |  928,069 |
| `total_cycles        ` |  1,567,487 |  1,567,487 |  1,567,487 |  1,567,487 |
| `execute_time_ms     ` |  103 |  206 |  206 |  0 |
| `trace_gen_time_ms   ` |  176 |  352 |  323 |  29 |
| `stark_prove_excluding_trace_time_ms` |  11,417.50 |  22,835 |  22,431 |  404 |
| `main_trace_commit_time_ms` |  3,461.50 |  6,923 |  6,873 |  50 |
| `generate_perm_trace_time_ms` |  28 |  56 |  45 |  11 |
| `perm_trace_commit_time_ms` |  2,199 |  4,398 |  4,347 |  51 |
| `quotient_poly_compute_time_ms` |  292.50 |  585 |  555 |  30 |
| `quotient_poly_commit_time_ms` |  4,132.50 |  8,265 |  8,208 |  57 |
| `pcs_opening_time_ms ` |  1,298.50 |  2,597 |  2,397 |  200 |



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
| Rv32HintStoreAir | 21000000 | 4 | 18 | 23 | 
| Sha256VmAir | 21000000 | 4 | 50 | 779 | 
| VariableRangeCheckerAir | 21000000 | 1 | 1 | 4 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 | 4 | 20 | 31 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 | 4 | 18 | 36 | 
| VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 | 4 | 24 | 85 | 
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
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 4 | 18 | 25 | 
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 4 | 17 | 31 | 
| VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 4 | 25 | 76 | 
| VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 4 | 24 | 23 | 
| VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 4 | 19 | 13 | 
| VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 4 | 11 | 12 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 411 | 373 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<1, 4, 8, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 1,716 | 1,283 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 156 | 149 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 12, 12, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 4,370 | 3,258 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 422 | 346 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 4, 10, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 1,303 | 965 | 
| VmAirWrapper<Rv32VecHeapAdapterAir<2, 4, 12, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 2,903 | 2,170 | 
| VmAirWrapper<Rv32VecHeapTwoReadsAdapterAir<12, 10, 12, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 3,977 | 2,964 | 
| VmAirWrapper<Rv32VecHeapTwoReadsAdapterAir<4, 2, 4, 32, 32>, FieldExpressionCoreAir> | 21000000 | 4 | 565 | 416 | 
| VmConnectorAir | 21000000 | 4 | 3 | 8 | 

| block_number | execute_time_ms |
| --- | --- |
| 21000000 | 216 | 

| group | air_name | block_number | rows | quotient_deg | prep_cols | perm_cols | main_cols | interactions | constraints | cells |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | AccessAdapterAir<16> | 21000000 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<2> | 21000000 | 524,288 | 8 |  | 12 | 11 | 5 | 12 | 12,058,624 | 
| agg_keygen | AccessAdapterAir<32> | 21000000 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<4> | 21000000 | 262,144 | 8 |  | 12 | 13 | 5 | 12 | 6,553,600 | 
| agg_keygen | AccessAdapterAir<64> | 21000000 |  | 2 |  |  |  | 5 | 12 |  | 
| agg_keygen | AccessAdapterAir<8> | 21000000 | 8,192 | 8 |  | 12 | 17 | 5 | 12 | 237,568 | 
| agg_keygen | BitwiseOperationLookupAir<8> | 21000000 |  | 2 |  |  |  | 2 | 4 |  | 
| agg_keygen | FriReducedOpeningAir | 21000000 | 524,288 | 8 |  | 44 | 27 | 39 | 60 | 37,224,448 | 
| agg_keygen | MemoryMerkleAir<8> | 21000000 |  | 2 |  |  |  | 4 | 39 |  | 
| agg_keygen | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 131,072 | 8 |  | 160 | 399 | 136 | 533 | 73,269,248 | 
| agg_keygen | PersistentBoundaryAir<8> | 21000000 |  | 2 |  |  |  | 3 | 6 |  | 
| agg_keygen | PhantomAir | 21000000 | 16,384 | 4 |  | 8 | 6 | 3 | 5 | 229,376 | 
| agg_keygen | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 21000000 |  | 2 |  |  |  | 1 | 286 |  | 
| agg_keygen | ProgramAir | 21000000 | 131,072 | 1 |  | 8 | 10 | 1 | 4 | 2,359,296 | 
| agg_keygen | RangeTupleCheckerAir<2> | 21000000 |  | 1 |  |  |  | 1 | 4 |  | 
| agg_keygen | Rv32HintStoreAir | 21000000 |  | 2 |  |  |  | 18 | 28 |  | 
| agg_keygen | VariableRangeCheckerAir | 21000000 | 262,144 | 1 | 2 | 8 | 1 | 1 | 4 | 2,359,296 | 
| agg_keygen | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 1,048,576 | 8 |  | 20 | 29 | 15 | 23 | 51,380,224 | 
| agg_keygen | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 262,144 | 8 |  | 16 | 23 | 11 | 22 | 10,223,616 | 
| agg_keygen | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 32,768 | 8 |  | 12 | 9 | 7 | 6 | 688,128 | 
| agg_keygen | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 64 | 8 |  | 16 | 23 | 11 | 23 | 2,496 | 
| agg_keygen | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 524,288 | 8 |  | 24 | 22 | 15 | 16 | 24,117,248 | 
| agg_keygen | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 131,072 | 8 |  | 24 | 31 | 15 | 16 | 7,208,960 | 
| agg_keygen | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 131,072 | 8 |  | 20 | 38 | 15 | 23 | 7,602,176 | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 20 | 37 |  | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 18 | 40 |  | 
| agg_keygen | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 24 | 91 |  | 
| agg_keygen | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 21000000 |  | 2 |  |  |  | 11 | 20 |  | 
| agg_keygen | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 13 | 35 |  | 
| agg_keygen | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 |  | 2 |  |  |  | 10 | 18 |  | 
| agg_keygen | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 21000000 |  | 2 |  |  |  | 16 | 20 |  | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 18 | 30 |  | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 |  | 2 |  |  |  | 17 | 37 |  | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 25 | 84 |  | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 24 | 31 |  | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 |  | 2 |  |  |  | 19 | 19 |  | 
| agg_keygen | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 |  | 2 |  |  |  | 11 | 15 |  | 
| agg_keygen | VmConnectorAir | 21000000 | 2 | 4 | 1 | 8 | 4 | 3 | 9 | 24 | 
| agg_keygen | VolatileBoundaryAir | 21000000 | 131,072 | 4 |  | 8 | 11 | 4 | 16 | 2,490,368 | 

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
| internal.0 | AccessAdapterAir<8> | 21000000 | 0 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 1 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 2 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 3 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 4 | 8,192 |  | 12 | 17 | 237,568 | 
| internal.0 | AccessAdapterAir<8> | 21000000 | 5 | 4,096 |  | 12 | 17 | 118,784 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 0 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 1 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 2 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 3 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 4 | 262,144 |  | 44 | 27 | 18,612,224 | 
| internal.0 | FriReducedOpeningAir | 21000000 | 5 | 131,072 |  | 44 | 27 | 9,306,112 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 0 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 1 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 2 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 3 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 4 | 65,536 |  | 160 | 399 | 36,634,624 | 
| internal.0 | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 5 | 32,768 |  | 160 | 399 | 18,317,312 | 
| internal.0 | PhantomAir | 21000000 | 0 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.0 | PhantomAir | 21000000 | 1 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.0 | PhantomAir | 21000000 | 2 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.0 | PhantomAir | 21000000 | 3 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.0 | PhantomAir | 21000000 | 4 | 16,384 |  | 8 | 6 | 229,376 | 
| internal.0 | PhantomAir | 21000000 | 5 | 8,192 |  | 8 | 6 | 114,688 | 
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
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 0 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 1 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 2 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 3 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 4 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| internal.0 | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 5 | 524,288 |  | 20 | 29 | 25,690,112 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 0 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 1 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 2 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 3 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 4 | 262,144 |  | 16 | 23 | 10,223,616 | 
| internal.0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 5 | 131,072 |  | 16 | 23 | 5,111,808 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 0 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 1 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 2 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 3 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 4 | 32,768 |  | 12 | 9 | 688,128 | 
| internal.0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 5 | 16,384 |  | 12 | 9 | 344,064 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 0 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 1 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 2 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 3 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 4 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 21000000 | 5 | 64 |  | 16 | 23 | 2,496 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 0 | 524,288 |  | 24 | 22 | 24,117,248 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 1 | 524,288 |  | 24 | 22 | 24,117,248 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 2 | 524,288 |  | 24 | 22 | 24,117,248 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 3 | 524,288 |  | 24 | 22 | 24,117,248 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 4 | 524,288 |  | 24 | 22 | 24,117,248 | 
| internal.0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 5 | 262,144 |  | 24 | 22 | 12,058,624 | 
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
| internal.0 | VolatileBoundaryAir | 21000000 | 0 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 1 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 2 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 3 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 4 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.0 | VolatileBoundaryAir | 21000000 | 5 | 131,072 |  | 8 | 11 | 2,490,368 | 
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
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 6 | 524,288 |  | 24 | 22 | 24,117,248 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 7 | 524,288 |  | 24 | 22 | 24,117,248 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 8 | 524,288 |  | 24 | 22 | 24,117,248 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 6 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 7 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.1 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 8 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 6 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 7 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 8 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.1 | VmConnectorAir | 21000000 | 6 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VmConnectorAir | 21000000 | 7 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VmConnectorAir | 21000000 | 8 | 2 | 1 | 8 | 4 | 24 | 
| internal.1 | VolatileBoundaryAir | 21000000 | 6 | 262,144 |  | 8 | 11 | 4,980,736 | 
| internal.1 | VolatileBoundaryAir | 21000000 | 7 | 262,144 |  | 8 | 11 | 4,980,736 | 
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
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 10 | 262,144 |  | 24 | 22 | 12,058,624 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 9 | 524,288 |  | 24 | 22 | 24,117,248 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 10 | 65,536 |  | 24 | 31 | 3,604,480 | 
| internal.2 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 9 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 10 | 65,536 |  | 20 | 38 | 3,801,088 | 
| internal.2 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 9 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.2 | VmConnectorAir | 21000000 | 10 | 2 | 1 | 8 | 4 | 24 | 
| internal.2 | VmConnectorAir | 21000000 | 9 | 2 | 1 | 8 | 4 | 24 | 
| internal.2 | VolatileBoundaryAir | 21000000 | 10 | 131,072 |  | 8 | 11 | 2,490,368 | 
| internal.2 | VolatileBoundaryAir | 21000000 | 9 | 262,144 |  | 8 | 11 | 4,980,736 | 
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
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 11 | 524,288 |  | 24 | 22 | 24,117,248 | 
| internal.3 | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 11 | 131,072 |  | 24 | 31 | 7,208,960 | 
| internal.3 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 11 | 131,072 |  | 20 | 38 | 7,602,176 | 
| internal.3 | VmConnectorAir | 21000000 | 11 | 2 | 1 | 8 | 4 | 24 | 
| internal.3 | VolatileBoundaryAir | 21000000 | 11 | 262,144 |  | 8 | 11 | 4,980,736 | 
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
| leaf | AccessAdapterAir<4> | 21000000 | 7 | 1,048,576 |  | 12 | 13 | 26,214,400 | 
| leaf | AccessAdapterAir<4> | 21000000 | 8 | 1,048,576 |  | 12 | 13 | 26,214,400 | 
| leaf | AccessAdapterAir<4> | 21000000 | 9 | 524,288 |  | 12 | 13 | 13,107,200 | 
| leaf | AccessAdapterAir<8> | 21000000 | 0 | 32,768 |  | 12 | 17 | 950,272 | 
| leaf | AccessAdapterAir<8> | 21000000 | 1 | 16,384 |  | 12 | 17 | 475,136 | 
| leaf | AccessAdapterAir<8> | 21000000 | 10 | 16,384 |  | 12 | 17 | 475,136 | 
| leaf | AccessAdapterAir<8> | 21000000 | 2 | 16,384 |  | 12 | 17 | 475,136 | 
| leaf | AccessAdapterAir<8> | 21000000 | 3 | 16,384 |  | 12 | 17 | 475,136 | 
| leaf | AccessAdapterAir<8> | 21000000 | 4 | 32,768 |  | 12 | 17 | 950,272 | 
| leaf | AccessAdapterAir<8> | 21000000 | 5 | 32,768 |  | 12 | 17 | 950,272 | 
| leaf | AccessAdapterAir<8> | 21000000 | 6 | 32,768 |  | 12 | 17 | 950,272 | 
| leaf | AccessAdapterAir<8> | 21000000 | 7 | 32,768 |  | 12 | 17 | 950,272 | 
| leaf | AccessAdapterAir<8> | 21000000 | 8 | 32,768 |  | 12 | 17 | 950,272 | 
| leaf | AccessAdapterAir<8> | 21000000 | 9 | 16,384 |  | 12 | 17 | 475,136 | 
| leaf | FriReducedOpeningAir | 21000000 | 0 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| leaf | FriReducedOpeningAir | 21000000 | 1 | 524,288 |  | 44 | 27 | 37,224,448 | 
| leaf | FriReducedOpeningAir | 21000000 | 10 | 524,288 |  | 44 | 27 | 37,224,448 | 
| leaf | FriReducedOpeningAir | 21000000 | 2 | 524,288 |  | 44 | 27 | 37,224,448 | 
| leaf | FriReducedOpeningAir | 21000000 | 3 | 524,288 |  | 44 | 27 | 37,224,448 | 
| leaf | FriReducedOpeningAir | 21000000 | 4 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| leaf | FriReducedOpeningAir | 21000000 | 5 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| leaf | FriReducedOpeningAir | 21000000 | 6 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| leaf | FriReducedOpeningAir | 21000000 | 7 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| leaf | FriReducedOpeningAir | 21000000 | 8 | 1,048,576 |  | 44 | 27 | 74,448,896 | 
| leaf | FriReducedOpeningAir | 21000000 | 9 | 524,288 |  | 44 | 27 | 37,224,448 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 0 | 131,072 |  | 160 | 399 | 73,269,248 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 1 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 10 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 2 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 3 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 4 | 131,072 |  | 160 | 399 | 73,269,248 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 5 | 131,072 |  | 160 | 399 | 73,269,248 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 6 | 131,072 |  | 160 | 399 | 73,269,248 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 7 | 131,072 |  | 160 | 399 | 73,269,248 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 8 | 131,072 |  | 160 | 399 | 73,269,248 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 21000000 | 9 | 65,536 |  | 160 | 399 | 36,634,624 | 
| leaf | PhantomAir | 21000000 | 0 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21000000 | 1 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21000000 | 10 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21000000 | 2 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21000000 | 3 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21000000 | 4 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21000000 | 5 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21000000 | 6 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21000000 | 7 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21000000 | 8 | 8,192 |  | 8 | 6 | 114,688 | 
| leaf | PhantomAir | 21000000 | 9 | 8,192 |  | 8 | 6 | 114,688 | 
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
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 0 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 1 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 10 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 2 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 3 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 4 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 5 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 6 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 7 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 8 | 2,097,152 |  | 20 | 29 | 102,760,448 | 
| leaf | VmAirWrapper<AluNativeAdapterAir, FieldArithmeticCoreAir> | 21000000 | 9 | 1,048,576 |  | 20 | 29 | 51,380,224 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 0 | 524,288 |  | 16 | 23 | 20,447,232 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 1 | 262,144 |  | 16 | 23 | 10,223,616 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 10 | 262,144 |  | 16 | 23 | 10,223,616 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 2 | 262,144 |  | 16 | 23 | 10,223,616 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 3 | 262,144 |  | 16 | 23 | 10,223,616 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 4 | 524,288 |  | 16 | 23 | 20,447,232 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 5 | 524,288 |  | 16 | 23 | 20,447,232 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 6 | 524,288 |  | 16 | 23 | 20,447,232 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 7 | 524,288 |  | 16 | 23 | 20,447,232 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 8 | 524,288 |  | 16 | 23 | 20,447,232 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 21000000 | 9 | 262,144 |  | 16 | 23 | 10,223,616 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 0 | 32,768 |  | 12 | 9 | 688,128 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 1 | 32,768 |  | 12 | 9 | 688,128 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 10 | 32,768 |  | 12 | 9 | 688,128 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 2 | 32,768 |  | 12 | 9 | 688,128 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 3 | 32,768 |  | 12 | 9 | 688,128 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 4 | 65,536 |  | 12 | 9 | 1,376,256 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 5 | 65,536 |  | 12 | 9 | 1,376,256 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 6 | 65,536 |  | 12 | 9 | 1,376,256 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 7 | 65,536 |  | 12 | 9 | 1,376,256 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 8 | 65,536 |  | 12 | 9 | 1,376,256 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 21000000 | 9 | 32,768 |  | 12 | 9 | 688,128 | 
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
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 0 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 1 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 10 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 2 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 3 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 4 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 5 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 6 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 7 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 8 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 9 | 524,288 |  | 24 | 22 | 24,117,248 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 0 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 1 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 10 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 2 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 3 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 4 | 262,144 |  | 24 | 31 | 14,417,920 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 5 | 262,144 |  | 24 | 31 | 14,417,920 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 6 | 262,144 |  | 24 | 31 | 14,417,920 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 7 | 262,144 |  | 24 | 31 | 14,417,920 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 8 | 262,144 |  | 24 | 31 | 14,417,920 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 9 | 131,072 |  | 24 | 31 | 7,208,960 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 21000000 | 0 | 262,144 |  | 20 | 38 | 15,204,352 | 
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
| leaf | VolatileBoundaryAir | 21000000 | 0 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21000000 | 1 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21000000 | 10 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21000000 | 2 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21000000 | 3 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21000000 | 4 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21000000 | 5 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21000000 | 6 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21000000 | 7 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21000000 | 8 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
| leaf | VolatileBoundaryAir | 21000000 | 9 | 1,048,576 |  | 8 | 11 | 19,922,944 | 
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
| root | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 21000000 | 0 | 262,144 |  | 16 | 22 | 9,961,472 | 
| root | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 21000000 | 0 | 65,536 |  | 16 | 31 | 3,080,192 | 
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
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 0 | 1 |  | 48 | 36 | 84 | 
| agg_keygen | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 0 | 1 |  | 52 | 41 | 93 | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 0 | 1 |  | 72 | 59 | 131 | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 0 | 1 |  | 72 | 39 | 111 | 
| agg_keygen | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 0 | 1 |  | 52 | 31 | 83 | 
| agg_keygen | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 21000000 | 0 | 1 |  | 28 | 21 | 49 | 
| agg_keygen | VmConnectorAir | 21000000 | 0 | 2 | 1 | 12 | 4 | 32 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<16> | 21000000 | 0 | 64 |  | 12 | 25 | 2,368 | 
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
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 0 | 32 |  | 12 | 41 | 1,696 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 10 | 8 |  | 12 | 41 | 424 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 4 | 524,288 |  | 12 | 41 | 27,787,264 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 5 | 131,072 |  | 12 | 41 | 6,946,816 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 6 | 131,072 |  | 12 | 41 | 6,946,816 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 7 | 131,072 |  | 12 | 41 | 6,946,816 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 8 | 131,072 |  | 12 | 41 | 6,946,816 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<32> | 21000000 | 9 | 4,096 |  | 12 | 41 | 217,088 | 
| reth.prove_e2e.block_21000000 | AccessAdapterAir<4> | 21000000 | 0 | 64 |  | 12 | 13 | 1,600 | 
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
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 0 | 4 |  | 8 | 6 | 56 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 1 | 1 |  | 8 | 6 | 14 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 10 | 1 |  | 8 | 6 | 14 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 2 | 1 |  | 8 | 6 | 14 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 3 | 2 |  | 8 | 6 | 28 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 4 | 256 |  | 8 | 6 | 3,584 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 5 | 4 |  | 8 | 6 | 56 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 6 | 32 |  | 8 | 6 | 448 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 7 | 8 |  | 8 | 6 | 112 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 8 | 32 |  | 8 | 6 | 448 | 
| reth.prove_e2e.block_21000000 | PhantomAir | 21000000 | 9 | 8 |  | 8 | 6 | 112 | 
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
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 0 | 1,048,576 |  | 16 | 18 | 35,651,584 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 1 | 1,048,576 |  | 16 | 18 | 35,651,584 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 10 | 32,768 |  | 16 | 18 | 1,114,112 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 21000000 | 2 | 1,048,576 |  | 16 | 18 | 35,651,584 | 
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
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 0 | 1,048,576 |  | 28 | 36 | 67,108,864 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 1 | 1,048,576 |  | 28 | 36 | 67,108,864 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 10 | 32,768 |  | 28 | 36 | 2,097,152 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 2 | 1,048,576 |  | 28 | 36 | 67,108,864 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 3 | 524,288 |  | 28 | 36 | 33,554,432 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 4 | 1,048,576 |  | 28 | 36 | 67,108,864 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 5 | 1,048,576 |  | 28 | 36 | 67,108,864 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 6 | 2,097,152 |  | 28 | 36 | 134,217,728 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 7 | 1,048,576 |  | 28 | 36 | 67,108,864 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 8 | 1,048,576 |  | 28 | 36 | 67,108,864 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 21000000 | 9 | 1,048,576 |  | 28 | 36 | 67,108,864 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 0 | 8,388,608 |  | 28 | 41 | 578,813,952 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 1 | 8,388,608 |  | 28 | 41 | 578,813,952 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 10 | 1,048,576 |  | 28 | 41 | 72,351,744 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 2 | 8,388,608 |  | 28 | 41 | 578,813,952 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 3 | 4,194,304 |  | 28 | 41 | 289,406,976 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 4 | 8,388,608 |  | 28 | 41 | 578,813,952 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 5 | 8,388,608 |  | 28 | 41 | 578,813,952 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 6 | 8,388,608 |  | 28 | 41 | 578,813,952 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 7 | 8,388,608 |  | 28 | 41 | 578,813,952 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 8 | 8,388,608 |  | 28 | 41 | 578,813,952 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 21000000 | 9 | 8,388,608 |  | 28 | 41 | 578,813,952 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 4 | 64 |  | 40 | 59 | 6,336 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 5 | 256 |  | 40 | 59 | 25,344 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 6 | 256 |  | 40 | 59 | 25,344 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 7 | 64 |  | 40 | 59 | 6,336 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 8 | 128 |  | 40 | 59 | 12,672 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 21000000 | 9 | 512 |  | 40 | 59 | 50,688 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 21000000 | 1 | 512 |  | 40 | 39 | 40,448 | 
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
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 1 | 1,024 |  | 28 | 31 | 60,416 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 10 | 8,192 |  | 28 | 31 | 483,328 | 
| reth.prove_e2e.block_21000000 | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 21000000 | 2 | 4,096 |  | 28 | 31 | 241,664 | 
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
| agg_keygen | 21000000 | 323 | 22,960 | 1,567,487 | 238,004,696 | 22,431 | 555 | 8,208 | 4,347 | 2,397 | 1 | 6,873 | 88,171,310 | 10,581,013 | 32,747 | 45 | 206 | 
| halo2_outer | 21000000 |  | 59,025 |  |  |  |  |  |  |  |  |  | 87,097,606 |  |  |  |  | 
| halo2_wrapper | 21000000 |  | 73,849 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
| reth.prove_e2e.block_21000000 | 21000000 |  |  |  |  |  |  |  |  |  | 11 |  |  |  |  |  |  | 

| group | block_number | cell_tracker_span | simple_advice_cells | lookup_advice_cells | fixed_cells |
| --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | VerifierProgram | 589,248 | 150,761 | 186,899 | 
| agg_keygen | 21000000 | VerifierProgram;PoseidonCell | 19,600 |  | 5,800 | 
| agg_keygen | 21000000 | VerifierProgram;stage-c-build-rounds | 335,431 | 737 | 94,619 | 
| agg_keygen | 21000000 | VerifierProgram;stage-c-build-rounds;PoseidonCell | 46,550 |  | 13,775 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs | 24,538,343 | 55,584 | 6,926,894 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;PoseidonCell | 3,777,900 |  | 1,117,950 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify | 561,881 | 2,122 | 163,227 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;PoseidonCell | 68,600 |  | 20,300 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;cache-generator-powers | 65,212 | 11,088 | 19,880 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;compute-reduced-opening;single-reduced-opening-eval | 8,365,448 | 382,648 | 1,564,444 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;pre-compute-rounds-context | 76,596 | 11,168 | 22,344 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch | 80,808 |  | 18,648 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;PoseidonCell | 9,509,304 |  | 2,820,720 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-batch;verify-batch-reduce-fast;PoseidonCell | 8,160,684 | 231,168 | 2,546,740 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query | 970,536 | 161,532 | 279,328 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext | 165,984 |  | 38,304 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;PoseidonCell | 15,711,024 |  | 4,660,320 | 
| agg_keygen | 21000000 | VerifierProgram;stage-d-verify-pcs;stage-d-verifier-verify;verify-query;verify-batch-ext;verify-batch-reduce-fast;PoseidonCell | 1,526,364 | 48,160 | 468,972 | 
| agg_keygen | 21000000 | VerifierProgram;stage-e-verify-constraints | 9,588,472 | 1,884,652 | 2,960,147 | 

| group | block_number | idx | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal.0 | 21000000 | 0 | 616 | 4,350 | 1,499,902 | 185,248,216 | 3,268 | 491 | 749 | 485 | 873 | 555 | 77,743,364 | 111 | 466 | 
| internal.0 | 21000000 | 1 | 627 | 4,328 | 1,499,279 | 185,248,216 | 3,233 | 491 | 725 | 493 | 871 | 555 | 77,810,004 | 93 | 468 | 
| internal.0 | 21000000 | 2 | 623 | 4,315 | 1,500,927 | 185,248,216 | 3,222 | 497 | 720 | 486 | 868 | 559 | 77,857,816 | 89 | 470 | 
| internal.0 | 21000000 | 3 | 622 | 4,344 | 1,500,871 | 185,248,216 | 3,256 | 498 | 754 | 489 | 869 | 550 | 77,857,312 | 92 | 466 | 
| internal.0 | 21000000 | 4 | 600 | 4,276 | 1,500,026 | 185,248,216 | 3,212 | 497 | 712 | 485 | 863 | 556 | 77,833,217 | 96 | 464 | 
| internal.0 | 21000000 | 5 | 330 | 2,591 | 749,983 | 94,984,664 | 2,029 | 278 | 490 | 287 | 600 | 320 | 39,802,333 | 50 | 232 | 
| internal.1 | 21000000 | 6 | 603 | 4,269 | 1,472,317 | 185,248,216 | 3,235 | 499 | 711 | 490 | 878 | 560 | 74,850,404 | 93 | 431 | 
| internal.1 | 21000000 | 7 | 610 | 4,302 | 1,472,288 | 185,248,216 | 3,260 | 495 | 751 | 485 | 871 | 552 | 74,850,143 | 101 | 432 | 
| internal.1 | 21000000 | 8 | 611 | 4,252 | 1,464,413 | 185,248,216 | 3,221 | 494 | 705 | 495 | 863 | 554 | 74,278,008 | 106 | 420 | 
| internal.2 | 21000000 | 10 | 312 | 2,593 | 736,474 | 94,984,664 | 2,065 | 273 | 519 | 292 | 590 | 329 | 38,316,173 | 58 | 216 | 
| internal.2 | 21000000 | 9 | 617 | 4,281 | 1,472,271 | 185,248,216 | 3,235 | 493 | 736 | 486 | 871 | 554 | 74,849,990 | 92 | 429 | 
| internal.3 | 21000000 | 11 | 592 | 4,248 | 1,464,483 | 185,248,216 | 3,235 | 496 | 721 | 497 | 872 | 554 | 74,278,638 | 91 | 421 | 
| leaf | 21000000 | 0 | 1,356 | 9,723 | 2,537,319 | 491,440,600 | 7,830 | 1,175 | 1,652 | 1,222 | 2,246 | 1,334 | 153,004,232 | 196 | 537 | 
| leaf | 21000000 | 1 | 908 | 7,224 | 1,584,480 | 318,278,104 | 5,924 | 773 | 1,234 | 872 | 1,972 | 942 | 100,744,103 | 127 | 392 | 
| leaf | 21000000 | 10 | 926 | 7,141 | 1,641,237 | 318,278,104 | 5,836 | 780 | 1,197 | 844 | 1,956 | 929 | 102,839,489 | 126 | 379 | 
| leaf | 21000000 | 2 | 892 | 7,168 | 1,584,763 | 318,278,104 | 5,884 | 773 | 1,223 | 847 | 1,980 | 930 | 100,751,597 | 127 | 392 | 
| leaf | 21000000 | 3 | 926 | 7,209 | 1,578,555 | 318,278,104 | 5,901 | 779 | 1,251 | 852 | 1,955 | 940 | 100,043,559 | 120 | 382 | 
| leaf | 21000000 | 4 | 1,536 | 10,304 | 2,926,134 | 514,542,040 | 8,175 | 1,232 | 1,712 | 1,275 | 2,327 | 1,415 | 172,314,571 | 211 | 593 | 
| leaf | 21000000 | 5 | 1,459 | 10,159 | 2,921,141 | 514,542,040 | 8,109 | 1,232 | 1,682 | 1,263 | 2,322 | 1,398 | 172,197,614 | 209 | 591 | 
| leaf | 21000000 | 6 | 1,476 | 10,244 | 2,923,462 | 514,542,040 | 8,179 | 1,227 | 1,722 | 1,286 | 2,326 | 1,408 | 172,284,731 | 207 | 589 | 
| leaf | 21000000 | 7 | 1,491 | 10,248 | 2,870,397 | 514,542,040 | 8,171 | 1,241 | 1,704 | 1,283 | 2,337 | 1,398 | 170,286,996 | 204 | 586 | 
| leaf | 21000000 | 8 | 1,512 | 10,282 | 2,897,383 | 514,542,040 | 8,180 | 1,236 | 1,694 | 1,289 | 2,357 | 1,394 | 171,352,023 | 207 | 590 | 
| leaf | 21000000 | 9 | 998 | 7,327 | 1,818,610 | 318,278,104 | 5,905 | 781 | 1,233 | 841 | 1,965 | 957 | 111,686,014 | 125 | 424 | 
| root | 21000000 | 0 | 327 | 23,094 | 736,982 | 80,353,432 | 22,553 | 555 | 8,150 | 4,489 | 2,422 | 6,871 | 38,338,736 | 61 | 214 | 

| group | block_number | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 0 | 29 | 433 |  | 7,747,677 | 404 | 30 | 57 | 51 | 200 | 50 | 928,069 | 11 | 0 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 8,875 | 33,768 | 20,419,372 | 2,011,852,513 | 22,191 | 3,824 | 3,569 | 3,636 | 5,064 | 5,313 | 994,597,939 | 773 | 2,702 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 8,614 | 32,799 | 20,434,219 | 2,011,928,725 | 21,298 | 3,827 | 3,294 | 3,461 | 4,949 | 4,971 | 991,970,061 | 786 | 2,887 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 3,136 | 9,511 | 1,960,882 | 509,783,854 | 6,095 | 1,021 | 1,051 | 825 | 1,555 | 1,504 | 290,300,354 | 131 | 280 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 9,027 | 33,267 | 20,453,308 | 2,012,150,421 | 21,311 | 3,842 | 3,322 | 3,465 | 4,941 | 4,959 | 993,560,028 | 773 | 2,929 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 4,939 | 33,200 | 5,875,069 | 2,845,093,940 | 27,391 | 8,826 | 2,114 | 2,934 | 2,942 | 9,967 | 1,454,817,186 | 600 | 870 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 10,255 | 44,206 | 23,495,832 | 2,655,977,688 | 27,680 | 6,285 | 3,841 | 4,260 | 5,464 | 6,912 | 1,361,034,708 | 904 | 6,271 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 10,035 | 36,686 | 23,901,347 | 2,112,939,728 | 23,248 | 4,418 | 3,754 | 3,829 | 5,322 | 5,082 | 1,100,001,177 | 828 | 3,403 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 10,901 | 39,746 | 24,792,369 | 2,258,517,208 | 25,080 | 4,710 | 4,120 | 4,261 | 5,618 | 5,469 | 1,156,952,722 | 884 | 3,765 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 10,277 | 37,197 | 24,275,047 | 2,096,753,736 | 23,330 | 4,447 | 3,856 | 3,831 | 5,285 | 5,090 | 1,117,733,199 | 807 | 3,590 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 10,872 | 38,956 | 23,946,393 | 2,265,220,184 | 24,581 | 5,035 | 3,771 | 3,936 | 5,318 | 5,665 | 1,287,982,506 | 843 | 3,503 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 9,220 | 50,435 | 14,174,037 | 4,176,526,984 | 38,349 | 10,828 | 3,379 | 4,551 | 5,188 | 13,339 | 1,933,574,592 | 1,054 | 2,866 | 

</details>


Commit: https://github.com/axiom-crypto/openvm-reth-benchmark/commit/f4f8ac7d43ccfcdded59d0831e3ca9dbaae8f3d6

Max Segment Length: 8388508

Instance Type: m8g.24xlarge

Memory Allocator: jemalloc

[Benchmark Workflow](https://github.com/axiom-crypto/openvm-reth-benchmark/actions/runs/13662904960)
