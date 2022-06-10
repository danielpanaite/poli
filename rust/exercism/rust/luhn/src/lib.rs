/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let numbers: Vec<char> = code.chars().collect();
    if numbers.len() < 2 {
        return false;
    }
    let mut ncount = 0;
    let mut sum = 0;
    for n in 0..numbers.len(){
        //check if all digits are valid
        if !(numbers[n] == ' ') && !(numbers[n].is_numeric()){
            return false;
        }
        if numbers[n].is_numeric() {
            if ncount%2 == 0 {
                let mut tempd = (numbers[n].to_digit(10).unwrap())*2;
                if tempd > 9 {
                    tempd -= 9;
                }
                sum = sum + tempd;
            }else{
                sum = sum + numbers[n].to_digit(10).unwrap();
            }
            ncount+=1;
        }
    }
    if sum%10 == 0 {
        return true;
    }else{
        return false;
    }
}