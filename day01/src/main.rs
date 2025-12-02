use std::fs;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("input")
}
#[derive(Debug, PartialEq)]
enum Rotation{
    L,
    R
}

#[derive(Debug)]
struct InstructionSet{
    direction: Rotation,
    amount: i32
}

impl InstructionSet {
    fn turn (&self, current:i32) -> i32{
        (current + self.amount*(if self.direction == Rotation::L {-1} else { 1 })).rem_euclid(100) 
    }

    fn turn_wo_modulo (&self, current:i32) -> (i32, i32){
        let turns = self.amount % 100;
        let mut zero_count = self.amount / 100;
        if self.direction == Rotation::R {
            let mut new_pos = current + turns % 100;
            zero_count += new_pos/100;
            new_pos %=100;
            (new_pos, zero_count)
        } else {
            let new_pos = (current - turns + 100) % 100;
            if (new_pos > current || new_pos == 0) && current != 0 {
                zero_count += 1;
            }
            (new_pos, zero_count)
        }
    }
}
fn parse_line (line: &str)->Option<InstructionSet>{
    let direction = match line.chars().nth(0){
        Some('L') => Rotation::L,
        Some('R') => Rotation::R,
        _ => return None
    };

    let amount: i32 = match line[1..line.len()].parse(){
        Ok(n) => n,
        Err(_) => return None
    };

    Some(InstructionSet { direction, amount })    
    

}
fn main() {
    let raw_data = read_input("day01/src/input01.txt");
    let instruction_set_vec: Vec<InstructionSet> = raw_data.lines().filter_map(parse_line).collect();


    println!("{:?}", instruction_set_vec);

    let mut password = 0;
    let mut start_point = 50;
    for instruction_set in &instruction_set_vec {
        start_point = instruction_set.turn(start_point);
        if start_point == 0 {
            password+=1;
        }
    }

    println!("{}", password);
    password = 0;
    start_point = 50;
    for instruction_set in &instruction_set_vec {

        let (new_start_point, click) = instruction_set.turn_wo_modulo(start_point);
        start_point = new_start_point;
        password +=click;
        println!("start_point after modulo= {}", start_point);
        println!("password = {}", password);
        println!("****************************");
        
    }
    
    println!("{}", password);
}
