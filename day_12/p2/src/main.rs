use std::io;
use std::io::{Read, BufReader};
use std::cell::{RefCell, Cell};
use std::ptr::eq;

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
    let mut history = Vec::new();
    //meh this one is too annoying to parse
    let mut simulation = Simulation::new();
    println!("Initial state:");
    simulation.print();

    let test = simulation.clone();
    history.push(simulation.clone());

    let count_moons = simulation.size();

    println!("End Initial State");
    let mut i = 0;
    loop {
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

        if repeats(&history, &simulation) {
            println!("Found repeat at iteration: {}", i);
            break;
        }

        history.push(simulation.clone());
        i += 1;
        //simulation.print();
        // This is ONE step
        //println!("END STEP {}\n", i);
    }


    // After 1000 steps calculate total energy:
    // TE = POT *  KE // FOR EACH MOON
    //    POT = SUM(abs(grav(x, y, z)))
    //    KE = SUM(abs(velo(x, y, z)))
    // Result: Total energy in the system (Sum of all moons energies)
    println!("Total system energy {}", simulation.total_energy());

    Ok(())
}

fn repeats(hist: &Vec<Simulation>, sim: &Simulation) -> bool {
    hist.iter().any(|hist_sim| hist_sim.are_equal(sim))
}