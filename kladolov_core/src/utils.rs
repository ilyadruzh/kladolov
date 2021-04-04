// Для обработки ошибок, возвращаемое значение оборачивается в Result
// Возвращаем `Iterator` для построчного чтения файла.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn wrap_scep256k1_keys_to_bitcoin_keys() {
    let secp = Secp256k1::new();
    let (sec_key, pub_key) = secp.generate_keypair(&mut bitcoin::secp256k1::rand::thread_rng());
    let public_key = bitcoin::util::key::PublicKey {
        compressed: true,
        key: pub_key,
    };
    let private_key = bitcoin::util::key::PrivateKey {
        compressed: true,
        network: Network::Bitcoin,
        key: sec_key,
    };
}
