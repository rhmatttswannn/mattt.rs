fn main() {
    println!("Menghitung dari 1 sampai 5:");

    for angka in 1..=5 {
        println!("Angka: {}", angka);
    }

    let mut total = 0;

    for angka in 1..=5 {
        total += angka;
    }

    println!("Total penjumlahan 1 sampai 5: {}", total);
}