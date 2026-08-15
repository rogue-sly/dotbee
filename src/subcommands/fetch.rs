use crate::context::Context;
use anyhow::Result;
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum FetchMethod {
    Http,
    Git,
}

pub fn run(_context: &mut Context, _method: FetchMethod) -> Result<()> {
    todo!();
}
