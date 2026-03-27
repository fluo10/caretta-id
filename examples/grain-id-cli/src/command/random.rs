use grain_id::GrainId;
use clap::Args;

#[derive(Args, Debug)]
/// Generate random grain-id
pub struct RandomCommandArgs;

impl RandomCommandArgs {
    pub fn run(self) {
        println!("{}", rand::random::<GrainId>())
    }
}
