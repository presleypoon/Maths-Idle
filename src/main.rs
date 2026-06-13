#![allow(unused)]

use crossterm::cursor::MoveTo;
use crossterm::terminal::{Clear, ClearType};
use std::io::{Write, stdin, stdout};
use std::ops::Index;
use std::process::ExitStatus;
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

    let mut game_state: GameState = GameState {
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
            if theory.name.to_lowercase() == input.to_lowercase()
                && theory.shown
                && !theory.unlocked
                && game_state.point >= theory.cost
            {
                input_index = Some(i);
                game_state.point -= theory.cost;
            }
        }
        if let Some(i) = input_index {
            theories[i].unlocked = true;
        }

        // let _ = get_user_input("Testing").parse().unwrap();
        
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
    let ign_point: String = number_to_ign(point);
    let ign_pps: String = number_to_ign(pps);

    println!(
        "Score: {:>6}    PPS: {:>6}    Worker(s): {:>3}",
        ign_point, ign_pps, worker
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
    print!("{}: ", prompt);

    let _ = stdout().flush();

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}

fn number_to_ign(number: u128) -> String {
    if number < 1000 {
        return format!("{:>4}  ", number);
    }

    let suffixes: [&str; 12] = [
        "K ", "M ", "B ", "T ", "Qa", "Qi", "Sx", "Sp", "O ", "N ", "D ", "U ",
    ];

    let f_num: f32 = number as f32;
    let exp_i_div_3: usize = (f_num.log10() / 3.0) as usize;
    let coef: f32 = f_num / 1000_f32.powi(exp_i_div_3 as i32);
    let suffix: &str = suffixes[exp_i_div_3 - 1];

    return if coef >= 100.0 {
        format!("{:>4.0}{}", coef, suffix)
    } else if coef >= 10.0 {
        format!("{:>4.1}{}", coef, suffix)
    } else {
        format!("{:>4.2}{}", coef, suffix)
    };
}
