//! Value-exact compatibility tests against networkx.greedy_color.
//!
//! All expected results are hardcoded from real networkx 3.6.1 output
//! (PYTHONHASHSEED=0, Python 3.12.13, conda-forge Clang 19.1.7).
//! Never derived from our own implementation.

use rsomics_greedy_color::{Graph, Strategy, greedy_color};
use std::collections::HashMap;

fn run(edge_list: &str, strategy: Strategy) -> HashMap<String, usize> {
    let g = Graph::from_edge_list(edge_list);
    greedy_color(&g, strategy).into_iter().collect()
}

fn check(result: &HashMap<String, usize>, expected: &[(&str, usize)]) {
    for (node, exp_color) in expected {
        let got = *result.get(*node).unwrap_or_else(|| {
            panic!("node '{node}' missing from result");
        });
        assert_eq!(
            got, *exp_color,
            "node '{node}': got color {got}, expected {exp_color}"
        );
    }
    assert_eq!(
        result.len(),
        expected.len(),
        "result has {} nodes, expected {}",
        result.len(),
        expected.len()
    );
}

fn check_valid(result: &HashMap<String, usize>, edge_list: &str) {
    let g = Graph::from_edge_list(edge_list);
    for (u_idx, adj) in g.adj.iter().enumerate() {
        let u = &g.names[u_idx];
        for &v_idx in adj {
            let v = &g.names[v_idx];
            let cu = result[u];
            let cv = result[v];
            assert_ne!(cu, cv, "adjacent nodes '{u}' and '{v}' share color {cu}");
        }
    }
}

// ─── path graph P5 ────────────────────────────────────────────────────────────

#[test]
fn path5_largest_first() {
    let edges = "0 1\n1 2\n2 3\n3 4";
    let r = run(edges, Strategy::LargestFirst);
    check(&r, &[("0", 1), ("1", 0), ("2", 1), ("3", 0), ("4", 1)]);
    check_valid(&r, edges);
}

#[test]
fn path5_dsatur() {
    let edges = "0 1\n1 2\n2 3\n3 4";
    let r = run(edges, Strategy::SaturationLargestFirst);
    check(&r, &[("0", 1), ("1", 0), ("2", 1), ("3", 0), ("4", 1)]);
    check_valid(&r, edges);
}

// ─── cycle graph C5 ───────────────────────────────────────────────────────────

#[test]
fn cycle5_largest_first() {
    let edges = "0 1\n1 2\n2 3\n3 4\n4 0";
    let r = run(edges, Strategy::LargestFirst);
    check(&r, &[("0", 0), ("1", 1), ("2", 0), ("3", 1), ("4", 2)]);
    check_valid(&r, edges);
}

#[test]
fn cycle5_dsatur() {
    let edges = "0 1\n1 2\n2 3\n3 4\n4 0";
    let r = run(edges, Strategy::SaturationLargestFirst);
    check(&r, &[("0", 0), ("1", 1), ("2", 0), ("3", 1), ("4", 2)]);
    check_valid(&r, edges);
}

// ─── complete graph K4 ────────────────────────────────────────────────────────

#[test]
fn complete4_largest_first() {
    let edges = "0 1\n0 2\n0 3\n1 2\n1 3\n2 3";
    let r = run(edges, Strategy::LargestFirst);
    check(&r, &[("0", 0), ("1", 1), ("2", 2), ("3", 3)]);
    check_valid(&r, edges);
}

#[test]
fn complete4_dsatur() {
    let edges = "0 1\n0 2\n0 3\n1 2\n1 3\n2 3";
    let r = run(edges, Strategy::SaturationLargestFirst);
    check(&r, &[("0", 0), ("1", 1), ("2", 2), ("3", 3)]);
    check_valid(&r, edges);
}

// ─── star graph K1,4 ──────────────────────────────────────────────────────────

#[test]
fn star5_largest_first() {
    let edges = "c a\nc b\nc d\nc e";
    let r = run(edges, Strategy::LargestFirst);
    check(&r, &[("a", 1), ("b", 1), ("c", 0), ("d", 1), ("e", 1)]);
    check_valid(&r, edges);
}

#[test]
fn star5_dsatur() {
    let edges = "c a\nc b\nc d\nc e";
    let r = run(edges, Strategy::SaturationLargestFirst);
    check(&r, &[("a", 1), ("b", 1), ("c", 0), ("d", 1), ("e", 1)]);
    check_valid(&r, edges);
}

// ─── bipartite K3,3 ───────────────────────────────────────────────────────────

#[test]
fn bipartite33_largest_first() {
    let edges = "a0 b0\na0 b1\na0 b2\na1 b0\na1 b1\na1 b2\na2 b0\na2 b1\na2 b2";
    let r = run(edges, Strategy::LargestFirst);
    check(
        &r,
        &[
            ("a0", 0),
            ("a1", 0),
            ("a2", 0),
            ("b0", 1),
            ("b1", 1),
            ("b2", 1),
        ],
    );
    check_valid(&r, edges);
}

#[test]
fn bipartite33_dsatur() {
    let edges = "a0 b0\na0 b1\na0 b2\na1 b0\na1 b1\na1 b2\na2 b0\na2 b1\na2 b2";
    let r = run(edges, Strategy::SaturationLargestFirst);
    check(
        &r,
        &[
            ("a0", 0),
            ("a1", 0),
            ("a2", 0),
            ("b0", 1),
            ("b1", 1),
            ("b2", 1),
        ],
    );
    check_valid(&r, edges);
}

// ─── Petersen graph ───────────────────────────────────────────────────────────

#[test]
fn petersen_largest_first() {
    let edges = "0 1\n1 2\n2 3\n3 4\n4 0\n0 5\n1 6\n2 7\n3 8\n4 9\n5 7\n7 9\n9 6\n6 8\n8 5";
    let r = run(edges, Strategy::LargestFirst);
    check(
        &r,
        &[
            ("0", 0),
            ("1", 1),
            ("2", 0),
            ("3", 1),
            ("4", 2),
            ("5", 1),
            ("6", 0),
            ("7", 2),
            ("8", 2),
            ("9", 1),
        ],
    );
    check_valid(&r, edges);
}

#[test]
fn petersen_dsatur() {
    let edges = "0 1\n1 2\n2 3\n3 4\n4 0\n0 5\n1 6\n2 7\n3 8\n4 9\n5 7\n7 9\n9 6\n6 8\n8 5";
    let r = run(edges, Strategy::SaturationLargestFirst);
    check(
        &r,
        &[
            ("0", 0),
            ("1", 1),
            ("2", 0),
            ("3", 1),
            ("4", 2),
            ("5", 1),
            ("6", 0),
            ("7", 2),
            ("8", 2),
            ("9", 1),
        ],
    );
    check_valid(&r, edges);
}

// ─── wheel graph W6 ───────────────────────────────────────────────────────────

#[test]
fn wheel6_largest_first() {
    let edges = "h 0\nh 1\nh 2\nh 3\nh 4\n0 1\n1 2\n2 3\n3 4\n4 0";
    let r = run(edges, Strategy::LargestFirst);
    check(
        &r,
        &[("0", 1), ("1", 2), ("2", 1), ("3", 2), ("4", 3), ("h", 0)],
    );
    check_valid(&r, edges);
}

#[test]
fn wheel6_dsatur() {
    let edges = "h 0\nh 1\nh 2\nh 3\nh 4\n0 1\n1 2\n2 3\n3 4\n4 0";
    let r = run(edges, Strategy::SaturationLargestFirst);
    check(
        &r,
        &[("0", 1), ("1", 2), ("2", 1), ("3", 2), ("4", 3), ("h", 0)],
    );
    check_valid(&r, edges);
}

// ─── disconnected graph ───────────────────────────────────────────────────────

#[test]
fn disconnected_largest_first() {
    let edges = "a b\nb c\nx y\ny z";
    let r = run(edges, Strategy::LargestFirst);
    check(
        &r,
        &[("a", 1), ("b", 0), ("c", 1), ("x", 1), ("y", 0), ("z", 1)],
    );
    check_valid(&r, edges);
}

#[test]
fn disconnected_dsatur() {
    let edges = "a b\nb c\nx y\ny z";
    let r = run(edges, Strategy::SaturationLargestFirst);
    check(
        &r,
        &[("a", 1), ("b", 0), ("c", 1), ("x", 1), ("y", 0), ("z", 1)],
    );
    check_valid(&r, edges);
}

// ─── empty graph ──────────────────────────────────────────────────────────────

#[test]
fn empty_graph() {
    let r = run("", Strategy::LargestFirst);
    assert!(r.is_empty());
    let r = run("", Strategy::SaturationLargestFirst);
    assert!(r.is_empty());
}

// ─── comments and blank lines in input ───────────────────────────────────────

#[test]
fn input_with_comments_and_blanks() {
    let edges = "# graph header\n\n0 1\n# edge comment\n1 2\n\n2 3\n3 4\n";
    let r = run(edges, Strategy::LargestFirst);
    check(&r, &[("0", 1), ("1", 0), ("2", 1), ("3", 0), ("4", 1)]);
}

// ─── parallel-edge dedup ─────────────────────────────────────────────────────

#[test]
fn parallel_edges_deduped() {
    // Duplicate edges must be treated as a simple graph (same as nx.Graph).
    let edges = "0 1\n0 1\n1 2\n1 2";
    let r = run(edges, Strategy::LargestFirst);
    assert_eq!(r.len(), 3);
    // node 1 has degree 2, colored first with 0; 0 and 2 get 1
    check_valid(&r, "0 1\n1 2");
}

// ─── random-50 graph ─────────────────────────────────────────────────────────
// Golden from: PYTHONHASHSEED=0, networkx 3.6.1, Python 3.12.13
// Edges generated: random.seed(42), 50 nodes, 200 random picks → deduped set

#[test]
fn random50_largest_first() {
    let edges = include_str!("golden/random50_edges.txt");
    let r = run(edges, Strategy::LargestFirst);
    let expected: HashMap<String, usize> = serde_json::from_str(
        r#"{"15":0,"4":1,"3":1,"34":0,"14":0,"5":1,"13":2,"38":0,"46":2,"40":2,"35":1,"41":3,"48":2,"10":3,"16":4,"7":3,"17":3,"6":4,"43":2,"37":2,"27":1,"32":3,"0":4,"29":0,"42":1,"20":1,"25":3,"24":2,"23":0,"36":1,"31":0,"47":0,"8":3,"2":2,"12":2,"21":0,"33":2,"1":1,"45":0,"44":3,"26":4,"28":4,"9":1,"18":3,"49":0,"30":2,"19":1,"22":3,"39":0,"11":0}"#,
    ).unwrap();
    assert_eq!(r, expected);
    check_valid(&r, edges);
}

#[test]
fn random50_dsatur() {
    let edges = include_str!("golden/random50_edges.txt");
    let r = run(edges, Strategy::SaturationLargestFirst);
    let expected: HashMap<String, usize> = serde_json::from_str(
        r#"{"15":0,"4":1,"48":2,"17":3,"32":0,"14":2,"38":2,"24":0,"0":1,"5":3,"46":2,"29":0,"34":0,"13":1,"16":3,"10":1,"25":2,"3":3,"7":1,"40":0,"43":2,"37":1,"35":0,"23":3,"36":2,"31":3,"20":1,"47":3,"2":0,"21":4,"1":5,"33":3,"28":2,"9":2,"30":1,"19":1,"39":1,"41":2,"6":1,"42":0,"45":3,"12":1,"26":3,"27":0,"22":3,"8":1,"49":0,"44":1,"18":2,"11":1}"#,
    ).unwrap();
    assert_eq!(r, expected);
    check_valid(&r, edges);
}

// ─── random-200 graph ────────────────────────────────────────────────────────

#[test]
fn random200_largest_first() {
    let edges = include_str!("golden/random200_edges.txt");
    let r = run(edges, Strategy::LargestFirst);
    let expected: HashMap<String, usize> = serde_json::from_str(
        r#"{"182":0,"32":0,"171":1,"196":0,"34":2,"160":0,"59":1,"49":1,"16":0,"106":0,"172":1,"68":1,"111":1,"94":1,"137":0,"163":0,"100":2,"167":0,"127":1,"10":2,"85":2,"87":0,"86":1,"179":0,"114":2,"26":1,"107":2,"95":0,"46":2,"18":0,"99":0,"39":1,"60":2,"169":3,"22":2,"41":0,"198":3,"96":2,"23":2,"36":2,"52":0,"130":1,"43":3,"150":1,"136":3,"139":2,"173":3,"132":2,"77":3,"109":1,"83":0,"129":1,"50":2,"112":3,"79":4,"166":0,"13":2,"187":0,"170":1,"5":3,"110":1,"67":1,"120":0,"124":2,"8":2,"21":3,"48":2,"93":2,"58":2,"128":4,"195":1,"162":3,"82":3,"89":1,"6":3,"14":1,"153":0,"91":1,"119":2,"12":3,"164":0,"25":1,"143":3,"40":1,"0":3,"1":1,"134":2,"101":1,"165":1,"73":1,"184":3,"113":4,"118":2,"142":0,"156":0,"92":3,"63":3,"61":0,"140":2,"27":2,"62":3,"11":0,"74":2,"146":3,"133":3,"155":1,"144":4,"168":2,"141":2,"116":3,"199":2,"126":3,"192":1,"2":3,"158":3,"7":3,"194":0,"30":1,"189":4,"57":2,"149":2,"31":0,"44":4,"154":3,"180":0,"88":1,"33":2,"72":3,"47":4,"157":4,"84":3,"102":0,"152":0,"80":1,"78":2,"123":0,"3":3,"131":0,"185":0,"90":4,"103":0,"15":3,"45":1,"65":5,"69":3,"177":0,"190":3,"37":2,"197":3,"98":0,"115":1,"35":1,"145":1,"178":1,"56":4,"188":2,"135":4,"54":2,"53":2,"121":4,"24":3,"9":4,"97":1,"122":1,"193":0,"42":3,"51":2,"117":2,"76":0,"186":1,"28":1,"105":1,"191":1,"19":0,"38":0,"66":3,"17":0,"175":2,"4":0,"161":1,"176":4,"55":2,"75":3,"148":0,"159":3,"104":0,"183":2,"20":1,"81":2,"70":2,"64":0,"108":2,"71":1,"29":0,"125":1,"181":0,"147":1,"151":0,"174":0,"138":1}"#,
    ).unwrap();
    assert_eq!(r, expected);
    check_valid(&r, edges);
}

#[test]
fn random200_dsatur() {
    let edges = include_str!("golden/random200_edges.txt");
    let r = run(edges, Strategy::SaturationLargestFirst);
    let expected: HashMap<String, usize> = serde_json::from_str(
        r#"{"182":0,"171":1,"34":2,"83":0,"49":1,"36":2,"164":0,"133":2,"113":1,"6":0,"46":2,"169":1,"111":2,"89":1,"0":3,"16":2,"39":1,"106":0,"26":2,"60":0,"167":1,"165":0,"114":3,"196":2,"186":1,"160":0,"77":3,"131":1,"132":3,"173":0,"66":0,"32":1,"107":2,"63":3,"130":0,"40":0,"197":2,"172":1,"68":0,"112":3,"22":0,"166":2,"150":3,"94":2,"10":1,"96":3,"95":0,"119":4,"43":3,"85":1,"5":3,"86":2,"27":2,"199":0,"88":2,"153":3,"198":1,"59":2,"8":4,"99":2,"93":3,"136":0,"179":3,"50":1,"41":3,"139":1,"156":4,"162":3,"163":0,"18":3,"100":1,"170":1,"124":0,"87":3,"79":2,"152":4,"12":3,"137":0,"78":4,"13":2,"127":1,"134":4,"187":0,"80":4,"21":1,"48":0,"128":2,"82":0,"67":3,"14":2,"91":0,"25":1,"1":1,"120":0,"73":2,"52":0,"158":4,"23":0,"184":1,"92":2,"191":4,"140":0,"62":2,"74":3,"44":4,"143":0,"146":0,"144":2,"195":3,"168":1,"141":3,"116":0,"126":2,"155":3,"11":2,"192":1,"118":0,"2":1,"189":3,"154":0,"180":0,"33":3,"72":1,"123":3,"28":4,"57":2,"47":0,"157":1,"102":1,"185":3,"90":1,"65":3,"69":1,"31":0,"190":2,"45":0,"98":0,"58":3,"109":2,"103":1,"7":2,"53":4,"84":1,"121":4,"61":2,"30":2,"177":0,"115":2,"35":1,"142":0,"149":2,"56":1,"188":3,"135":1,"54":3,"24":3,"194":0,"110":2,"15":0,"9":0,"97":3,"122":1,"193":1,"42":0,"51":3,"117":3,"76":1,"19":3,"55":4,"38":3,"17":0,"175":3,"176":1,"148":1,"159":2,"70":3,"125":1,"129":2,"101":0,"3":2,"37":0,"145":1,"178":1,"105":2,"64":3,"4":2,"161":2,"75":2,"104":2,"183":0,"20":1,"81":1,"108":1,"71":2,"29":2,"181":0,"147":1,"151":0,"174":0,"138":1}"#,
    ).unwrap();
    assert_eq!(r, expected);
    check_valid(&r, edges);
}
