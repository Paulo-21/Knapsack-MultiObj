use crate::local_search::mise_a_jour;

pub fn pls2(
    m: usize,
    w: &[u32],
    p: usize,
    v: &[Vec<u32>],
    max_cap: u32,
) -> Vec<(Vec<bool>, Vec<u32>)> {
    println!("PLS1 Start");
    let mut weight_sorted_idx: Vec<usize> = (0..w.len()).collect();
    weight_sorted_idx.sort_by_key(|&i| w[i]);

    let mut pareto_front = gen_init_pop(w, v, max_cap, m);
    pareto_front.sort_by(|a, b| a.1[0].cmp(&b.1[0]));

    let mut pop = pareto_front.clone();
    let mut pop_aux: Vec<(Vec<bool>, Vec<u32>)> = Vec::new();
    println!("Start with {} pt", pop.len());
    while !pop.is_empty() {
        for p in pop.iter_mut() {
            let all_pprime = get_voisins(p, w, v, max_cap, &weight_sorted_idx);
            for pp in all_pprime {
                if !(p.1[0] >= pp.1[0] && p.1[1] >= pp.1[1]) {
                    if mise_a_jour2(&mut pareto_front, pp.clone()) {
                        mise_a_jour(&mut pop_aux, pp);
                    }
                }
            }
        }

        println!("{}", pop_aux.len());
        //println!("{}", pop_aux.len());
        std::mem::swap(&mut pop, &mut pop_aux);
        pop_aux.clear();
    }
    //println!("{:?}", pareto_front.len());
    return pareto_front;
}
fn get_voisins(
    x: &mut (Vec<bool>, Vec<u32>),
    w: &[u32],
    v: &[Vec<u32>],
    max_cap: u32,
    weight_sorted_idx: &Vec<usize>,
) -> Vec<(Vec<bool>, Vec<u32>)> {
    let (take, profit) = x;
    let mut tot_weight = 0;
    for (ti, wi) in take.iter().zip(w) {
        if *ti {
            tot_weight += wi;
        }
    }
    let mut voisins = Vec::new();

    for k in 0..take.len() {
        if take[k] {
            take[k] = false;
            profit[0] -= v[k][0];
            profit[1] -= v[k][1];
            tot_weight -= w[k];
            for s in weight_sorted_idx {
                if *s != k && !take[*s] && tot_weight + w[*s] <= max_cap {
                    profit[0] += v[*s][0];
                    profit[1] += v[*s][1];
                    take[*s] = true;
                    voisins.push((take.clone(), profit.clone()));
                    take[*s] = false;
                    profit[0] -= v[*s][0];
                    profit[1] -= v[*s][1];
                } else if tot_weight + w[*s] > max_cap {
                    break;
                }
            }
            take[k] = true;
            profit[0] += v[k][0];
            profit[1] += v[k][1];
            tot_weight += w[k];
        }
    }
    voisins
}
pub fn mise_a_jour2(
    pareto_front: &mut Vec<(Vec<bool>, Vec<u32>)>,
    x: (Vec<bool>, Vec<u32>),
) -> bool {
    let mut updated = pareto_front.is_empty();
    let mut indices = Vec::new();
    let mut index = match pareto_front.binary_search_by(|pt| pt.1[0].cmp(&x.1[0])) {
        Ok(_) => return false,
        Err(index) => index,
    };
    let done = false;
    while !done {
        if pareto_front[index].1[1] >= x.1[1] {
            return false;
        } else {
            updated = true;
            if pareto_front[index].1[0] <= x.1[0] && pareto_front[index].1[1] <= x.1[1] {
                //indices.push(k);
            }
        }
    }
    for (k, xp) in pareto_front.iter_mut().enumerate() {
        if xp.1[0] >= x.1[0] && xp.1[1] >= x.1[1] {
            return false;
        } else {
            updated = true;
            if xp.1[0] <= x.1[0] && xp.1[1] <= x.1[1] {
                indices.push(k);
            }
        }
    }
    if updated {
        for (i, idx) in indices.iter().enumerate() {
            pareto_front.remove(idx - i);
        }
        pareto_front.push(x);
    }
    updated
}
fn gen_init_pop(w: &[u32], v: &[Vec<u32>], max_cap: u32, m: usize) -> Vec<(Vec<bool>, Vec<u32>)> {
    let mut pareto_front: Vec<(Vec<bool>, Vec<u32>)> = Vec::new();
    for i in 0..=m {
        let q = (1. / m as f32) * i as f32;
        let greedy_sol = gen_sol_q(w, v, max_cap, q);
        mise_a_jour(&mut pareto_front, greedy_sol);
    }
    pareto_front
}
fn gen_sol_q(w: &[u32], v: &[Vec<u32>], max_cap: u32, q: f32) -> (Vec<bool>, Vec<u32>) {
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
