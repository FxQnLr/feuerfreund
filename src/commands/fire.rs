use super::{reply, Context, Error};

#[derive(poise::ChoiceParameter)]
#[allow(non_camel_case_types)]
enum Subcommands {
    connect,
    disconnect,
}

#[poise::command(slash_command)]
pub async fn fire(
    ctx: Context<'_>,
    #[description = "fire commands"] command: Subcommands,
) -> Result<(), Error> {
    match command {
        Subcommands::connect => {
            reply(ctx, "connect").await?;
        }
        Subcommands::disconnect => {
            reply(ctx, "disconnect").await?;
        }
    }
    Ok(())
}
