fn main() {
    let nilai = 85;
    println!("Nilai ujian: {}", nilai);

    if nilai >= 80 {
        println!("Lulus dengan baik");
    } else if nilai >= 60 {
        println!("Lulus");
    } else {
        println!("Perlu belajar lagi");
    }
}