use std::{io, fs, path::Path};

use crate::{generic, input_controller::InputController};

const FILE: &str = "scores.txt";
const MAX_SCORES: usize = 10;

pub fn input_score(score: u32, cursor_x: u16, cursor_y: u16) {
    generic::move_cursor(cursor_x, cursor_y);

    if score == 0 {
        println!("Press ENTER to continue");
        InputController::wait_for_enter();
        return;
    }

    let mut name_scores = read_scores();
    if name_scores.len() == MAX_SCORES {
        let last = name_scores.last().unwrap();
        if last.score >= score {
            println!("Press ENTER to continue");
            InputController::wait_for_enter();
            return;
        }
    }

    println!("Enter name and continue with ENTER");
    let cursor_y = cursor_y + 1;

    generic::move_cursor(cursor_x, cursor_y);
    print!("> ");
    let cursor_x = cursor_x + 2;

    generic::hide_cursor(false);

    // TODO: Name length limit
    // TODO: Only allow letters, numbers, and whitespace
    let mut name = String::new();
    while str::is_empty(name.trim()) {
        generic::move_cursor(cursor_x, cursor_y);
        io::stdin().read_line(&mut name).unwrap();
    }

    generic::hide_cursor(true);

    name_scores.push(NameScore { name, score });
    write_scores(name_scores);
}

pub fn write_scores(mut name_scores: Vec<NameScore>) {
    sort_scores(&mut name_scores);

    let mut name_scores_str = String::new();
    for (i, name_score) in name_scores.iter().enumerate() {
        if i == MAX_SCORES {
            break;
        }

        name_scores_str.push_str(&format!("{}:{};", &name_score.name.trim(), name_score.score));
    }

    if let Err(_) = fs::write(FILE, name_scores_str) {
        generic::error_print(&format!("Failed to write to score file {}", FILE));
    }
}

pub fn read_scores() -> Vec<NameScore> {
    if !Path::new(FILE).exists() {
        return Vec::new();
    }

    if let Ok(name_scores_str) = fs::read_to_string(FILE) {
        let mut name_scores = parse_score_str(&name_scores_str);
        cleanup_scores(&mut name_scores);
        return name_scores;
    } else {
        generic::error_print(&format!("Failed to read from score file {}", FILE));
    }

    Vec::new()
}

pub fn parse_score_str(score_str: &str) -> Vec<NameScore> {
    let mut name_scores: Vec<NameScore> = Vec::new();

    if str::is_empty(&score_str) || !str::contains(&score_str, ';') {
        return name_scores;
    }

    for single_score_str in score_str.split(';').into_iter() {
        if !single_score_str.contains(':') {
            continue;
        }

        let mut split = single_score_str.split(':');
        let name: String = String::from(split.next().unwrap());
        let score: u32 = split.next().unwrap().parse().unwrap_or(0);

        name_scores.push(NameScore { name, score });
    }

    name_scores
}

pub fn cleanup_scores(name_scores: &mut Vec<NameScore>) {
    name_scores.retain(|name_score| {
        name_score.score > 0
    });
    sort_scores(name_scores);

    if name_scores.len() > 10 {
        name_scores.drain(MAX_SCORES..);
    }
}

pub fn sort_scores(name_scores: &mut Vec<NameScore>) {
    name_scores.sort_by(|a, b| b.score.cmp(&a.score));
}

pub struct NameScore {
    pub name: String,
    pub score: u32
}
