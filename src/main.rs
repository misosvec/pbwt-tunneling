mod pbwt;

fn main() {
    let matrix = bwm(String::from("Tomorrow_and_tomorrow_and_tomorrow$"));
    print!("matrix {:?} ", matrix);
    let transform = bwt(String::from("Tomorrow_and_tomorrow_and_tomorrow$"));
    print!("transform {:?} ", transform);
}

fn string_rotations(str: String) -> Vec<String> {
    let mut rotations: Vec<String> = vec![];
    let str_len = str.len();
    for delimiter in 0..str_len {
        let (prefix, sufix) = str.split_at(delimiter);
        let result = [
            prefix.chars().rev().collect::<String>(),
            sufix.chars().rev().collect::<String>(),
        ]
        .join("");
        rotations.push(result.chars().rev().collect::<String>());
    }
    return rotations;
}

fn bwm(text: String) -> Vec<String> {
    let mut rotations = string_rotations(text);
    rotations.sort();
    return rotations;
}

fn bwt(text: String) -> Vec<char> {
    let bmw = bwm(text);
    let mut last_col: Vec<char> = vec![];
    for r in bmw {
        last_col.push(r.chars().last().unwrap());
    }
    return last_col;
}
