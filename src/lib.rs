pub mod lsystems;
use std::{
    collections::{HashMap, VecDeque},
    f64::consts::PI,
};

use lsystems::{fill_points_to_draw, Parameters, Point, State};
use std::panic;
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;
extern crate web_sys;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    Ok(())
}

#[wasm_bindgen]
pub fn create_points(input: &str) -> JsValue {
    let in_params: Parameters = serde_json::from_str(&input).expect("Failed to parse input string");

    let mut out_params: Parameters;

    let lines: Vec<&str> = in_params.system.lines().collect();

    let rules_str = lines[0];

    let mut rules = HashMap::new();

    let individual_rules = rules_str.split(",");

    for rule in individual_rules {
        let split: Vec<&str> = rule.split(": ").collect();
        if split.len() == 2 {
            //TODO: improve
            rules.insert(split[0].chars().collect::<Vec<char>>()[0], split[1]);
        }
    }

    let mut state: VecDeque<State> = VecDeque::new();

    let current_state: State = State {
        angle: (PI * in_params.angle) / 180.0,
        point: in_params.center_point,
    };

    let mut to_draw: Vec<[Point; 2]> = vec![];

    out_params = Parameters {
        angle: in_params.angle,
        line_length: in_params.line_length,
        recursion: in_params.recursion,
        system: String::from(lines[1]),
        center_point: in_params.center_point,
    };

    fill_points_to_draw(
        &rules,
        &mut state,
        current_state,
        &mut out_params,
        &mut to_draw,
        1,
    );

    JsValue::from_serde(&to_draw).unwrap()
}
