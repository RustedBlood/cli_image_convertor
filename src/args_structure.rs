use clap::{Parser, builder::Str};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[arg(short, long)]
    pub file: String, //Имя файла для конвертации

    #[arg(short, long)]
    pub extension: String,

    #[arg(short, long)]
    pub name: String, //Имя файла после конвертации
}
