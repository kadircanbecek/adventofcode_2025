use std:: fs;
use good_lp::{Expression, Solution, SolverModel, constraint, default_solver, variable, variables};
pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("input")
}
fn parse_line_pt1(line: &str)-> Option<(Vec<bool>, Vec<Vec<i64>>)>{
    let parts: Vec<&str> = line.split(" ").collect();
    let mut mapp: Vec<bool> = Vec::new();
    let mut buttons: Vec<Vec<i64>> = Vec::new();
    for pp in parts{
       match pp.chars().collect::<Vec<char>>()[0]{
        '['=>{
            for pc in pp.chars(){
                match pc {
                    '.'=> mapp.push(false),
                    '#'=>mapp.push(true),
                    _=>{}
                };
            }
        },
        '('=>{
            let button_parts: Vec<i64> = pp.replace("(", "").replace(")", "")
                .split(",")
                .filter_map(|s| s.parse().ok())
                .collect();
            if !button_parts.is_empty(){
                buttons.push(button_parts);
            }
        },
        _=>{

        }
       }
    }
    Some((mapp, buttons))
}

fn solve_part1(mapp: Vec<bool>, buttons: Vec<Vec<i64>>)->usize{
    let mut shortest=buttons.len()*20;
    for i in 0..buttons.len(){
        let mut comb: Vec<usize> = (0..i+1).collect();
        let mut data_gen = true;
        while data_gen {
            let mut mapp_new: Vec<bool> = vec![false; mapp.len()]; 
            for comm in &comb{
                let button_comb = &buttons[*comm];
                for bb in button_comb{
                    mapp_new[*bb as usize] = !mapp_new[*bb as usize];
                }
            }
            if (mapp_new == mapp) && (comb.len()<shortest){
                shortest = comb.len();
            }
            data_gen = generate_data(&mut comb, buttons.len()-1);
        }
    }
    // println!("{shortest}");
    shortest
}
fn generate_data(comb: &mut Vec<usize>, max_num: usize) -> bool {
    // Find rightmost position that can be incremented
    let mut pos = comb.len();
    
    while pos > 0 {
        pos -= 1;
        
        // Check if this position can be incremented
        // Position i can go up to max_num - (len - i - 1)
        if comb[pos] < max_num - (comb.len() - pos - 1) {
            comb[pos] += 1;
            // Reset all positions to the right
            for i in (pos + 1)..comb.len() {
                comb[i] = comb[i - 1] + 1;
            }
            return true;
        }
    }
    
    false  // No more combinations
}

fn parse_line_pt2(line: &str)-> Option<(Vec<usize>, Vec<Vec<usize>>)>{
    let parts: Vec<&str> = line.split(" ").collect();
    let mut joltages: Vec<usize> = Vec::new();
    let mut buttons: Vec<Vec<usize>> = Vec::new();
    for pp in parts{
       match pp.chars().collect::<Vec<char>>()[0]{
        '{'=>{
            joltages = pp.replace("{", "").replace("}", "")
                .split(",")
                .filter_map(|s| s.parse().ok())
                .collect();
        },
        '('=>{
            let button_parts: Vec<usize> = pp.replace("(", "").replace(")", "")
                .split(",")
                .filter_map(|s| s.parse().ok())
                .collect();
            if !button_parts.is_empty(){
                buttons.push(button_parts);
            }
        },
        _=>{

        }
       }
    }
    Some((joltages, buttons))
}

fn solve_part2(joltage: Vec<usize>, buttons: Vec<Vec<usize>>) -> usize{
    let mut shortest=0;
    let n = buttons.len();
    let m = joltage.len();
    let mut matrix_a: Vec<Vec<usize>> = vec![vec![0;n]; m];
    for (i, button) in buttons.iter().enumerate(){
        for lamp in button{
            matrix_a[*lamp][i]+=1;
        }
    }
    let mut vars = variables!();
    let mut x_vars = Vec::with_capacity(n);

    for _ in 0..n {
        let v = vars.add(variable().integer().min(0));
        x_vars.push(v);
    }

    // build objective: minimize sum_j x_j
    let objective = x_vars.iter().fold(Expression::from(0), |acc, &v| acc + v);
    
    let mut model = vars.minimise(objective).using(default_solver);
    for i in 0..m {
        let expr = x_vars.iter().enumerate()
            .fold(Expression::from(0), |acc, (j, &v)| acc + (matrix_a[i][j] as f64) * v);
        model = model.with(constraint!(expr == joltage[i] as f64));
    }

    // solve the model (handle Result/Errors in real code)
    let solution = model.solve().expect("Failed to solve model");
    
    // Calculate shortest from solution
    for &var in &x_vars {
        shortest += solution.value(var) as usize;
    }
    // println!("{shortest}");
    shortest
}
fn main() {
    let raw_input = read_input("day10/src/input10.txt");
    let result_1:usize = raw_input.lines().filter_map(parse_line_pt1).map(|(mapp, buttons)| solve_part1(mapp, buttons)).sum();
    
    let result_2:usize = raw_input.lines().filter_map(parse_line_pt2).map(|(joltage, buttons)| solve_part2(joltage, buttons)).sum();
    println!("{result_1}");
    println!("{result_2}");
}
