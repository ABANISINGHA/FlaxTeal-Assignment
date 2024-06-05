// Define the function f(t, y) which represents the differential equation dy/dt = 1 - t^2 + y
fn f(t: f64, y: f64) -> f64 {
    1.0 - t.powi(2) + y
}

// Define the function rk4 which implements the Runge-Kutta 4th order method
fn rk4(f: fn(f64, f64) -> f64, t0: f64, tf: f64, y0: f64, n: usize) -> f64 {
    // Compute the step size h
    let h = (tf - t0) / n as f64;
    
    // Initialize y with the initial condition y0
    let mut y = y0;
    
    // Initialize t with the initial time t0
    let mut t = t0;
    
    // Initialize a count for iterations
    let mut count = 0;
    
    // Define a small epsilon to avoid floating-point precision issues
    let eps = 0.000001;
    
    // Loop until t is less than the final time tf (considering the epsilon)
    while t < tf - eps {
        // Compute the intermediate values k1, k2, k3, k4 based on the RK4 method
        let k1 = h * f(t, y);
        let k2 = h * f(t + h / 2.0, y + k1 / 2.0);
        let k3 = h * f(t + h / 2.0, y + k2 / 2.0);
        let k4 = h * f(t + h, y + k3);
        
        // Update the value of y using the weighted sum of k1, k2, k3, and k4
        y = y + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
        
        // Increment the value of t by the step size h
        t = t + h;
        
        // Increment the iteration count
        count += 1;
        
        // Print the current iteration, time t, and value of y
        println!("{:2}th iteration: t = {:.2}  y = {:.6}", count, t, y);
    }
    
    // Return the final computed value of y
    y
}

fn main() {
    // Initial condition
    let t0 = 0.0;      // Initial time
    let y0 = 0.5;      // Value of y at t = 0
    let tf = 2.0;      // Final value of t
    let n = 10;        // Number of steps

    // Function call to solve the differential equation using RK4 method
    let y = rk4(f, t0, tf, y0, n);

    // Print the final result
    println!("The solution of the differential equation at t = 2 is y(2) = {:.6}", y);
}

