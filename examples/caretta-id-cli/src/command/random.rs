use caretta_id::CarettaId;
use clap::Args;

#[derive(Args, Debug)]
/// Generate random caretta-id
pub struct RandomCommandArgs;

impl RandomCommandArgs {
    pub fn run(self) {
        println!("{}", rand::random::<CarettaId>())
    }
}
