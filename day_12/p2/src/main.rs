use std::io;
use std::io::{Read, BufReader};
use std::collections::HashSet;

extern crate strum;
#[macro_use] extern crate strum_macros;
use strum::IntoEnumIterator;

extern crate num_bigint;
extern crate num_integer;

use num_bigint::BigInt;
use num_integer::Integer;

extern crate num_traits;
use num_traits::cast::ToPrimitive;

#[derive(EnumIter,Debug)]
enum Axis {
    GravX,
    GravY,
    GravZ
}

#[derive(Copy, Clone, Debug)]
struct Moon {
    // x, y ,z
    gravity: (i32, i32, i32),
    velocity: (i32, i32, i32) // for each axis as well
}

impl Moon {
    fn new(gravity: (i32, i32, i32)) -> Moon {
        Moon {
            gravity,
            velocity: (0, 0, 0)
        }
    }

    fn total_energy(&self) -> i32 {
        let pot = self.gravity.0.abs() + self.gravity.1.abs() + self.gravity.2.abs();
        let ke = self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs();

        pot * ke
    }

    fn apply_gravity(&mut self, other: (i32, i32, i32)) {
        // Compare X and apply. Apply the inverse (or the same) in the other moon
        self.velocity.0 += Moon::value_change(self.gravity.0, other.0);
        self.velocity.1 += Moon::value_change(self.gravity.1, other.1);
        self.velocity.2 += Moon::value_change(self.gravity.2, other.2);
    }

    fn apply_velocity(&mut self) {
        self.gravity.0 += self.velocity.0;
        self.gravity.1 += self.velocity.1;
        self.gravity.2 += self.velocity.2;
    }

    fn value_change(origin: i32, other: i32) -> i32 {
        if origin < other {
            1
        } else if origin > other {
            -1
        } else {
            0
        }
    }
}

#[derive(Clone, Debug)]
struct Simulation {
    stars: Vec<Moon>,
}

impl Simulation {
    fn new() -> Simulation {
        let mut moons = Vec::new();
        moons.push(Moon::new((-19, -4, 2)));
        moons.push(Moon::new((-9, 8, -16)));
        moons.push(Moon::new((-4, 5, -11)));
        moons.push(Moon::new((1, 9, -13)));
//
//        moons.push(Moon::new((-1, 0, 2)));
//        moons.push(Moon::new((2, -10, -7)));
//        moons.push(Moon::new((4, -8, 8)));
//        moons.push(Moon::new((3, 5, -1)));

        Simulation {
            stars: moons
        }
    }

    fn apply_velocity(&mut self) {
        for moon in self.stars.iter_mut() {
            moon.apply_velocity();
        }
    }

    fn apply_gravity(&mut self, i1: usize, other_coords: (i32, i32, i32)) {
        let moon = self.stars.get_mut(i1).unwrap();
        moon.apply_gravity(other_coords);
    }

    fn total_energy(&self) -> i32 {
        let mut result = 0;
        for moon in self.stars.iter() {
            result += moon.total_energy();
        }

        result
    }

    fn size(&self) -> usize {
        self.stars.len()
    }

    fn get_moon(&self, idx: usize) -> &Moon {
        self.stars.get(idx).unwrap()
    }

    fn print(&self) {
        for (idx, moon) in self.stars.iter().enumerate() {
            println!("moon: {} - Gravity: {:?} ------ Velocity: {:?}", idx, moon.gravity, moon.velocity);
        }
    }

    fn are_equal(&self, other: &Simulation) -> bool {
        let mut equal = true;
        for (m1, m2) in self.stars.iter().zip(&other.stars) {
            if m1.velocity != m2.velocity || m1.gravity != m2.gravity {
                equal = false;
            }
        }

        equal
    }
}

fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    //meh this one is too annoying to parse
    let mut simulation = Simulation::new();
    println!("Initial state:");
    simulation.print();

    let original = simulation.clone();
    let count_moons = simulation.size();

    println!("End Initial State");
    let mut lcm= BigInt::from(1);
    for axis in Axis::iter() {
        simulation = original.clone();
        println!("====================== chaging TO axis {:?}===============\n", axis);
        simulation.print();
        println!("======================= CLONE END ========================");

        let mut grav_repeat = HashSet::new();
        let mut vel_repeat = HashSet::new();
        fill_axis_hist(&mut grav_repeat, &mut vel_repeat,  &simulation, &axis);

        let mut i = 0;
        loop {
            // for each axis in gravity and velocity look for frequency. Makes a total of 6 combinations
            // First step:
            // Compare pair of moons apply gravity (changes each moon velocity)
            for idx in 0..count_moons {
                for pair_idx in 0..count_moons {
                    if pair_idx != idx {
                        let other = simulation.get_moon(pair_idx).clone();
                        simulation.apply_gravity(idx, other.gravity);
                    }
                }
            }

            // After all gravities computed update each Moon position by adding the respective velocity
            simulation.apply_velocity();

            simulation.print();
            println!("END STEP {}\n", i);
            i += 1;

            if repeats(&mut grav_repeat, &mut vel_repeat, &simulation, &axis) {
                println!("Found repeat at iteration: {}", i);
                // without counting the initial state... subtract 1



                let freq = i;
                println!("FREQUENCY!!!: {:?}", freq);
                lcm = lcm.lcm(&BigInt::from(freq));
                grav_repeat.clear();
                break;
            }

        }

        // This is ONE step
    }


    // At this point you have LCM for every axis... output
    println!("The number of moves until rep {}", lcm.to_i64().unwrap());

    Ok(())
}

fn repeats(grav_hist: &mut HashSet<(i32, i32, i32, i32)>, vel_hist: &mut HashSet<(i32, i32, i32, i32)>, sim: &Simulation, axis: &Axis) -> bool {
    let (grav, vel) = match axis {
        Axis::GravX => {
            ((sim.get_moon(0).gravity.0, sim.get_moon(1).gravity.0, sim.get_moon(2).gravity.0, sim.get_moon(3).gravity.0),
            (sim.get_moon(0).velocity.0, sim.get_moon(1).velocity.0, sim.get_moon(2).velocity.0, sim.get_moon(3).velocity.0))
        },
        Axis::GravY => {
            ((sim.get_moon(0).gravity.1, sim.get_moon(1).gravity.1, sim.get_moon(2).gravity.1, sim.get_moon(3).gravity.1),
            (sim.get_moon(0).velocity.1, sim.get_moon(1).velocity.1, sim.get_moon(2).velocity.1, sim.get_moon(3).velocity.1))
        },
        Axis::GravZ => {
            ((sim.get_moon(0).gravity.2, sim.get_moon(1).gravity.2, sim.get_moon(2).gravity.2, sim.get_moon(3).gravity.2),
            (sim.get_moon(0).velocity.2, sim.get_moon(1).velocity.2, sim.get_moon(2).velocity.2, sim.get_moon(3).velocity.2))
        },
    };

    grav_hist.contains(&grav) && vel_hist.contains(&vel)
}

fn fill_axis_hist(grav_hist: &mut HashSet<(i32, i32, i32, i32)>, vel_hist: &mut HashSet<(i32, i32, i32, i32)>, sim: &Simulation, axis: &Axis) {
    match axis {
        Axis::GravX => {
            let x = (sim.get_moon(0).gravity.0, sim.get_moon(1).gravity.0, sim.get_moon(2).gravity.0, sim.get_moon(3).gravity.0);
            grav_hist.insert(x);
            vel_hist.insert((0, 0, 0,0));
        },
        Axis::GravY => {
            let x = (sim.get_moon(0).gravity.1, sim.get_moon(1).gravity.1, sim.get_moon(2).gravity.1, sim.get_moon(3).gravity.1);
            grav_hist.insert(x);
            vel_hist.insert((0, 0,0,0));
        },
        Axis::GravZ => {
            let x = (sim.get_moon(0).gravity.2, sim.get_moon(1).gravity.2, sim.get_moon(2).gravity.2, sim.get_moon(3).gravity.2);
            grav_hist.insert(x);
            vel_hist.insert((0, 0,0,0));
        },
    }
}