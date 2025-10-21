use crate::indicator::mise_a_jour;

pub fn pls(m: usize, w: &[u32], p: usize, v: &[Vec<u32>], max_cap: u32) {
    println!("PLS1 Start");
    let mut weight_sorted_idx: Vec<usize> = (0..w.len()).collect();
    weight_sorted_idx.sort_by_key(|&i| w[i]);

    let mut parento_front = gen_init_pop(w, v, max_cap, m);
    let mut parento_aux: Vec<(Vec<bool>, Vec<u32>)> = Vec::new();
    while !parento_front.is_empty() {
        for p in parento_front.iter_mut() {
            let pprime = get_voisins(p, w, max_cap, &weight_sorted_idx);
        }
    }
    println!("{:?}", parento_front);
}
fn get_voisins(
    x: &mut (Vec<bool>, Vec<u32>),
    w: &[u32],
    max_cap: u32,
    weight_sorted_idx: &Vec<usize>,
) {
    // -> Vec<(Vec<bool>, Vec<u32>)> {
    let (take, profit) = x;
    let mut tot_weight = 0;
    for (c, l) in take.iter().zip(w) {
        tot_weight += l * (*c as u32); //Ajoute l si c:1, Branchless
    }
    let mut voisins = Vec::new();

    for (k, n) in take.iter().enumerate() {
        if *n {
            *n = false;
            for s in weight_sorted_idx {
                if *s < k && tot_weight + w[*s] <= max_cap {
                    let mut copy = take.clone();
                    voisins.push((copy,));
                }
            }
            *n = true;
        }
    }
}
fn gen_init_pop(w: &[u32], v: &[Vec<u32>], max_cap: u32, m: usize) -> Vec<(Vec<bool>, Vec<u32>)> {
    let mut pareto_front: Vec<(Vec<bool>, Vec<u32>)> = Vec::new();
    for i in 0..m {
        let q = (1. / m as f32) * i as f32;
        let greedy_sol = gen_sol_q(w, v, max_cap, q);
        mise_a_jour(&mut pareto_front, greedy_sol);
    }
    pareto_front
}
fn gen_sol_q(w: &[u32], v: &[Vec<u32>], max_cap: u32, q: f32) -> (Vec<bool>, Vec<u32>) {
    let q = 0.5;
    let obj: Vec<usize> = (0..w.len()).collect();
    let mut sol = vec![false; w.len()];
    let mut a = Vec::with_capacity(w.len());
    for ((wi, vi), xi) in w.iter().zip(v).zip(obj) {
        a.push((wi, vi, xi));
    }
    a.sort_unstable_by(|(wa, va, _), (wb, vb, _)| {
        ((q * (va[0]) as f32 + (1. - q) * va[1] as f32) / (**wa as f32))
            .partial_cmp(&((q * (vb[0]) as f32 + (1. - q) * vb[1] as f32) / (**wb as f32)))
            .unwrap()
    });
    let mut tot_weight: u32 = 0;
    let mut v2: Vec<u32> = vec![0; v[0].len()];
    for (wi, vi, xi) in a.iter().rev() {
        if **wi + tot_weight <= max_cap {
            tot_weight += **wi;
            for (k, value) in v2.iter_mut().zip(*vi) {
                *k += value;
            }
            sol[*xi] = true;
        }
    }
    (sol, v2)
}
