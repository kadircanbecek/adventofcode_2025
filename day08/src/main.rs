use std::{collections::HashSet, fs};
pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("input")
}

fn parse_line(line: &str) -> Option<(i128, i128, i128)>{
    let parts: Vec<&str> = line.split(",").collect();

    if parts.len()!=3{
        return None;
    }

    let num1:i128 = match parts[0].parse(){
        Ok(n) => n,
        Err(_) => {return None;}
    };
    let num2:i128 = match parts[1].parse(){
        Ok(n) => n,
        Err(_) => {return None;}
    };
    let num3:i128 = match parts[2].parse(){
        Ok(n) => n,
        Err(_) => {return None;}
    };

    Some((num1,num2,num3))
}
fn calc_dists(coords: &Vec<(i128,i128,i128)>) -> Vec<(usize, usize, i128)>{
    let mut dists:Vec<(usize, usize, i128)> = Vec::new(); 
    for i in 0..coords.len(){
        for j in i+1..coords.len(){
            let coord1 = coords[i];
            let coord2 = coords[j];
            let dist = (coord1.0 - coord2.0).pow(2) + (coord1.1 - coord2.1).pow(2) +(coord1.2 - coord2.2).pow(2);
            dists.push((i,j,dist));
        }
    }
    dists.sort_by_key(|k| k.2);
    dists
}
fn solve_part1(dists: &Vec<(usize, usize, i128)>, conns:usize){
    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    for dist in &dists[0..conns] {
        let mut found = false;
        for cc in &mut circuits {
            if cc.contains(&dist.0) || cc.contains(&dist.1){
                cc.insert(dist.0);
                cc.insert(dist.1);
                found = true;
                break;
            } 
        }
        if !found {
            let mut cc: HashSet<usize> = HashSet::new();
            cc.insert(dist.0);
            cc.insert(dist.1);
            circuits.push(cc);
        }
    }

    let mut i = 0;
    let mut cl = circuits.len();
    while i < cl {
        let mut j = i + 1;
        let mut merged = false;
        while j < circuits.len() {
            if !circuits[i].is_disjoint(&circuits[j]) {
                let to_merge = circuits.remove(j);
                circuits[i].extend(to_merge);
                merged = true;
            } else {
                j += 1;
            }
        }
        if merged {
            i = 0;  // Restart from the beginning after a merge
            cl = circuits.len();
        } else {
            i += 1;
        }
    }
    // println!("{:?}", circuits);
    // println!("{:?}", circuits.len());
    circuits.sort_by_key(|c| c.len());
    let result: usize = circuits.iter().rev().take(3).map(|k| k.len()).product();

    println!("{}", result);

}


fn solve_part2(dists: &Vec<(usize, usize, i128)>, coords: &Vec<(i128,i128,i128)>){
    let mut last_con = 0;
    let mut in_conn : HashSet<usize> = HashSet::new();

    for dist in dists{
        if !in_conn.contains(&dist.0) || !in_conn.contains(&dist.1){
            in_conn.insert(dist.0);
            in_conn.insert(dist.1);
            last_con=coords[dist.0].0*coords[dist.1].0;
        } 

    }

    println!("{}", last_con);

}

fn main() {
    let raw_input = read_input("day08/src/input08.txt");
    let coords: Vec<(i128, i128, i128)> = raw_input.lines().filter_map(parse_line).collect();
    let dists = calc_dists(&coords);
    solve_part1(&dists,1000);
    solve_part2(&dists, &coords);
}
