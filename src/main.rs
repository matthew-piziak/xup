extern crate clap;
extern crate yaml_rust;

use std::collections::HashMap;
use std::env::home_dir;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use clap::{App, Arg, SubCommand};

use self::yaml_rust::{Yaml, YamlLoader};

mod doctrine;

fn yaml_cfg_path() -> PathBuf {
    let yaml_cfg_relative_path = "Dropbox/Log/Xup/src/xup.yaml";
    let mut home_dir = home_dir().expect("Could not resolve home directory.");
    home_dir.push(yaml_cfg_relative_path);
    home_dir
}

fn file_contents(path: &Path) -> String {
    let mut file = File::open(&path).expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can't read file");
    contents
}

fn load_yaml_cfg() -> Yaml {
    let yaml_cfg_path = yaml_cfg_path();
    let file_contents = file_contents(&yaml_cfg_path);
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

fn load_doctrines() -> HashMap<DoctrineName, doctrine::Doctrine> {
    let doctrines = doctrine::Doctrine::many_from_yaml(&load_yaml_cfg()).expect("Something");
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
        .subcommand(SubCommand::with_name("ls").about("Lists available doctrines"))
        .get_matches();

    let doctrines = load_doctrines();

    if let Some(_) = matches.subcommand_matches("ls") {
        for doctrine_name in doctrines.keys() {
            println!("{}", doctrine_name);
        }
        return;
    }

    match matches.value_of("doctrine") {
        Some(doctrine_name) => {
            match doctrines.get(doctrine_name) {
                Some(doctrine) => println!("{}", xup(&doctrine)),
                None => println!("Requested doctrine {} not found.", doctrine_name),
            }
        }
        None => println!("No doctrine requested."),
    }
}
