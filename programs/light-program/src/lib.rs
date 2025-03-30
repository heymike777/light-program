use anchor_lang::prelude::*;

declare_id!("LiGHTBN3hMBuTXgBSsrikpM59s2isfF7p8eAznkFfnu");

#[program]
pub mod light_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
