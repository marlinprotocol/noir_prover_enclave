use actix_web::{http::StatusCode, web, App, HttpResponse, HttpServer, Responder};
use ethers::core::types::{Address, U256};
use ethers::types::Bytes;
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{LocalWallet, Signer, Wallet},
};
use kalypso_helper::response::response;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{Error, ErrorKind};
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_path = "./app/config.toml";
    let config: handler::Config =
        toml::from_str(&fs::read_to_string(config_path).expect("/app/config.toml not found"))
            .expect("Could not parse the config.toml file");
    let config_data = web::Data::new(Arc::new(Mutex::new(config)));
    let lock = web::Data::new(Arc::new(Mutex::new(()))); // Create a lock
    HttpServer::new(move || {
        App::new()
            .app_data(config_data.clone())
            .app_data(lock.clone())
            .configure(handler::routes)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use crate::handler;
    use actix_web::web::Data;
    use actix_web::{test, App};
    use kalypso_ivs_models::models::EncryptedInputPayload;
    use log::warn;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::sync::{Arc, Mutex};
    use tokio::fs;
    #[actix_rt::test]
    async fn test_server() {
        let app = test::init_service(App::new().service(handler::test)).await;
        let req = test::TestRequest::get().uri("/test").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let result = test::read_body(resp).await;
        let result_json: Value = serde_json::from_slice(&result).unwrap();
        let expected_json = json!({
            "message": "The Noir prover is running!!",
            "data": "Noir Prover is running!"
        });

        assert_eq!(result_json, expected_json);
    }

    #[actix_rt::test]
    async fn test_generate_proof() {
        // let enclave_key = fs::read("./app/secp.sec").await.unwrap();
        // let enclave_key = Arc::new(Mutex::new(enclave_key));

        // let app = test::init_service(
        //     App::new()
        //         .service(handler::generate_proof)
        //         .app_data(Data::new(enclave_key)),
        // )
        // .await;
        // // let private_input = fs::read("./app/sample_auth.txt").await.unwrap();

        // let payload = kalypso_generator_models::models::InputPayload::from_plain_secrets(
        //     [
        //         123, 10, 32, 32, 34, 120, 34, 58, 32, 34, 49, 34, 44, 10, 32, 32,34, 121, 34, 58, 32, 34, 50, 34, 10, 125
        //     ]
        //     .into(),
        //     [1,2],
        // );

        // fs::write(
        //     "generate_proof_payload.json",
        //     serde_json::to_string(&payload).unwrap(),
        // )
        // .await
        // .unwrap();
    }
}
