mod cli;
mod db;
mod decrypt;
mod encrypt;
mod password;

#[allow(dead_code)]
struct DataSchema {
    dataname: String,
    email: String,
    password: String,
    notes: String,
}

fn main() {
    let password = b"zeroone";
    let data = b"Helolo World";
    let (enc, salt, nonce) = encrypt::encrypt(data, password);
    let decry = decrypt::decrypt(&enc, &salt, &nonce, password).unwrap();
    println!("{}", String::from_utf8_lossy(&decry));
}
