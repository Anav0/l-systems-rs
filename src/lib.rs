pub mod lsystems;

use std::{
    collections::{HashMap, VecDeque},
    f32::consts::PI,
};

use lsystems::{fill_points_to_draw, Parameters, Point, State};
use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn create_points() -> String {
    let input = String::from("F: F+F-F\nF");
    // let input = String::from("A: AB,B: A\nA");
    // let input = String::from("1: 11,0: 1[0]0\n0");
    let max_recursion: u8 = 3;

    let line_length: f32 = 8.0;

    let lines: Vec<&str> = input.lines().collect();

    let rules_str = lines[0];

    let mut rules = HashMap::new();

    let individual_rules = rules_str.split(",");

    for rule in individual_rules {
        let split: Vec<&str> = rule.split(": ").collect();
        //TODO: improve
        rules.insert(split[0].chars().collect::<Vec<char>>()[0], split[1]);
    }

    let angle: f32 = (PI * 90.0) / 180.0;

    let params = Parameters {
        angle,
        recursion: max_recursion,
        line_length,
    };

    let starting_point = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let mut state: VecDeque<State> = VecDeque::new();

    let current_state: State = State {
        angle,
        point: starting_point,
    };

    let mut to_draw: Vec<Point> = vec![];

    fill_points_to_draw(
        String::from(lines[1]),
        &rules,
        &mut state,
        current_state,
        &params,
        &mut to_draw,
        1,
    );

    serde_json::to_string(&to_draw).unwrap()
}
