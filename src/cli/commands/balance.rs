use monobank_api::Client;

use crate::cli::commands::helpers::get_token;

pub(crate) async fn get_balance() -> Result<f32, Box<dyn std::error::Error>> {
    let token = get_token()?;

    let client = Client::new()?.with_token(&token)?;

    let info = client.account_info().await?;

    match info.main_card() {
        Some(card) => Ok(card.balance),
        None => Err(Box::new(std::io::Error::other("main card not found"))),
    }
}
