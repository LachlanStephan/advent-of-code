struct Directions {
    direction: String,
    amount: String,
}

fn main() {
    let data = get_file();
    let split = split_file(&data);
    let formatted: Vec<Directions> = format_file(split);
    let result = get_sub_position(formatted);
    println!("{}", result);
}

fn get_file() -> String {
    let content = include_str!("../../../input/dive_input.txt");

    let data = String::from(content);
    if data.chars().count() == 0 {
        panic!("no file");
    }

    return data;
}

fn split_file(file: &str) -> Vec<&str> {
    let split: Vec<&str> = file.split("\n").collect();
    return split;
}

fn format_file(data: Vec<&str>) -> Vec<Directions> {
    let mut formatted: Vec<Directions> = Vec::new();
    let mut counter = 0;
    let limit = data.len() - 1;
    for n in data {
        if counter == limit {
            break;
        }
        let details: Vec<&str> = n.split(" ").collect();
        let dir = details[0];
        let change = details[1];
        let dirs = Directions {
            direction: dir.to_string(),
            amount: change.to_string(),
        };
        formatted.push(dirs);
        counter = counter + 1;
    }
    return formatted;
}

fn decrease_value(curr: i64, val: i64) -> i64 {
    return curr - val;
}

fn increase_value(curr: i64, val: i64) -> i64 {
    return curr + val;
}

fn multiply_values(val_1: i64, val_2: i64) -> i64 {
    return val_1 * val_2;
}

fn get_sub_position(data: Vec<Directions>) -> i64 {
    let mut horizontal = 0;
    let mut depth: i64 = 0;
    let mut aim = 0;
    
    let mut counter = 0;
    let limit = data.len();

    for n in data {
        // format amount to 1st char 
        let mut format_amount = n.amount;
        format_amount.truncate(1);
        // cast it to i64 
        let amount = format_amount.parse::<i64>().unwrap();

        if counter == limit {
            break;
        }

        if n.direction == "forward" {
            horizontal = increase_value(horizontal, amount);
            depth = increase_value(depth, multiply_values(aim, amount));
        }

        if n.direction == "down" {
            aim = increase_value(aim, amount);
        }
        
        if n.direction == "up" {
            aim = decrease_value(aim, amount);
        }

        counter = counter + 1;
    }
    return horizontal * depth;
}
