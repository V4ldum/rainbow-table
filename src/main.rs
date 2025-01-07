use crate::cli::*;
use clap::Parser;
use rainbow::{RainbowTableBuilder, RainbowTableConfig};

mod cli;

fn main() {
    let args = RainbowCli::parse();

    match args {
        RainbowCli::Generate(args) => generate(args),
        RainbowCli::Crack(args) => crack(args),
        #[cfg(debug_assertions)]
        RainbowCli::Debug(_) => debug(),
    }
}

fn generate(args: GenerateArgs) {
    let table = RainbowTableBuilder::new(RainbowTableConfig {
        charset: args
            .charset
            .unwrap_or("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string()),
        chain_length: args.chain_length.unwrap_or(5),
        chain_number: args.chain_number.unwrap_or(10),
        password_length: args.password_length.unwrap_or(12),
        #[cfg(debug_assertions)]
        debug: false,
    })
    .generate();

    if let Some(output_file) = args.output_file {
        table.write_to_file(&output_file).expect("filename should be correct");
    } else {
        println!("{table}");
    }
}

fn crack(args: CrackArgs) {
    let hash = args.hash;
    let input_file = format!("./{}", args.input_file.unwrap_or("table.txt".into()));

    let table = RainbowTableBuilder::from_file(&input_file).expect("The file should be properly formatted");

    let result = table.crack(&hash);

    match result {
        Ok(clear) => println!("Found cleartext for {hash}: {clear}"),
        Err(error) => println!("{error}"),
    }
}
#[cfg(debug_assertions)]
fn debug() {
    let table = RainbowTableBuilder::new(RainbowTableConfig {
        charset: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string(),
        chain_length: 3,
        chain_number: 2,
        password_length: 8,
        #[cfg(debug_assertions)]
        debug: true,
    })
    .generate();

    // The generated table is as follows :
    // ObdcZGEh-> 4c66ef60d6f45926830e8dfc6867b13f -> GlNNswN0-> bf344d84f2088a2644b303f0fb50ffe1 -> wCGGoVGw-> 4993f791810d44968eb5b8596a5bb8b8 -> ZCwRZ4V4
    // 2WA9Pfo5-> e1821241bddd1fd9d3e8ea33012e5638 -> s4V848G4-> 58bfcd8f3c5522647966fe15d6c125af -> VhwloVwC-> 30ceed6211ef66b7997f48f10af199bc -> lssoN844

    // First chain
    let Ok(hash) = table.crack("4c66ef60d6f45926830e8dfc6867b13f") else {
        panic!("This hash should be cracked");
    };
    assert_eq!(hash, "ObdcZGEh");
    let Ok(hash) = table.crack("bf344d84f2088a2644b303f0fb50ffe1") else {
        panic!("This hash should be cracked");
    };
    assert_eq!(hash, "GlNNswN0");
    let Ok(hash) = table.crack("4993f791810d44968eb5b8596a5bb8b8") else {
        panic!("This hash should be cracked");
    };
    assert_eq!(hash, "wCGGoVGw");
    let Err(_) = table.crack("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa") else {
        panic!("This hash should not be cracked");
    };

    // Second chain
    let Ok(hash) = table.crack("e1821241bddd1fd9d3e8ea33012e5638") else {
        panic!("This hash should be cracked");
    };
    assert_eq!(hash, "2WA9Pfo5");
    let Ok(hash) = table.crack("58bfcd8f3c5522647966fe15d6c125af") else {
        panic!("This hash should be cracked");
    };
    assert_eq!(hash, "s4V848G4");
    let Ok(hash) = table.crack("30ceed6211ef66b7997f48f10af199bc") else {
        panic!("This hash should be cracked");
    };
    assert_eq!(hash, "VhwloVwC");
    let Err(_) = table.crack("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb") else {
        panic!("This hash not should be cracked");
    };
}
