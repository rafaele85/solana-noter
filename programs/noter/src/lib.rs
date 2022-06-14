use anchor_lang::prelude::*;

declare_id!("CuQie8F24BKWSLrXnagBQd9vwKo6ndA8utPnAqsSSF8j");

#[program]
pub mod noter {
    use super::*;

    pub fn create_note(ctx: Context<CreateNote>, content: String) -> Result<()> {
        let note = &mut ctx.accounts.note;
        let user = &mut ctx.accounts.user;

        note.content = content;
        note.user = *user.key;

        Ok(())

    }
}

#[derive(Accounts)]
pub struct CreateNote<'info> {
    #[account(init, payer = user, space = 2000)]
    pub note: Account<'info, Note>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Note {
    pub content: String,
    pub user: Pubkey,
}
