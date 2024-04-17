// Define the function f(t, y)
fn f(t: f64, y: f64) -> f64 {
    1.0 - t.powi(2) + y
}

// Define the RK4 function
fn rk4(f: fn(f64, f64) -> f64, t0: f64, tf: f64, n: usize) -> f64 {
    let h = (tf - t0) / n as f64;
    let mut y = 0.5; // Initial value of y
    let mut t = t0;
    let mut count = 0;
    let eps = 0.000001; // Consider epsilon to prevent the precision issues in floating-point arithmetic
    while t < tf - eps {
        let k1 = h * f(t, y);
        let k2 = h * f(t + h / 2.0, y + k1 / 2.0);
        let k3 = h * f(t + h / 2.0, y + k2 / 2.0);
        let k4 = h * f(t + h, y + k3);
        y = y + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
        t = t + h;
        count += 1;
        println!("{:?}th iteration: t = {:.2}, y = {:.6}", count, t, y);
    }
    y
}

fn main() {
    let t0 = 0.0; // Initial time
    let tf = 2.0; // Final value of t
    let n = 10;
    let y = rk4(f, t0, tf, n);
    println!("The solution of the differential equation at t = 2 is y(2) = {:.6}", y);
}
