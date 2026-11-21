fn main() {
    let nama = String::from("Budi");
    println!("Sebelum dipindahkan: {}", nama);

    let pemilik_baru = nama;
    println!("Setelah dipindahkan: {}", pemilik_baru);

    let mut salinan = pemilik_baru.clone();
    salinan.push_str(" Santoso");

    println!("Teks asli: {}", pemilik_baru);
    println!("Salinan: {}", salinan);
}