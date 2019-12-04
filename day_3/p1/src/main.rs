use std::io::{self, BufRead, BufReader, Read};

//... yeah, there goes the unassigned integer assumption!!!
fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);

    let mut graph: Vec<Vec<Point>> = Vec::new();

    let mut line_id = 0;
    for line in buffer.lines() {
        graph.push(produce_line_vec(line?, line_id));
        line_id += 1;
    }

    brute(graph.as_mut());

    Ok(())
}

fn brute(graph: &mut Vec<Vec<Point>>) {
    let mut first_line = graph.pop().unwrap();
    let mut second_line = graph.pop().unwrap();

    let mut shortest: i32 = i32::max_value();

    for point in &first_line {
        for second_point in &second_line {
            if (point.x == second_point.x) && (point.y == second_point.y) {
                let distance = point.x.abs() + point.y.abs();
                shortest = shortest.min(distance);
                println!("x:{} - y:{} - distance: {} - shortest: {}", point.x, point.y, distance, shortest);
            }
        }
    }

    println!("Shortest {}", shortest);
}

fn produce_line_vec(line_path: String, line_id: i32) -> Vec<Point> {
    // We have the line... now process each command...
    // split by ',' and at each position read the command
    let mut result_points: Vec<Point> = Vec::new();
    let line_points: Vec<&str> = line_path.split(",").collect();

    let mut current_point: &Point = &Point {
        x: 0,
        y: 0,
        owner: line_id
    };

    for command in line_points {
        let mut points = process_command_and_produce_points(command, current_point); // extra one cuz of borrow checker
        result_points.append(points.as_mut());
        current_point = result_points.last().unwrap();
    }

    result_points
}

fn process_command_and_produce_points(command: &str, starting_point: &Point) -> Vec<Point> {
    // Each command is at the first position of the string... the argument is the remaining
    // Generate as many points as the move says (e.g U100 Generates 100 points changing the y coordinate)

    let (command, amount) = command.split_at(1);
    let move_amount = amount.parse::<i32>().unwrap();
    let mut point_vec = Vec::new();

    match command.chars().nth(0).unwrap() {
        'R' => {
            // We start creating the new point already. The starting one is already on the list
            let start_x = starting_point.x + 1;
            for x_value in start_x..=(starting_point.x + move_amount) {
                point_vec.push(Point {
                    x: x_value,
                    y: starting_point.y,
                    owner: starting_point.owner
                });
            }

            point_vec
        }
        'L' => {
            // We start creating the new point already. The starting one is already on the list
            let start_x = starting_point.x - 1;
            for x_value in ((starting_point.x - move_amount)..=start_x).rev() { // its important to insert points in order...
                point_vec.push(Point {
                    x: x_value,
                    y: starting_point.y,
                    owner: starting_point.owner
                });
            }

            point_vec
        }
        'U' => {
            // We start creating the new point already. The starting one is already on the list
            let start_y = starting_point.y + 1;
            for y_value in start_y..=(starting_point.y + move_amount) {
                point_vec.push(Point {
                    x: starting_point.x,
                    y: y_value,
                    owner: starting_point.owner
                });
            }

            point_vec
        }
        _ => { // Down
            // We start creating the new point already. The starting one is already on the list
            let start_y = starting_point.y - 1;
            for y_value in ((starting_point.y - move_amount)..=start_y).rev() {
                point_vec.push(Point {
                    x: starting_point.x,
                    y: y_value,
                    owner: starting_point.owner
                });
            }

            point_vec
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    owner: i32 // Overkill, just identifies the line of this point.
}