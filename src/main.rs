use chrono::Utc;
use slack_hook::{PayloadBuilder, Slack};

const SLACK_WEBHOOK_URL_VAR: &str = "SLACK_WEBHOOK_URL";
const SLACK_CHANNEL_VAR: &str = "SLACK_CHANNEL";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let slack = std::env::var(SLACK_WEBHOOK_URL_VAR)
        .expect(&format!("{} not configured", SLACK_WEBHOOK_URL_VAR));
    let slack = Slack::new(slack.as_str()).expect("failed to instantiate Slack");

    let channel =
        std::env::var(SLACK_CHANNEL_VAR).expect(&format!("{} not configured", SLACK_CHANNEL_VAR));

    let post = |text| {
        let p = PayloadBuilder::new()
            .username("ojigineko")
            .icon_emoji(":pizzacat83:")
            .channel(channel)
            .text(text)
            .build()?;

        slack.send(&p)
    };

    let path = "state.json".into();
    let mut store = ojigineko_life::Store::new(path)?;
    let ojigineko = store.ojigineko();

    if ojigineko.forward(Utc::now()).is_none() {
        eprintln!("no forward...");
        return Ok(());
    }

    post(ojigineko.to_text())?;

    Ok(())
}
