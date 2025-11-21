| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: green'>(+0 [NaN%])</span> 0 | <span style='color: green'>(+0 [NaN%])</span> 0 |


| reth.execute.block_23845032 |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `execute_e1_time_ms  ` |  1,943 |  1,943 |  1,943 |  1,943 |
| `execute_e1_insn_mi/s` |  179.52 | -          |  179.52 |  179.52 |



<details>
<summary>Detailed Metrics</summary>

|  | reth-block_time_ms |
| --- |
|  | 2,488 | 

| block_number | sdk.execute_time_ms | host.execute_time_ms |
| --- | --- | --- |
| 23845032 | 2,405 | 78 | 

| group | block_number | header.hash_slow_time_ms | execute_e1_time_ms | execute_e1_insns | execute_e1_insn_mi/s | client.execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| reth.execute.block_23845032 | 23845032 | 0 | 1,943 | 348,932,301 | 179.52 | 78 | 

</details>


**[Firefox Profiler](https://profiler.firefox.com/public/3c55zsg6neeg8vv36wrcyr3z8tf2pxr9f6p5r6r)**

Commit: https://github.com/axiom-crypto/openvm-reth-benchmark/commit/a83bb1ff0a5d3acb7dd5d92b1dec08561372ef10

Max Segment Length: 4194304

Instance Type: g6e.8xlarge

Memory Allocator: jemalloc

[Benchmark Workflow](https://github.com/axiom-crypto/openvm-reth-benchmark/actions/runs/19579521531)
