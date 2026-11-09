fn main() {
    println!("Angka ganjil dari 1 sampai 6:");

    for angka in 1..=6 {
        if angka % 2 == 0 {
            continue;
        }

        println!("Angka: {}", angka);
    }

    println!("Perulangan selesai");
}