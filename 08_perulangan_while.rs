fn main() {
    let mut hitungan = 3;

    println!("Mulai hitung mundur:");

    while hitungan > 0 {
        println!("Hitungan: {}", hitungan);
        hitungan -= 1;
    }

    println!("Hitungan akhir: {}", hitungan);
    println!("Selesai!");
}