use hemtt_bin_error::Error;

use crate::context::Context;

pub mod archive;
pub mod pbo;

mod binarize;
mod file_patching;
mod files;
mod new;
mod preprocessor;
mod sign;

pub use binarize::Binarize;
pub use file_patching::FilePatching;
pub use files::Files;
pub use new::Licenses;
pub use preprocessor::Preprocessor;
pub use sign::Sign;

pub trait Module {
    fn name(&self) -> &'static str;
    fn init(&mut self, _ctx: &Context) -> Result<(), Error> {
        Ok(())
    }
    fn check(&self, _ctx: &Context) -> Result<(), Error> {
        Ok(())
    }
    fn pre_build(&self, _ctx: &Context) -> Result<(), Error> {
        Ok(())
    }
    fn post_build(&self, _ctx: &Context) -> Result<(), Error> {
        Ok(())
    }
    fn pre_release(&self, _ctx: &Context) -> Result<(), Error> {
        Ok(())
    }
    fn post_release(&self, _ctx: &Context) -> Result<(), Error> {
        Ok(())
    }
}
