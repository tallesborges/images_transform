use std::fs;
use image::{ImageFormat};
use std::env;

fn main() {
    // Get the first argument
    let args: Vec<String> = env::args().collect();

    // Check if has enough arguments
    if args.len() < 2 {
        println!("Usage: {} <folder>", args[0]);
        return;
    }

    let folder = env::current_dir().unwrap().join(args[1].as_str());

    // Check if the path is a directory
    if !folder.is_dir(){
        println!("{} is not a directory", args[1]);
        return;
    }

    println!("Directory: {:?}", folder);

    let dimensions = ["iphone6_5", "iphone5_5", "ipad12_9"];

    for path in dimensions {
        let path = folder.parent().unwrap().join(path);
        if !path.exists() {
            let _ = fs::create_dir(path);
        }
    }

    let paths = fs::read_dir(folder).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let img = image::open(&path);
        if let Ok(img) = img {
            for dimension_name in dimensions {
                let parent = path.parent().unwrap();
                let file_name = String::from(path.file_stem().unwrap().to_str().unwrap());
                let new_path = parent.join(format!("../{}/{}.jpeg", dimension_name, file_name));

                println!("path {:?}", new_path);

                let dimension: (u32, u32);
                match dimension_name {
                    "iphone6_5" => {
                        dimension = (1242, 2688);
                    }
                    "iphone5_5" => {
                        dimension = (1242, 2208);
                    }
                    "ipad12_9_3" => {
                        dimension = (2048, 2732);
                    }
                    &_ => {
                        dimension = (1242, 2688);
                    }
                }

                if img.height() > img.width() {
                    let new_img = img.resize_exact(dimension.0, dimension.1, image::imageops::FilterType::Nearest);
                    let _ = new_img.save_with_format(new_path, ImageFormat::Jpeg);
                } else {
                    let new_img = img.resize_exact(dimension.1, dimension.0, image::imageops::FilterType::Nearest);
                    let _ = new_img.save_with_format(new_path, ImageFormat::Jpeg);
                }
            }
        }
    }
}
