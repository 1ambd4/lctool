use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// update local database
    Cache,

    /// show today question
    Today,

    /// query question with id
    #[command(arg_required_else_help = true)]
    Query(IdArgs),

    /// write question with id
    #[command(arg_required_else_help = true)]
    Write(IdArgs),

    /// show lctool info
    Info,
}

#[derive(Args, Debug)]
pub struct IdArgs {
    /// question id
    pub id: i32,
}
