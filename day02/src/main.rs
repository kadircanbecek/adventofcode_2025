use std::{cmp::max, collections::HashSet, env, fs};

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("input")
}
struct IDSet{
    start:i64,
    end:i64
}

fn parse_line (line: &str)->Option<IDSet>{
    let parts: Vec<&str> = line.split("-").collect();
    if parts.len()!= 2{
        return None;
    }
    let start:i64 = match parts[0].parse(){
        Ok(n) => n,
        _ => return None
    };

    let end:i64 = match parts[1].parse(){
        Ok(n) => n,
        _ => return None
    };

    Some(IDSet { start, end })    
    

}

fn is_debug()-> bool{
    env::var("RUST_BACKTRACE").unwrap_or_default() == "1"
}
fn process_part1 (id_set: IDSet) -> i64{
    let mut id_start = id_set.start;
    let mut id_end = id_set.end;
    let mut id_start_digit_count = if id_start>0 {id_start.ilog10()+1} else {0};
    let mut id_end_digit_count = if id_end>0 {id_end.ilog10()+1} else {0};
    if id_start_digit_count%2!=0 {
        id_start = 10i64.pow(id_start_digit_count);
        id_start_digit_count+=1;
    }
    if id_end_digit_count%2!=0 {
        id_end = 10i64.pow(id_end_digit_count-1)-1;
        id_end_digit_count-=1;
    }
    
    let id_start_half_digits = id_start/(10i64.pow(id_start_digit_count/2));
    let id_end_half_digits = id_end/(10i64.pow(id_end_digit_count/2));
    let id_start_repeat = id_start_half_digits*10i64.pow(id_start_digit_count/2)+i64::from(id_start_half_digits);
    let id_end_repeat = id_end_half_digits*10i64.pow(id_end_digit_count/2)+i64::from(id_end_half_digits);
    let mut count:HashSet<i64> = HashSet::new();

    if id_start_repeat>=id_start && id_start_repeat<=id_end {
        count.insert(id_start_repeat);
    }
    
    if id_end_repeat>=id_start && id_end_repeat<=id_end {
        count.insert(id_end_repeat);
    }

    if id_end_repeat>id_start_repeat{
        let mut index = 1;
        let to_substact = 10i64.pow(id_start_digit_count/2)+1;
        loop{
            let num_check = id_end_repeat-to_substact*index;
            if num_check>id_start_repeat{
                count.insert(num_check);
                index+=1;
            } else {
                break;
            }
        }
    }
    if is_debug(){
        println!("id_start             {}", id_start);
        println!("id_end               {}", id_end);
        println!("id_start_half_digits {}", id_start_half_digits);
        println!("id_end_half_digits   {}", id_end_half_digits);
        println!("id_start_repeat      {}", id_start_repeat);
        println!("id_end_repeat        {}", id_end_repeat);
        println!("{:?}", count );
        println!("***************************************************************");
    }
    count.iter().sum()
}
fn get_repeat (rep_part:i64, rep_index:u32, rep_amount:u32)->i64{
    let mut repeat_number = rep_part;
    for _ in 1..rep_amount{
        repeat_number= repeat_number*10i64.pow(rep_index)+rep_part;
    }

    repeat_number
}
fn process_part2 (id_set: IDSet) -> i64{
    let id_start = id_set.start;
    let id_end = id_set.end;
    let id_start_digit_count = max(2,if id_start>0 {id_start.ilog10()+1} else {0});
    let id_end_digit_count = max(2, if id_end>0 {id_end.ilog10()+1} else {0});
    let mut count:HashSet<i64> = HashSet::new();
    if is_debug(){
        println!("id_start       {}", id_start);
        println!("id_end         {}", id_end);
    }
    for digit_count in id_start_digit_count..id_end_digit_count+1{
        for rep_index in 1..(digit_count+1)/2+1{
            if digit_count%rep_index!=0{
                continue;
            }
            let new_id_start = if id_start_digit_count==digit_count {id_start} else {10i64.pow(digit_count-1)}; 
            let new_id_end = if id_end_digit_count==digit_count {id_end} else {10i64.pow(digit_count)-1}; 
            if is_debug(){
                println!("digit_count          {}", digit_count);
                println!("rep_index            {}",rep_index);
                println!("new_id_start         {}", new_id_start);
                println!("new_id_end           {}", new_id_end);
            }    
            let id_start_half_digits = new_id_start/(10i64.pow(digit_count - rep_index));
            let id_end_half_digits = new_id_end/(10i64.pow(digit_count - rep_index));
            if is_debug(){
                println!("id_start_half_digits {}", id_start_half_digits);
                println!("id_end_half_digits   {}", id_end_half_digits); 
            }
               
            let id_start_repeat = get_repeat(id_start_half_digits, rep_index, digit_count/rep_index);
            let id_end_repeat = get_repeat(id_end_half_digits, rep_index, digit_count/rep_index);

            if id_start_repeat>=new_id_start && id_start_repeat<=new_id_end {
                count.insert(id_start_repeat);
            }
            
            if id_end_repeat>=new_id_start && id_end_repeat<=new_id_end {
                count.insert(id_end_repeat);
            }

            if id_end_repeat>id_start_repeat{
                let mut index = 1;
                let to_substact = get_repeat(1, rep_index, digit_count/rep_index);
                
                if is_debug(){
                    println!("to_substact          {}", to_substact);
                }
                loop{
                    let num_check = id_end_repeat-to_substact*index;
                    if num_check>id_start_repeat{
                        count.insert(num_check);
                        index+=1;
                    } else {
                        break;
                    }
                }
            }
            if is_debug(){
                println!("id_start_repeat      {}", id_start_repeat);
                println!("id_end_repeat        {}", id_end_repeat);
                
                println!("***************************************************************");
            }
        }
    }
    if is_debug(){
        println!("{:?}", count );
        println!("***************************************************************");
    }
    count.iter().sum()
}
fn main() {
    let raw_input = read_input("day02/src/input02.txt");
    let parsed_input: i64 = raw_input.split(",").filter_map(parse_line).map(process_part1).sum();

    println!("part1: {}", parsed_input);
    let parsed_input: i64 = raw_input.split(",").filter_map(parse_line).map(process_part2).sum();
    println!("part2: {}", parsed_input);
}
