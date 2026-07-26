use arinc424_rs::parser::Arinc424VersionedEntity;
use arinc424_rs::parser::{Arinc424Reader, Arinc424Version};
use arinc424_rs::rev18_faa::entities::entity::Entity;
use std::path::Path;
#[test]
fn test_parse_file() {
    let reader =
        Arinc424Reader::from_path(Arinc424Version::Rev18FAA, Path::new("data/FAACIFP18.txt"))
            .expect("Failed to open file");
    for entity in reader {
        match entity {
            Ok(Arinc424VersionedEntity::Rev18FAA(e)) => match e {
                Entity::Navaid(navaid) => {
                    println!("Navaid: {:?}", navaid);
                }
            },
            Err(e) => match e.kind() {
                std::io::ErrorKind::InvalidData => {}
                _ => {
                    println!("Error: {:?}", e);
                }
            },
        }
    }
}
