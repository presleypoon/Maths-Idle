// #![allow(unused)]

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::io::{Write, stdout};
use std::time::{Duration, Instant};
use terminal_size::{Width, terminal_size};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Theory {
    id: u8,
    name: String,
    #[serde(skip)]
    name_len: usize,
    #[serde(skip)]
    name_revealed: Vec<bool>,
    equation: String,
    #[serde(skip)]
    equ_len: usize,
    #[serde(skip)]
    equ_revealed: Vec<bool>,
    cost: u128,
    unlock_criteria: Vec<u8>,
    check: Vec<u8>,
    unlocked: bool,
    ppt: u128,
    shown: bool,
}

struct GameState {
    point: u128,
    worker: u16,
    theories: Vec<Theory>,
    total_pps: u128,
}

fn main() -> () {
    let loaded_theories: Vec<Theory> = load_theories();

    let mut game_state: GameState = GameState {
        point: 0,
        worker: 1,
        total_pps: 0,
        theories: loaded_theories,
    };

    let width: usize = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);

    let _ = enable_raw_mode();
    let _ = execute!(stdout(), Hide);
    let _ = write!(stdout(), "{}", Clear(ClearType::All));

    let mut current_input: String = String::new();
    let mut lag_accumaulator: Duration = Duration::new(0, 0);
    let mut last_tick: Instant = Instant::now();

    loop {
        if game_tick(
            &mut last_tick,
            &mut lag_accumaulator,
            &mut game_state,
            width,
            &mut current_input,
        ) {
            return;
        }
        std::thread::sleep(Duration::from_millis(33));
    }
}

fn load_theories() -> Vec<Theory> {
    let mut file: File = File::open("resources/theories.json")
        .expect("CRITICAL: Missing resources/theories.json file!");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("CRITICAL: Failed to read theories.json contents!");
    let mut theories: Vec<Theory> = serde_json::from_str(&contents)
        .expect("CRITICAL: theories.json syntax error or field mismatch!");

    for theory in &mut theories {
        theory.name_len = theory.name.len();
        theory.name_revealed = vec![false; theory.name_len];
        theory.equ_len = theory.equation.len();
        theory.equ_revealed = vec![false; theory.equ_len];
    }
    theories
}

fn game_tick(
    last_tick: &mut Instant,
    lag_accumaulator: &mut Duration,
    game_state: &mut GameState,
    width: usize,
    current_input: &mut String,
) -> bool {
    let current_time: Instant = Instant::now();
    let frame_time: Duration = current_time.duration_since(*last_tick);
    let cost: u128 = (50.0 * 1.07_f32.powi(game_state.worker as i32)) as u128;
    *last_tick = current_time;
    *lag_accumaulator += frame_time;

    point(lag_accumaulator, game_state);
    render(width, &game_state, cost);
    worker_theories(&mut game_state.theories, game_state.worker);
    player_input(current_input, width, game_state, cost)
}

fn point(lag_accumaulator: &mut Duration, game_state: &mut GameState) -> () {
    let _ = write!(stdout(), "{}", MoveTo(0, 0));
    while *lag_accumaulator >= Duration::from_secs(1) {
        game_state.point += game_state.total_pps;
        *lag_accumaulator -= Duration::from_secs(1);
    }
}

fn render(width: usize, game_state: &GameState, cost: u128) -> () {
    println!(
        "Score: {:>6}    PPS: {:>6}    Worker(s): {:>3}    Worker Cost: {:>6}",
        number_to_ign(game_state.point),
        number_to_ign(game_state.total_pps),
        game_state.worker,
        if game_state.worker < 1000 {
            number_to_ign(cost)
        } else {
            "MAXED!".to_string()
        },
    );
    println!("{}", "=".repeat(width));

    for theory in &game_state.theories {
        if !theory.shown {
            continue;
        }

        let mut display_name: String = String::new();
        for (i, c) in theory.name.chars().enumerate() {
            if theory.unlocked || theory.name_revealed.get(i).copied().unwrap_or(false) {
                display_name.push(c);
            } else {
                display_name.push('_');
            }
        }

        let mut display_equ: String = String::new();
        for (i, c) in theory.equation.chars().enumerate() {
            if theory.unlocked || theory.equ_revealed.get(i).copied().unwrap_or(false) {
                display_equ.push(c);
            } else {
                display_equ.push('_');
            }
        }

        println!(
            "{:>3}. {}\n     {:.<24} Cost: {:.<5}.....{:<width$}",
            theory.id,
            display_name,
            display_equ,
            number_to_ign(theory.cost),
            if theory.unlocked {
                "Unlocked"
            } else {
                "Locked"
            },
            width = width,
        );
    }
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

fn player_input(
    current_input: &mut String,
    width: usize,
    game_state: &mut GameState,
    cost: u128,
) -> bool {
    prompt(current_input, width);
    input_event(current_input, game_state, cost)
}

fn prompt(current_input: &mut String, width: usize) -> () {
    print!(
        "\nWhat do you want to unlock: {:<width$}",
        current_input,
        width = width
    );
    let _ = stdout().flush();
}

fn input_event(current_input: &mut String, game_state: &mut GameState, cost: u128) -> bool {
    while event::poll(Duration::from_millis(0)).unwrap() {
        let Event::Key(key_event) = event::read().unwrap() else {
            continue;
        };
        if key_event.kind != event::KeyEventKind::Press {
            continue;
        }
        if button_macro(key_event, current_input, game_state, cost) {
            return true;
        };
    }
    false
}

fn button_macro(
    key_event: KeyEvent,
    current_input: &mut String,
    game_state: &mut GameState,
    cost: u128,
) -> bool {
    match key_event.code {
        KeyCode::Enter => {
            let trimmed_input: String = current_input.trim().to_string();
            unlock_show(game_state, trimmed_input, cost);
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
            let _ = execute!(stdout(), Show);
            return true;
        }
        _ => {}
    };
    false
}

fn worker_theories(theories: &mut Vec<Theory>, worker: u16) {
    let mut rng: rand::prelude::ThreadRng = rand::rng();

    for _ in 0..worker {
        if rng.random_bool(0.5) {
            worker_names(theories, &mut rng);
        } else {
            worker_equations(theories, &mut rng);
        }
    }
}

fn worker_names(theories: &mut Vec<Theory>, rng: &mut rand::prelude::ThreadRng) -> () {
    if rng.random_range(0..100) != 0 {
        return;
    }
    let mut done: Vec<bool> = vec![false; theories.len()];

    loop {
        let len: usize = theories.len();
        let index: usize = rng.random_range(0..len);
        let theory: &mut Theory = &mut theories[index];
        if done.iter().all(|&x| x) {
            return;
        }
        if !theory.shown || done[index] {
            done[index] = true;
            continue;
        }
        worker_name(theory, rng);
        return;
    }
}

fn worker_name(theory: &mut Theory, rng: &mut rand::prelude::ThreadRng) -> () {
    let mut done: Vec<bool> = vec![false; theory.name_len];
    loop {
        let index: usize = rng.random_range(0..theory.name_revealed.len());
        if theory.name_revealed.iter().all(|&x| x) || done.iter().all(|&x| x) {
            return;
        }
        if theory.name_revealed[index] || done[index] {
            done[index] = true;
            continue;
        }
        theory.name_revealed[index] = true;
        return;
    }
}

fn worker_equations(theories: &mut Vec<Theory>, rng: &mut rand::prelude::ThreadRng) -> () {
    if rng.random_range(0..100) != 0 {
        return;
    }
    let mut done: Vec<bool> = vec![false; theories.len()];

    loop {
        let len: usize = theories.len();
        let index: usize = rng.random_range(0..len);
        let theory: &mut Theory = &mut theories[index];
        if done.iter().all(|&x| x) {
            return;
        }
        if !theory.shown || done[index] {
            done[index] = true;
            continue;
        }
        worker_equation(theory, rng);
        return;
    }
}

fn worker_equation(theory: &mut Theory, rng: &mut rand::prelude::ThreadRng) -> () {
    let mut done: Vec<bool> = vec![false; theory.equ_len];
    loop {
        let index: usize = rng.random_range(0..theory.equ_revealed.len());
        if theory.equ_revealed.iter().all(|&x| x) || done.iter().all(|&x| x) {
            return;
        }
        if theory.equ_revealed[index] || done[index] {
            done[index] = true;
            continue;
        }
        theory.equ_revealed[index] = true;
        return;
    }
}

fn unlock_show(game_state: &mut GameState, input: String, cost: u128) {
    if input == "worker" {
        check_unlock_worker(game_state, cost);
        return;
    }
    let mut input_index: Option<usize> = None;
    for (i, theory) in game_state.theories.iter_mut().enumerate() {
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
        game_state.theories[i].unlocked = true;
        game_state.total_pps += game_state.theories[i].ppt;

        let check: Vec<u8> = game_state.theories[i].check.clone();
        for j in check {
            let unlock_critia: Vec<u8> = game_state.theories[j as usize].unlock_criteria.clone();
            let mut unlock: bool = true;

            for k in unlock_critia {
                if !game_state.theories[k as usize].shown {
                    unlock = false;
                    break;
                }
            }

            if unlock == false {
                continue;
            }

            game_state.theories[j as usize].shown = true;
        }
    }
}

fn check_unlock_worker(game_state: &mut GameState, cost: u128) -> () {
    if cost > game_state.point || game_state.worker >= 1000 {
        return;
    }

    game_state.point -= cost;
    game_state.worker += 1;
}
