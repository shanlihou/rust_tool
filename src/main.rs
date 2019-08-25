extern crate image;
use std::path::{Path, PathBuf};
use image::imageops;
use image::FilterType;
pub mod notation;
pub mod just_show;

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

fn resize_png(img: &Path) {
    //let out_path = Path::new(dst_path).join(img.file_name().unwrap());
    //println!("img is :{:?}, {:?}", img, out_path);
    let mut dst_parent = img.parent()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    dst_parent.push_str("_small");
    std::fs::create_dir(&dst_parent).unwrap();
    let out_path = Path::new(&dst_parent).join(img.file_name().unwrap());
    println!("small path:{:?}", out_path);

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

fn oldman() {
    // let dst_path = r"E:\shgithub\others\kaixinxiaoxiaole\my_resource_out";
    process_file_of_path(PathBuf::from(r"E:\shgithub\others\kaixinxiaoxiaole\my_resource"), &|filename| {
        resize_png(filename);
    });


    //process_file_of_path(PathBuf::from(r"E:\shgithub\others\kaixinxiaoxiaole\my_resource_out"), &png_to_gray);
}

fn main() {
    let mut filename = String::from("");
    let mut is_resize = false;
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("img tool");
        ap.refer(&mut is_resize)
            .add_option(&["-r"], argparse::StoreFalse, "will resize");
        ap.refer(&mut filename).add_argument("filename", argparse::Store, "file name");
        ap.parse_args_or_exit();
    }
    if is_resize {
        oldman();

        let not = notation::Notation{
            filename
        };
        not.test();
    }
    just_show::glium_test();
}
