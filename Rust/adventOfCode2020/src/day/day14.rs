
use std::collections::HashMap;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub struct BitMask{
    one_mask: u64,
    x_mask: u64,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub struct MemoryOperation{
    target: u64,
    value: u64,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub enum Instruction{
    SetBitMask(BitMask),
    SetMemory(MemoryOperation),
}


pub struct Day14 {}

impl super::Day for Day14{
    type PuzzleInput = Vec<Instruction>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        text.lines()
            .filter_map(parsed_instruction)
            .collect()
    }

    fn solve_part1(&self, instructions: Self::PuzzleInput) -> std::string::String {
        let initial_bit_mask = BitMask {one_mask: 0, x_mask: 0};
        let memory_manager = DockingComputerMemoryManagerMk1 {};
        let mut computer = DockingComputer::new(memory_manager, initial_bit_mask);
        computer.execute(instructions);
        let result = computer.memory_sum();
        result.to_string()
    }

    fn solve_part2(&self, instructions: Self::PuzzleInput) -> std::string::String {
        let initial_bit_mask = BitMask {one_mask: 0, x_mask: 0};
        let memory_manager = DockingComputerMemoryManagerMk2 {};
        let mut computer = DockingComputer::new(memory_manager, initial_bit_mask);
        computer.execute(instructions);
        let result = computer.memory_sum();
        result.to_string()
    }
}

fn parsed_instruction(instruction_text: &str) -> Option<Instruction>{
    if instruction_text.starts_with("mask"){
        let bit_mask = parsed_bitmask(instruction_text)?;
        return Some(Instruction::SetBitMask(bit_mask));
    }
    let memory_operation = parsed_memory_operation(instruction_text)?;
    Some(Instruction::SetMemory(memory_operation))
}

fn parsed_bitmask(bit_mask_line: &str) -> Option<BitMask>{
    let mask_text = bit_mask_line.strip_prefix("mask = ")?;
    let one_mask = u64::from_str_radix(&mask_text.replace("X", "0"),2).ok()?;
    let x_mask = u64::from_str_radix(
        &mask_text
            .replace("1", "0")
            .replace("X", "1"),
        2)
        .ok()?;
    Some(BitMask {one_mask, x_mask})
}

fn parsed_memory_operation(operation_text: &str) -> Option<MemoryOperation>{
    lazy_static! {
        static ref OPERATION_RE: regex::Regex = regex::Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    }
    let captures: regex::Captures = OPERATION_RE.captures(operation_text)?;
    let target = captures[1].parse::<u64>().ok()?;
    let value = captures[2].parse::<u64>().ok()?;
    Some(MemoryOperation {target, value})
}

trait DockingComputerMemoryManager{
    fn set_memory(&self, memory: &mut HashMap<u64, u64>, bit_mask: &BitMask, operation: MemoryOperation);
}

struct DockingComputer<T>{
    mask: BitMask,
    memory: HashMap<u64, u64>,
    memory_manager: T,
}

impl<T: DockingComputerMemoryManager> DockingComputer<T> {
    pub fn new(memory_manager: T, initial_bit_mask: BitMask) -> DockingComputer<T>{
        DockingComputer {mask: initial_bit_mask, memory: HashMap::new(), memory_manager}
    }

    pub fn execute(&mut self, instructions: Vec<Instruction>){
        for instruction in instructions.into_iter(){
            self.execute_instruction(instruction);
        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction){
        match instruction{
            Instruction::SetBitMask(bit_mask) => self.set_mask(bit_mask),
            Instruction::SetMemory(operation) => self.set_memory(operation),
        }
    }

    fn set_mask(&mut self, bit_mask: BitMask){
        self.mask = bit_mask;
    }

    fn set_memory(&mut self, operation: MemoryOperation){
        self.memory_manager.set_memory(&mut self.memory, &self.mask, operation);
    }

    pub fn memory_sum(&self) -> u128 {
        self.memory
            .values()
            .map(|value| u128::from(*value))
            .sum()
    }
}

struct DockingComputerMemoryManagerMk1 {}

impl DockingComputerMemoryManager for DockingComputerMemoryManagerMk1{
    fn set_memory(&self, memory: &mut HashMap<u64, u64>, bit_mask: &BitMask, operation: MemoryOperation) {
        let value_to_store = (operation.value & bit_mask.x_mask) | bit_mask.one_mask;
        memory.insert(operation.target, value_to_store);
    }
}

struct DockingComputerMemoryManagerMk2 {}

impl DockingComputerMemoryManager for DockingComputerMemoryManagerMk2{
    fn set_memory(&self, memory: &mut HashMap<u64, u64>, bit_mask: &BitMask, operation: MemoryOperation) {
        let fixed_address = (operation.target | bit_mask.one_mask) & !bit_mask.x_mask;
        let varying_bits = bits(bit_mask.x_mask);
        let floating_addresses = bit_combinations(varying_bits.into_iter());
        let target_addresses = floating_addresses.into_iter()
            .map(|floating_address| floating_address | fixed_address);
        for address in target_addresses{
            memory.insert(address, operation.value);
        }
    }
}

fn bits(number: u64) -> Vec<u64>{
    let mut bit_vector = vec![];
    let mut current_bit: u64 = 1;
    for _ in 0..62{
        if number & current_bit != 0{
            bit_vector.push(current_bit);
        }
        current_bit <<= 1;
    }
    if number & current_bit != 0{
        bit_vector.push(current_bit);
    }
    bit_vector
}

fn bit_combinations(bits: impl Iterator<Item=u64>) -> Vec<u64>{
    let mut combinations = vec![0];
    for bit in bits{
        let mut new_combinations: Vec<u64> = combinations.iter()
            .map(|number| number | bit)
            .collect();
        combinations.append(&mut new_combinations);
    }
    combinations
}



#[cfg(test)]
mod day14_tests {
    use super::*;
    use crate::input;
    use crate::day;

    fn example_input() -> String{
        String::from(
"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0")
    }

    fn example_input2() -> String{
        String::from(
"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1")
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day14{});
        let problem_input = example_input();
        let expected_result = 165.to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day14{});
        let problem_input = example_input2();
        let expected_result = 208.to_string();
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day14{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 14, part: 1}).unwrap();
        let expected_result = String::from("12408060320841");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day14{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 14, part: 2}).unwrap();
        let expected_result = String::from("4466434626828");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}