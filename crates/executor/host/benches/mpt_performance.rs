use bincode::config::standard;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use openvm_client_executor::io::ClientExecutorInput;
use std::fs;

fn benchmark_mpt_operations(c: &mut Criterion) {
    // Load the benchmark data file (this is not counted in benchmark timing)
    let buffer = fs::read("client_input_benchmark.bin")
        .expect("Failed to read benchmark data. Run the integration test first to generate it.");

    println!("Loaded benchmark data: {} bytes", buffer.len());

    let bincode_config = standard();

    // c.bench_function("deserialize_client_input", |b| {
    //     b.iter(|| {
    //         let (client_input, _): (ClientExecutorInput, _) =
    //             bincode::serde::decode_from_slice(black_box(&buffer), bincode_config).unwrap();
    //         black_box(client_input)
    //     })
    // });

    // Deserialize once outside the benchmark for the witness_db test
    // let (client_input, _): (ClientExecutorInput, _) =
    //     bincode::serde::decode_from_slice(&buffer, bincode_config).unwrap();

    // c.bench_function("witness_db_creation", |b| {
    //     b.iter(|| {
    //         let witness_db = black_box(&client_input).witness_db().unwrap();
    //         black_box(witness_db)
    //     })
    // });

    c.bench_function("deserialize_and_witness_db", |b| {
        b.iter(|| {
            let (client_input, _): (ClientExecutorInput, _) =
                bincode::serde::decode_from_slice(black_box(&buffer), bincode_config).unwrap();
            let witness_db = client_input.witness_db().unwrap();
            black_box(witness_db)
        })
    });
}

criterion_group!(benches, benchmark_mpt_operations);
criterion_main!(benches);
