use crate::context::Context;
use anyhow::Result;
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum FetchMethod {
    Http,
    Git,
}

pub fn run(context: &mut Context, method: FetchMethod) -> Result<()> {
    todo!();
}
