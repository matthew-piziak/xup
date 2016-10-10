extern crate yaml_rust;

use self::yaml_rust::{yaml, Yaml};

#[derive(Debug, PartialEq)]
pub struct Doctrine {
    pub name: String,
    pub categories: Vec<Category>,
}

#[derive(Debug, PartialEq)]
pub struct Category {
    pub ships: Vec<String>,
}

pub type Result<T> = ::std::result::Result<T, &'static str>; //' just fixing the CR pretty-printer

fn load_doctrine(mut doctrine: yaml::Hash) -> Result<Doctrine> {
    let name_key = Yaml::String("name".into());
    let name = try!(doctrine.remove(&name_key)
                            .ok_or("Doctrine name not found.")
                            .and_then(|name| {
                                name.into_string()
                                    .ok_or("Doctrine name not a string.")
                            }));
    let categories_key = Yaml::String("categories".into());
    let categories = try!(doctrine.remove(&categories_key)
                                  .ok_or("Doctrine has no categories.")
                                  .and_then(Category::many_from_yaml));
    Ok(Doctrine {
        name: name,
        categories: categories,
    })
}

impl Doctrine {
    fn from_yaml(yaml: Yaml) -> Result<Self> {
        yaml.into_hash()
            .ok_or("Expected doctrine.")
            .and_then(|doctrine| load_doctrine(doctrine))
    }

    pub fn many_from_yaml(doctrines: Yaml) -> Result<Vec<Self>> {
        doctrines.into_vec()
                 .ok_or("Expected list of doctrines.")
                 .and_then(|doctrines| doctrines.into_iter().map(Doctrine::from_yaml).collect())
    }
}

fn load_ships(category_ships: Yaml) -> Vec<String> {
    category_ships.as_vec()
                  .map(|ships| {
                      ships.iter()
                           .map(|ship| {
                               let name = ship.as_str().expect("Ship name was not a string");
                               String::from(name)
                           })
                           .collect()
                  })
                  .unwrap_or_else(Vec::new)
}

fn load_category(mut category: yaml::Hash) -> Result<Category> {
    let category_key = Yaml::String("category".into());
    category.remove(&category_key)
            .ok_or("Could not find category.")
            .map(|category_ships| {
                let ships = load_ships(category_ships);
                Category { ships: ships }
            })
}

impl Category {
    fn from_yaml(yaml: Yaml) -> Result<Self> {
        yaml.into_hash()
            .ok_or("Expected category.")
            .and_then(load_category)
    }

    fn many_from_yaml(categories: Yaml) -> Result<Vec<Self>> {
        categories.into_vec()
                  .ok_or("Expected list of categories.")
                  .and_then(|categories| categories.into_iter().map(Category::from_yaml).collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::yaml_rust::{Yaml, YamlLoader};

    fn quick_parse(s: &str) -> Yaml {
        YamlLoader::load_from_str(&s).expect("Couldn't parse test YAML").swap_remove(0)
    }

    #[test]
    fn test_parse_categories() {
        let yaml = quick_parse(r#"
- category:
    [
    Blackbird,
    Celestis,
    Maller,
    ]
"#);

        let res = Category::many_from_yaml(yaml);
        assert_eq!(res,
                   Ok(vec![
            Category { ships: vec!["Blackbird".into(), "Celestis".into(), "Maller".into()] },
        ]));
    }

    #[test]
    fn test_parse_empty_category() {
        let yaml = quick_parse(r#"
- category:
    [
    # empty
    ]
- category:
    [Blackbird]
"#);

        let res = Category::many_from_yaml(yaml);
        assert_eq!(res,
                   Ok(vec![
            Category { ships: vec![]},
            Category { ships: vec!["Blackbird".into()] },
        ]));
    }

    #[test]
    fn test_parse_doctrines() {
        let yaml = quick_parse(r#"
- name: Armor Battleships
  categories:
  - category:
      [
      Blackbird,
      Celestis,
      Maller,
     ]
"#);


        let res = Doctrine::many_from_yaml(yaml);
        assert_eq!(res,
                   Ok(vec![
            Doctrine { name: "Armor Battleships".into(),
                       categories: vec![
                           Category { ships: vec!["Blackbird".into(),
                                                  "Celestis".into(),
                                                  "Maller".into()] },
                       ] },
        ]));
    }

    #[test]
    fn test_parse_doctrines_with_anchors() {
        let yaml = quick_parse(r#"
- name: Armor Battleships
  categories:
  - category: &ewar
      [
      Blackbird,
      Celestis,
      Maller,
      ]
- name: Armor Confessors
  categories:
  - category: *ewar
"#);

        let res = Doctrine::many_from_yaml(yaml);
        assert_eq!(res,
                   Ok(vec![
            Doctrine { name: "Armor Battleships".into(),
                       categories: vec![
                           Category { ships: vec!["Blackbird".into(),
                                                  "Celestis".into(),
                                                  "Maller".into()] },
                       ] },
            Doctrine { name: "Armor Confessors".into(),
                       categories: vec![
                           Category { ships: vec!["Blackbird".into(),
                                                  "Celestis".into(),
                                                  "Maller".into()] },
                       ] },
        ]));
    }
}
