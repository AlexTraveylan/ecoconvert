use image::{DynamicImage, imageops::FilterType};

pub enum ResizeOption {
    Percentage(f32),
    Dimensions(u32, u32),
    Width(u32),
    Height(u32),
}

pub fn resize_image(img: &DynamicImage, resize_option: ResizeOption) -> DynamicImage {
    match resize_option {
        ResizeOption::Percentage(percent) => {
            let new_width = (img.width() as f32 * percent / 100.0) as u32;
            let new_height = (img.height() as f32 * percent / 100.0) as u32;
            img.resize(new_width, new_height, FilterType::Lanczos3)
        },
        ResizeOption::Dimensions(width, height) => {
            let (new_width, new_height) = calculate_dimensions(img.width(), img.height(), width, height);
            let resized = img.resize(new_width, new_height, FilterType::Lanczos3);
            resized.crop_imm(0, 0, width, height)
        },
        ResizeOption::Width(width) => {
            let height = (width as f32 * img.height() as f32 / img.width() as f32) as u32;
            img.resize(width, height, FilterType::Lanczos3)
        },
        ResizeOption::Height(height) => {
            let width = (height as f32 * img.width() as f32 / img.height() as f32) as u32;
            img.resize(width, height, FilterType::Lanczos3)
        },
    }
}

fn calculate_dimensions(orig_width: u32, orig_height: u32, target_width: u32, target_height: u32) -> (u32, u32) {
    let width_ratio = target_width as f32 / orig_width as f32;
    let height_ratio = target_height as f32 / orig_height as f32;
    let ratio = width_ratio.min(height_ratio);

    let new_width = (orig_width as f32 * ratio) as u32;
    let new_height = (orig_height as f32 * ratio) as u32;

    (new_width, new_height)
}
