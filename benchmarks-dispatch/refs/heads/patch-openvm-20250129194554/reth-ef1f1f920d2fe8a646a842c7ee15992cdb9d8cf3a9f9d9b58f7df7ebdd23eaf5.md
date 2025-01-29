| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total |  1,234.43 |  279.37 |
| reth.prove_e2e.block_21000000 |  733.39 |  90.32 |
| leaf |  236.20 |  26.90 |
| internal.0 |  80.38 |  14.51 |
| internal.1 |  42.43 |  14.30 |
| internal.2 |  22.69 |  14.01 |
| internal.3 |  14.18 |  14.18 |
| root |  31.98 |  31.98 |
| halo2_outer |  50.08 |  50.08 |
| halo2_wrapper |  23.10 |  23.10 |
| agg_keygen |  34.07 |  32.58 |


| reth.prove_e2e.block_21000000 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  66,672 |  733,392 |  90,320 |  29,495 |
| `main_cells_used     ` |  1,144,063,113.91 |  12,584,694,253 |  1,924,175,955 |  289,667,926 |
| `total_cycles        ` |  18,477,009.45 |  203,247,104 |  24,802,065 |  1,959,760 |
| `execute_time_ms     ` |  5,566.73 |  61,234 |  9,645 |  468 |
| `trace_gen_time_ms   ` |  20,482.64 |  225,309 |  25,882 |  9,853 |
| `stark_prove_excluding_trace_time_ms` |  40,622.64 |  446,849 |  59,483 |  12,531 |
| `main_trace_commit_time_ms` |  5,703.64 |  62,740 |  10,576 |  1,413 |
| `generate_perm_trace_time_ms` |  6,400.18 |  70,402 |  8,571 |  1,058 |
| `perm_trace_commit_time_ms` |  3,623.18 |  39,855 |  4,506 |  1,073 |
| `quotient_poly_compute_time_ms` |  6,219.64 |  68,416 |  13,579 |  1,517 |
| `quotient_poly_commit_time_ms` |  5,118.82 |  56,307 |  6,357 |  2,465 |
| `pcs_opening_time_ms ` |  13,543.64 |  148,980 |  20,213 |  4,994 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  21,472.82 |  236,201 |  26,902 |  16,636 |
| `main_cells_used     ` |  193,840,255.27 |  2,132,242,808 |  277,125,809 |  141,756,696 |
| `total_cycles        ` |  4,163,571.55 |  45,799,287 |  5,811,147 |  3,059,668 |
| `execute_time_ms     ` |  663.64 |  7,300 |  896 |  433 |
| `trace_gen_time_ms   ` |  3,837.64 |  42,214 |  5,696 |  2,794 |
| `stark_prove_excluding_trace_time_ms` |  16,971.55 |  186,687 |  20,310 |  13,395 |
| `main_trace_commit_time_ms` |  1,819.36 |  20,013 |  2,224 |  1,360 |
| `generate_perm_trace_time_ms` |  2,083.27 |  22,916 |  3,585 |  1,000 |
| `perm_trace_commit_time_ms` |  1,524.36 |  16,768 |  2,015 |  1,106 |
| `quotient_poly_compute_time_ms` |  2,015.55 |  22,171 |  2,620 |  1,555 |
| `quotient_poly_commit_time_ms` |  3,216.64 |  35,383 |  3,694 |  2,658 |
| `pcs_opening_time_ms ` |  6,307.64 |  69,384 |  7,103 |  5,546 |

| internal.0 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  13,396.50 |  80,379 |  14,507 |  9,063 |
| `main_cells_used     ` |  136,071,390.67 |  816,428,344 |  148,289,234 |  75,278,043 |
| `total_cycles        ` |  3,579,022.67 |  21,474,136 |  3,905,088 |  1,951,851 |
| `execute_time_ms     ` |  527.33 |  3,164 |  597 |  304 |
| `trace_gen_time_ms   ` |  2,666.33 |  15,998 |  2,980 |  1,476 |
| `stark_prove_excluding_trace_time_ms` |  10,202.83 |  61,217 |  11,032 |  7,283 |
| `main_trace_commit_time_ms` |  1,097.50 |  6,585 |  1,310 |  879 |
| `generate_perm_trace_time_ms` |  968.83 |  5,813 |  1,158 |  539 |
| `perm_trace_commit_time_ms` |  875 |  5,250 |  989 |  648 |
| `quotient_poly_compute_time_ms` |  1,194.67 |  7,168 |  1,291 |  751 |
| `quotient_poly_commit_time_ms` |  2,291 |  13,746 |  2,455 |  1,832 |
| `pcs_opening_time_ms ` |  3,771.50 |  22,629 |  4,056 |  2,630 |

| internal.1 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  14,144.33 |  42,433 |  14,300 |  14,026 |
| `main_cells_used     ` |  142,375,008.33 |  427,125,025 |  142,960,674 |  141,205,967 |
| `total_cycles        ` |  3,745,301.33 |  11,235,904 |  3,765,833 |  3,704,467 |
| `execute_time_ms     ` |  563.67 |  1,691 |  592 |  516 |
| `trace_gen_time_ms   ` |  2,766.33 |  8,299 |  2,796 |  2,718 |
| `stark_prove_excluding_trace_time_ms` |  10,814.33 |  32,443 |  10,999 |  10,719 |
| `main_trace_commit_time_ms` |  1,100.33 |  3,301 |  1,137 |  1,060 |
| `generate_perm_trace_time_ms` |  1,297.67 |  3,893 |  1,436 |  1,225 |
| `perm_trace_commit_time_ms` |  909.67 |  2,729 |  956 |  866 |
| `quotient_poly_compute_time_ms` |  1,276 |  3,828 |  1,282 |  1,271 |
| `quotient_poly_commit_time_ms` |  2,357.67 |  7,073 |  2,391 |  2,332 |
| `pcs_opening_time_ms ` |  3,868 |  11,604 |  3,939 |  3,825 |

| internal.2 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  11,347 |  22,694 |  14,010 |  8,684 |
| `main_cells_used     ` |  107,806,048.50 |  215,612,097 |  142,959,074 |  72,653,023 |
| `total_cycles        ` |  2,824,445.50 |  5,648,891 |  3,765,673 |  1,883,218 |
| `execute_time_ms     ` |  447 |  894 |  595 |  299 |
| `trace_gen_time_ms   ` |  2,069 |  4,138 |  2,713 |  1,425 |
| `stark_prove_excluding_trace_time_ms` |  8,831 |  17,662 |  10,702 |  6,960 |
| `main_trace_commit_time_ms` |  943 |  1,886 |  1,036 |  850 |
| `generate_perm_trace_time_ms` |  784 |  1,568 |  1,162 |  406 |
| `perm_trace_commit_time_ms` |  844.50 |  1,689 |  987 |  702 |
| `quotient_poly_compute_time_ms` |  1,014.50 |  2,029 |  1,273 |  756 |
| `quotient_poly_commit_time_ms` |  2,007 |  4,014 |  2,298 |  1,716 |
| `pcs_opening_time_ms ` |  3,233.50 |  6,467 |  3,942 |  2,525 |

| internal.3 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  14,177 |  14,177 |  14,177 |  14,177 |
| `main_cells_used     ` |  141,207,637 |  141,207,637 |  141,207,637 |  141,207,637 |
| `total_cycles        ` |  3,704,634 |  3,704,634 |  3,704,634 |  3,704,634 |
| `execute_time_ms     ` |  515 |  515 |  515 |  515 |
| `trace_gen_time_ms   ` |  2,794 |  2,794 |  2,794 |  2,794 |
| `stark_prove_excluding_trace_time_ms` |  10,868 |  10,868 |  10,868 |  10,868 |
| `main_trace_commit_time_ms` |  1,111 |  1,111 |  1,111 |  1,111 |
| `generate_perm_trace_time_ms` |  1,304 |  1,304 |  1,304 |  1,304 |
| `perm_trace_commit_time_ms` |  849 |  849 |  849 |  849 |
| `quotient_poly_compute_time_ms` |  1,273 |  1,273 |  1,273 |  1,273 |
| `quotient_poly_commit_time_ms` |  2,410 |  2,410 |  2,410 |  2,410 |
| `pcs_opening_time_ms ` |  3,916 |  3,916 |  3,916 |  3,916 |

| root |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  31,977 |  31,977 |  31,977 |  31,977 |
| `main_cells_used     ` |  72,674,075 |  72,674,075 |  72,674,075 |  72,674,075 |
| `total_cycles        ` |  1,883,681 |  1,883,681 |  1,883,681 |  1,883,681 |
| `execute_time_ms     ` |  227 |  227 |  227 |  227 |
| `trace_gen_time_ms   ` |  1,398 |  1,398 |  1,398 |  1,398 |
| `stark_prove_excluding_trace_time_ms` |  30,352 |  30,352 |  30,352 |  30,352 |
| `main_trace_commit_time_ms` |  6,925 |  6,925 |  6,925 |  6,925 |
| `generate_perm_trace_time_ms` |  423 |  423 |  423 |  423 |
| `perm_trace_commit_time_ms` |  4,929 |  4,929 |  4,929 |  4,929 |
| `quotient_poly_compute_time_ms` |  1,281 |  1,281 |  1,281 |  1,281 |
| `quotient_poly_commit_time_ms` |  11,203 |  11,203 |  11,203 |  11,203 |
| `pcs_opening_time_ms ` |  5,585 |  5,585 |  5,585 |  5,585 |

| halo2_outer |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  50,078 |  50,078 |  50,078 |  50,078 |
| `main_cells_used     ` |  62,033,753 |  62,033,753 |  62,033,753 |  62,033,753 |

| halo2_wrapper |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  23,097 |  23,097 |  23,097 |  23,097 |

| agg_keygen |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` |  17,037 |  34,074 |  32,579 |  1,495 |
| `main_cells_used     ` |  87,438,446.50 |  174,876,893 |  173,948,824 |  928,069 |
| `total_cycles        ` |  4,502,913 |  4,502,913 |  4,502,913 |  4,502,913 |
| `execute_time_ms     ` |  112.50 |  225 |  224 |  1 |
| `trace_gen_time_ms   ` |  678 |  1,356 |  1,337 |  19 |
| `stark_prove_excluding_trace_time_ms` |  16,246.50 |  32,493 |  31,018 |  1,475 |
| `main_trace_commit_time_ms` |  3,447.50 |  6,895 |  6,733 |  162 |
| `generate_perm_trace_time_ms` |  266.50 |  533 |  406 |  127 |
| `perm_trace_commit_time_ms` |  2,581 |  5,162 |  5,009 |  153 |
| `quotient_poly_compute_time_ms` |  683.50 |  1,367 |  1,287 |  80 |
| `quotient_poly_commit_time_ms` |  5,912.50 |  11,825 |  11,636 |  189 |
| `pcs_opening_time_ms ` |  3,351.50 |  6,703 |  5,942 |  761 |



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
| 21000000 | 298 | 

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
| agg_keygen | 21000000 | 1,337 | 32,579 | 4,502,913 | 488,719,320 | 31,018 | 1,287 | 11,636 | 5,009 | 5,942 | 1 | 6,733 | 173,948,824 | 5,447,564 | 16,075 | 406 | 224 | 
| halo2_outer | 21000000 |  | 50,078 |  |  |  |  |  |  |  |  |  | 62,033,753 |  |  |  |  | 
| halo2_wrapper | 21000000 |  | 23,097 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
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
| internal.0 | 21000000 | 0 | 2,980 | 14,439 | 3,904,643 | 318,587,864 | 10,911 | 1,272 | 2,385 | 834 | 4,015 | 1,310 | 148,214,826 | 1,089 | 548 | 
| internal.0 | 21000000 | 1 | 2,883 | 14,051 | 3,903,446 | 318,587,864 | 10,571 | 1,289 | 2,396 | 982 | 3,952 | 1,058 | 148,199,442 | 890 | 597 | 
| internal.0 | 21000000 | 2 | 2,893 | 14,244 | 3,905,067 | 318,587,864 | 10,758 | 1,283 | 2,325 | 904 | 4,046 | 1,086 | 148,215,756 | 1,110 | 593 | 
| internal.0 | 21000000 | 3 | 2,883 | 14,507 | 3,905,088 | 318,587,864 | 11,032 | 1,291 | 2,455 | 989 | 4,056 | 1,079 | 148,231,043 | 1,158 | 592 | 
| internal.0 | 21000000 | 4 | 2,883 | 14,075 | 3,904,041 | 318,587,864 | 10,662 | 1,282 | 2,353 | 893 | 3,930 | 1,173 | 148,289,234 | 1,027 | 530 | 
| internal.0 | 21000000 | 5 | 1,476 | 9,063 | 1,951,851 | 162,834,136 | 7,283 | 751 | 1,832 | 648 | 2,630 | 879 | 75,278,043 | 539 | 304 | 
| internal.1 | 21000000 | 6 | 2,785 | 14,026 | 3,765,833 | 318,587,864 | 10,725 | 1,282 | 2,350 | 956 | 3,840 | 1,060 | 142,960,674 | 1,232 | 516 | 
| internal.1 | 21000000 | 7 | 2,796 | 14,107 | 3,765,604 | 318,587,864 | 10,719 | 1,271 | 2,391 | 866 | 3,825 | 1,137 | 142,958,384 | 1,225 | 592 | 
| internal.1 | 21000000 | 8 | 2,718 | 14,300 | 3,704,467 | 318,587,864 | 10,999 | 1,275 | 2,332 | 907 | 3,939 | 1,104 | 141,205,967 | 1,436 | 583 | 
| internal.2 | 21000000 | 10 | 1,425 | 8,684 | 1,883,218 | 162,834,136 | 6,960 | 756 | 1,716 | 702 | 2,525 | 850 | 72,653,023 | 406 | 299 | 
| internal.2 | 21000000 | 9 | 2,713 | 14,010 | 3,765,673 | 318,587,864 | 10,702 | 1,273 | 2,298 | 987 | 3,942 | 1,036 | 142,959,074 | 1,162 | 595 | 
| internal.3 | 21000000 | 11 | 2,794 | 14,177 | 3,704,634 | 318,587,864 | 10,868 | 1,273 | 2,410 | 849 | 3,916 | 1,111 | 141,207,637 | 1,304 | 515 | 
| leaf | 21000000 | 0 | 5,696 | 26,902 | 5,811,147 | 838,477,528 | 20,310 | 2,620 | 3,542 | 1,919 | 7,103 | 2,088 | 277,125,809 | 3,034 | 896 | 
| leaf | 21000000 | 1 | 3,042 | 17,112 | 3,125,427 | 426,387,160 | 13,575 | 1,555 | 2,658 | 1,142 | 5,578 | 1,558 | 144,133,767 | 1,079 | 495 | 
| leaf | 21000000 | 10 | 2,794 | 17,061 | 3,103,743 | 426,394,584 | 13,834 | 1,581 | 2,757 | 1,211 | 5,601 | 1,525 | 143,980,192 | 1,154 | 433 | 
| leaf | 21000000 | 2 | 2,865 | 17,215 | 3,125,047 | 426,387,160 | 13,905 | 1,558 | 2,742 | 1,106 | 5,573 | 1,360 | 144,129,967 | 1,563 | 445 | 
| leaf | 21000000 | 3 | 2,801 | 16,636 | 3,059,668 | 426,387,160 | 13,395 | 1,559 | 2,712 | 1,136 | 5,546 | 1,437 | 141,756,696 | 1,000 | 440 | 
| leaf | 21000000 | 4 | 4,427 | 24,113 | 4,849,675 | 732,571,352 | 18,928 | 2,367 | 3,670 | 1,725 | 7,008 | 2,044 | 225,479,831 | 2,107 | 758 | 
| leaf | 21000000 | 5 | 4,412 | 23,889 | 4,845,028 | 732,571,352 | 18,692 | 2,350 | 3,557 | 1,681 | 6,801 | 1,904 | 225,417,972 | 2,395 | 785 | 
| leaf | 21000000 | 6 | 4,342 | 24,882 | 4,846,748 | 732,571,352 | 19,767 | 2,343 | 3,535 | 1,674 | 6,786 | 1,840 | 225,514,075 | 3,585 | 773 | 
| leaf | 21000000 | 7 | 4,157 | 24,054 | 4,760,534 | 719,464,152 | 19,130 | 2,289 | 3,694 | 1,651 | 6,938 | 2,224 | 222,556,488 | 2,328 | 767 | 
| leaf | 21000000 | 8 | 4,259 | 25,355 | 4,804,363 | 719,464,152 | 20,248 | 2,330 | 3,587 | 2,015 | 6,816 | 2,123 | 224,068,271 | 3,373 | 848 | 
| leaf | 21000000 | 9 | 3,419 | 18,982 | 3,467,907 | 430,188,248 | 14,903 | 1,619 | 2,929 | 1,508 | 5,634 | 1,910 | 158,079,740 | 1,298 | 660 | 
| root | 21000000 | 0 | 1,398 | 31,977 | 1,883,681 | 139,764,120 | 30,352 | 1,281 | 11,203 | 4,929 | 5,585 | 6,925 | 72,674,075 | 423 | 227 | 

| group | block_number | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| agg_keygen | 21000000 | 0 | 19 | 1,495 |  | 7,747,673 | 1,475 | 80 | 189 | 153 | 761 | 162 | 928,069 | 127 | 1 | 
| reth.prove_e2e.block_21000000 | 21000000 | 0 | 16,697 | 60,569 | 20,460,479 | 1,976,080,644 | 40,415 | 4,330 | 5,960 | 4,506 | 12,137 | 5,814 | 988,367,119 | 7,644 | 3,457 | 
| reth.prove_e2e.block_21000000 | 21000000 | 1 | 15,569 | 55,714 | 20,511,282 | 1,976,246,920 | 34,466 | 4,207 | 5,123 | 3,209 | 11,033 | 4,037 | 986,008,056 | 6,847 | 5,679 | 
| reth.prove_e2e.block_21000000 | 21000000 | 10 | 16,496 | 29,495 | 1,959,760 | 499,288,864 | 12,531 | 1,517 | 2,465 | 1,073 | 4,994 | 1,413 | 289,667,926 | 1,058 | 468 | 
| reth.prove_e2e.block_21000000 | 21000000 | 2 | 18,220 | 59,848 | 20,514,514 | 1,976,246,920 | 35,673 | 4,295 | 4,516 | 3,294 | 11,740 | 4,540 | 986,899,624 | 7,277 | 5,955 | 
| reth.prove_e2e.block_21000000 | 21000000 | 3 | 9,853 | 62,106 | 5,365,524 | 2,815,176,728 | 50,567 | 11,739 | 3,850 | 2,928 | 15,269 | 10,576 | 1,423,195,914 | 6,196 | 1,686 | 
| reth.prove_e2e.block_21000000 | 21000000 | 4 | 25,403 | 83,660 | 23,489,873 | 2,637,345,368 | 48,612 | 7,514 | 5,884 | 4,174 | 17,571 | 7,217 | 1,359,659,449 | 6,236 | 9,645 | 
| reth.prove_e2e.block_21000000 | 21000000 | 5 | 21,585 | 67,651 | 23,902,155 | 2,094,138,520 | 40,052 | 5,057 | 6,068 | 4,079 | 13,538 | 5,510 | 1,090,210,175 | 5,787 | 6,014 | 
| reth.prove_e2e.block_21000000 | 21000000 | 6 | 25,882 | 76,545 | 24,802,065 | 2,238,667,032 | 42,875 | 5,334 | 6,357 | 4,334 | 14,111 | 6,699 | 1,150,321,884 | 6,025 | 7,788 | 
| reth.prove_e2e.block_21000000 | 21000000 | 7 | 24,149 | 70,851 | 24,266,967 | 2,078,010,200 | 39,080 | 5,045 | 5,228 | 4,266 | 13,231 | 4,340 | 1,108,338,482 | 6,954 | 7,622 | 
| reth.prove_e2e.block_21000000 | 21000000 | 8 | 25,725 | 76,633 | 23,897,913 | 2,246,467,992 | 43,095 | 5,799 | 6,046 | 3,911 | 15,143 | 4,376 | 1,277,849,669 | 7,807 | 7,813 | 
| reth.prove_e2e.block_21000000 | 21000000 | 9 | 25,730 | 90,320 | 14,076,572 | 4,158,290,456 | 59,483 | 13,579 | 4,810 | 4,081 | 20,213 | 8,218 | 1,924,175,955 | 8,571 | 5,107 | 

</details>


Commit: https://github.com/axiom-crypto/openvm-reth-benchmark/commit/c8acc0a88a4d5e45fd9b821fe1d4bed7091649d7

Max Segment Length: 8388508

Instance Type: m7a.48xlarge

Memory Allocator: jemalloc

[Benchmark Workflow](https://github.com/axiom-crypto/openvm-reth-benchmark/actions/runs/13039260948)
