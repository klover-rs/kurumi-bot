use std::io::{self, Write};
use image::{DynamicImage, GenericImage, GenericImageView};
use fastrand::Rng;
use std::fs;

use std::path;
use std::env;

use serde_json::json;
use std::process::{Command, Stdio};

pub enum OutputFormat {
    PNG,
    HTML,
}

pub fn init() {
    if !check_if_node_is_installed() {
        println!("\x1b[31mNode.js is not installed on this system. Please install Node.js from https://nodejs.org/en/download and try again.\x1b[0m");
        panic!();
    } else {

        let folder_path = "./src/crates/image_to_ascii/js_src/node_modules";
        if !std::path::Path::new(folder_path).exists() {

            let original_directory = match env::current_dir() {
                Ok(path) => path,
                Err(err) => {
                    eprintln!("Failed to get current directory: {}", err);
                    return;
                }
            };

            if let Err(err) = env::set_current_dir("./src/crates/image_to_ascii/js_src") {
                eprintln!("Failed to change directory: {}", err);
                return;
            }

            let output = Command::new("node")
            .args(&["-e", "require('child_process').exec('npm install', (err, stdout, stderr) => { if (err) { console.error(err); return; } console.log(stdout); });"])
            .output()
            .expect("Failed to execute command");
    
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("npm output: {}", stdout);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Error executing npm command: {}", stderr);
            }
            
            env::set_current_dir(original_directory).unwrap();
        }

        
    }
}

fn check_if_node_is_installed() -> bool {
    let output = Command::new("node")
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Node.js version installed: {}", stdout);
        true
    } else {
        println!("Node.js is not installed on this system.");
        false
    }
}

pub fn convert_to_ascii(buffer: Vec<u8>, quality: u8) -> io::Result<Vec<u8>> {
    println!("Converting  to ASCII");

    let json = convert(buffer, quality);

    let output = base64::encode(&json.to_string());

    let output = Command::new("node")
        .args(&["./src/crates/image_to_ascii/js_src/index.js", &output])
        .stdout(Stdio::piped())
        .output()?;
    
    let output_str = if output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
    
        fs::write("output.txt", &stdout)?;
        stdout
    } else {
        let stderr = String::from_utf8(output.stderr).unwrap();
        eprintln!("Error running Node.js: {}", stderr);
        return Err(io::Error::new(io::ErrorKind::Other, "Node.js execution failed"));
    };

    

    
    let image = stretch_image(image::open(&output_str.trim_end()).unwrap(), 2.2, 1.0);
    

    let directory_path = path::Path::new("./generated_images");

    if !directory_path.exists() {
        match fs::create_dir(directory_path) {
            Ok(_) => println!("Directory created successfully"),
            Err(err) => eprintln!("Error creating directory: {}", err),
        }
    }

    let random_filename = format!("{}.png", generate_random_string());

    image.save(format!("./generated_images/{}", random_filename)).unwrap();

    let image = fs::read(format!("./generated_images/{}", random_filename)).unwrap();

    fs::remove_file(format!("./generated_images/{}", random_filename)).unwrap();
    fs::remove_file(output_str.trim_end()).unwrap();
    
    
    print!("finished");
    
    Ok(image)
}




fn convert(buffer: Vec<u8>, quality: u8) -> serde_json::Value {
    println!("Converting image to ASCII");
    
    let img = image::load_from_memory(&buffer).unwrap();

    let gray_img = img.grayscale();

    let ascii_chars = "@%#*+=-:. /|\\:,()";

    let (width, height) = gray_img.dimensions();

    let mut desired_width = quality as u32;

    if desired_width > width {
        desired_width = width;
    }

    let width_ratio = width as f32 / desired_width as f32;
    
    let desired_height = ((height as f32) / width_ratio) as u32;

    let mut ascii_string = String::new();
    
    for y in 0..desired_height {
        for x in 0..desired_width {
            let px = (x as f32 * width_ratio).round() as u32;
            let py = (y as f32 * width_ratio).round() as u32;
            let pixel = gray_img.get_pixel(px, py);
            let brightness = pixel[0] as f32 / 255.0;
            let char_index = ((brightness * (ascii_chars.len() - 1) as f32).round()) as usize;
            ascii_string.push(ascii_chars.chars().nth(char_index).unwrap());
        }
        ascii_string.push('\n');
    }

    let html = generate_html(&ascii_string);

    let json = json!({
        "path": html,
        "width": width * 5,
        "height": desired_height
    });

    json

}

fn generate_html(ascii_string: &str) -> String {

    let mut html = String::from(r#"<html><code>"#);

    let style = r#"style="color: black; background: pink;
        display:inline-block;
        white-space:pre;
        letter-spacing:1px;
        line-height:0.9;
        font-family:'Consolas','BitstreamVeraSansMono','CourierNew',Courier,monospace;
        font-size:8px;""#;

    html.push_str(&format!(r#"<span class="ascii" {}>"#, style));

    for c in ascii_string.chars() {
        if c == '\n' {
            html.push_str("</span><br>");
        } else {
            html.push_str(&format!(r#"<span>{}</span>"#, c));
        }
    }

    html.push_str("</span></code></html>");
    
    let file_name = format!("{}.html", generate_random_string());

    let mut file = fs::File::create(&file_name).unwrap();

    let file_path = fs::canonicalize(&file_name).unwrap();
 
    file.write_all(html.as_bytes()).unwrap();

    let file_path_str = file_path
    .to_string_lossy() // Convert to string with normal slash separator
    .to_string(); // Convert to owned String

    // Remove the leading \\?\ prefix
    let file_path_str = if file_path_str.starts_with("\\\\?\\") {
        file_path_str[4..].to_string()
    } else {
        file_path_str
    };

    file_path_str.replace("\\", "/")

}

fn generate_random_string() -> String {
    let mut rng = Rng::new();
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut result = String::with_capacity(8);

    for _ in 0..8 {
        let idx = rng.usize(..charset.len());
        result.push(charset[idx] as char);
    }

    result
}

fn stretch_image(image: DynamicImage, width_scale: f32, height_scale: f32) -> DynamicImage {
    let (width, height) = image.dimensions();
    let target_width = (width as f32 * width_scale) as u32;
    let target_height = (height as f32 * height_scale) as u32;
    let mut stretched_image = DynamicImage::new_rgb8(target_width, target_height);

    for y in 0..target_height {
        for x in 0..target_width {
            let source_x = (x as f32 / target_width as f32 * width as f32) as u32;
            let source_y = (y as f32 / target_height as f32 * height as f32) as u32;
            let pixel = image.get_pixel(source_x, source_y);
            unsafe { stretched_image.unsafe_put_pixel(x, y, pixel) };
        }
    }

    stretched_image
}