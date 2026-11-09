fn main() {
    let angka_pertama: i32 = 10;
    let angka_kedua: i32 = 3;

    println!("Penjumlahan: {}", angka_pertama + angka_kedua);
    println!("Pengurangan: {}", angka_pertama - angka_kedua);
    println!("Perkalian: {}", angka_pertama * angka_kedua);
    println!("Pembagian bulat: {}", angka_pertama / angka_kedua);
    println!("Sisa pembagian: {}", angka_pertama % angka_kedua);

    let pembagian_desimal: f64 = 10.0 / 4.0;
    println!("Pembagian desimal: {}", pembagian_desimal);

    let harga_buku = 15000;
    let jumlah_buku = 3;
    let total_harga = harga_buku * jumlah_buku;
    println!("Total harga buku: Rp{}", total_harga);
}