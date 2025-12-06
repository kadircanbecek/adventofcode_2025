use core::num;
use std::fs;
pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("input")
}

fn parse_numbers (lines: Vec<&str>) -> Vec<Vec<i64>>{
    let mut nums: Vec<Vec<i64>> = Vec::new();
    for line in lines{
        let mut line_nums: Vec<i64> = Vec::new();
        let str_nums: Vec<&str> = line.split_whitespace().collect();
        for str_num in str_nums {
            match str_num.parse(){
                Ok(n) => line_nums.push(n),
                _ => continue
            }
        }
        nums.push(line_nums);
    }
    nums
}

fn parse_operands (line : &str) -> Vec<char>{
    let mut line_ops: Vec<char> = Vec::new();
    let str_ops: Vec<char> = line.chars().collect();
    for str_op in str_ops {
        match str_op{
            '*'=> line_ops.push('*'),
            '+'=> line_ops.push('+'),
            _ => continue
        }
    }
    line_ops
}

fn solve_part1(number_lines: Vec<Vec<i64>>, operands: Vec<char>) -> i64{
    let mut total = 0;
    for i in 0..operands.len() {
        let mut subtotal = if operands[i] == '+' { 0 } else {1} ;
        for j in 0..number_lines.len(){
            if operands[i] == '*' {
                subtotal*= number_lines[j][i];
            } else {
                subtotal+= number_lines[j][i];
            }
        }
        total+=subtotal
    }
    total
}

fn parse_numbers_pt2 (lines: Vec<&str>, line_starts: Vec<usize>) -> Vec<Vec<i64>>{
    let mut nums: Vec<Vec<i64>> = Vec::new();
    let mut new_lines: Vec<Vec<char>> = Vec::new();
    for lline in &lines {
        new_lines.push(lline.chars().collect());
    }

    for ls in 0..line_starts.len()-1{
        let mut num: Vec<i64> = Vec::new();
        
        let line_s = line_starts[ls];
        let line_e = line_starts[ls+1];

        for ll in line_s..line_e-1 {
            let mut number = 0;
            for line in &new_lines{
                if line[ll].is_numeric(){
                    number*=10;
                    number+= line[ll] as i64 - 0x30;
                }
            }
            num.push(number);
        }
        nums.push(num);
    }

    nums
}
fn parse_operands_pt2 (line : &str) -> (Vec<char>, Vec<usize>){
    let mut line_ops: Vec<char> = Vec::new();
    let mut line_start: Vec<usize> = Vec::new(); 
    let str_ops: Vec<char> = line.chars().collect();
    let mut idx = 0;
    for str_op in &str_ops {
        match str_op{
            '*'=> {
                line_ops.push('*');
                line_start.push(idx);
            },
            '+'=> {
                line_ops.push('+');
                line_start.push(idx);
        },
            _ => {}
        }
        idx+=1;

    }
    line_start.push(str_ops.len()+1);
    (line_ops,line_start)
}

fn solve_part2(number_lines: Vec<Vec<i64>>, operands: Vec<char>) -> i64{
    let mut total = 0;
    for i in 0..operands.len() {
        let mut subtotal = if operands[i] == '+' { 0 } else {1} ;
        for j in 0..number_lines[i].len(){
            if operands[i] == '*' {
                subtotal*= number_lines[i][j];
            } else {
                subtotal+= number_lines[i][j];
            }
        }
        total+=subtotal
    }
    total
}
fn main() {
    let raw_input = read_input("day06/src/input06.txt");
    let input_lines: Vec<&str> = raw_input.lines().collect();

    let number_lines: Vec<Vec<i64>> = parse_numbers(input_lines[0..input_lines.len()-1].to_vec());
    let operands: Vec<char> = parse_operands(input_lines[input_lines.len()-1]);

    let part1_res = solve_part1(number_lines, operands);
    println!("{}", part1_res);
    let (operands, line_starts) = parse_operands_pt2(input_lines[input_lines.len()-1]);
    // println!("{:?}", &line_starts);
    let number_lines: Vec<Vec<i64>> = parse_numbers_pt2(input_lines[0..input_lines.len()-1].to_vec(), line_starts);

    // println!("{:?}", &number_lines);
    let part2_res = solve_part2(number_lines, operands);
    println!("{}", part2_res);

}
