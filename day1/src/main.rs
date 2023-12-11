use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::str::Chars;

fn main() {
    let mut number_map = HashMap::new();

    number_map.insert(String::from("one"), 1);
    number_map.insert(String::from("two"), 2);
    number_map.insert(String::from("three"), 3);
    number_map.insert(String::from("four"), 4);
    number_map.insert(String::from("five"), 5);
    number_map.insert(String::from("six"), 6);
    number_map.insert(String::from("seven"), 7);
    number_map.insert(String::from("eight"), 8);
    number_map.insert(String::from("nine"), 9);
    

    let mut sum = 0;
    if let Ok(lines) = get_text_file(String::from("input.txt")) {
        for line in lines {
            if let Ok(sequence) = line {
                let mut num_list: Vec<i32> = Vec::new();
                let sequence_arr: Vec<char> = sequence.chars().collect();
                    parser(&mut num_list, &sequence_arr, &number_map);
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
fn num_from_word(ch: &mut usize, sequence: &Vec<char>, number_map: &HashMap<String, i32>) -> i32 {
    let mut result_string = String::from("");
    let mut temp = -1;
    let iter_start= *ch;
    for i in iter_start..sequence.len() {
        for j in i..sequence.len() {
            result_string.push(sequence[j]);
            if number_map.contains_key(&result_string) {
                let result = number_map.get(&result_string).unwrap();
                temp = *result;
                break;
            }
        }
        result_string = String::from("");
    }
    temp
}
fn parser(num_list: &mut Vec<i32>, sequence: &Vec<char>, number_map: &HashMap<String, i32>) {
    let mut result_string = String::from("");
    let mut temp = -1;

    for i in 0..sequence.len() {
        let temp_parse = try_parse_char_to_int(sequence[i]).unwrap();
        if temp_parse >= 0 {
            num_list.push(temp_parse);
            continue;
        }
        for j in i..sequence.len() {
            result_string.push(sequence[j]);
            if number_map.contains_key(&result_string) {
                let result = number_map.get(&result_string).unwrap();
                temp = *result;
                num_list.push(temp);
                break;
            }
        }
        result_string = String::from("");
    }

}