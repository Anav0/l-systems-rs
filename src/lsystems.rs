use std::{
    collections::{HashMap, VecDeque},
    f32::consts::PI,
};

use serde::Serialize;

#[derive(Serialize, Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub point: Point,
    pub angle: f32,
}

pub struct Parameters {
    pub angle: f32,
    pub line_length: f32,
    pub recursion: u8,
}

pub fn fill_points_to_draw<'a>(
    text: String,
    rules: &HashMap<char, &'a str>,
    states: &'a mut VecDeque<State>,
    mut state: State,
    params: &Parameters,
    to_draw: &mut Vec<Point>,
    depth: u8,
) {
    if depth > params.recursion {
        return;
    }

    let mut i = 0;
    let mut combination: String = String::from("");
    for char in text.chars() {
        match char {
            '-' => {
                state.angle -= (PI * 45.0) / 180.0;
                combination += "-";
            }
            '+' => {
                state.angle += (PI * 45.0) / 180.0;
                combination += "+";
            }
            '[' => {
                let mut state = State {
                    angle: params.angle,
                    point: state.point,
                };
                states.push_front(state);
                combination += "[";
            }
            ']' => {
                let poped_state = states.pop_front().expect("Failed to pop state");
                state = poped_state;
                combination += "]";
            }
            _ => {
                if rules.contains_key(&char) {
                    let rule = *rules.get(&char).unwrap();

                    combination += rule;

                    state.point.y += params.angle.sin() * params.line_length;
                    state.point.x += params.angle.cos() * params.line_length;

                    to_draw.push(state.point);
                }
            }
        }
        i += 1;
    }
    println!("{} {:?}", depth + 1, combination);
    fill_points_to_draw(
        combination,
        rules,
        states,
        state,
        params,
        to_draw,
        depth + 1,
    );
}
