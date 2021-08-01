use std::fs;
use std::env;
use std::io::Read;
use std::fs::File;
use std::io::Write;
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::seq::SliceRandom;
use chrono::{DateTime};

type AesCbc = Cbc<Aes256, Pkcs7>;
const ENCRYPTED_TEXT: &str = "";
const KEY: &str = "";
const TIME: &str = "";

const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

fn read_file(path: &str) -> Option<Vec<u8>> {
    let mut file = File::open(path).unwrap();
    let metadata = fs::metadata(path).unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    match file.read(&mut buffer) {
        Ok(_) => (),
        Err(_) => return None,
    }
    return Some(buffer);
}
fn gen_ascii_chars(size: usize) -> String {
    let mut rng = &mut rand::thread_rng();
    String::from_utf8(
        BASE_STR.as_bytes()
            .choose_multiple(&mut rng, size)
            .cloned()
            .collect()
    ).unwrap()
}


fn encrypt(key: &str, data: &[u8]) -> String {
    let iv_str = gen_ascii_chars(16);
    let iv = iv_str.as_bytes();
    let cipher = AesCbc::new_var(key.as_bytes(), iv).unwrap();
    let ciphertext = cipher.encrypt_vec(data);
    let mut buffer = bytebuffer::ByteBuffer::from_bytes(iv);
    buffer.write_bytes(&ciphertext);
    base64::encode(buffer.to_bytes())
}

fn decrypt(key: &str, data: &str) -> Vec<u8> {
    let bytes = base64::decode(data).unwrap();
    let cipher = AesCbc::new_var(key.as_bytes(), &bytes[0..16]).unwrap();
    cipher.decrypt_vec(&bytes[16..]).unwrap()
}

fn save(filename: &str, data: Vec<u8>) {
    let mut file = File::create(filename).unwrap();
    file.write_all(&data).unwrap();
    file.flush().unwrap();
}

fn is_time_to_open() -> bool {
        let time_to_open = DateTime::parse_from_str(TIME, "%Y-%m-%d-%H-%M-%S-%z").unwrap();
    let result = sntpc::request("time.google.com", 123);
    if let Ok(sntpc::NtpResult {
        sec, nsec: _, roundtrip: _, offset: _,
    }) = result {
        return i64::from(sec) > time_to_open.timestamp();
    }
    return false;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        if !is_time_to_open() {
            println!("It's not the time to open");
            return;
        }
        let mut path = std::env::current_exe().unwrap();
        path.pop();
        path.push("data");
        let decrypted = decrypt(KEY, &ENCRYPTED_TEXT);
        save(&path.into_os_string().into_string().unwrap(), decrypted);
        return;
    }
    let target: &str = &args[1];
    let key: &str = &args[2];
    let data = read_file(target);
    print!("{}", encrypt(key, &data.unwrap()));
}
