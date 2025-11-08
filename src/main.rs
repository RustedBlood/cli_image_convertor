use clap::Parser;

mod args_structure;
mod img_convertor;

fn main() {
    let args = args_structure::Args::parse();
    img_convertor::convert_image(args.file, args.extension, args.name);
}
