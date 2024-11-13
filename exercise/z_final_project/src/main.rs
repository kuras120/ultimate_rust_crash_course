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

use clap::Parser;
use std::path::Path;
use mirage::{blur, brighten, crop, fractal, generate, grayscale, invert, rotate,
             split_command_vector, ChainCommands, Cli};

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let cli = Cli::parse();
    println!("{:?}", cli.command_vector);
    let chain_commands = split_command_vector(&cli.command_vector);
    for (index, &command) in chain_commands.iter().enumerate() {
        let infile: Option<String> = if Path::new(&cli.outfile).exists() && index != 0 {
            Some(cli.outfile.clone())
        } else {
            cli.infile.clone()
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
        }
    }
    if Path::new(&cli.outfile).exists() {
        println!("Result was generated at {:?}", Path::new(&cli.outfile).canonicalize().unwrap());
    } else {
        println!("No file was generated. Provide infile or generate image first via fractal or square function");
    }
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
