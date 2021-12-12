use std::collections::HashMap;
use bit_vec::BitVec;

extern crate pprof;
extern crate test;

type Id = u8;
type CaveId = (bool, Id);
type Neighbors = Vec<CaveId>;
type Graph = Vec<Neighbors>;

const START_ID: CaveId = (true, 0);
const END_ID: CaveId = (true, 1);

// static mut path: Vec<CaveId> = Vec::new();

pub fn main() {
    let lines = parse(utils::get_input(12, true));
    {
        let solved = solve_first(&lines);
        println!("{:?}", solved)
    }
    {
        let solved = solve_second(&lines);
        println!("{:?}", solved)
    }
}

#[inline]
fn get_id(cave_id: CaveId) -> Id {
    cave_id.1
}

#[inline]
fn get_idx(cave_id: CaveId) -> usize {
    get_id(cave_id) as usize
}

#[inline]
fn is_small(cave_id: CaveId) -> bool {
    cave_id.0
}

fn visit_first(id: CaveId, graph: &Graph, visited_nodes: &mut BitVec, visited_edges: &mut Vec<BitVec>) -> usize {
    let idx = get_idx(id);
    if is_small(id) {
        visited_nodes.set(idx, true)
    }

    let mut num_paths = 0usize;
    for &node in &graph[idx] {
        let node_idx = get_idx(node);

        if (is_small(node) && visited_nodes[get_idx(node)]) || visited_edges[idx][node_idx] {
            continue;
        }
        else if node == END_ID {
            num_paths += 1;
        }
        else {
            visited_edges[idx].set(node_idx, true);
            num_paths += visit_first(node, graph, visited_nodes, visited_edges);
            visited_edges[idx].set(node_idx, false);
        }
    }

    if is_small(id) {
        visited_nodes.set(idx, false);
    }

    num_paths
}

fn solve_first(graph: &Graph) -> usize {
    let mut visited_nodes = BitVec::from_elem(graph.len(), false);
    let mut visited_edges = vec![visited_nodes.clone(); graph.len()];
    
    visit_first(START_ID, graph, &mut visited_nodes, &mut visited_edges)
}

fn check_visit_fast(from: CaveId, to: CaveId, visited_nodes: &mut BitVec, visited_edges: &mut [BitVec], visited_twice: Id) -> (bool, bool)
{
    let can_visit = !((is_small(to) && visited_nodes[get_idx(to)]) || visited_edges[get_idx(from)][get_idx(to)]);
    if can_visit || to == START_ID {
        (can_visit, false)
    }
    else if is_small(to) {
        (visited_twice == 1, true)
    }
    else {
        (from.1 == visited_twice, false)
    }
}

fn check_visit(from: CaveId, to: CaveId, visited_nodes: &mut BitVec, visited_edges: &mut[BitVec], visited_twice: Id) -> (bool, bool)
{
    let can_visit = !((is_small(to) && visited_nodes[get_idx(to)]) || visited_edges[get_idx(from)][get_idx(to)]);
    if can_visit || to == START_ID {
        (can_visit, false)
    }
    else if is_small(to) {
        (visited_twice == 1, true)
    }
    else {
        (from.1 == visited_twice, false)
    }
}

fn visit_second(id: CaveId, graph: &Graph, visited_nodes: &mut BitVec, visited_edges: &mut [BitVec], visited_twice: Id) -> usize {
    // unsafe { path.push(id); }

    let idx = get_idx(id);
    if is_small(id) {
        visited_nodes.set(idx, true)
    }

    let mut num_paths = 0usize;
    for &node in &graph[idx] {
        let node_idx = get_idx(node);

        let (can_visit, is_second_small_visit) = check_visit(id, node, visited_nodes, visited_edges, visited_twice);
        if !can_visit {
            continue;
        }
        else if node == END_ID {
            // unsafe {println!("{:?}", path);}
            num_paths += 1;
        }
        else if is_second_small_visit {
            num_paths += visit_second(node, graph, visited_nodes, visited_edges, get_id(node));
        }
        else {
            visited_edges[idx].set(node_idx, true);
            num_paths += visit_second(node, graph, visited_nodes, visited_edges, visited_twice);
            visited_edges[idx].set(node_idx, false);
        }
    }

    if is_small(id) && id.1 != visited_twice {
        visited_nodes.set(idx, false);
    }

    // unsafe { path.pop(); }

    num_paths
}

fn visit_second_fast(id: CaveId, graph: &Graph, visited_nodes: &mut BitVec, visited_edges: &mut [BitVec], visited_twice: Id) -> usize {
    // unsafe { path.push(id); }

    let idx = get_idx(id);
    if is_small(id) {
        visited_nodes.set(idx, true)
    }

    let mut num_paths = 0usize;
    for &node in &graph[idx] {
        let node_idx = get_idx(node);

        let (can_visit, is_second_small_visit) = check_visit_fast(id, node, visited_nodes, visited_edges, visited_twice);
        if !can_visit {
            continue;
        }
        else if node == END_ID {
            // unsafe {println!("{:?}", path);}
            num_paths += 1;
        }
        else if is_second_small_visit {
            num_paths += visit_second_fast(node, graph, visited_nodes, visited_edges, get_id(node));
        }
        else {
            visited_edges[idx].set(node_idx, true);
            num_paths += visit_second_fast(node, graph, visited_nodes, visited_edges, visited_twice);
            visited_edges[idx].set(node_idx, false);
        }
    }

    if is_small(id) && id.1 != visited_twice {
        visited_nodes.set(idx, false);
    }

    // unsafe { path.pop(); }

    num_paths
}

fn solve_second(graph: &Graph) -> usize {
    let mut visited_nodes = BitVec::from_elem(graph.len(), false);
    let mut visited_edges = vec![visited_nodes.clone(); graph.len()];
    
    visit_second(START_ID, graph, &mut visited_nodes, &mut visited_edges, get_id(END_ID))
}

fn solve_second_fast(graph: &Graph) -> usize {
    let mut visited_nodes = BitVec::from_elem(graph.len(), false);
    let mut visited_edges = vec![visited_nodes.clone(); graph.len()];
    
    visit_second_fast(START_ID, graph, &mut visited_nodes, &mut visited_edges, get_id(END_ID))
}
  

fn parse(lines: Vec<String>) -> Graph {
    let mut nameToId = HashMap::from([
        (String::from("start"), START_ID),
        (String::from("end"), END_ID)
    ]);

    let mut graph = vec![Vec::new(), Vec::new()];

    for line in lines {
        let caves: Vec<&str> = line.split('-').collect();
        let ids: Vec<CaveId> = caves.iter().map(|&name| {
            if nameToId.contains_key(name) {
                *nameToId.get(name).unwrap()
            }
            else {
                let newId = (!name.chars().nth(0).unwrap().is_uppercase(), nameToId.len() as Id);
                nameToId.insert(String::from(name), newId);
                graph.push(Vec::new());

                assert_eq!(nameToId.len(), graph.len());
                newId
            }
        }).collect();

        assert!(ids.len() == 2);
        graph[get_idx(ids[0])].push(ids[1]);
        graph[get_idx(ids[1])].push(ids[0]);
    }
    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn twelve(b: &mut Bencher) -> () {
        let lines = parse(utils::get_input(12, true));

        // start profiling
        let guard = pprof::ProfilerGuard::new(100).unwrap();

        // run benchmark
        b.iter(|| solve_second(&lines));

        // build flamegraph
        if let Ok(report) = guard.report().build() {
            use std::fs::File;
            let file = File::create("flamegraph.svg").unwrap();
            report.flamegraph(file).unwrap();
        };

        // Put this into Cargo.toml if you want a useful flamegraph
        // [profile.release]
        // debug = true
    }

    #[bench]
    fn twelve_fast(b: &mut Bencher) -> () {
        let lines = parse(utils::get_input(12, true));

        assert_eq!(solve_second(&lines), solve_second_fast(&lines));
        
        b.iter(|| solve_second_fast(&lines));
    }
}
