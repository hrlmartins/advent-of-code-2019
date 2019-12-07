use std::collections::{HashMap, LinkedList};
use std::io::{self, BufRead, BufReader, Read};

struct Graph {
    in_count: HashMap<String, i32>,
    adj_list: HashMap<String, LinkedList<String>>,
    connections: HashMap<String, i32>, // Stores information about the # of each node. In the end it is a matter of running the keys and sum them
}

fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let mut graph: Graph = Graph {
        in_count: HashMap::new(),
        adj_list: HashMap::new(),
        connections: HashMap::new(),
    };

    for line in buffer.lines() {
        // Read input and form graph
        let input = line?;
        let edge: Vec<&str> = input.split(")").collect();
        create_nodes(edge[0], edge[1], &mut graph);
        make_connection(edge[0], edge[1], &mut graph);
    }

    let start = find_center_mass(&graph);

    fill_node_info(&mut graph, &start);

    let result: i32 = graph.connections.values().sum();

    println!("Number of connections: {}", result);
    Ok(())
}

fn fill_node_info(graph: &mut Graph, node: &String) -> i32 {
    if let None = graph.adj_list.get_mut(node) {
        return 0;
    }

    let proximity = graph.adj_list.get_mut(node).unwrap().clone(); // damn you borrow checker!!!
    let mut sum = 0;
    for orbit in proximity {
        sum += 1 + fill_node_info(graph, &orbit);
    }

    graph.connections.insert(node.clone(), sum);

    sum
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
