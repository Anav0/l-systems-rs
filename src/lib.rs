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
pub fn create_points(input: String) -> String {
    println!("{}", input);
    let in_params: Parameters = serde_json::from_str(&input).unwrap();
    println!("{:?}", in_params);
    let mut out_params: Parameters;

    let lines: Vec<&str> = in_params.system.lines().collect();

    let rules_str = lines[0];

    let mut rules = HashMap::new();

    let individual_rules = rules_str.split(",");

    for rule in individual_rules {
        let split: Vec<&str> = rule.split(": ").collect();
        //TODO: improve
        rules.insert(split[0].chars().collect::<Vec<char>>()[0], split[1]);
    }

    let starting_point = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let mut state: VecDeque<State> = VecDeque::new();

    let current_state: State = State {
        angle: (PI * in_params.angle) / 180.0,
        point: starting_point,
    };

    let mut to_draw: Vec<Point> = vec![];

    out_params = Parameters {
        angle: current_state.angle,
        line_length: in_params.line_length,
        recursion: in_params.recursion,
        system: String::from(lines[1]),
    };

    fill_points_to_draw(
        &rules,
        &mut state,
        current_state,
        &mut out_params,
        &mut to_draw,
        1,
    );

    serde_json::to_string(&to_draw).unwrap()
}
