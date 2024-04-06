use anyhow::{bail, Context};
use clap::{Args, Parser, Subcommand};
use image::{GenericImageView, ImageBuffer, Pixel, SubImage};
use std::{ops::Deref, path::PathBuf};

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    cmd: Command,
}

/// Doc comment
#[derive(Debug, Subcommand)]
enum Command {
    /// Scan a page of handwriting and extract the letters as individual images.
    ///
    /// ## Prerequisites
    ///
    /// This app needs a scanned image of a page of handwriting to work. A
    /// template is included in this app's repository. The file is named
    /// `handwriting-scan-grid.png`. Then, download and print the template.
    ///
    /// Next, fill in the boxes with the letterforms or symbols, one per cell.
    /// You don't need to fill in every box; Feel free to leave some empty.
    ///
    /// Scan the sheet of paper, preferably at 300 DPI. Then, pass the scan
    /// image's file path as an argument to this command.
    ///
    /// _Good luck!_ － Zelda
    #[command()]
    Scan(ScanArgs),
}

/// Doc comment
#[derive(Args, Debug)]
struct ScanArgs {
    /// The image file that will be scanned.
    ///
    /// See
    /// https://github.com/image-rs/image/blob/main/README.md#supported-image-formats
    /// for a list of supported image formats.
    ///
    /// An example handwriting scan image is included in this app's repository.
    /// The file is named `example-handwriting-scan.jpeg`.
    #[arg(short, long)]
    input_file: PathBuf,

    /// The directory that the letter images will be written to. If not
    /// provided, the images will be written to the current working directory.
    #[arg(short, long)]
    output_dir: Option<PathBuf>,

    /// By default, the app will ask for confirmation before saving the images.
    /// If you want to skip confirmation, pass this flag.
    #[arg(short, long, default_value_t = false)]
    yes: bool,

    /// Threshold value to use during processing. The default value is 190.
    /// This value should be between 0 and 255.
    #[arg(short, long, default_value = "190")]
    threshold: u8,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args.cmd {
        Command::Scan(scan_args) => {
            scan(scan_args)?;
        }
    }

    Ok(())
}

fn scan(args: ScanArgs) -> anyhow::Result<()> {
    let ScanArgs {
        input_file,
        output_dir,
        yes,
        threshold,
    } = args;
    // validate input file
    if !input_file.is_file() {
        bail!("input_file path doesn't exist or is not a file.");
    }

    // validate output directory
    let output_dir = match output_dir {
        Some(dir) => dir,
        None => std::env::current_dir()?,
    };
    if output_dir.is_file() {
        bail!("output_dir path must be a directory.");
    }

    println!("Loading image...");
    let mut image = image::open(&input_file).context("opening input_file")?;
    if image.height() > image.width() {
        image = image.rotate270();
    }
    println!("Scanning handwriting...");
    // Generic sharpening filter
    let image = image.filter3x3(&[0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0]);
    // Threshold the image
    let image = imageproc::contrast::threshold(&image.to_luma8(), threshold);
    // Crop the border
    let (width, height) = image.dimensions();
    let x = (width as f32 * 0.066).floor() as u32;
    let y = (height as f32 * 0.079).floor() as u32;
    let width = width - x * 2;
    let height = height - y * 2 + (y as f32 * 0.2) as u32;

    let image = image.view(x, y, width, height).to_image();

    let letter_images = grid_cut_image(&image, 12, 9);

    println!(
        "Scan complete; {} letterforms were detected.",
        letter_images.len()
    );
    let confirmation = if yes {
        true
    } else {
        loop {
            println!("OK to save the images? (y/n)");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input.starts_with('y') {
                break true;
            } else if input.starts_with('n') {
                break false;
            } else {
                println!("Invalid input.");
            }
        }
    };
    if confirmation {
        println!("Saving images...");
        std::fs::create_dir_all(&output_dir).context("creating output dir")?;
        for (i, letter_image) in letter_images.iter().enumerate() {
            let output_file = output_dir.join(format!("letter-{}.jpeg", i));
            letter_image.to_image().save(&output_file)?;
        }
        println!("Images saved successfully.");
    } else {
        println!("Very well. Exiting without saving...");
    }

    Ok(())
}

fn grid_cut_image<P, Container>(
    image_buffer: &ImageBuffer<P, Container>,
    width: u32,
    height: u32,
) -> Vec<SubImage<&ImageBuffer<P, Container>>>
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]>,
{
    let mut letter_images = Vec::with_capacity((width * height) as usize);
    // Divide the image into a grid of cells. Then, push each cell into the
    // `letter_images` vec before returning it.
    let (image_width, image_height) = image_buffer.dimensions();
    let cell_width = image_width / width;
    let cell_height = image_height / height;

    for row in 0..height {
        for col in 0..width {
            let x = col * cell_width;
            let y = row * cell_height;
            let sub_image = image_buffer.view(x, y, cell_width, cell_height);
            letter_images.push(sub_image);
        }
    }

    letter_images
}

/*
glyphs
1 2 3 4 5 6 7 8 9 0 - plus
! @ # $ % ^ & * ( ) _ 
, . / ; ' [ ] \ < > ? : " { } | ` ~ 

*/