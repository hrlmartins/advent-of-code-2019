use std::io;
use std::io::{Read, BufReader, BufRead};
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::cmp::Ordering;

type Point = (isize, isize);
type Grid = HashMap<Point, char>;
type AdjList = HashMap<Point, Vec<(Point, KeyPath)>>;

#[derive(Clone)]
struct KeyPath {
    distance: i32,
    deps: HashSet<char>
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Node {
    distance: i32,
    point: Point,
    keys_collected: HashSet<Point>
}

impl Node {
    fn new(distance: i32, point: Point, keys_collected: HashSet<Point>) -> Node {
        Node {
            distance,
            keys_collected,
            point
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance).then_with(|| self.keys_collected.len().cmp(&other.keys_collected.len()))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl KeyPath {
    fn new(distance: i32, deps: HashSet<char>) -> KeyPath {
        KeyPath {
            distance,
            deps
        }
    }

    fn insert_dep(&mut self, key: char) {
        self.deps.insert(key);
    }

    fn increment_dist(&mut self) {
        self.distance += 1;
    }
}

fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let mut grid = HashMap::new();
    let mut keys = HashMap::new();
    let mut start: Point = (0, 0);

    for (y , line) in buffer.lines().enumerate() {
        for (x, c) in line?.chars().enumerate() {
            let p: Point = (x as isize, y as isize);
            grid.insert(p, c);

            if is_key(p, &grid) {
                keys.insert(c, p);
            }

            if is_start(p, &grid) {
                start = p;
            }

        }
    }

    let distances = keys.values()
        .chain([start].iter())
        .map(|p| (*p, fill_distances(p, &grid)))
        .collect::<AdjList>();

    let shortest = find_shortest(&start, &keys, &distances);
    println!("{:?}", shortest);

    Ok(())
}

// There are A LOT of array transversals here. that probably can be improve A LOT.
fn find_shortest(from: &Point, keys: &HashMap<char, Point>, distances: &AdjList) -> i32 {
    // This has to be a min priority Qeuue....
    let mut work = BinaryHeap::new();
    let mut seen = HashSet::new(); // Avoid processing duplicates... it just makes us take longer
    work.push(Node::new(0, *from, HashSet::new()));

    while let Some(node) = work.pop() {
        if node.keys_collected.len() == keys.len() {
            // We found all keys in the shortest possible distance!!
            return node.distance;
        }

        let mut col = node.keys_collected.iter().map(|x| *x).collect::<Vec<_>>();
        col.sort(); // Since the set iteration order is not guaranteed we sort it after
        if !seen.insert((node.point, col)) {
            continue;
        }

        //println!("{:?}", node);
        // Enqueue the next unlocked keys
        distances[&node.point].iter()
            .filter(|(point, info)| !node.keys_collected.contains(point) && is_unlocked(&deps_to_points(&info.deps, keys), &node.keys_collected))
            .for_each(|(point, info)| {
                let mut collected = node.keys_collected.clone();
                collected.insert(*point);
                let new_node = Node::new(node.distance + info.distance, *point, collected);

                work.push(new_node);
            })
    }

    panic!("Did not found a shortest path? wat")
}

fn is_unlocked(deps: &HashSet<Point>, collected: &HashSet<Point>) -> bool {
    deps.is_empty() || collected.is_superset(deps) // probably the superset definition is enough...
}

fn fill_distances(from: &Point, grid: &Grid) -> Vec<(Point, KeyPath)> {
    let mut seen = HashSet::new();
    let mut work: VecDeque<(Point, KeyPath)> = VecDeque::new();
    let mut result = Vec::new();
    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    work.push_back(((from.0, from.1), KeyPath::new(0, HashSet::new())));

    while let Some((p, path)) = work.pop_front() {
        if seen.insert(p.clone()) {
            for (dx, dy) in dirs.iter() {
                let new_p = (p.0 + *dx, p.1 + *dy);
                let mut new_path = path.clone();
                if is_valid(new_p) && !is_wall(new_p, grid) {
                    if is_door(new_p, grid) {
                        new_path.insert_dep(grid[&new_p].to_ascii_lowercase());
                    }

                    new_path.increment_dist();

                    if is_key(new_p, grid) {
                        result.push((new_p, new_path.clone()));
                    }
                    work.push_back((new_p, new_path));
                }
            }
        }
    }

    result
}

// Hammer Time
fn deps_to_points(deps: &HashSet<char>, keys: &HashMap<char, Point>) -> HashSet<Point> {
    deps.iter().map(|c| keys[c]).collect()
}

fn is_valid(p: Point) -> bool {
    p.0 >= 0 && p.1 >= 0
}

fn is_wall(p: Point, grid: &Grid) -> bool {
    if let Some(p) = grid.get(&p) {
        return *p == '#';
    }

    false
}

fn is_door(p: Point, grid: &Grid) -> bool {
    if let Some(p) = grid.get(&p) {
        return p.is_ascii_uppercase();
    }

    false
}

fn is_key(p: Point, grid: &Grid) -> bool {
    if let Some(p) = grid.get(&p) {
        return *p as u8 >= 'a' as u8  && *p as u8 <= 'z' as u8;
    }

    false
}

fn is_start(p: Point, grid: &Grid) -> bool {
    if let Some(p) = grid.get(&p) {
        return *p == '@';
    }

    false
}
