use std::io;
use std::io::{Read, BufReader};
use std::cell::{RefCell, Cell};

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

struct Simulation {
    stars: Vec<Moon>,
}

impl Simulation {
    fn new() -> Simulation {
        let mut moons = Vec::new();
        moons.push(Moon::new((-1, 0, 2)));
        moons.push(Moon::new((2, -10, -7)));
        moons.push(Moon::new((4, -8, 8)));
        moons.push(Moon::new((3, 5, -1)));

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
            println!("moon: {} - Gravity: {:?} ------ Velocity: {:?} - {}", idx, moon.gravity.0, moon.velocity.0, moon.velocity.1);
        }
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

    let count_moons = simulation.size();

    println!("End Initial State");
    let mut x = 0;
    for i in 0..40 {
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
        println!("END STEP {}\n", x);
        x +=1;
    }


    // After 1000 steps calculate total energy:
    // TE = POT *  KE // FOR EACH MOON
    //    POT = SUM(abs(grav(x, y, z)))
    //    KE = SUM(abs(velo(x, y, z)))
    // Result: Total energy in the system (Sum of all moons energies)
    println!("Total system energy {}", simulation.total_energy());

    Ok(())
}

fn is_init_state(sim: &Simulation) -> bool {
    let g1 = (-19, -4, 2);
    let g2 = (-9, 8, -16);
    let g3 = (-4, 5, -11);
    let g4 = (1, 9, -13);

    let m1 = sim.get_moon(0).gravity;
    let m2 = sim.get_moon(1).gravity;
    let m3 = sim.get_moon(2).gravity;
    let m4 = sim.get_moon(3).gravity;

    let mv1 = sim.get_moon(0).velocity;
    let mv2 = sim.get_moon(1).velocity;
    let mv3 = sim.get_moon(2).velocity;
    let mv4 = sim.get_moon(3).velocity;


    g1 == m1 && g2 == m2 && g3 == m3 && g4 == m4 && mv1 == (0, 0, 0) && mv1 == (0, 0, 0)  && mv1 == (0, 0, 0) && mv1 == (0, 0, 0)
}