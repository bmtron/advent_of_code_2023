use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let mut sum = 0;
    if let Ok(lines) = get_text_file(String::from("input.txt")) {
        for line in lines {
            if let Ok(checked_line) = line {
                sum = sum + check_game(&checked_line);
            }
        }
    }
    println!("{}", sum);
}


fn get_text_file(file_name: String) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>> {
    let file = File::open(file_name)?;
    let buf = BufReader::new(file);

    Ok(buf.lines())
}


fn check_game(line: &String) -> i32 {
    let mut game_num = 0;
    
    let game_num_split: Vec<&str> = line.split(':').collect();
    let game_num_split_0: Vec<&str> = game_num_split[0].split(' ').collect();
    if let Ok(game_num_result) = game_num_split_0[1].parse::<i32>() {
        game_num = game_num_result;
    }

    let games: Vec<&str> = game_num_split[1].split(';').collect();
    for game in games {
        let mut cube_map = HashMap::new();
        let cubes_splitter: Vec<&str> = game.split(',').collect();
        for cube_set in cubes_splitter {
            let trimmed_cube_set = cube_set.trim();
            let cube: Vec<&str> = trimmed_cube_set.split(' ').collect();
            cube_map.insert(cube[1], cube[0].parse::<i32>().unwrap());
        }
        if cube_map.contains_key("red") {
            let red_num = *cube_map.get("red").unwrap();
            if red_num > 12 {
                return 0;
            }
        }
        if cube_map.contains_key("blue") {
            let blue_num = *cube_map.get("blue").unwrap();
            if blue_num > 14 {
                return 0
            }
        }
        if cube_map.contains_key("green") {
            let green_num = *cube_map.get("green").unwrap();
            if green_num > 13 {
                return 0;
            }
        }
    }
    return game_num;
}

fn try_parse_char_to_int(ch: char) -> std::io::Result<i32> {
    let char_str = String::from(ch);
    let possible_num = char_str.parse::<i32>();
    match possible_num {
        Ok(result) => Ok(result),
        Err(_error) => Ok(-1)
    }
}
