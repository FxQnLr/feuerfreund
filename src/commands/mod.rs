mod help;
mod minecraft;

pub use help::help;
pub use minecraft::minecraft;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, crate::Data, Error>;

async fn reply<'rep>(ctx: Context<'rep>, content: &str) -> Result<poise::ReplyHandle<'rep>, serenity::Error> {
    let reply = poise::CreateReply::default()
        .content(content)
        .ephemeral(ctx.data().config.ephemeral_replies);

    ctx.send(reply).await
}
