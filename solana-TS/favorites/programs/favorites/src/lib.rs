use anchor_lang::prelude::*;

declare_id!("3GyHVeksm844U5o6mTevca329BDet9HZee3jtmnfQc8G");

#[program]
pub mod favorites {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
