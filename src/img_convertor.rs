use clap::builder::Str;
use image::ImageFormat;

pub fn convert_image(file_name: String, extension_to_convert: String, save_name: String) {
    let img = image::open(file_name)
        .expect("Ошибка открытия файла! Убедитесь, что файл в той же директории");

    let convert_type: &str = &extension_to_convert;

    let file_format = match convert_type {
        "png" => ImageFormat::Png,
        "jpeg" => ImageFormat::Jpeg,
        _ => panic!("Введен неверный или же неподдерживаемый формат!"),
    };

    img.save_with_format(save_name, file_format).unwrap();
}
