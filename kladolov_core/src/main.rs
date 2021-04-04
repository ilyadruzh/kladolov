#![allow(unused)]

extern crate bitcoin;
extern crate num_bigint;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate time;

use bitcoin::network::constants::Network;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::address::Address;
use bitcoin::util::key::PrivateKey;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufRead, Read};
use std::path::Path;
use std::str::FromStr;
// use std::{concat, env, process};
use time::Instant;
use zip::write::FileOptions;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressResponse {
    pub final_balance: i64,
    pub total_received: i64,
}

fn main() {
    // run_create_archives();
}

fn has_balance(addr: String) -> bool {
    // сделать запрос к локальному узлу
    let mut response = reqwest::get(&format!("https://blockchain.info/rawaddr/{}", &addr)).unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    if response.status() != 200 {
        return false;
    }

    let result: AddressResponse = serde_json::from_str(&body).unwrap();
    //println!("{:?}", result);
    result.final_balance > 0 || result.total_received > 0
}

fn write_new_line(addr: String, private_key: PrivateKey, has: bool) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("klad")
        .unwrap();

    if let Err(e) = writeln!(
        file,
        "address: {} with private_key: {} has: {}",
        addr, private_key.key, has
    ) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn create_secp256k1_secret_key_list() {
    let start = Instant::now();

    // проверить файлы из папки db и найти последний по порядку файл
    // создать файл с следующим по номер
    let paths = std::fs::read_dir("./db").unwrap();
    let path_count = &paths.count();
    let new_path = format!("./db/{}", (path_count + 1).to_string()); // проверка на отсутсвие такого имени
                                                                     // let mut file_name = File::create(Path::new(&new_path)).unwrap();

    let secp = Secp256k1::new();
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(new_path)
        .unwrap();

    let mut index = 0;

    while index < 1_000_000 {
        // поработать над этим рандомом, сделать настраиваемым, чтобы можно было ключевые слова указывать
        let (sec_key, _) = secp.generate_keypair(&mut bitcoin::secp256k1::rand::thread_rng());

        // записывать не в новую линию каждый ключ, а последовательно в одну большую строку
        if let Err(e) = writeln!(file, "{}", sec_key) {
            eprintln!("Couldn't write to file: {}", e);
        }

        index += 1;
    }
    println!(
        "{:?} время создания списка",
        start.elapsed().as_seconds_f32()
    ); // перевести в минуты
}

fn run_check() {
    let start = Instant::now();
    let paths = std::fs::read_dir("./db").unwrap();
    let path_count = &paths.count(); // проверка что такой файл есть
    let mut count = *path_count as i32;

    while count > 0 {
        let start2 = Instant::now();
        let checked_path = format!("./db/{}", count.to_string());
        if (Path::new(&checked_path).exists()) {
            println!("проверка спиcка: {}", &checked_path);
            let file_name = Path::new(&checked_path);
            check_secp256k1_secret_key_list(file_name);
            println!(
                "{:?} сек на проверку списка {}",
                start2.elapsed().as_seconds_f32(),
                &checked_path
            );
        }
        count -= 1;
    }
    println!(
        "{:?} сек на проверку всех списков",
        start.elapsed().as_seconds_f32()
    ); // перевести в минуты
}

fn run_create_archives() {
    let start = Instant::now();
    let paths = std::fs::read_dir("./db").unwrap();
    let path_count = &paths.count(); // проверка что такой файл есть
    let mut count = *path_count as i32;
    while count > 0 {
        let start2 = Instant::now();

        let checked_path = format!("./db/{}", count.to_string());
        archive_secret_keys_list(&checked_path[..]);
        println!(
            "{:?} сек на создание архива {}",
            start2.elapsed().as_seconds_f32(),
            &checked_path
        );
        count -= 1;
    }
    println!(
        "{:?} сек на создание всех архивов",
        start.elapsed().as_seconds_f32()
    ); // перевести в минуты
}

fn check_secp256k1_secret_key_list<T: AsRef<Path>>(filename: T) {
    // в несколько потоков разные файлы, если количество строк в них == 1_000_000
    // let start = Instant::now();
    let secp = Secp256k1::new();
    let lines = io::BufReader::new(File::open(filename).unwrap()).lines();
    let mut count: i32 = 0;
    // сделать возможность проверки одной больщой строки со сдивгом нового ключа на 1 элемент
    for line in lines {
        count += 1;
        match &line {
            Ok(ln) => {
                let private_key = bitcoin::util::key::PrivateKey {
                    compressed: true,
                    network: Network::Bitcoin,
                    key: secp256k1::SecretKey::from_str(&ln[..]).expect("wrong secret key"),
                };
                let pk = bitcoin::PublicKey::from_private_key(&secp, &private_key);
                let _address = Address::p2pkh(&pk, Network::Bitcoin);
                println!("check: {}", count);
                match has_balance(_address.to_string()) {
                    false => (),
                    true => write_new_line(_address.to_string(), private_key, true),
                }
            }
            Err(e) => println!("some error: {}", e),
        }
    }
}

fn check_correctnes<T: secp256k1::Context + bitcoin::secp256k1::Signing>(
    secp: Secp256k1<T>,
    pub_key: secp256k1::PublicKey,
    sec_key: secp256k1::SecretKey,
    public_key: bitcoin::util::key::PublicKey,
    private_key: bitcoin::util::key::PrivateKey,
) {
    assert_eq!(
        pub_key,
        secp256k1::key::PublicKey::from_secret_key(&secp, &sec_key)
    );

    assert_eq!(
        public_key,
        bitcoin::util::key::PublicKey::from_private_key(&secp, &private_key)
    );
}

fn lines_in_file<T: AsRef<Path>>(filename: T) -> usize {
    let lines = io::BufReader::new(File::open(filename).unwrap()).lines();
    lines.count()

    // проверить производительность этого куска кода
    // let mut count: i32 = 0;
    // for line in lines {
    //     count = count + 1;
    // }
    //
}

// создание архива файла со списком ключей
fn archive_secret_keys_list(filename: &str) -> zip::result::ZipResult<()> {
    let path = std::path::Path::new(filename);
    let new_path = Path::new("archive")
        .join(
            path.file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap(),
        )
        .with_extension("zip");
    let new_file = std::fs::File::create(&new_path).unwrap();
    let mut zip = zip::ZipWriter::new(&new_file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Bzip2)
        .unix_permissions(0o755);

    //  захардкожено
    match zip.start_file(
        path.file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap(),
        options,
    ) {
        Ok(res) => (),
        Err(e) => println!("err:{}", e),
    };

    // let lines = io::BufReader::new(File::open(&path).unwrap()).lines();
    let mut file_content = Vec::new();
    let mut file = File::open(&path).unwrap();
    file.read_to_end(&mut file_content).expect("Unable to read");
    match zip.write_all(&file_content) {
        Ok(_) => println!("archive is ready"),
        Err(e) => println!("something went wrong with archive: {}", e),
    };

    zip.finish()?;
    Ok(())
}

// отправка архива на сервер
fn send_archive_to_yadro() {}

// fn archive_secret_keys_list<T: AsRef<Path>>(filename: T) {}
fn find_dublicate() {}
