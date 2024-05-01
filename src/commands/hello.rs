use crate::commands::{Context, Error};

#[poise::command(slash_command)]
pub async fn hello(
    ctx: Context<'_>
) -> Result<(), Error> {
    let response = "world";
    ctx.say(response).await?;
    Ok(())
}
