use std::io::{self, Write};

fn main() {
    print!("Masukkan nama Anda: ");
    io::stdout().flush().unwrap();

    let mut nama = String::new();
    io::stdin().read_line(&mut nama).unwrap();
    let nama = nama.trim();

    if !nama.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
        println!("Input tidak valid! Nama hanya boleh berisi huruf.");
        return;
    }

    println!("Halo, {}! Selamat datang di Rust.", nama);
}
