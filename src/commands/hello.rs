use crate::commands::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn hello(
    ctx: Context<'_>
    //#[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let response = "world";
    ctx.say(response).await?;
    Ok(())
}
