use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;

// Import your Chunk type and MboEvent type. 
// Adjust the path as needed, e.g. "use mycrate::{Chunk};" 
// or "use mycrate::mbo_types::MboEvent;"
// For this example, we'll replicate a minimal MboEvent here:
#[derive(Debug)]
pub struct MboEvent {
    pub ts_event: u64,
    pub ts_delta: u32,
    pub order_id: u64,
    pub price:    u32,
    pub size:     u32,
    pub flags:    u8,
    pub action:   u8,
    pub side:     u8,
}

use your_crate_name::Chunk;

fn generate_random_events(num: usize, start_ts: u64) -> Vec<MboEvent> {
    let mut rng = rand::thread_rng();
    (0..num)
        .map(|i| {
            let ts_event = start_ts + i as u64; // or random distribution
            MboEvent {
                ts_event,
                // For demonstration, let's say ts_delta is ts_event - start_ts
                ts_delta: (ts_event - start_ts) as u32,
                order_id: rng.gen(),
                price:    rng.gen_range(1000..10_000),
                size:     rng.gen_range(1..100),
                flags:    rng.gen_range(0..16),
                action:   rng.gen_range(0..8),
                side:     rng.gen_range(0..2),
            }
        })
        .collect()
}

fn bench_chunk_push_event(c: &mut Criterion) {
    let mut group = c.benchmark_group("chunk_push_event");

    // We'll try a few sizes to see scaling
    for &num_events in &[10_000, 100_000, 1_000_000] {
        // Generate random events
        let events = generate_random_events(num_events, 1_600_000_000);

        group.bench_with_input(
            BenchmarkId::from_parameter(num_events),
            &num_events,
            |b, &num| {
                b.iter(|| {
                    // We'll create a fresh chunk each iteration
                    let mut chunk = Chunk::new(123, num, events[0].ts_event);
                    for evt in &events {
                        chunk.push_event(black_box(evt));
                    }
                    // black_box to ensure chunk isn't optimized away
                    black_box(&chunk);
                });
            },
        );
    }

    group.finish();
}

// Standard Criterion macros
criterion_group!(benches, bench_chunk_push_event);
criterion_main!(benches);
