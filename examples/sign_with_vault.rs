use std::convert::TryFrom;

use base64::Engine;
use pasetors::{claims::Claims, keys::AsymmetricPublicKey, token::UntrustedToken};
use vaultrs::client::VaultClientSettingsBuilder;

#[tokio::main]
async fn main() {
    let mut c = Claims::new().unwrap();
    c.subject("project1").unwrap();

    let msg = c.to_string().unwrap();
    let pae = pasetors::version2::PublicToken::pae(msg.as_bytes(), &vec![]).unwrap();

    //println!("{}", hex::encode(pae));

    let vault = vaultrs::client::VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address("http://localhost:8200")
            .token("hvs.VI8fhqdNCiG2oKP53S2WUdUw")
            .build()
            .unwrap(),
    )
    .unwrap();

    let signed = vaultrs::transit::data::sign(
        &vault,
        "transit",
        "demeter-api-root",
        &base64::engine::general_purpose::STANDARD.encode(&pae),
        None,
    )
    .await
    .unwrap();

    let signed = base64::engine::general_purpose::STANDARD
        .decode(signed.signature.split(":").into_iter().last().unwrap())
        .unwrap();

    let token =
        pasetors::version2::PublicToken::concatenate(&msg.as_bytes(), &signed, &vec![]).unwrap();

    println!("{}", token);

    let key =
        hex::decode("42c94f582e73d42fe772cab2c9703582abce03589cadec7adf92373ac52732cd").unwrap();

    let key = AsymmetricPublicKey::from(&key).unwrap();

    let token =
        UntrustedToken::<pasetors::Public, pasetors::version2::V2>::try_from(&token).unwrap();

    pasetors::version2::PublicToken::verify(&key, &token, None).unwrap();
}
