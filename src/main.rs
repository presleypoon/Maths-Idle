#![allow(unused)]

use crossterm::cursor::MoveTo;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
use std::fs::remove_dir;
use std::io::{Write, stdin, stdout};
use std::ops::Index;
use std::process::ExitStatus;
use std::time::{Duration, Instant};
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

    let _ = enable_raw_mode();
    let mut current_input: String = String::new();
    let mut lag_accumaulator: Duration = Duration::new(0, 0);
    let mut last_tick: Instant = Instant::now();

    loop {
        if game_loop(
            &mut last_tick,
            &mut lag_accumaulator,
            &mut game_state,
            &mut theories,
            width,
            &mut current_input,
        ) {
            return;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn game_loop(
    mut last_tick: &mut Instant,
    lag_accumaulator: &mut Duration,
    game_state: &mut GameState,
    theories: &mut Vec<Theory>,
    width: usize,
    current_input: &mut String,
) -> bool {
    let current_time: Instant = Instant::now();
    let frame_time: Duration = current_time.duration_since(*last_tick);
    *last_tick = current_time;
    *lag_accumaulator += frame_time;

    while *lag_accumaulator >= Duration::from_secs(1) {
        game_state.point += game_state.total_pps;
        *lag_accumaulator -= Duration::from_secs(1);
    }

    let _ = write!(stdout(), "{}", MoveTo(0, 0));
    let _ = write!(stdout(), "{}", Clear(ClearType::All));

    render(theories, width, &game_state);

    print!("\nWhat do you want to unlock: {}", current_input);
    let _ = stdout().flush();

    while event::poll(Duration::from_millis(0)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.kind == event::KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Enter => {
                        let trimmed_input: String = current_input.trim().to_string();
                        unlock_show(theories, game_state, trimmed_input);
                        current_input.clear();
                    }
                    KeyCode::Backspace => {
                        current_input.pop();
                    }
                    KeyCode::Char(c) => {
                        current_input.push(c);
                    }
                    KeyCode::Esc => {
                        let _ = disable_raw_mode();
                        return true;
                    }
                    _ => {}
                }
            }
        }
    }
    false
}

fn render(theories: &mut Vec<Theory>, width: usize, game_state: &GameState) -> () {
    let ign_point: String = number_to_ign(game_state.point);
    let ign_pps: String = number_to_ign(game_state.total_pps);

    println!(
        "Score: {:>6}    PPS: {:>6}    Worker(s): {:>3}",
        ign_point, ign_pps, game_state.worker
    );
    println!("{}", "=".repeat(width));

    for theory in theories {
        if !theory.shown {
            continue;
        }

        let ign_cost = number_to_ign(theory.cost);

        println!(
            "{:>3}. {:.<24} Cost: {:.<5}.....{}",
            theory.id,
            theory.name,
            ign_cost,
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

fn unlock_show(theories: &mut Vec<Theory>, game_state: &mut GameState, input: String) {
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
        game_state.total_pps += theories[i].ppt;

        let mut check: Vec<u8> = theories[i].check.clone();
        for j in check {
            let mut unlock_critia: Vec<u8> = theories[j as usize].unlock_critia.clone();
            let mut unlock: bool = true;

            for k in unlock_critia {
                if !theories[k as usize].shown {
                    unlock = false;
                    break;
                }
            }

            if unlock == false {
                continue;
            }

            theories[j as usize].shown = true;
        }
    }
}
