use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt").expect("works");
    let networks = extract_lan_connections(&input);
    println!("part1: {}", find_networks(&networks));
    println!("part2: {}", find_longest_network(&networks));
}

fn find_networks(networks: &HashMap<String, Vec<String>>) -> u64 {
    let mut result: HashSet<Vec<String>> = HashSet::new();

    for (x, conn_x) in networks.iter().filter(|(k, _)| k.starts_with('t')) {
        for y in conn_x {
            for z in networks[y].clone() {
                if conn_x.contains(&z) {
                    let mut con: Vec<String> = vec![x.to_string(), y.to_string(), z.to_string()];
                    con.sort();
                    result.insert(con.clone());
                }
            }
        }
    }
    result.len() as u64
}

fn find_longest_network(networks: &HashMap<String, Vec<String>>) -> String {
    let mut cliques: HashSet<String> = HashSet::new();
    for (k, v) in networks {
        bron_kerbosch(
            networks,
            &HashSet::from([k.clone()]),
            &HashSet::from_iter(v.into_iter().cloned()),
            &mut HashSet::new(),
            &mut cliques,
        )
    }
    cliques.into_iter().max_by_key(|x| x.len()).unwrap()
}

fn bron_kerbosch(
    networks: &HashMap<String, Vec<String>>,
    r: &HashSet<String>,
    p: &HashSet<String>,
    x: &mut HashSet<String>,
    o: &mut HashSet<String>,
) {
    if p.len() == 0 && x.len() == 0 {
        o.insert(r.into_iter().sorted().join(","));
        return;
    }
    let mut pc: HashSet<String> = p.into_iter().cloned().collect();
    for v in p {
        let c: HashSet<String> = HashSet::from_iter(networks[v].iter().cloned());
        let new_r: HashSet<String> = r.union(&HashSet::from([v.clone()])).cloned().collect();
        let new_p: HashSet<String> = pc.intersection(&c).cloned().collect();
        let mut new_x: HashSet<String> = x.intersection(&c).cloned().collect();
        bron_kerbosch(&networks, &new_r, &new_p, &mut new_x, o);
        pc.remove(v);
        x.insert(v.to_string());
    }
}

fn extract_lan_connections(input: &str) -> HashMap<String, Vec<String>> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let computers: Vec<String> = line.split('-').map(ToString::to_string).collect();
        result
            .entry(computers[0].clone())
            .or_insert(vec![])
            .push(computers[1].clone());
        result
            .entry(computers[1].clone())
            .or_insert(vec![])
            .push(computers[0].clone());
    }
    result
}
