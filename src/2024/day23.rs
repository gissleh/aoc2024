use arrayvec::ArrayVec;
use common::graph::Graph;
use common::runner::Runner;
use rustc_hash::FxHashSet;

type LanGraph = Graph<[u8; 2], (), 16>;

pub fn main(r: &mut Runner, input: &[u8]) {
    let graph = r.prep("Parse", || parse(&input));

    r.part("Part 1", || part_1(&graph));
    r.part("Part 2", || part_2(&graph));

    r.info("Computers", &graph.len());
    r.info("Connections", &graph.edge_count());
}

fn part_1(graph: &LanGraph) -> u32 {
    let mut total = 0;
    let mut seen = FxHashSet::<[[u8; 2]; 3]>::default();
    for (i, a) in graph.nodes().iter().enumerate() {
        if a[0] != b't' {
            continue;
        }

        let edges = graph.edges(i);

        for j in 0..edges.len() {
            let ji = edges[j].0;
            let b = graph.node(edges[j].0);
            for k in (j + 1)..edges.len() {
                let ki = edges[k].0;
                if graph.edges(ji).iter().find(|(n, _)| *n == ki).is_none() {
                    continue;
                }

                let c = graph.node(edges[k].0);
                let mut v = [*a, *b, *c];
                v.sort_unstable();
                if seen.insert(v) {
                    total += 1;
                }
            }
        }
    }

    total
}

fn part_2(graph: &LanGraph) -> String {
    let mut longest = ArrayVec::<[u8; 2], 16>::new();
    for i in 0..graph.len() {
        let mut cluster = ArrayVec::<usize, 16>::new();
        cluster.push(i);
        cluster.extend(graph.edges(i).iter().map(|(ei, _)| *ei));

        let mut masks = ArrayVec::<u16, 16>::new();

        for j in 0..cluster.len() {
            let mut mask = ((1 << cluster.len()) - 1) | 1;
            let ji = cluster[j];
            for k in 0..cluster.len() {
                if j == k {
                    continue;
                }

                let ki = cluster[k];
                if graph.edges(ji).iter().find(|(n, _)| *n == ki).is_none() {
                    mask ^= 1 << k
                }
            }

            masks.push(mask);
        }

        for mask_a in masks.iter() {
            for mask_b in masks.iter() {
                let combined = *mask_a & *mask_b;
                if combined.count_ones() <= longest.len() as u32 {
                    continue;
                }

                let mut candidate = ArrayVec::<[u8; 2], 16>::new();
                for (i, ci) in cluster.iter().enumerate() {
                    if masks[i] & combined == combined {
                        candidate.push(*graph.node(*ci));
                    }
                }

                if candidate.len() > longest.len() {
                    longest = candidate.clone();
                }
            }
        }
    }

    longest.sort_unstable();

    let mut res = String::with_capacity(longest.len() * 3);
    for computer in longest.iter() {
        if !res.is_empty() {
            res.push(',');
        }

        res.push(computer[0] as char);
        res.push(computer[1] as char);
    }

    res
}

fn parse(input: &[u8]) -> LanGraph {
    input
        .iter()
        .array_chunks::<6>()
        .map(|c| ([*c[0], *c[1]], [*c[3], *c[4]]))
        .fold(LanGraph::new(), |mut g, (a, b)| {
            let a = g.ensure_node(a);
            let b = g.ensure_node(b);
            g.connect_mutual(a, b, ());
            g
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    #[test]
    fn part2_works_on_example() {
        let graph = parse(EXAMPLE);
        assert_eq!(part_2(&graph).as_str(), "co,de,ka,ta");
    }
}
