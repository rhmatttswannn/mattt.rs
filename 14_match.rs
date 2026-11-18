fn main() {
    let pilihan = 2;

    println!("Pilihan menu: {}", pilihan);

    match pilihan {
        1 => println!("Membuka profil"),
        2 => println!("Membuka daftar nilai"),
        3 => println!("Keluar dari menu"),
        _ => println!("Pilihan tidak tersedia"),
    }
}