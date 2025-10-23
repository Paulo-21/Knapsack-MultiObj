use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod indicator;
mod local_search;
mod local_search2;
mod parser;
mod utils;

use local_search::pls1;
use local_search2::pls2;
use parser::{read_file, read_points};
use std::{env, time::Instant};
use utils::plot_points;

fn main() {
    println!("TME enssemble non-dominé");
    let num_instance = 9;
    let n = 100;
    let p = 2;
    let pls_version = 2;
    let mut save = false;

    if env::args().len() > 1 {
        save = true;
        println!("Plot mode");
    }

    let filename_dat = format!("Data/{}_items/2KP{}-TA-{}.dat", n, n, num_instance);
    let (w, v, max_cap) = read_file(filename_dat);

    let m = 100;
    let start = Instant::now();
    let approx_yn = match pls_version {
        1 => pls1(m, &w, p, &v, max_cap),
        2 => pls2(m, &w, p, &v, max_cap),
        //3 => pls3(m, &w, p, &v, max_cap),
        _ => panic!("PLS version non supporté"),
    };
    println!("Computed in {}ms", start.elapsed().as_millis());
    // Lecture des points non dominés
    if save {
        let filename_eff = format!("Data/{}_items/2KP{}-TA-{}.eff", n, n, num_instance);
        let yn = read_points(filename_eff, p);
        let approx_pt: Vec<Vec<u32>> = approx_yn.iter().map(|(_, y)| y.clone()).collect();
        plot_points(&yn, &approx_pt, "points.png").expect("RE");
    } else {
        println!();
        println!("Pareto size {}", approx_yn.len());
    }
}
