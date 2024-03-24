use crate::{Context, Error};


#[poise::command(prefix_command, slash_command)]
pub async fn authorize(
    ctx: &Context,
    #[description = "Your dashboard username"] username: String,
    #[description = "Your dashboard password"] password: String
) -> Result<(), Error> {
    
}