use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
fn main() {
    let mut sum = 0;
    if let Ok(lines) = get_text_file(String::from("input.txt")) {
        for line in lines {
            if let Ok(sequence) = line {
                let mut num_list: Vec<i32> = Vec::new();
                for ch in sequence.chars() {
                    let parsed_num = try_parse_char_to_int(ch).unwrap();
                    if parsed_num >= 0 {
                        num_list.push(parsed_num);
                    }
                }
                sum_the_list(&mut sum, &mut num_list);
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

fn try_parse_char_to_int(ch: char) -> std::io::Result<i32> {
    let char_str = String::from(ch);
    let possible_num = char_str.parse::<i32>();
    match possible_num {
        Ok(result) => Ok(result),
        Err(_error) => Ok(-1)
    }

}

fn sum_the_list(sum: &mut i32, list: &mut Vec<i32>) {
    if list.len() == 0 {
        return
    }
    let mut new_sum = *sum;
    let first_val = list[0];
    let last_val = list[list.len() - 1];

    if list[list.len() - 1] == 0 {
        new_sum = new_sum + ((first_val * 10) + first_val);
    } else {
        new_sum = new_sum + ((first_val * 10) + last_val);
    }
    *sum = new_sum;
}
