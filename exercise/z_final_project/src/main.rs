// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use std::path::Path;
use clap::{value_parser, CommandFactory, Parser, Subcommand, ValueEnum, ValueHint};
use image::DynamicImage;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(value_hint = ValueHint::FilePath)]
    outfile: String,
    #[arg(value_hint = ValueHint::FilePath, required = false)]
    infile: Option<String>,
    #[arg(value_parser = value_parser!(String))]
    command_vector: Vec<String>,
}

#[derive(ValueEnum, Clone, Debug)]
enum Rotation {
    Ninety = 90,
    OneEighty = 180,
    TwoSeventy = 270,
}

#[derive(Debug)]
enum ChainCommands {
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
    // #[command(external_subcommand)]
    // Default(Vec<String>),
}

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let cli = Cli::parse();
    println!("{:?}", cli.command_vector);
    for command in cli.command_vector {
        let infile: Option<String> = if Path::new(&cli.outfile).exists() {
            Some(cli.outfile.clone())
        } else {
            cli.infile.clone().or(None)
        };
        let outfile = cli.outfile.clone();
        match command {
            ChainCommands::Blur {} => {
                if let Some(infile) = infile {
                    println!("Blur infile {} and outfile {}", infile, outfile);
                    // **OPTION**
                    // Improve the blur implementation -- see the blur() function below
                    blur(infile, outfile);
                }
            }
            ChainCommands::Brighten { brightness } => {
                if let Some(infile) = infile {
                    println!("Brighten infile {} and outfile {}", infile, outfile);
                    brighten(infile, outfile, brightness);
                }
            }
            ChainCommands::Crop { x, y, width, height } => {
                if let Some(infile) = infile {
                    println!("Crop infile {} and outfile {}", infile, outfile);
                    crop(infile, outfile, x, y, width, height);
                }
            }
            ChainCommands::Rotate { rotation } => {
                if let Some(infile) = infile {
                    println!("Rotate infile {} and outfile {}", infile, outfile);
                    rotate(infile, outfile, rotation);
                }
            }
            ChainCommands::Invert {} => {
                if let Some(infile) = infile {
                    println!("Invert infile {} and outfile {}", infile, outfile);
                    invert(infile, outfile);
                }
            }
            ChainCommands::Grayscale {} => {
                if let Some(infile) = infile {
                    println!("Grayscale infile {} and outfile {}", infile, outfile);
                    grayscale(infile, outfile);
                }
            }
            ChainCommands::Fractal {} => {
                fractal(outfile);
            }
            ChainCommands::Square { red, green, blue } => {
                generate(outfile, red, green, blue);
            }
            // ChainCommands::Default(_) => {
            //     print_usage_and_exit();
            // }
        }
    }
    // match cli.command_vector {
    //     ChainCommands::Blur {} => {
    //         println!("Blur infile {} and outfile {}", infile, outfile);
    //         // **OPTION**
    //         // Improve the blur implementation -- see the blur() function below
    //         blur(infile, outfile);
    //     }
    //     ChainCommands::Brighten { brightness } => {
    //         println!("Brighten infile {} and outfile {}", infile, outfile);
    //         brighten(infile, outfile, brightness);
    //     }
    //     ChainCommands::Crop { x, y, width, height } => {
    //         println!("Crop infile {} and outfile {}", infile, outfile);
    //         crop(infile, outfile, x, y, width, height);
    //     }
    //     ChainCommands::Rotate { rotation } => {
    //         println!("Rotate infile {} and outfile {}", infile, outfile);
    //         rotate(infile, outfile, rotation);
    //     }
    //     ChainCommands::Invert {} => {
    //         println!("Invert infile {} and outfile {}", infile, outfile);
    //         invert(infile, outfile);
    //     }
    //     ChainCommands::Grayscale {} => {
    //         println!("Grayscale infile {} and outfile {}", infile, outfile);
    //         grayscale(infile, outfile);
    //     }
    //     ChainCommands::Fractal {} => {
    //         fractal(outfile);
    //     }
    //     ChainCommands::Square { red, green, blue } => {
    //         generate(outfile, red, green, blue);
    //     }
    //     // ChainCommands::Default(_) => {
    //     //     print_usage_and_exit();
    //     // }
    // }
    // }
}

fn print_usage_and_exit() {
    let mut cmd = Cli::command();
    cmd.print_help().expect("Should print help list");
    // println!("USAGE (when in doubt, use a .png extension on your filenames)");
    // println!("blur INFILE OUTFILE");
    // println!("fractal OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

fn blur(infile: String, outfile: String) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 = img.blur(2.0);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, brightness: i32) {
    // See blur() for an example of how to open / save an image.
    let img = image::open(infile).expect("Failed to open INFILE.");
    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.
    let img2 = img.brighten(brightness);

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
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

fn rotate(infile: String, outfile: String, rotation: Rotation) {
    // See blur() for an example of how to open an image.
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!
    let mut img2: DynamicImage;
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

fn invert(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.
    img.invert();

    // See blur() for an example of how to save the image.
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    // .grayscale() takes no arguments. It returns a new image.
    let img2 = img.grayscale();

    // See blur() for an example of how to save the image.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(outfile: String, mut red: u8, mut green: u8, mut blue: u8) {
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
fn fractal(outfile: String) {
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

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
