pub fn reverse(input: &str) -> String {

    let instr: Vec<char> = input.chars().collect();
    let mut revstr: Vec<char> = input.chars().collect();
    let mut revcount = 0;
    for n in (0..instr.len()).rev() {
        revstr[revcount] = instr[n];
        revcount += 1;
    }
    return revstr.iter().collect::<String>();

}
