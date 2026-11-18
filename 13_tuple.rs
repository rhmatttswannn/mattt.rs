fn main() {
    let profil: (&str, i32, bool) = ("Budi", 20, true);

    println!("Mengakses dengan indeks:");
    println!("Nama: {}", profil.0);
    println!("Umur: {}", profil.1);
    println!("Aktif: {}", profil.2);

    let (nama, umur, aktif) = profil;

    println!("Mengakses dengan destructuring:");
    println!("Nama: {}", nama);
    println!("Umur: {}", umur);
    println!("Aktif: {}", aktif);
}