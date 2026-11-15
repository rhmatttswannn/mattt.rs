fn main() {
    let nilai_ujian: [i32; 3] = [80, 90, 85];

    println!("Nilai pertama: {}", nilai_ujian[0]);
    println!("Jumlah elemen: {}", nilai_ujian.len());

    let mut total = 0;

    for nilai in nilai_ujian {
        println!("Nilai ujian: {}", nilai);
        total += nilai;
    }

    println!("Total nilai: {}", total);
}