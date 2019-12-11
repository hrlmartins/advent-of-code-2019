extern crate num_rational;

use std::io::{self, BufReader, Read, BufRead};
use num_rational::Ratio;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
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
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Line {
    m: num_rational::Rational,
    b: num_rational::Rational,
    up: bool // up... but in fact i'll use it for horizontal lines as well
}

impl Line {
    fn new(p1: &Point, p2: &Point) ->  Line {
        if p1.x == p2.x {
            // It's vertical... gonna create standard value..
            Line {
                m: num_rational::Rational::from(Ratio::new(0, 1)),
                b: num_rational::Rational::from(Ratio::new(0, 1)),
                up: p2.y > p1.y
            }
        } else {
            let m = num_rational::Rational::from(Ratio::new((p2.y - p1.y) as isize, (p2.x - p1.x) as isize));
            Line {
                m,
                b: Line::find_b(&p1, &m),
                up: if p1.y == p2.y {
                    p2.x > p1.x // going right if true, left if false
                } else {
                    p2.y > p1.y // this one actually says if we're going up or down... well sorta
                }
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

    let mut max_count = 0;
    for asteroid in asteroids.iter() {
        max_count = max_count.max(count_lines(asteroid, &asteroids));
    }

    println!("Max possible sights is: {}", max_count);

    // Point x: 29, y: 28
    Ok(())
}

fn count_lines(asteroid: &Point, asteroids: &Vec<Point>) -> i32 {
    let mut lines = HashSet::new();

    for other_asteroid in asteroids {
        if other_asteroid != asteroid {
            lines.insert(Line::new(asteroid, other_asteroid));
        }
    }

    lines.len() as i32
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