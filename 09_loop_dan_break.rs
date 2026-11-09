fn main() {
    let mut angka = 1;

    loop {
        println!("Angka: {}", angka);

        if angka >= 3 {
            break;
        }

        angka += 1;
    }

    println!("Perulangan selesai");
}