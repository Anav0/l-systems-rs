use std::{
    collections::{HashMap, VecDeque},
    f64::consts::PI,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub point: Point,
    pub angle: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Parameters {
    pub angle: f64,
    pub line_length: f64,
    pub recursion: u8,
    pub system: String,
    pub center_point: Point,
}

pub fn fill_points_to_draw<'a>(
    rules: &HashMap<char, &'a str>,
    states: &'a mut VecDeque<State>,
    mut state: State,
    params: &mut Parameters,
    to_draw: &mut Vec<[Point; 2]>,
    depth: u8,
) {
    if depth > params.recursion {
        return;
    }

    // log!("{}", params.system);
    let mut combination: String = String::from("");
    for char in params.system.chars() {
        match char {
            '-' => {
                state.angle -= (PI * params.angle) / 180.0;
                combination += "-"
            }
            '+' => {
                state.angle += (PI * params.angle) / 180.0;
                combination += "+"
            }
            '[' => {
                states.push_front(state.clone());
                combination += "[";
            }
            ']' => {
                let poped_state = states.pop_front().expect("Failed to pop state");
                state = poped_state;
                combination += "]";
            }
            _ => {
                if rules.contains_key(&char) {
                    combination += *rules.get(&char).unwrap();
                }
                let prev_point = state.point.clone();
                state.point.y += state.angle.sin() * params.line_length;
                state.point.x += state.angle.cos() * params.line_length;

                to_draw.push([prev_point, state.point.clone()]);
            }
        }
    }
    params.system = combination;
    fill_points_to_draw(rules, states, state, params, to_draw, depth + 1);
}
