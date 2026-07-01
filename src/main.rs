use clap::Parser;
use rsomics_greedy_color::{Graph, Strategy, greedy_color};
use serde_json::json;
use std::io::{self, Read};

#[derive(Parser)]
#[command(
    name = "rsomics-greedy-color",
    version,
    about = "Greedy graph coloring (networkx-compatible)"
)]
struct Cli {
    /// Ordering strategy. Supported: largest_first, saturation_largest_first (DSATUR).
    ///
    /// Strategies omitted (non-deterministic for string nodes): smallest_last,
    /// random_sequential, independent_set, connected_sequential_bfs/dfs.
    #[arg(long, default_value = "largest_first")]
    strategy: String,

    /// Output as JSON object {"node": color, ...}.
    #[arg(long)]
    json: bool,
}

fn main() {
    let cli = Cli::parse();

    let strategy = match Strategy::parse(&cli.strategy) {
        Some(s) => s,
        None => {
            eprintln!(
                "error: unknown strategy '{}'.\n\
                 Supported: largest_first, saturation_largest_first",
                cli.strategy
            );
            std::process::exit(1);
        }
    };

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read stdin");

    let graph = Graph::from_edge_list(&input);
    let result = greedy_color(&graph, strategy);

    if cli.json {
        let obj: serde_json::Map<String, serde_json::Value> =
            result.iter().map(|(k, &v)| (k.clone(), json!(v))).collect();
        println!(
            "{}",
            serde_json::to_string(&serde_json::Value::Object(obj)).unwrap()
        );
    } else {
        // Plain output: node<TAB>color, lex-sorted by node name.
        let mut pairs: Vec<(&str, usize)> = result.iter().map(|(k, &v)| (k.as_str(), v)).collect();
        pairs.sort_unstable_by_key(|(k, _)| *k);
        for (node, color) in pairs {
            println!("{node}\t{color}");
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn cli_debug_assert() {
        Cli::command().debug_assert();
    }
}
