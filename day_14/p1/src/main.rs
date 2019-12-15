use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Chemical {
    // For each dependency the quantity needed for a unit of production
    dependencies: HashMap<String, i128>,
    available: i128,
    production_unit: i128,
    name: String,
}

impl Chemical {
    fn new(production_unit: i128, name: String) -> Chemical {
        Chemical {
            dependencies: HashMap::new(),
            available: 0,
            production_unit,
            name
        }
    }

    fn add_dep(&mut self, name: String, quantity: i128) {
        self.dependencies.insert(name, quantity);
    }
    
    fn get_deps(&self) -> HashMap<String, i128> {
        self.dependencies.clone()
    }
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, Chemical>
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: HashMap::new()
        }
    }

    fn add_chemical(&mut self, chem: Chemical) {
        let chem_name = chem.name.clone();
        self.nodes.insert(chem_name, chem);
    }

    fn cost(&mut self, chemical: String, quant_requested: i128) -> i128 {
        let chem = self.get_deps_of(chemical.clone());
        let deps = chem.get_deps();
        let production_unit = chem.production_unit;
        let available = chem.available;

        // Lets figure out how much it is needed of the current chemical
        if quant_requested <= available {
            // It costs no additional ORE. But update the chemical!
            self.update_chem_available(chemical.clone(), available - quant_requested) ;
            return 0;
        }

        // Otherwise we need to figure out how much we need and ask for supply on the dependencies
        let amount_unit = ((quant_requested - available) as f32 / production_unit as f32).ceil() as i128;

        let mut cost = 0;

        for (name, quantity) in deps {
            //process node
            if name.as_str() == "ORE"  {
                cost += quantity * amount_unit;
            } else {
                cost += self.cost(name.clone(), quantity * amount_unit);
            }
        }

        // we are refilled...
        self.update_chem_available(chemical.clone(), available + ((amount_unit * production_unit) - quant_requested));

        cost
    }

    fn get_deps_of(&mut self, name: String) -> &mut Chemical {
        self.nodes.get_mut(&name).unwrap()
    }

    fn update_chem_available(&mut self, name: String, new_amount: i128) {
        let chemical = self.nodes.get_mut(&name).unwrap();
        chemical.available = new_amount;
    }
}

fn main() {
    read_and_compute_by_line(io::stdin());
}

fn read_and_compute_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buffer = BufReader::new(reader);
    let mut dep_graph = Graph::new();

    for line in buffer.lines() {
        // Right side is the dependent
        // Left side is the dependencies needed to produce the chemical
        let input_line = line?; // AYE "BORROR" CHECKER LOL
        let dep_definition: Vec<&str> = input_line.split("=>").collect();

        let deps = extract_chemical_deps(dep_definition[0]);
        let mut chemical = extract_chemical(dep_definition[1]);

        deps.iter().for_each(|(dep, quant)| chemical.add_dep(dep.clone(), *quant));

        dep_graph.add_chemical(chemical);
    };

    let cost = dep_graph.cost(String::from("FUEL"), 1);

    println!("COST: {:?}", cost);

    Ok(())
}

fn extract_chemical_deps(input: &str) -> Vec<(String, i128)> {
    let list_chemicals: Vec<&str> = input.split(",").collect(); // separated by ,
    let trimmed: Vec<&str> = list_chemicals.iter().map(|&chem| chem.trim()).collect();
    let mut deps = Vec::new();

    trimmed.iter().for_each(|chem| {
        // 0 - Quantity, 1 - Chemical
        let quant_chem: Vec<&str> = (*chem).split(" ").collect();
        let quantity = *quant_chem.get(0).unwrap();
        let chem = *quant_chem.get(1).unwrap();
        deps.push((String::from(chem), i128::from_str(quantity).unwrap()))
    });

    deps
}

fn extract_chemical(input: &str) -> Chemical {
    let trimmed = input.trim();
    let quant_chem: Vec<&str> = trimmed.split(" ").collect();
    let prod_unit = *quant_chem.get(0).unwrap();
    let chem = *quant_chem.get(1).unwrap();

    Chemical::new(i128::from_str(prod_unit).unwrap(), String::from(chem))
}