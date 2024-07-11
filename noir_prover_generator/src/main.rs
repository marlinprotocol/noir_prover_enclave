use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::process::{Command, Output, Stdio};
use std::fs;
use std::io::Write;
use std::sync::Arc;
use tokio::sync::Mutex;
use toml;

#[derive(Debug, Deserialize)]
struct GenerateProofInputs {
    ask: String,
    private_inputs: Vec<String>,
    ask_id: Vec<String>,
}

#[derive(Debug, Serialize)]
struct JsonResponse<T> {
    message: String,
    data: T,
}

#[derive(Debug, Deserialize)]
struct Config {
    toml_path: String,
    output_path: String,
}

async fn generate_proof(
    inputs: web::Json<GenerateProofInputs>,
    config: web::Data<Arc<Mutex<Config>>>,
) -> impl Responder {
    println!("Hello, world!");
    let config = config.lock().await;

    // Write private inputs to askId.toml
    if let Err(err) = write_private_inputs_to_toml(&inputs, &config.toml_path) {
        return HttpResponse::InternalServerError().json(JsonResponse {
            message: format!("Failed to write private inputs to TOML file: {:?}", err),
            data: "None",
        });
    }
    // println!(config.toml_path);
    // println!(config.output_path);
    // Generate proof using external command `nargo prove`
    let result = execute_prove_command(&inputs, &config.toml_path, &config.output_path).await;

    println!("Hello, world!");
    match result {
        Ok(file_path) => {
            // Read the output file generated by `nargo prove`
            match read_output_file(&file_path) {
                Ok(file_contents) => {
                    // Construct JSON response
                    HttpResponse::Ok().json(JsonResponse {
                        message: "Proof generated successfully.".to_string(),
                        data: Some(file_contents),
                    })
                }
                Err(err) => {
                    HttpResponse::InternalServerError().json(JsonResponse {
                        message: format!("Failed to read output file: {:?} for file {:?}", err,file_path),
                        data: "None",
                    })
                }
            }
        }
        Err(err) => {
            // println!(err);
            HttpResponse::InternalServerError().json(JsonResponse {
                message: format!("Failed to generate proof: {:?}", err),
                data: "None",
            })
        }
    }
}

fn write_private_inputs_to_toml(
    inputs: &GenerateProofInputs,
    toml_path: &str,
) -> Result<(), std::io::Error> {
    let ask_id = &inputs.ask_id[0]; // Assuming ask_id is a single item list
    let file_path = format!("{}/{}.toml", toml_path, ask_id);

    // Ensure there are at least two private inputs for x and y
    if inputs.private_inputs.len() < 2 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not enough private inputs"));
    }

    // Create the TOML content
    let mut toml_content = String::new();
    
    // Add x and y variables
    toml_content.push_str(&format!("x = \"{}\"\n", inputs.private_inputs[0]));
    toml_content.push_str(&format!("y = \"{}\"\n", inputs.private_inputs[1]));
    
    // Write the TOML content to the file
    let mut file = fs::File::create(&file_path)?;
    file.write_all(toml_content.as_bytes())?;

    Ok(())
}

async fn execute_prove_command(
    inputs: &GenerateProofInputs,
    toml_path: &str,
    output_path: &str,
) -> Result<String, std::io::Error> {
    let ask_id = &inputs.ask_id[0]; // Assuming ask_id is a single item list
    let toml_file_path = format!("{}/{}.toml", toml_path, ask_id);
    let output_file_path = format!("{}/hello_world.proof", output_path);

    let mut cmd = Command::new("nargo");
    cmd.arg("prove")
        .arg("-p")
        .arg(&toml_file_path)
        .current_dir(&toml_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Execute the command asynchronously
    let output = cmd.output()?;

    // // Write the stdout to a file
    // fs::write(&output_file_path, &output.stdout)?;

    Ok(output_file_path)
}

fn read_output_file(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}


async fn test() -> impl Responder {
    // Implement your logic to generate the proof here
    // For demonstration purposes, we'll just return a JSON response
    let response_json = json!({
        "message": "Not implemented."
    });

    // Respond with JSON
    actix_web::HttpResponse::Ok().json(response_json)
}

async fn benchmark() -> impl Responder {
    // Implement your logic to generate the proof here
    // For demonstration purposes, we'll just return a JSON response
    let response_json = json!({
        "message": "Not implemented."
    });

    // Respond with JSON
    actix_web::HttpResponse::Ok().json(response_json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_path = "/app/config.toml";
    let config: Config = toml::from_str(&fs::read_to_string(config_path)?)?;
    let config_data = web::Data::new(Arc::new(Mutex::new(config)));
    HttpServer::new(move || {
        App::new()
            .app_data(config_data.clone())
            .route("/generate_proof", web::post().to(generate_proof))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await

}

