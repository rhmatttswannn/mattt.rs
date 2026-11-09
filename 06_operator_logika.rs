fn main() {
    let umur = 20;
    let punya_identitas = true;
    let boleh_mendaftar = umur >= 17 && punya_identitas;

    println!("Boleh mendaftar: {}", boleh_mendaftar);

    if boleh_mendaftar {
        println!("Syarat pendaftaran terpenuhi");
    } else {
        println!("Syarat pendaftaran belum terpenuhi");
    }

    let punya_tiket = false;
    let punya_undangan = true;
    let boleh_masuk = punya_tiket || punya_undangan;
    println!("Boleh masuk: {}", boleh_masuk);

    let sedang_hujan = false;
    let tidak_hujan = !sedang_hujan;
    println!("Tidak hujan: {}", tidak_hujan);
}