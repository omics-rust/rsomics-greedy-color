use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rsomics_greedy_color::{Graph, Strategy, greedy_color};

fn bench_coloring(c: &mut Criterion) {
    // 5000-node Erdős-Rényi-style graph, ~10% edge density
    let n = 5000usize;
    let edges: String = {
        let mut out = String::new();
        let mut state = 12345u64;
        let mut rng = || -> u64 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let mut edge_set = std::collections::HashSet::new();
        while edge_set.len() < 50_000 {
            let u = (rng() as usize) % n;
            let v = (rng() as usize) % n;
            if u != v {
                let key = (u.min(v), u.max(v));
                if edge_set.insert(key) {
                    out.push_str(&format!("{u} {v}\n"));
                }
            }
        }
        out
    };

    let graph = Graph::from_edge_list(&edges);

    let mut group = c.benchmark_group("coloring_5k");
    group.bench_function("largest_first", |b| {
        b.iter(|| greedy_color(black_box(&graph), Strategy::LargestFirst))
    });
    group.bench_function("saturation_largest_first", |b| {
        b.iter(|| greedy_color(black_box(&graph), Strategy::SaturationLargestFirst))
    });
    group.finish();
}

criterion_group!(benches, bench_coloring);
criterion_main!(benches);
