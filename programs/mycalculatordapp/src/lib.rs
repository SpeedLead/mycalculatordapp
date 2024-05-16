use anchor_lang::prelude::*;

declare_id!("4TxEXVTzzhwMGjNCGWbKFcGFjJY56xYJUkuuULCL5bNc");

#[program]
pub mod mycalculatordapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
