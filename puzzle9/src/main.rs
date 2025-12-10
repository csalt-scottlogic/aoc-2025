fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let lines = content.lines();
    let mut all_coords = Vec::<Coords>::new();
    for line in lines {
        let c = parse_coords(line);
        all_coords.push(c);
    }
    let mut all_edges = Vec::<Edge>::new();
    for i in 1..all_coords.len() {
        all_edges.push(Edge::from_coords(&all_coords[i], &all_coords[i - 1]));
    }
    all_edges.push(Edge::from_coords(&all_coords[0], &all_coords[all_coords.len() - 1]));
    let mut all_boxes = Vec::<Rect>::new();
    for i in 0..(all_coords.len() - 1) {
        for j in (i + 1)..all_coords.len() {
            let r= Rect::from_coords(&all_coords[i], &all_coords[j]);
            if !all_edges.iter().any(|e| r.intersects_edge(e)) {
                all_boxes.push(r);
            }
        }
    }
    all_boxes.sort_by(|a, b| a.area().cmp(&b.area()));
    all_boxes.reverse();
    for i in 0..5 {
        println!("{:?}", all_boxes[i]);
    }
    for i in 0..5 {
        let part2_result = all_boxes[i].area();
        println!("{part2_result}");
    }

    // let mut part1_result = 0;
    // for corner in &all_coords {
    //     println!("{corner:?}");
    //     let area = max_area(&all_coords, corner);
    //     if area > part1_result {
    //         part1_result = area;
    //     }
    // }
    // println!("{part1_result}");
}

#[derive(Debug)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    fn from_str(xt: &str, yt: &str) -> Coords {
        Coords {
            x: xt.parse().unwrap(),
            y: yt.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Edge {
    a: Coords,
    b: Coords,
}

impl Edge {
    fn from_coords(c: &Coords, d: &Coords) -> Edge {
        Edge {
            a: Coords { x: c.x.min(d.x), y: c.y.min(d.y) },
            b: Coords { x: c.x.max(d.x), y: c.y.max(d.y) }
        }
    }

    fn is_horiz(&self) -> bool {
        self.a.y == self.b.y
    }
}

#[derive(Debug)]
struct Rect {
    top_left: Coords,
    bottom_right: Coords,
}

impl Rect {
    fn from_coords(c: &Coords, d: &Coords) -> Rect {
        let left = c.x.min(d.x);
        let right= c.x.max(d.x);
        let top = c.y.min(d.y);
        let bottom = c.y.max(d.y);
        Rect {
            top_left: Coords { x: left, y: top },
            bottom_right: Coords { x: right, y: bottom },
        }
    }

    fn area(&self) -> usize {
        (1 + self.bottom_right.x - self.top_left.x) * (1 + self.bottom_right.y - self.top_left.y)
    }

    fn intersects_edge(&self, e: &Edge) -> bool {
        let result;
        if e.is_horiz() {
            result = e.a.x < self.bottom_right.x && e.b.x > self.top_left.x && e.a.y > self.top_left.y && e.a.y < self.bottom_right.y;
        }
        else {
            result = e.a.y < self.bottom_right.y && e.b.y > self.top_left.y && e.a.x > self.top_left.x && e.a.x < self.bottom_right.x;
        }
        result
    }
}

fn parse_coords(text: &str) -> Coords {
    let fields = Vec::from_iter(text.split(","));
    Coords::from_str(fields[0], fields[1])
}
