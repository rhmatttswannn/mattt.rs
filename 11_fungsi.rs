fn main() {
    sapa("Budi");
    sapa("Siti");

    let hasil = tambah(10, 5);
    println!("Hasil penjumlahan: {}", hasil);
}

fn sapa(nama: &str) {
    println!("Halo, {}!", nama);
}

fn tambah(angka_pertama: i32, angka_kedua: i32) -> i32 {
    angka_pertama + angka_kedua
}