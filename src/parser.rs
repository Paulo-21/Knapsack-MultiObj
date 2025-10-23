use std::fs;

pub fn read_file(filename: String) -> (Vec<u32>, Vec<Vec<u32>>, u32) {
    let mut w = Vec::new();
    let mut v = Vec::new();
    let f = fs::read_to_string(filename).unwrap();
    let mut i = 0;
    let mut max_cap = 0;
    for line in f.split('\n') {
        v.push(Vec::new());
        if line.starts_with('i') {
            let mut iter_line = line.split_whitespace();
            iter_line.next();
            let data = iter_line
                .map(|s| s.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            w.push(data[0]);
            v[i].push(data[1]);
            v[i].push(data[2]);
            i += 1;
        } else if line.starts_with('n') {
            let mut s = line.split_whitespace();
            s.next();
            let size: usize = s.next().unwrap().parse().unwrap();
            w.reserve_exact(size);
            v.reserve_exact(size);
        } else {
            if line.starts_with('W') {
                let mut iter_line = line.split_whitespace();
                iter_line.next();
                max_cap = iter_line.next().unwrap().parse().unwrap();
            }
        }
        if v.last().unwrap().is_empty() {
            v.pop();
        }
    }
    return (w, v, max_cap);
}

pub fn read_points(filename: String, p: usize) -> Vec<Vec<u32>> {
    let f = fs::read_to_string(filename).unwrap();
    let nb_pnd = f.split('\n').count();

    let mut yn = vec![vec![0; p]; nb_pnd];
    let mut i = 0;
    for (k, line) in f.split('\n').enumerate() {
        if line.is_empty() {
            yn.remove(k);
            continue;
        }
        let data = line
            .split_whitespace()
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        yn[i] = data;
        i += 1;
    }
    return yn;
}
