use http::uri::Authority;
use rcgen::{Certificate, CertificateParams};
use rustls::PrivateKey;
use std::fs::File;
use std::io::Read;

pub fn generate_cert(authority: &Authority) -> (PrivateKey, Vec<rustls::Certificate>) {
    // TODO use COW
    let mut key_file = File::open("/Users/quantum/Desktop/code/mitm/cert.key").unwrap();
    let mut cert_file = File::open("/Users/quantum/Desktop/code/mitm/cert.crt").unwrap();

    let mut key_buf = String::new();
    key_file.read_to_string(&mut key_buf).unwrap();

    let mut cert_buf = String::new();
    cert_file.read_to_string(&mut cert_buf).unwrap();

    let key_pair = rcgen::KeyPair::from_pem(&key_buf[..]).expect("Invalid certificate key");
    let mut param =
        CertificateParams::from_ca_cert_pem(&cert_buf[..], key_pair).expect("Invalid pem key");
    param.subject_alt_names = vec![rcgen::SanType::DnsName(authority.to_string())];

    let cert = Certificate::from_params(param).expect("Unable to create cert");

    let pkey = PrivateKey(cert.serialize_private_key_der());
    let vcert = vec![rustls::Certificate(
        cert.serialize_der().expect("Error on serializing der"),
    )];

    std::fs::write("hello.txt", cert.serialize_pem().unwrap()).unwrap();

    return (pkey, vcert);
}
