use std::collections::HashMap;

pub fn create_negative_prompts() -> HashMap<String, Vec<String>> {
    let mut negative_prompts = HashMap::new();

    negative_prompts.insert(
        "Image Quality Issues".to_string(),
        vec![
            "Worst quality".to_string(),
            "Low quality".to_string(),
            "Low res".to_string(),
            "Blurry".to_string(),
            "Jpeg artifacts".to_string(),
            "Grainy".to_string(),
            "Pixelated".to_string(),
        ],
    );

    negative_prompts.insert(
        "Anatomical Issues".to_string(),
        vec![
            "Bad anatomy".to_string(),
            "Bad proportions".to_string(),
            "Deformed".to_string(),
            "Disfigured".to_string(),
            "Extra limbs".to_string(),
            "Extra fingers".to_string(),
            "Missing limbs".to_string(),
            "Poorly drawn hands".to_string(),
            "Poorly drawn face".to_string(),
            "Long neck".to_string(),
            "Malformed limbs".to_string(),
        ],
    );

    negative_prompts.insert(
        "Unwanted Elements".to_string(),
        vec![
            "Text".to_string(),
            "Logo".to_string(),
            "Watermark".to_string(),
            "Signature".to_string(),
            "Duplicate".to_string(),
            "Cloned face".to_string(),
            "Out of frame".to_string(),
            "Cropped".to_string(),
        ],
    );

    negative_prompts.insert(
        "Artistic Style".to_string(),
        vec![
            "Cartoon".to_string(),
            "Anime".to_string(),
            "Illustration".to_string(),
            "Painting".to_string(),
            "CGI".to_string(),
            "3D render".to_string(),
            "Sketch".to_string(),
            "Drawing".to_string(),
        ],
    );

    negative_prompts.insert(
        "Facial Features".to_string(),
        vec![
            "Extra eyes".to_string(),
            "Oversized eyes".to_string(),
            "Fused face".to_string(),
            "Cloned face".to_string(),
        ],
    );

    negative_prompts.insert(
        "General Negative Traits".to_string(),
        vec![
            "Ugly".to_string(),
            "Boring".to_string(),
            "Gross proportions".to_string(),
            "Mutation".to_string(),
            "Mutated hands".to_string(),
        ],
    );

    negative_prompts
}
