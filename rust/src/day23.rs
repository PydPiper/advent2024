use std::fs::{read, File};
use std::io::{self, BufRead, Read};
use std::collections::{HashMap, HashSet};

/*
given 2-pairs of IDs
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn

find unique 3 set connections
aq,cg,yn
aq,vc,wq
co,de,ka
co,de,ta
co,ka,ta
de,ka,ta
kh,qp,ub
qp,td,wh
tb,vc,wq
tc,td,wh
td,wh,yn
ub,vc,wq

but only count 3-connection if it contains an ID starting with "t", so the above being 7
co,de,ta
co,ka,ta
de,ka,ta
qp,td,wh
tb,vc,wq
tc,td,wh
td,wh,yn

store it in a hashmap
key: connections

loop of each starting "t" key 
    loop key's children (kc)
        loop kc's children to create 3-set
    add key to visited to prevent backward same match finding between starting t keys
*/

fn read_data(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("unable to open file");
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().map(|line|line.expect("unable to parse line")).collect();
    lines
}

pub fn part1(filename: &str) {
    let lines = read_data(&filename);

    let mut connections: HashMap<&str,HashSet<&str>> = HashMap::new();
    let mut visited: HashSet<&str> = HashSet::new();

    for line in lines.iter() {
        let line_split: Vec<&str> = line.split("-").collect();
        let val1 = line_split[0];
        let val2 = line_split[1];
        // val1-2
        let val = connections.entry(val1).or_insert(HashSet::from([val2]));
        val.insert(val2);
        // val2-1
        let val = connections.entry(val2).or_insert(HashSet::from([val1]));
        val.insert(val1);
    }

    // println!("{:?}", connections);

    // this would be the answer if they were asking for all connection (allow re-use)
    // let mut three_sets: Vec<String> = vec![];
    // for key in connections.keys() {
    //     if key.starts_with('t') && !visited.contains(key){
    //         // add it to used to prevent backward dup between t keys
    //         visited.insert(&key);
    //         let mut local_visted: HashSet<&str> = HashSet::new();
    //         for key_child in connections.get(key).unwrap() {
    //             local_visted.insert(key_child);
    //             if !visited.contains(key_child) {
    //                 for key_child_child in connections.get(key_child).unwrap() {
    //                     if !visited.contains(key_child_child) && !local_visted.contains(key_child_child) {
    //                         let str_set = [*key, key_child, key_child_child].join(",");
    //                         three_sets.push(str_set);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // this would be the answer if they were looking for non-reuse edges
    // let mut three_sets: Vec<String> = vec![];
    // // you have to pull out a clone of the keys because Rust doesnt allow dup lock that is immutable and mutable for it's val (set manipulation)
    // let keys: Vec<&str> = connections.keys().cloned().collect();
    // for key in keys {
    //     if key.starts_with('t') && !visited.contains(key){
    //         // add it to used to prevent backward dup between t keys
    //         visited.insert(&key);

    //         let child_keys = connections.get(key).unwrap().clone();
    //         for child in child_keys.iter() {
    //             // remove connection from key to child
    //             let mut keyset = connections.get_mut(key).unwrap();
    //             keyset.remove(child);
    //             // remove connection from child to key
    //             let mut sub_keyset = connections.get_mut(child).unwrap();
    //             sub_keyset.remove(key);

    //             let sub_child_keys = sub_keyset.clone();
    //             for child_child in sub_child_keys.iter(){
    //                 // remove connection from child to cc
    //                 let mut sub_keyset = connections.get_mut(child).unwrap();
    //                 sub_keyset.remove(child_child);
    //                 // remove connection from cc to child
    //                 let mut sub_keyset = connections.get_mut(child_child).unwrap();
    //                 sub_keyset.remove(child);

    //                 let str_set = [key, child, child_child].join(",");
    //                 three_sets.push(str_set);
    //                 break
    //             }
    //         }
    //     }
    // }


    // problem is asking to find cycles of size 3
    let mut three_sets: Vec<String> = vec![];
    // you have to pull out a clone of the keys because Rust doesnt allow dup lock that is immutable and mutable for it's val (set manipulation)
    for key in connections.keys() {
        if key.starts_with('t') {
            visited.insert(key);

            let mut local_visited: Vec<&str> = Vec::new();
            for child in connections.get(key).unwrap() {
                if !visited.contains(child){
                    for child_child in connections.get(child).unwrap(){
                        if !local_visited.contains(child_child) && !visited.contains(child_child) {
                            // if child_child links to key then thats a cycle of 3
                            if connections.get(child_child).unwrap().contains(key) {
                                let str_set = [*key, *child, *child_child].join(",");
                                three_sets.push(str_set);
                                local_visited.push(child);
                            }
                        }
                    }
                }
            }
        }
    }

    // println!("{:?}", three_sets);
    println!("Part 23.1 = {:?}", three_sets.len());

}

pub fn part2(filename: &str) {
    let lines = read_data(&filename);

    let mut connections: HashMap<&str,HashSet<&str>> = HashMap::new();
    let mut visited: HashSet<&str> = HashSet::new();

    for line in lines.iter() {
        let line_split: Vec<&str> = line.split("-").collect();
        let val1 = line_split[0];
        let val2 = line_split[1];
        // val1-2
        let val = connections.entry(val1).or_insert(HashSet::from([val2]));
        val.insert(val2);
        // val2-1
        let val = connections.entry(val2).or_insert(HashSet::from([val1]));
        val.insert(val1);
    }

    let mut max_cycle = 0;
    for parent in connections.keys() {
        let mut i_cycle = 1;
        let mut node = parent;
        let mut unused_size = 1;



        for node in connections.get(node).unwrap() {
            let children = connections.get(node).unwrap();
            unused_size = visited.difference(children).collect::<Vec<_>>().len();
            // if unused_size != 0 then do these lines again but where node is the 0th child
            if unused_size != 0 {
                // go deeper
            }
        }

        while unused_size != 0 {
            
            unused_size = visited.difference(connections.get(node).unwrap()).collect::<Vec<_>>().len();
        }





        while !visited.contains(node) {
            visited.insert(node);
            i_cycle += 1;
            for child in connections.get(node).unwrap() {
                if !visited.contains(child) {
                    visited.insert(child);
                    node = child;
                    break
                }
                else {

                }


                if connections.get(child).unwrap().contains(parent) {
                    // cycle
                    if i_cycle > max_cycle {
                        max_cycle = i_cycle;
                    }
                }
            }
            node = 
        }
    }

}

fn depth(connections){
    for node in connections.get(node).unwrap() {
        let children = connections.get(node).unwrap();
        unused_size = visited.difference(children).collect::<Vec<_>>().len();
        // if unused_size != 0 then do these lines again but where node is the 0th child
        if unused_size != 0 {
            // go deeper
        }
    }
}