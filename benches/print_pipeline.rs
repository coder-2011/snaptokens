use std::error::Error;

use snaptokens::Tokenizer;

fn main() -> Result<(), Box<dyn Error>> {
    let Some(tokenizer_json) = std::env::args().nth(1) else {
        return Err("specify a tokenizer.json path".into());
    };

    let tok = Tokenizer::load_file(std::path::Path::new(&tokenizer_json))?;

    println!("Tokenizer: {tokenizer_json}\n");

    println!("Normalizer:      {}", opt_debug(tok.normalizer()));
    println!("Pre-tokenizer:   {}", opt_debug(tok.pre_tokenizer()));
    println!("Model:           {:?}", tok.model());

    Ok(())
}

fn opt_debug<T: std::fmt::Debug>(v: Option<&T>) -> String {
    v.map_or_else(|| "(none)".to_owned(), |step| format!("{step:#?}"))
}
