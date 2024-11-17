use std::str::FromStr;
use clap::{value_parser, CommandFactory, Parser, ValueHint};
use image::DynamicImage;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[arg(value_hint = ValueHint::FilePath)]
    pub outfile: String,
    #[arg(value_parser = value_parser!(String))]
    pub command_vector: Vec<String>,
    #[arg(value_hint = ValueHint::FilePath, required = false, long = "infile")]
    pub infile: Option<String>,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Rotation {
    Ninety = 90,
    OneEighty = 180,
    TwoSeventy = 270,
}

impl FromStr for Rotation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(90) => Ok(Rotation::Ninety),
            Ok(180) => Ok(Rotation::OneEighty),
            Ok(270) => Ok(Rotation::TwoSeventy),
            _ => Err("Must be one of 90, 180, 270")
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ChainCommands {
    Blur {},
    Brighten {
        brightness: i32,
    },
    Crop {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    Rotate {
        rotation: Rotation,
    },
    Invert {},
    Grayscale {},
    Fractal {},
    Square {
        red: u8,
        green: u8,
        blue: u8,
    },
}

pub fn split_command_vector(commands: &Vec<String>) -> Vec<ChainCommands> {
    let mut chain_commands = Vec::<ChainCommands>::new();
    let chain_commands_raw = commands
        .split(|elem| "/".eq(elem))
        .filter(|elem| !elem.is_empty());
    for command in chain_commands_raw.filter(|elem| !elem.is_empty()) {
        match command.get(0).unwrap().as_str() {
            "blur" => {
                chain_commands.push(ChainCommands::Blur {});
            },
            "brighten" => {
                if command.len() != 2 {
                    print_specific_usage_and_exit("Brighten", "<brightness>");
                }
                chain_commands.push(ChainCommands::Brighten {
                    brightness: command.get(1).unwrap().parse::<i32>()
                                       .expect("argument must be a number")
                });
            },
            "crop" => {
                if command.len() != 5 {
                    print_specific_usage_and_exit("Crop", "<x> <y> <width> <height>");
                }
                chain_commands.push(ChainCommands::Crop {
                    x: command.get(1).unwrap().parse::<u32>()
                              .expect("argument must be a number"),
                    y: command.get(2).unwrap().parse::<u32>()
                              .expect("argument must be a number"),
                    width: command.get(3).unwrap().parse::<u32>()
                                  .expect("argument must be a number"),
                    height: command.get(4).unwrap().parse::<u32>()
                                   .expect("argument must be a number"),
                });
            },
            "rotate" => {
                if command.len() != 2 {
                    print_specific_usage_and_exit("Rotate", "<rotation (90, 180, 270)>");
                }
                chain_commands.push(ChainCommands::Rotate {
                    rotation: command.get(1).unwrap().parse::<Rotation>()
                                     .expect("Invalid value for rotation"),
                });
            },
            "invert" => {
                chain_commands.push(ChainCommands::Invert {});
            },
            "grayscale" => {
                chain_commands.push(ChainCommands::Grayscale {});
            },
            "fractal" => {
                chain_commands.push(ChainCommands::Fractal {});
            },
            "square" => {
                if command.len() != 4 {
                    print_specific_usage_and_exit("Square", "<red> <green> <blue>");
                }
                chain_commands.push(ChainCommands::Square {
                    red: command.get(1).unwrap().parse::<u8>()
                                .expect("argument must be a number"),
                    green: command.get(2).unwrap().parse::<u8>()
                                  .expect("argument must be a number"),
                    blue: command.get(3).unwrap().parse::<u8>()
                                 .expect("argument must be a number"),
                });
            },
            _ => {
                print_usage_and_exit();
            }
        }
    }
    chain_commands
}

fn print_specific_usage_and_exit(command: &str, message: &str) {
    let mut cmd = Cli::command();
    println!("{} {}", command, message);
    cmd.print_help().expect("Should print help list");
    std::process::exit(-1);
}

fn print_usage_and_exit() {
    let mut cmd = Cli::command();
    cmd.print_help().expect("Should print help list");
    std::process::exit(-1);
}

pub fn blur(infile: String, outfile: String) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 = img.blur(2.0);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn brighten(infile: String, outfile: String, brightness: i32) {
    // See blur() for an example of how to open / save an image.
    let img = image::open(infile).expect("Failed to open INFILE.");
    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.
    let img2 = img.brighten(brightness);

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    // See blur() for an example of how to open an image.
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.
    let img2 = img.crop(x, y, width, height);
    // Challenge: parse the four values from the command-line and pass them
    // through to this function.

    // See blur() for an example of how to save the image.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn rotate(infile: String, outfile: String, rotation: Rotation) {
    // See blur() for an example of how to open an image.
    let img = image::open(infile).expect("Failed to open INFILE.");
    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!
    let img2: DynamicImage;
    match rotation {
        Rotation::Ninety => {
            img2 = img.rotate90();
        },
        Rotation::OneEighty => {
            img2 = img.rotate180();
        },
        Rotation::TwoSeventy => {
            img2 = img.rotate270();
        }
    }
    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.

    // See blur() for an example of how to save the image.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn invert(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.
    img.invert();

    // See blur() for an example of how to save the image.
    img.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn grayscale(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.
    let img = image::open(infile).expect("Failed to open INFILE.");
    // .grayscale() takes no arguments. It returns a new image.
    let img2 = img.grayscale();

    // See blur() for an example of how to save the image.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn generate(outfile: String, red: u8, green: u8, blue: u8) {
    // Create an ImageBuffer -- see fractal() for an example
    let square_size = 100;

    let mut imgbuf = image::ImageBuffer::new(square_size, square_size);
    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut new_green = green;
        let mut new_blue = blue;
        if x == 0 || x == square_size - 1 {
            println!("x {} y {}", x, y);
            new_green = 255 - new_green;
        } else if y == 0 || y == square_size - 1 {
            println!("x {} y {}", x, y);
            new_green = 255 - new_green;
        }
        if x as i32 - y as i32 == 0 || x + y == square_size - 1 {
            new_blue = 255 - new_blue;
        }
        *pixel = image::Rgb([red, new_green, new_blue]);
    }

    // See blur() for an example of how to save the image
    imgbuf.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
pub fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}