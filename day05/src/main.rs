use std::fs;
pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("input")
}

fn parse_lines(lines: Vec<&str>)->Option<(Vec<(i128,i128)>, Vec<i128>)>{
    let mut ranges: Vec<(i128, i128)> = Vec::new();
    let mut ids: Vec<i128> = Vec::new();

    for line in lines{
        if line.contains("-"){
            let parts:Vec<&str> = line.split("-").collect();
            if parts.len() != 2{
                return None;
            }
            let start: i128 = match parts[0].parse(){
                Ok(n)=> n,
                _ => return None
            };
            
            let end: i128 = match parts[1].parse(){
                Ok(n)=> n,
                _ => return None
            };

            ranges.push((start, end));
        } else {
            match line.parse(){
                Ok(n) => ids.push(n),
                _ => continue
            };
        }
    }

    Some((ranges,ids))
}

fn solve_part1(ranges: &Vec<(i128,i128)>, ids: &Vec<i128>)-> i128{
    let mut valid_id_total = 0;
    for id_ in ids{
        for ranng in ranges{
            if *id_>=ranng.0 && *id_<=ranng.1{
                valid_id_total+=1;
                break;
            }
        }
    }
    valid_id_total
}

fn solve_part2(ranges: &Vec<(i128, i128)>) -> i128 {
    let mut new_ranges = ranges.clone();
    new_ranges.sort_by_key(|k| k.0);
    
    let mut merged_ranges: Vec<(i128, i128)> = Vec::new();
    
    for range in new_ranges {
        if merged_ranges.is_empty() {
            merged_ranges.push(range);
        } else {
            let last_idx = merged_ranges.len() - 1;
            let last = merged_ranges[last_idx];
            
            if range.0 <= last.1 + 1 {
                merged_ranges[last_idx] = (last.0, last.1.max(range.1));
            } else {
                merged_ranges.push(range);
            }
        }
    }
    
    // Calculate total coverage
    merged_ranges.iter().map(|r| r.1 - r.0 + 1).sum()
}

fn main() {
    let raw_input = read_input("day05/src/input05.txt");
    let (ranges,ids ) = match parse_lines(raw_input.lines().collect()){
        Some(n)=>n,
        _ => return
    };
    // println!("{:?} {:?}", ranges, ids);
    let part1_res = solve_part1(&ranges, &ids);
    println!("{}", part1_res);
    let part2_res = solve_part2(&ranges);
    println!("{}", part2_res);
    
}
