use std::u32;

pub fn multi_obj_progdyn(
    w: &[u32],
    v: &[Vec<u32>],
    max_cap: u32,
) -> Vec<(Vec<bool>, Vec<u32>, u32)> {
    let mut last: Vec<Vec<(Vec<bool>, Vec<u32>, u32)>> =
        vec![vec![(vec![false; w.len()], vec![0; 2], 0); 1]; w.len()];
    let mut actual: Vec<Vec<(Vec<bool>, Vec<u32>, u32)>> =
        vec![vec![(vec![false; w.len()], vec![0; 2], 0); 1]; w.len()];

    let mut k = 0;
    while k < w.len() {
        let mut updated_once = false;
        for j in 0..w.len() {
            if j < k {
                continue;
            }
            if j > 0 {
                actual[j] = actual[j - 1].clone();
            }
            let idx = j - ((j > 0) as usize * 1);
            for sol in last[idx].iter_mut() {
                if sol.2 + w[j] <= max_cap {
                    sol.0[j] = true;
                    sol.1[0] += v[j][0];
                    sol.1[1] += v[j][1];
                    updated_once |= mise_a_jour2(&mut actual[j], &sol);
                    sol.0[j] = false;
                    sol.1[0] -= v[j][0];
                    sol.1[1] -= v[j][1];
                }
            }
            //print!("K : {k},{j} ");
            //display(&actual[j]);
        }
        k += 1;
        if !updated_once || k > w.len() {
            break;
        }
        //println!("--------------------------------------------------------------");
        //println!();
        last = actual;
        actual = vec![vec![(vec![false; w.len()], vec![0; 2], 0); 1]; w.len()];
    }
    return last.pop().unwrap();
}
fn mise_a_jour2(
    pareto_front: &mut Vec<(Vec<bool>, Vec<u32>, u32)>,
    xpt: &(Vec<bool>, Vec<u32>, u32),
) -> bool {
    let x = [xpt.1[0], xpt.1[1]];
    let mut k = match pareto_front.binary_search_by(|pt| pt.1[0].cmp(&x[0])) {
        Ok(j) => j,
        Err(index) => index,
    };

    if k < pareto_front.len() {
        if x[0] == pareto_front[k].1[0] {
            if x[1] > pareto_front[k].1[1] {
                //Changer si meilleur que pareil
                pareto_front[k] = xpt.clone();
            } else {
                return false;
            }
        } else if x[0] < pareto_front[k].1[0] {
            if x[1] > pareto_front[k].1[1] {
                pareto_front.insert(k, xpt.clone());
            } else {
                return false;
            }
        }
    } else {
        pareto_front.insert(k, xpt.clone());
    }

    let mut done = k > 0;
    k -= done as usize;
    while done {
        if pareto_front[k].1[1] <= x[1] {
            pareto_front.remove(k);
        } else {
            break;
        }
        done = k > 0;
        k -= done as usize;
    }
    true
}

pub fn test() {
    let v: Vec<Vec<u32>> = Vec::from([
        Vec::from([1, 4]),
        Vec::from([2, 3]),
        Vec::from([5, 2]),
        Vec::from([2, 2]),
        Vec::from([3, 1]),
        Vec::from([2, 5]),
        Vec::from([3, 4]),
    ]);
    let w = vec![0; v.len()];
    let pareto = multi_obj_progdyn(&w, &v, u32::MAX);
    println!("{:?}", pareto);
}

fn display(l: &Vec<(Vec<bool>, Vec<u32>, u32)>) {
    for k in l {
        print!("{:?} ", k.1);
    }
    println!();
}
