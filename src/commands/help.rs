use crate::VERSION;

use super::{Context, Error};

#[poise::command(
    slash_command,
    description_localized("en-US", "Feuerfreund help"),
    description_localized("en-GB", "Feuerfreund help")
)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {

    let bottom = format!("Feuerfreund v{VERSION}");

    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: &bottom,
        ..Default::default()
    };

    poise::builtins::help(ctx, command.as_deref(), config).await?;

    Ok(())
}
