/* Eliminação de Gauss sem pivô */

use cpu_time::ProcessTime;
use rand::{thread_rng, Rng};
use std::env;
use std::time::Instant;

fn get_random_matrix(size: usize) -> Vec<Vec<f32>> {
    let mut mtx: Vec<Vec<f32>> = Vec::new();
    let mut rd = thread_rng();

    for _ in 0..size {
        let mut m: Vec<f32> = Vec::new();

        for _ in 0..size {
            // m.push(rd.gen_range(-10e5..10e5) as f32);
            m.push(rd.gen_range(-10..10) as f32);
        }

        mtx.push(m);
    }

    return mtx;
}

fn get_random_vec(size: usize) -> Vec<f32> {
    let mut arr: Vec<f32> = Vec::new();
    let mut rd = thread_rng();

    for _ in 0..size {
        // arr.push(rd.gen_range(-10e5..10e5) as f32);
        arr.push(rd.gen_range(-10..10) as f32);
    }

    return arr;
}

fn prints(arr_a: Vec<Vec<f32>>, arr_b: Vec<f32>) {
    if arr_b.len() <= 10 {
        /***** MOSTRA A MATRIX A *****/
        let size = arr_a.len();

        println!("A = ");

        for i in 0..size {
            print!("\t");
            for j in 0..size {
                print!(
                    "{:10.2}{} ",
                    arr_a[i][j],
                    if j == size - 1 { ";" } else { "," }
                );
            }
            println!("");
        }

        /***** MOSTRA A MATRIX B *****/

        print!("\nB = \t");

        for i in 0..size {
            print!(
                "{:10.2}{} ",
                arr_b[i],
                if i != size - 1 { "," } else { ";" }
            );
        }

        println!("\n");
    }
}

fn print_x(arr: Vec<f32>) {
    /***** MOSTRA A MATRIX X *****/

    if arr.len() <= 10 {
        let size = arr.len();

        print!("\n\nX = \t");

        for i in 0..size {
            print!("{:5.4}{} ", arr[i], if i != size - 1 { "," } else { ";" });
        }

        println!("\n");
    }
}

fn parse_config() -> usize {
    // lê o tamanho da matrix informado pelo terminal
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("\nUso: 'cargo run . <dimensão_matriz>'");
        std::process::exit(0)
    }

    let size = *(&args[1].parse::<usize>().unwrap());

    if size < 1 {
        println!("\nErro: Informe um tamanho válido para a matriz ( > 0)!");
        std::process::exit(0)
    }

    return size;
}

fn main() {
    let size = parse_config();

    println!("Initializing...");
    let mut a: Vec<Vec<f32>> = get_random_matrix(size);
    let mut b: Vec<f32> = get_random_vec(size);
    let mut x: Vec<f32> = vec![0.0; size];

    prints(a.clone(), b.clone());

    println!("Starting clock.");

    let start_cpu = ProcessTime::now();
    let start_time = Instant::now();

    gauss(&mut a, &mut b, &mut x, size); /**** GAUSS ****/

    let cpu = start_cpu.elapsed();
    let end_time = start_time.elapsed();

    println!("Stopped clock.");

    print_x(x.clone());

    println!("Elapsed time: {:?}", end_time);
    println!("CPU time: {:?}", cpu);
}

fn gauss(a: &mut Vec<Vec<f32>>, b: &mut Vec<f32>, x: &mut Vec<f32>, size: usize) {
    let mut multiplier: f32;

    for norm in 0..(size - 1) {
        for row in (norm + 1)..size {
            multiplier = a[row][norm] / a[norm][norm];

            for col in norm..size {
                a[row][col] -= a[norm][col] * multiplier;
            }

            b[row] -= b[norm] * multiplier;
        }
    }

    for row in (0..=size - 1).rev() {
        x[row] = b[row];

        for col in (row + 1..=size - 1).rev() {
            x[row] -= a[row][col] * x[col];
        }

        x[row] /= a[row][row];
    }
}
