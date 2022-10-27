fn main() {
    //println!("{:?}", string_rotations(String::from("Ahoj")));
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
