use crate::{tile, FormatData};

#[no_mangle]
pub extern "C" fn x_flipper_360_untile(
  output_buffer: *mut u8,
  output_buffer_len: usize,
  input_buffer: *const u8,
  input_buffer_len: usize,
  format: Option<&FormatData>,
  blocks_x: u32,
  blocks_y: u32,
  offset_x: u32,
  offset_y: u32,
) -> u32 {
  if let Some(format_data) = format {
    let output_buffer = unsafe { std::slice::from_raw_parts_mut(output_buffer, output_buffer_len) };
    let input_buffer = unsafe { std::slice::from_raw_parts(input_buffer, input_buffer_len) };
    tile::untile(
      output_buffer,
      input_buffer,
      format_data,
      blocks_x,
      blocks_y,
      offset_x,
      offset_y,
    )
  } else {
    0
  }
}

#[no_mangle]
pub extern "C" fn x_flipper_360_tile(
  output_buffer: *mut u8,
  output_buffer_len: usize,
  input_buffer: *const u8,
  input_buffer_len: usize,
  format: Option<&FormatData>,
  blocks_x: u32,
  blocks_y: u32,
  offset_x: u32,
  offset_y: u32,
) -> u32 {
  if let Some(format_data) = format {
    let output_buffer = unsafe { std::slice::from_raw_parts_mut(output_buffer, output_buffer_len) };
    let input_buffer = unsafe { std::slice::from_raw_parts(input_buffer, input_buffer_len) };
    tile::tile(
      output_buffer,
      input_buffer,
      format_data,
      blocks_x,
      blocks_y,
      offset_x,
      offset_y,
    )
  } else {
    0
  }
}
