#![allow(unused)]

use terminal_size::{Width, terminal_size};

struct Theory {
    id: u8,
    name: String,
    equation: String,
    cost: u128,
    unlock_critia: Vec<u8>,
    check: Vec<u8>,
    unlocked: bool, 
    ppt: u128,
    shown: bool,
}

struct GameState {
    point: u128,
    worker: u8,
    total_pps: u128,
}

fn main() -> () {
    let mut theories: Vec<Theory> = vec![
        Theory {
            id: 0,
            name: "Peano's First Step".to_string(),
            equation: "1 + 1 = 2".to_string(),
            cost: 0,
            unlocked: false,
            unlock_critia: vec![],
            check: vec![1],
            ppt: 1,
            shown: true,
        },
        Theory {
            id: 1,
            name: "Addition".to_string(),
            equation: "x + y = z".to_string(),
            cost: 30,
            unlocked: false,
            unlock_critia: vec![0],
            check: vec![],
            ppt: 5,
            shown: true,
        },
    ];

    let /* mut */ game_state: GameState = GameState{
        point: 0,
        worker: 0,
        total_pps: 0,
    };

    let width: usize = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);

    render(
        &mut theories,
        width,
        game_state.point,
        game_state.total_pps,
        game_state.worker,
    );
}

fn render(
    theories: &mut Vec<Theory>,
    width: usize,
    point: u128,
    pps: u128,
    workers: u8,
) -> () {
    let rounded_point: String = point.to_string();

    println!(
        "Score: {:>6}    PPS: {:>6}    Worker(s): {:>3}",
        rounded_point, pps, workers
    );
    println!("{}", "=".repeat(width));

    for theory in theories {
        if !theory.shown {
            continue;
        }

        println!(
            "{:>3}. {:.<24} Point: {:.<5}.....{}",
            theory.id,
            theory.name,
            theory.cost,
            if theory.unlocked {
                "Unlocked"
            } else {
                "Locked"
            },
        );
    }
}
