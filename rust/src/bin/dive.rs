struct Directions {
    direction: String,
    amount: String,
}

fn main() {
    let data = get_file();
    let split = split_file(&data);
    let formatted: Vec<Directions> = format_file(split);
    let result = get_sub_position(formatted);
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
    for n in data {
        let details: Vec<&str> = n.split(" ").collect();
        let dir = details[0];
        let change = details[1];
        let dirs = Directions{
            direction: dir.to_string(),
            amount: change.to_string(),
        };
        formatted.push(dirs);
    }
    return formatted;
}

fn decrease_value(curr: i32, val: i32) -> i32 {
    return curr - val;
}

fn increase_value(curr: i32, val: i32) -> i32 {
    return curr + val;
}

fn get_sub_position(data: Vec<Directions>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for commands in data {
        println!("{}", commands.amount);
    }

    return horizontal * depth;
}
