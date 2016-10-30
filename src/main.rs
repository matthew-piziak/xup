extern crate clap;
extern crate yaml_rust;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use clap::{App, Arg};

use self::yaml_rust::{Yaml, YamlLoader};

mod doctrine;

fn file_contents(path: &Path) -> String {
    let mut file = File::open(&path).expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can't read file");
    contents
}

fn load_yaml_cfg(yaml_cfg_path: &Path) -> Yaml {
    let file_contents = file_contents(yaml_cfg_path);
    let mut contents = YamlLoader::load_from_str(&file_contents).expect("Couldn't load");

    let num_documents = contents.len();
    if num_documents > 1 {
        println!("{} documents found in {} ...",
                 num_documents,
                 yaml_cfg_path.display());
        println!("Reading doctrines from the first document and ignoring the rest.");
    }
    contents.swap_remove(0)
}

type DoctrineName = String;

fn load_doctrines(yaml_cfg_path: &Path) -> HashMap<DoctrineName, doctrine::Doctrine> {
    let yaml_cfg = load_yaml_cfg(yaml_cfg_path);
    let doctrines = doctrine::Doctrine::many_from_yaml(yaml_cfg).expect("Something");
    doctrines.into_iter().map(|d| (d.name.clone(), d)).collect()
}

fn ships(doctrine: &doctrine::Doctrine) -> Vec<String> {
    doctrine.categories.iter().flat_map(|c| c.ships.iter().cloned()).collect()
}

fn xup(doctrine: &doctrine::Doctrine) -> String {
    "x ".to_string() + &ships(doctrine).join(" / ")
}

fn main() {

    let matches = App::new("xup")
        .about("Outputs x-up string for given doctrine")
        .arg(Arg::with_name("doctrine")
            .short("d")
            .long("doctrine")
            .value_name("DOCTRINE")
            .takes_value(true))
        .arg(Arg::with_name("cfg")
            .short("c")
            .long("cfg")
            .value_name("CFG")
            .takes_value(true)
            .required(true))
        .get_matches();

    let yaml_cfg_path = Path::new(matches.value_of("cfg")
        .expect("YAML configuration file not found."));

    let doctrines = load_doctrines(yaml_cfg_path);

    match matches.value_of("doctrine") {
        Some(doctrine_name) => {
            match doctrines.get(doctrine_name) {
                Some(doctrine) => println!("{}", xup(&doctrine)),
                None => println!("Requested doctrine {} not found.", doctrine_name),
            }
        }
        None => {
            for doctrine_name in doctrines.keys() {
                println!("{}", doctrine_name);
            }
            return;
        }
    }
}
