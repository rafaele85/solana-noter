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

    pub fn delete_note(ctx: Context<DeleteNote>) -> Result<()> {
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

#[derive(Accounts)]
pub struct DeleteNote<'info> {
    #[account(mut, has_one = user, close = user)]
    pub note: Account<'info, Note>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Note {
    pub content: String,
    pub user: Pubkey,
}
