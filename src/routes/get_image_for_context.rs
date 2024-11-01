use actix_web::{web, HttpResponse, Responder};
use base64::{engine::general_purpose, Engine as _};
use image::load_from_memory;
use serde_json::json;
use std::path::PathBuf;
use uuid::Uuid;

use crate::{
    structs::{AppState, GeneratedRecord, ImageQuery},
    utils::create_negative_prompts,
    CFG_SCALE, IMAGE_HEIGHT, IMAGE_WIDTH, SD_MODEL_CHECKPOINT,
};

#[actix_web::get("/generate-image")]
async fn generate_image(
    state: web::Data<AppState>,
    query: web::Query<ImageQuery>,
) -> impl Responder {
    let p_prompt = &query.positive;
    let _n_prompt = &query.negative;
    let steps = query.steps;
    let facial_features = &create_negative_prompts()["Facial Features"];
    let anatomical_issues = &create_negative_prompts()["Anatomical Issues"];
    let negs = facial_features
        .iter()
        .chain(anatomical_issues.iter())
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join(", ");
    let image_request_url = "http://127.0.0.1:7860/sdapi/v1/txt2img";
    let payload = json!({
       "prompt": p_prompt,
       "negative_prompt": negs,
       "steps": steps,
       "sampler_method": "Euler a",
       "width": IMAGE_WIDTH,
       "height": IMAGE_HEIGHT,
       "sd_model_checkpoint": SD_MODEL_CHECKPOINT,
       "cfg_scale": CFG_SCALE,
       "CLIP_stop_at_last_layers": 0
    });

    // Send request to Stable Diffusion API
    let response = match reqwest::Client::new()
        .post(image_request_url)
        .json(&payload)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to send request: {}", e))
        }
    };

    // Parse the JSON response
    let json_response: serde_json::Value = match response.json().await {
        Ok(json) => json,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Failed to parse JSON: {}", e))
        }
    };

    // Extract the base64 image string
    let base64_image_string = match json_response["images"].get(0) {
        Some(image_str) => {
            //Save the image string to Database for reference
            let image_str = image_str.as_str().unwrap_or("");
            let data = json!({"image": image_str});
            let created: Option<GeneratedRecord> =
                state.db.create("image").content(data).await.unwrap();
            dbg!("/image: created", created);

            image_str
        }
        None => return HttpResponse::InternalServerError().body("No image found in response"),
    };

    // Decode the base64 string
    let image_data = match general_purpose::STANDARD.decode(base64_image_string) {
        Ok(data) => data,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to decode base64: {}", e))
        }
    };

    // Load the image from memory
    let img = match load_from_memory(&image_data) {
        Ok(img) => img,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Failed to load image: {}", e))
        }
    };

    // Generate a unique filename
    let filename = format!("{}.png", Uuid::new_v4());
    let filepath = PathBuf::from("images").join(&filename);

    // Ensure the 'images' directory exists
    std::fs::create_dir_all("images").unwrap_or_else(|e| {
        eprintln!("Failed to create 'images' directory: {}", e);
    });

    // Save the image
    match img.save(&filepath) {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": "Image generated and saved successfully",
            "filepath": filepath.to_str().unwrap_or(""),
        })),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to save image: {}", e)),
    }
}
