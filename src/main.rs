extern crate wasm_pack;

extern crate indicatif;
#[macro_use]
extern crate quicli;
#[macro_use]
extern crate human_panic;

use quicli::prelude::*;
use wasm_pack::context::Context;
use wasm_pack::Cli;

main!(|args: Cli, log_level: verbosity| {
    setup_panic!();
    let mut context = Context::from(args);
    context.run()?;
});
