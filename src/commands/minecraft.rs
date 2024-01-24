use super::{reply, Context, Error};

#[derive(poise::ChoiceParameter)]
#[allow(non_camel_case_types)]
enum Subcommands {
    // start,
    status,
}

#[poise::command(slash_command)]
pub async fn minecraft(
    ctx: Context<'_>,
    #[description = "Description of arg1 here"] command: Subcommands,
) -> Result<(), Error> {
    match command {
        // Subcommands::start => {
        //     reply(ctx, "starting").await?;
        // }
        Subcommands::status => {
            let stats = mcsc_query::full_stats(&ctx.data().config.mc_server_ip)?;
            reply(ctx, &format!("{stats:#?}")).await?;
        },
    }
    Ok(())
}
