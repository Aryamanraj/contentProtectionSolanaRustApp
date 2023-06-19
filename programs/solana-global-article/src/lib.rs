use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("HAF3UFQkuLGsM1shC9dB5wB4MDrWfpDiYv81AimzuEDT");

// #[program]
// pub mod solana_global_article {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         Ok(())
//     }
// }
#[program]
pub mod solana_global_article {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        // Get the article
        let article_account = &mut ctx.accounts.article;
        // Initialize the variables (this is required)
        article_account.content = ("").to_string();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = person_that_pays,
        space = 8 // account discriminator
        + 32 // pubkey
        + 10000 // make the message max 10k bytes long
    )]
    pub article: Account<'info, Article>,
    #[account(mut)]
    pub person_that_pays: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Article {
    pub content: String,
}

#[derive(Accounts)]
pub struct WriteIntoArticle<'info> {
    // Here goes the info that you want to modify like this
    #[account(mut)]
    pub article: Account<'info, Article>,
}