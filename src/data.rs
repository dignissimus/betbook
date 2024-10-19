use serde::Serialize;

#[derive(Serialize)]
pub struct Event {
    pub identifier: String,
    pub author_identifier: String,
    pub title: String,
}

#[derive(Serialize)]
pub struct User {
    pub identifier: String,
    pub name: String,
}
/// Odds are written in the perspective of the bet taker
#[derive(Serialize)]
pub struct BetOffer {
    pub stake: u64,
    pub payout: u64,
    pub event_identifier: String,
    pub direction: Outcome,
}

#[derive(Serialize)]
pub enum Outcome {
    No,
    Yes,
}
