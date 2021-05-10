
pub fn chinese_remainder<T: num::Integer + Clone>(remainder_modulo_pairs: Vec<(T, T)>) -> Option<T>{
    if remainder_modulo_pairs.is_empty(){
        return None;
    }
    let mut current_solution_pair = remainder_modulo_pairs[0].clone();
    for pair in remainder_modulo_pairs.into_iter().skip(1){
        current_solution_pair = chinese_remainder_for_two(current_solution_pair, pair)?
    }
    let (solution, _final_modulo) = current_solution_pair;
    Some(solution)
}

fn chinese_remainder_for_two<T: num::Integer + Clone>((remainder1, modulo1): (T, T), (remainder2, modulo2): (T, T)) -> Option<(T, T)>{
    let extended_modulo_gcd = modulo1.extended_gcd(&modulo2);
    let gcd = extended_modulo_gcd.gcd;
    let first_coefficient = remainder1.clone() / gcd.clone();
    let second_coefficient = remainder2.clone() / gcd.clone();
    let offset = remainder1.mod_floor(&gcd);
    if remainder2.mod_floor(&gcd) != offset{
        return None;
    }
    let base_solution = first_coefficient * extended_modulo_gcd.y * modulo2.clone() + second_coefficient * extended_modulo_gcd.x * modulo1.clone() + offset;
    let next_modulo = (modulo1 * modulo2) / gcd;
    let solution = base_solution.mod_floor(&next_modulo);
    Some((solution, next_modulo))
}