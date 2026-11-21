fn main() {
    let nama: &str = "Budi";
    let mut sapaan: String = String::from("Halo");

    println!("Sebelum: {}", sapaan);

    sapaan.push_str(", ");
    sapaan.push_str(nama);
    sapaan.push('!');

    println!("Sesudah: {}", sapaan);
}