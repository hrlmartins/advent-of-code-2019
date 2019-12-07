use std::collections::{HashMap, LinkedList};
use std::io::{self, BufRead, BufReader, Read};

struct Graph {
    in_count: HashMap<String, i32>,
    adj_list: HashMap<String, LinkedList<String>>,
}

fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let mut graph: Graph = Graph {
        in_count: HashMap::new(),
        adj_list: HashMap::new(),
    };

    for line in buffer.lines() {
        // Read input and form graph
        let input = line?;
        let edge: Vec<&str> = input.split(")").collect();
        create_nodes(edge[0], edge[1], &mut graph);
        make_connection(edge[0], edge[1], &mut graph);
    }

    let start = find_center_mass(&graph);

    let mut path_you = path_to_node(&mut graph, &start, &"YOU".to_string());
    let mut path_san = path_to_node(&mut graph, &start, &"SAN".to_string());

    // find common path and length to discount from both paths... then just remove 2 to not count with YOU AND SAN :)
    let common_path_size = find_common_len(&mut path_you, &mut path_san);

    // Remove common path twice to discount path from center to YOU and center to SAN
    println!(
        "Min steps: {}",
        (path_you.len() as i32 + path_san.len() as i32 - (common_path_size * 2) - 2)
    );
    Ok(())
}

fn path_to_node(graph: &mut Graph, node: &String, goal_node: &String) -> Vec<String> {
    if goal_node == node {
        let mut path = Vec::new();
        path.push(node.clone());
        return path;
    }

    if graph.adj_list.get_mut(node) == None {
        return Vec::new();
    }

    let proximity = graph.adj_list.get_mut(node).unwrap().clone(); // damn you borrow checker!!!
    for orbit in proximity {
        let mut recurse_path = path_to_node(graph, &orbit, goal_node);
        if !recurse_path.is_empty() {
            // UHOO we found it!
            recurse_path.push(node.clone());
            return recurse_path;
        }
    }

    Vec::new()
}

fn find_common_len(you: &mut Vec<String>, san: &mut Vec<String>) -> i32 {
    you.reverse();
    san.reverse();
    // pop COM
    you.remove(0);
    san.remove(0);

    let mut count = 0;
    for (you_n, san_n) in you.iter().zip(san.iter()) {
        if you_n == san_n {
            count += 1;
        } else {
            break;
        }
    }

    count
}

// Yeh... I could have simply searched for the COM string... but just to make sure there is no tricky input :D
fn find_center_mass(graph: &Graph) -> String {
    graph
        .in_count
        .iter()
        .find(|(_key, value)| **value == 0)
        .map(|(key, _)| key)
        .unwrap()
        .clone()
}

fn make_connection(node_a: &str, node_b: &str, graph: &mut Graph) {
    let v = graph.in_count.get(&node_b.to_string()).unwrap().clone();
    graph.in_count.insert(node_b.to_string(), v + 1);

    if let Some(v) = graph.adj_list.get_mut(&node_a.to_string()) {
        v.push_back(node_b.to_string());
    } else {
        let mut new_adj = LinkedList::new();
        new_adj.push_back(node_b.to_string());
        graph.adj_list.insert(node_a.to_string(), new_adj);
    }
}

fn create_nodes(node_a: &str, node_b: &str, graph: &mut Graph) {
    if !graph.in_count.contains_key(&node_a.to_string()) {
        graph.in_count.insert(node_a.to_string(), 0);
    }

    if !graph.in_count.contains_key(&node_b.to_string()) {
        graph.in_count.insert(node_b.to_string(), 0);
    }
}
