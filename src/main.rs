#![allow(unused)]

use crossterm::cursor::MoveTo;
use crossterm::terminal::{Clear, ClearType};
use std::io::{Write, stdin, stdout};
use std::ops::Index;
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

    let mut game_state: GameState = GameState{
        point: 0,
        worker: 0,
        total_pps: 0,
    };

    let width: usize = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);

    let _ = write!(stdout(), "{}", MoveTo(0, 0));
    let _ = write!(stdout(), "{}", Clear(ClearType::All));

    render(
        &mut theories,
        width,
        game_state.point,
        game_state.total_pps,
        game_state.worker,
    );

    loop {
        let input: String = get_user_input("What do you want to unlock");
        let mut input_index: Option<usize> = None;
        for (i, theory) in theories.iter_mut().enumerate() {
            if theory.name.to_lowercase() == input.to_lowercase() && theory.shown && !theory.unlocked && game_state.point >= theory.cost {
                input_index = Some(i);
                game_state.point -= theory.cost;
            }
        }

        if let Some(i) = input_index {
            theories[i].unlocked = true;
        }

        let _ = write!(stdout(), "{}", MoveTo(0, 0));
        let _ = write!(stdout(), "{}", Clear(ClearType::All));

        render(
            &mut theories,
            width,
            game_state.point,
            game_state.total_pps,
            game_state.worker,
        );
    }
}

fn render(theories: &mut Vec<Theory>, width: usize, point: u128, pps: u128, worker: u8) -> () {
    let rounded_point: String = point.to_string();

    println!(
        "Score: {:>6}    PPS: {:>6}    Worker(s): {:>3}",
        rounded_point, pps, worker
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

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);

    let _ = stdout().flush();

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}
