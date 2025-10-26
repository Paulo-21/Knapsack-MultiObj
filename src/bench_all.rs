use crate::local_search::pls1;
use crate::local_search2::pls2;
use crate::local_search2_perf::pls2_perf;
use crate::parser::read_file;

use std::fs::File;
use std::io::Write;
use std::time::Instant;
//use crate::local_search::pls1;
pub fn bench_all() {
    let m = 100;
    let p = 2;
    let num_instance = 0;
    let mut dd = Vec::new();
    for n in (100..=700).step_by(100) {
        let filename_dat = format!("Data/{}_items/2KP{}-TA-{}.dat", n, n, num_instance);
        let (w, v, max_cap) = read_file(filename_dat);
        let mut xx = Vec::new();
        xx.push(n);
        for pls_version in 1..=3 {
            let start = Instant::now();
            let _ = match pls_version {
                1 => pls1(m, &w, p, &v, max_cap),
                2 => pls2(m, &w, p, &v, max_cap),
                //3 => pls3(m, &w, p, &v, max_cap),
                3 => pls2_perf(m, &w, p, &v, max_cap),
                _ => panic!("PLS version non supporté"),
            };
            xx.push(start.elapsed().as_millis());
            println!("Computed in {}ms", start.elapsed().as_millis());
        }
        dd.push(xx);
    }
    let mut f = File::create("output.txt").unwrap();

    for thing in &dd {
        write!(f, "{} {} {} {}\n", thing[0], thing[1], thing[2], thing[3]).unwrap();
    }
}
