#![allow(unused)]

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
    worker_no: u32,
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
    
    render(&mut theories);
}

fn render(theories: &mut Vec<Theory>) {
    for theory in theories {
        
    }
}
