use std::{
    collections::{HashMap, LinkedList, VecDeque},
    f32::consts::PI,
    io::LineWriter,
    str::Chars,
};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Clone, Copy)]
struct State {
    point: Point,
    angle: f32,
}

struct Parameters {
    angle: f32,
    line_length: f32,
    recursion: u8,
}

fn fill_points_to_draw<'a>(
    text: String,
    rules: &HashMap<char, &'a str>,
    states: &'a mut VecDeque<State>,
    mut state: State,
    params: &Parameters,
    to_draw: &mut Vec<(Point, Point)>,
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

                    let prev_point = state.point;
                    state.point.y += params.angle.sin() * params.line_length;
                    state.point.x += params.angle.cos() * params.line_length;

                    to_draw.push((prev_point, state.point));
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

fn main() {
    //Kocha 1
    // let input = String::from("A: AB,B: A\nA");
    let input = String::from("1: 11,0: 1[0]0\n0");
    let max_recursion: u8 = 3;

    let line_length: f32 = 4.0;

    let lines: Vec<&str> = input.lines().collect();

    let rules_str = lines[0];

    let mut rules = HashMap::new();

    let individual_rules = rules_str.split(",");

    for rule in individual_rules {
        let split: Vec<&str> = rule.split(": ").collect();
        //TODO: improve
        rules.insert(split[0].chars().collect::<Vec<char>>()[0], split[1]);
    }

    let mut angle: f32 = (PI * 90.0) / 180.0;

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

    let mut current_state: State = State {
        angle,
        point: starting_point,
    };

    let mut to_draw: Vec<(Point, Point)> = vec![];

    fill_points_to_draw(
        String::from(lines[1]),
        &rules,
        &mut state,
        current_state,
        &params,
        &mut to_draw,
        1,
    );

    for (start, end) in to_draw {
        println!("{:?} - {:?}", start, end);
    }
}
