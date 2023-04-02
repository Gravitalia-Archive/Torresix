use tract_onnx::prelude::*;
use anyhow::Result;

const RESULT: [&str; 8] = [
    "animals",
    "flower",
    "human",
    "landscape",
    "nude",
    "plant",
    "sport",
    "vehicle"
];

pub fn init() -> Result<super::TensorflowModel> {
    let model = tract_onnx::onnx()
        .model_for_path("./models/grenade.onnx")?
        .with_input_fact(0, f32::fact([64, 224, 224, 3]).into())?
        .into_optimized()?
        .into_runnable()?;
    
    Ok(model)
}

pub fn predict(model: super::TensorflowModel, buffer: Vec<u8>) -> Result<String> {
    let image = image::load_from_memory(&buffer)?;
    let resized =
        image::imageops::resize(&image, 224, 224, ::image::imageops::FilterType::Triangle);

    let image: Tensor = tract_ndarray::ArrayBase::from_shape_fn((64, 3, 224, 224), |(_, c, y, x)| {
        let mean = [0.485, 0.456, 0.406][c];
        let std = [0.229, 0.224, 0.225][c];
        (resized[(x as _, y as _)][c] as f32 / 255.0 - mean) / std
    })
    .into();

    let best = model.run(tvec!(image.permute_axes(&[0, 2, 3, 1])?.into()))?[0]
        .to_array_view::<f32>()?
        .iter()
        .cloned()
        .zip(1..)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    Ok(RESULT[best.unwrap().1-1].to_string())
}
