# Handwriting Scan Tool

Scan a page of handwriting and extract the letters as individual images. I use
this tool when creating fonts based on handwriting.

## Installation

```sh
cargo install handwriting-scan-tool --locked
```

## Prerequisites

This app needs a scanned image of a page of handwriting to work. A template is
included in this app's repository. The file is named
`handwriting-scan-grid.png`. Then, download and print the template.

Next, fill in the boxes with the letterforms or symbols, one per cell. You don't
need to fill in every box; Feel free to leave some empty.

Scan the sheet of paper, preferably at 300 DPI. Then, pass the scan image's file
path as an argument to this command. It's important that the scanned image is
straight, otherwise some characters may be cropped poorly.

## Usage

Once the images are scanned, it's time to extract the letters.

```sh
handwriting-scan-tool scan -i ./example-handwriting-scan.jpeg -o ./scans"
```

The command above will extract the letters from the scanned image and save them
to the `./scans` directory. Once all the letters are extracted, I drag-and-drop
them one-by-one into [Glyphs] and start tracing.

_Happy fonting!_ － Zelda

[Glyphs]: https://glyphsapp.com