use rand;
use std::thread;

const THREADS:u64 = 12;
const N:u64 = 100_000_000_000; 


fn rand_points(x:f64, y:f64) -> [f64; 2] {
    [
        rand::random_range(0.0..=x),
        rand::random_range(0.0..=y)
    ]
}

fn dis(x:f64, y:f64) -> f64 {
    (x.powi(2) + y.powi(2)).sqrt()
}

fn task(n:u64) -> [u64; 2] {
    let mut total  = 0;
    let mut dentro = 0;

    for _ in 0..n {
    
        let [x, y] = rand_points(1.0, 1.0);
    
        if dis(x,y) <= 1.0 { dentro += 1 }
        total += 1;
    
    }

    [total, dentro]
}

fn main() {
    
    let n = N;

    let div = n / THREADS;
    let sob = n % THREADS;

    let mut threads = Vec::new();

    for id in 0..THREADS {
        let quant = div + ( if id < sob {1} else {0} );

        let t = thread::spawn(move || {
            task(quant)
        });

        threads.push(t);
    }

    let mut total  = 0;
    let mut dentro = 0;

    for c in threads {
        let a:[u64; 2] = c.join().unwrap();

        total  += a[0];
        dentro += a[1];
    }


    let pi = 4.0 * (dentro as f64 / total as f64);

    println!("Dentro:\t{dentro}");
    println!("Fora:\t{}", total - dentro);
    println!("Total:\t{total}");
    println!("Pi:\t{pi}");

}
