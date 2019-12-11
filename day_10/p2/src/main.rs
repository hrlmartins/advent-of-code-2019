extern crate num_rational;

use std::io::{self, BufReader, Read, BufRead};
use num_rational::Ratio;
use std::collections::{HashSet, HashMap};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y
        }
    }

    fn distance(&self, other: &Point) -> i32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        dx * dx + dy * dy
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Line {
    m: num_rational::Rational,
    b: num_rational::Rational,
    up: bool // up... but in fact i'll use it for horizontal lines as well
}

#[derive(Debug)]
struct LineInfo {
    points: Vec<Point>,
    origin: Point
}

impl LineInfo {
    fn new(origin: &Point) -> LineInfo {
        LineInfo {
            points: Vec::new(),
            origin: origin.clone()
        }
    }

    fn push_point(&mut self, point: Point) {
        self.points.push(point.clone());
    }

    fn sort_points_by_distance(&mut self) {
        let origin = self.origin.clone();

        self.points.sort_by(|p1, p2| {
            let p1_distance = p1.distance(&origin);
            let p2_distance = p2.distance(&origin);

            if p1_distance > p2_distance {
                Ordering::Greater
            } else if p1_distance < p2_distance {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        self.points.reverse(); // because the pop operation pops from the back... maybe vecDeque is better?
    }

    fn zero_points(&self) -> bool {
        self.points.is_empty()
    }

    // pre condition: there are elements to pop
    fn pop(&mut self) -> Point {
        self.points.pop().unwrap()
    }

    fn calc_angle(&self) -> i32 {
        let point = &self.points[0];

        let mut angle = ((point.y - self.origin.y) as f64).atan2((point.x - self.origin.x) as f64);
        println!("Original: rad {} - degrees {} for point {:?}", angle, angle * (180.0 / std::f64::consts::PI), point);

        angle += std::f64::consts::FRAC_PI_2; // Angles comes in relation to X. If it is on top of X it will be 0 but needs to be 90 degrees!

        if angle < 0.0 {
            // if it is still negative it was on the fourth quadrant. We can get the positive angle
            // by subtracti to 360 (point of origin) the remaining value
            angle = (2.0 * std::f64::consts::PI) + angle; // angle is negative so this a subtraction actually
        }

        println!("After conversion: rad {} - degrees {} for point {:?}", angle, angle * (180.0 / std::f64::consts::PI), point);

        assert!(angle >= 0.0);

        (angle * 100_000.0) as i32 // just for comparison sake. At least no one really trusted epsilon in the forums.
    }
}

impl Line {
    fn new(blast_gun: &Point, p2: &Point) ->  Line {
        if blast_gun.x == p2.x {
            // It's vertical... gonna create standard value..
            Line {
                m: num_rational::Rational::from(Ratio::new(0, 1)),
                b: num_rational::Rational::from(Ratio::new(0, 1)),
                up: p2.y > blast_gun.y, // actually going down??? :lol:
            }
        } else {
            let m = num_rational::Rational::from(Ratio::new((p2.y - blast_gun.y) as isize, (p2.x - blast_gun.x) as isize));
            Line {
                m,
                b: Line::find_b(&blast_gun, &m),
                up: if blast_gun.y == p2.y {
                    p2.x > blast_gun.x // going right if true, left if false
                } else {
                    p2.y > blast_gun.y // this one actually says if we're going up or down... well sorta
                },
            }
        }
    }

    fn find_b(p: &Point, m: &num_rational::Rational) -> num_rational::Rational {
        // slope function is from type y=mx+b. Substitute and solve for b.
        // y - mx = b
        let y = num_rational::Ratio::from_integer(p.y as isize);
        let x = num_rational::Ratio::from_integer(p.x as isize);

        num_rational::Rational::from(y - (m * x))
    }
}

fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let asteroids = store_asteroids(buffer).unwrap();
    let blast_gun = Point::new(29, 28); // the point
    let mut line_infos = produce_lines(&blast_gun, &asteroids);

    println!("Before sort angle {:?}", line_infos.len());
    sort_angle(&mut line_infos);
    println!("After sort angle {:?}", line_infos.len());
    sort_distance(&mut line_infos);

    //iterate in order
    let mut count = 0;
    'outer: loop {
        for line in line_infos.iter_mut() {
            if !line.zero_points() {
                count += 1;
                let point = line.pop();

                if count == 200 {
                    println!("Found it! Point {:?} and result: {}", point, point.x * 100 + point.y);
                    break 'outer;
                }
            }
        }
    }

    // Point x: 29, y: 28
    Ok(())
}

fn sort_distance(lines: &mut Vec<LineInfo>) {
    for line in lines {
        line.sort_points_by_distance();
    }
}

fn sort_angle(lines: &mut Vec<LineInfo>) {
    lines.sort_by(|l1, l2| {
        let a1 = l1.calc_angle();
        let a2 = l2.calc_angle();

        if a1 < a2 {
            Ordering::Less
        } else if a1 > a2 {
            Ordering::Greater
        } else {
            Ordering::Equal // equality and floting point... ermm just multiplied with a big num... hope itś enough
        }
    });
}

fn produce_lines(blast_gun: &Point, asteroids: &Vec<Point>) -> Vec<LineInfo> {
    let mut lines: HashMap<Line, LineInfo> = HashMap::new();

    for other_asteroid in asteroids {
        if other_asteroid != blast_gun {
            let line = Line::new(blast_gun, other_asteroid);
            match lines.contains_key(&line) {
                true => {
                    let stored_line = lines.get_mut(&line).unwrap();
                    stored_line.push_point(other_asteroid.clone());
                },
                _ => {
                    let mut info = LineInfo::new(&blast_gun);
                    info.push_point(other_asteroid.clone());
                    lines.insert(line, info);
                }
            }
        }
    }

    lines.into_iter().map(|(_, v)| v).collect()
}

fn store_asteroids<T: Read>(buffer: BufReader<T>) -> io::Result<Vec<Point>> {
    let mut curr_x = 0;
    let mut curr_y = 0;
    let mut points: Vec<Point> = Vec::new();
    for line in buffer.lines() {
        line?.chars().for_each(|c| {
            print!("{}", c);
            match c {
                '#' => {
                    points.push(Point::new(curr_x, curr_y))
                },
                _ => {}
            };

            curr_x += 1;
        });

        curr_y += 1;
        curr_x = 0;
        println!();
    }

    Ok(points)
}