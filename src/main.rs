extern crate image;
use std::path::{Path, PathBuf};
use image::imageops;
use image::FilterType;

fn process_file_of_path(root: PathBuf, func: &dyn Fn(&Path)) {
    let paths = std::fs::read_dir(root).unwrap();
    for path in paths{
        let path = path
            .unwrap()
            .path();

        if path.is_dir() {
            process_file_of_path(path, &func);
        }
        else {
            if path.to_str()
                .unwrap()
                .ends_with(".png") {
                func(path.as_path());
            }
        }
    }
}

fn resize_png(img: &Path, dst_path: &str) {
    let out_path = Path::new(dst_path).join(img.file_name().unwrap());
    //println!("img is :{:?}, {:?}", img, out_path);
    let img = image::open(img).unwrap();
    let img_out = imageops::resize(&img, 51, 51, FilterType::Nearest);
    img_out.save(out_path).unwrap();
    //    .unwrap();
}

fn png_to_gray(img: &Path) {
    let mut out_path = img.to_str()
        .unwrap()
        .trim_end_matches(".png")
        .to_string();

    out_path.push_str("_gray.png");
    let img = image::open(img).unwrap();
    let img_out = imageops::grayscale(&img);
    img_out.save(out_path).unwrap();
}


fn main() {
    /*
    let dst_path = r"E:\shgithub\others\kaixinxiaoxiaole\my_resource_out";
    process_file_of_path(PathBuf::from(r"E:\shgithub\others\kaixinxiaoxiaole\my_resource"), &|filename| {
        resize_png(filename, dst_path);
    });*/

    process_file_of_path(PathBuf::from(r"E:\shgithub\others\kaixinxiaoxiaole\my_resource_out"), &png_to_gray);
}
