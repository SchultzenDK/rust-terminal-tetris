use std::{io, fs};

use crate::generic;

const FILE: &str = "scores.txt";
const MAX_SCORES: usize = 10;

// TODO: Stop blindly using `unwrap()` in this entire controller.
// It's unsafe, especially when reading from files.
// Doesn't make it better that it's a simple txt file, anyone can edit.

pub fn input_score(score: u32, cursor_x: u16, cursor_y: u16) {
    generic::move_cursor(cursor_x, cursor_y);

    let mut name_scores = read_scores();

    if name_scores.len() == MAX_SCORES {
        let last = name_scores.last().unwrap();
        if last.score > score {
            println!("Press ENTER to continue");
            let mut _buf = String::new();
            io::stdin().read_line(&mut _buf).unwrap();
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

    fs::write(FILE, name_scores_str).unwrap();
}

pub fn read_scores() -> Vec<NameScore> {
    if let Ok(name_scores_str) = fs::read_to_string(FILE) {
        return parse_score_str(&name_scores_str);
    }

    return Vec::new();
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
        let score: u32 = split.next().unwrap().parse().unwrap();

        name_scores.push(NameScore { name, score });
    }

    return name_scores;
}

pub fn sort_scores(name_scores: &mut Vec<NameScore>) {
    name_scores.sort_by(|a, b| b.score.cmp(&a.score));
}

pub struct NameScore {
    pub name: String,
    pub score: u32
}
