use clap::Parser;

#[derive(Parser)]
pub enum RainbowCli {
    Generate(GenerateArgs),
    Crack(CrackArgs),
    #[cfg(debug_assertions)]
    Debug(DebugArgs),
}

#[derive(clap::Args)]
pub struct GenerateArgs {
    #[arg(short = 'C')]
    pub charset: Option<String>,
    #[arg(short = 'l')]
    pub chain_length: Option<u32>,
    #[arg(short = 'n')]
    pub chain_number: Option<usize>,
    #[arg(short)]
    pub password_length: Option<u32>,
    #[arg(short)]
    pub output_file: Option<String>,
}

#[derive(clap::Args)]
pub struct CrackArgs {
    pub hash: String,
    #[arg(short = 'I')]
    pub input_file: Option<String>,
}

#[cfg(debug_assertions)]
#[derive(clap::Args)]
pub struct DebugArgs {
    // no args
}
