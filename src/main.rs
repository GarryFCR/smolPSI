mod utils;
mod setup;
use crate::setup::party;
fn main() {
    let c = party::Party::new(vec!["Hello".to_string(), "World".to_string()]);
    println!(
        "{:#?},{:#?},{:#?}",
        c.interests, c.key.privkey, c.key.pubkey
    );
}
