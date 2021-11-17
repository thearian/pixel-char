use image;
use std::fs;
use std::io::Write;
use image::io::Reader as ImageReader;
use std::env;
use image::imageops::FilterType;


static ALTS: [char; 12] = ['.',',','-','~',':',';','=','!','*','#','$','@'];
static ALTS_REV: [char; 12] = ['@', '$', '#', '*', '!', '=', ';', ':', '~', '-', ',', '.'];
static USAGE: &str = "pixel-chars <IMAGE_PATH> <RESOLUTION> <white || black>";


fn main() {
    let (path, resolution, should_black) = get_user_args();

    let decoded_img = ImageReader::open(&path)
        .expect("Failed to open file")
        .decode()
        .expect("Failed to decode as image");
    
    let resize_dimentions = (
        resolution,
        (resolution as f32 * (
            decoded_img.to_rgb8().height() as f32
            /
            decoded_img.to_rgb8().width() as f32
        )) as u32
    );
    
    let img = decoded_img
        .resize(resize_dimentions.0, resize_dimentions.1, FilterType::Triangle)
        .to_rgb8();
    
    let mut output = String::new();
    for row in img.rows() {
        for pixel in row {
            let dim = (
                pixel[0] as usize
                * pixel[1] as usize
                * pixel[2] as usize
            ) as usize;
            let index = set_max(dim,11);
            if should_black {
                output.push(ALTS_REV[index])
            }
            else {
                output.push(ALTS[index])
            }
        }
        output.push('\n');
    }
    let mut destination = path.split(".")
        .collect::<Vec<&str>>()
        [0]
        .to_owned();
    destination.push_str(".txt");
    write_file(&destination,&output)
        .expect("Failed to save output");

    println!("Successfully Done\nFile is saved at {}",destination)
}


fn set_max(inp: usize, max:usize) -> usize {
    (
        (inp as f32 / (255 * 255 * 255) as f32) as f32
        *
        (max as f32)
    ) as usize
}


fn get_user_args() -> (String, u32,bool) {
    let args = get_env_args();
    let path = args[1].to_owned();
    let resolution = args[2]
        .parse::<u32>()
        .expect(USAGE);
    let should_black = args[3] == "black";
    (path, resolution, should_black)
}


fn get_env_args() -> Vec<String> {
    env::args().collect()
}


fn write_file(destination: &String, content: &String) -> std::io::Result<()> {
    let mut file = fs::File::create(destination.to_owned())?;
    file.write_all(content.as_bytes() as &[u8])?;
    Ok(())
}