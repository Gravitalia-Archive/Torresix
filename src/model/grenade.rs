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
        .with_input_fact(0, f32::fact([1, 224, 224, 3]).into())?
        .with_output_fact(0, InferenceFact::dt_shape(f32::datum_type(), tvec![1, 8]))?
        .into_optimized()?
        .into_runnable()?;
    
    Ok(model)
}

pub fn predict(model: super::TensorflowModel, buffer: Vec<u8>) -> Result<String> {
    let img = image::load_from_memory(&buffer)?;
    let resized_img = image::imageops::resize(&img, 224, 224, ::image::imageops::FilterType::Nearest);

    let img_array: Tensor = tract_ndarray::Array::from_shape_fn((1, 3, 224, 224), |(_, c, y, x)| {
        resized_img.get_pixel(x as u32, y as u32)[c] as f32
    }).into();


    let outputs = model.run(tvec!(img_array.permute_axes(&[0, 2, 3, 1])?.into()))?;

    let best = outputs[0]
    .to_array_view::<f32>()?
    .iter()
    .cloned()
    .zip(1..)
    .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    Ok(RESULT[best.unwrap().1-1].to_string())
}
