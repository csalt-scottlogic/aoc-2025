use std::{collections::HashSet};

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();
    let mut map = Vec::<Junction>::new();
    let mut not_networked = HashSet::<String>::new();
    for line in lines {
        let jn = Junction::from_str(line);
        not_networked.insert(jn.tag.to_string());
        map.push(jn);
    }
    let mut links = Vec::<Link>::new();
    let jn_amt = map.len();
    for i in 0..(jn_amt - 1) {
        for j in (i + 1)..jn_amt {
            links.push(Link::from_junctions(&map[i], &map[j]));
        }
    }
    links.sort_by(|a, b| a.len.total_cmp(&b.len));
    //let mut networks = build_networks(&links);
    // networks.sort_by(|a, b| a.len().cmp(&b.len()));
    // networks.reverse();
    // for n in &networks {
    //     println!("{}", n.len());
    // }
    // let part1_answer = networks[0].len() * networks[1].len() * networks[2].len();
    // println!("{part1_answer}");
    let final_link_idx = build_networks(&links, &mut not_networked);
    let final_link = &links[final_link_idx];
    let final_box_0 = map.iter().find(|x| x.tag == final_link.tag_0).unwrap();
    let final_box_1 = map.iter().find(|x| x.tag == final_link.tag_1).unwrap();
    let result = final_box_0.x * final_box_1.x;
    println!("{result}");
}

type Network = HashSet<String>;

fn build_networks(links: &Vec<Link>, all_jns: &mut HashSet<String>) -> usize {
    let mut networks = Vec::<Network>::new();
    for i in 0..links.len() {
        let mut found_0: Option<usize> = None;
        let mut found_1: Option<usize> = None;
        let the_link = &links[i];
        for j in 0..networks.len() {
            if networks[j].contains(&the_link.tag_0) {
                found_0 = Some(j);
            }
            if networks[j].contains(&the_link.tag_1) {
                found_1 = Some(j);
            }
        }
        if let Some(tag_0_net) = found_0 {
            if let Some(tag_1_net) = found_1 {
                if tag_0_net != tag_1_net {
                    extend_hashset(&mut networks, tag_0_net, tag_1_net);
                }
            }
            else {
                networks[tag_0_net].insert(the_link.tag_1.to_string());
                all_jns.remove(&the_link.tag_1);
            }
        }
        else if let Some(tag_1_net) = found_1 {
            networks[tag_1_net].insert(the_link.tag_0.to_string());
            all_jns.remove(&the_link.tag_0);
        }
        else {
            let mut new_net = Network::new();
            new_net.insert(the_link.tag_0.to_string());
            new_net.insert(the_link.tag_1.to_string());
            all_jns.remove(&the_link.tag_0);
            all_jns.remove(&the_link.tag_1);
            networks.push(new_net);
        }
        if i > 1000 && networks.len() == 1 && all_jns.len() == 0 {
            return i;
        }
    }
    links.len() - 1
}

struct Junction {
    tag: String,
    x: f64,
    y: f64,
    z: f64,
}

impl Junction {
    fn from_str(data: &str) -> Junction {
        let parts = Vec::from_iter(data.split(","));
        let x = parts[0].parse::<i32>().unwrap();
        let y = parts[1].parse::<i32>().unwrap();
        let z = parts[2].parse::<i32>().unwrap();
        let tag = format!("X{x} Y{y} Z{z}");
        Junction { tag, x: f64::from(x), y: f64::from(y), z: f64::from(z) }
    }
}

#[derive(Debug)]
struct Link {
    tag_0: String,
    tag_1: String,
    len: f64,
}

impl Link {
    fn from_junctions(a: &Junction, b: &Junction) -> Link {
        Link {
            tag_0: a.tag.clone(),
            tag_1: b.tag.clone(),
            len: ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
        }
    }
}

fn extend_hashset(net: &mut Vec<Network>, a: usize, b: usize) {
    let (split_at, primary);
    if a < b {
        split_at = b;
        primary = a;
    }
    else {
        split_at = a;
        primary = b;
    }
    let (vec_a, vec_b) = net.split_at_mut(split_at);
    vec_a[primary].extend(vec_b[0].clone());
    net.remove(split_at);
}