#![allow(dead_code)] // TODO: disallow dead code

use std::env;
use std::fs;

#[macro_use]
extern crate serde_derive;

mod aggregate;
mod events;
mod state;

use aggregate::{Aggregate, OpticAggregate};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let incoming_events = events_from_file(filename);
    let mut aggregate = OpticAggregate::default();
    for event in incoming_events {
        aggregate.apply(event)
    }
    println!("Current state: {:?}", aggregate.get_state())
}

fn events_from_file(filename: &str) -> Vec<events::OpticEvent> {
    let file_contents =
        fs::read_to_string(filename).expect(&format!("File at {} could not be read", &filename));

    let events: Vec<events::OpticEvent> =
        serde_json::from_str(&file_contents).expect("File must be valid JSON");

    events
}

#[test]
fn can_read_events() {
    // let currentPath = std::path::Path::new()
    events_from_file(
        std::env::current_dir()
            .unwrap()
            .join("test-fixtures/uncompacted-spec.json")
            .to_str()
            .unwrap(),
    );
}
