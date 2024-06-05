# FlaxTeal-Assignment


# Runge-Kutta 4th Order Method in Rust

This repository contains a Rust implementation of the Runge-Kutta 4th order method for solving ordinary differential equations (ODEs). The specific ODE example is solved:

dy/dt = 1 - t^2 + y, y(0) = 0.5 and n = 10

## Prerequisites

To compile and run the Rust code, you need to have the Rust toolchain installed on your system. If you do not have Rust installed, you can download it from [rust-lang.org](https://www.rust-lang.org/).

You can install Rust using `rustup` by running the following command: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs`

Clone the Repository:
  ```sh
  git clone https://github.com/ABANISINGHA/FlaxTeal-Assignment.git
  cd FlaxTeal-Assignment
```

Create a new Cargo project:
```sh
cargo new rk4_rust
cd rk4_rust
```

Replace the contents of RK4.rs with the provided code.

Use Cargo to build and run the project:
```sh
cargo run
```
## Otherwise you can use online compiler [GDB](https://www.onlinegdb.com/online_rust_compiler), the GNU Project debugger to write and debugg.

# Burgers' Equation Solver

This repository contains a Python script to solve the 1D Burgers' equation using the Runge-Kutta method (RK4). The script initializes the velocity field, computes the solution over time, and plots the results.

## Requirements

- Python 3.x
- NumPy
- Matplotlib

## Installation

To install the required libraries, you can use `pip`:

```bash
pip install numpy matplotlib
```

Clone the repository
```bash
git clone https://github.com/ABANISINGHA/FlaxTeal-Assignment.git
cd FlaxTeal-Assignment
```


Run the script:

```bash
Burger's eq.py
```

Discritization of the Burger's equation and application of RK4 on that is described here in **Burger_s_equation_solving_by_Rk4_method.pdf**.

The output for the script is the value of the velocity u for each time step and plot over time steps. Theses you can get in this file **Burger's eq.ipynb**

Additional details for solve Burger's equation using RK4 you get  [ here](https://www.sciencedirect.com/science/article/pii/S2405844022010647).













 
