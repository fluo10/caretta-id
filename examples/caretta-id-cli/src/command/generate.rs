use caretta_id::{CarettaId, CarettaIdD, CarettaIdQ, CarettaIdS, CarettaIdT};
use clap::Args;

use crate::option::{LengthOption, LengthOptionArgs};

#[derive(Args, Debug)]
/// (deprecated) Generate random caretta-id
pub struct GenerateArgs {
    #[command(flatten)]
    length: LengthOptionArgs,
}

impl GenerateArgs {
    pub fn run(self) {
        match LengthOption::from(self.length) {
            LengthOption::Single => println!("{}", rand::random::<CarettaIdS>()),
            LengthOption::Double => println!("{}", rand::random::<CarettaIdD>()),
            LengthOption::Triple => println!("{}", rand::random::<CarettaIdT>()),
            LengthOption::Quadruple => println!("{}", rand::random::<CarettaIdQ>()),
            LengthOption::Unspecified => println!("{}", rand::random::<CarettaId>()),
        }
    }
}
