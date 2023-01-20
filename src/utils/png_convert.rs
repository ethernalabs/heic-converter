use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use libheif_rs::{Channel, RgbChroma, ColorSpace, HeifContext, Result};

pub fn convert_to_png(image_path: String, file_name: String) ->  Result<()> {

  // read heic file
  let ctx = HeifContext::read_from_file(&image_path)?;
  let handle = ctx.primary_image_handle()?;
  let image = handle.decode(ColorSpace::Rgb(RgbChroma::Rgba), false)?;
  let image_width = image.width(Channel::Interleaved)?;
  let image_height = image.height(Channel::Interleaved)?;
  let planes = image.planes();
  let interleaved_plane = planes.interleaved.unwrap();


  // save as png
  let file_output_path = format_args!("output/uncompressed/{file_name}.png").to_string();
  let path = Path::new(&file_output_path);
  let file = File::create(path).unwrap();
  let ref mut w = BufWriter::new(file);

  let mut encoder = png::Encoder::new(w, image_width, image_height);
  encoder.set_color(png::ColorType::Rgba);
  encoder.set_depth(png::BitDepth::Eight);
  encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
  encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded

  let mut writer = encoder.write_header().unwrap();

  writer.write_image_data(&interleaved_plane.data.to_owned()).unwrap();

  Ok(())
}
