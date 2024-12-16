mod party;
mod utils;

use crate::party::*;
fn main() {
    // First list of strings
    let list1 = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
    ];

    // Second list of strings
    let list2 = vec![
        "cherry".to_string(),
        "date".to_string(),
        "elderberry".to_string(),
        "fig".to_string(),
    ];

    let p1 = &mut Party::new(list1, Partytype::Sender); //Sender
    let p2 = &mut Party::new(list2, Partytype::Receiver); //Reciever

    let m=p1.send_round1();
    let poly = p2.recv_round1();

    let k: Vec<[u8; 32]> = p1.send_round2(poly);

    let list = p2.recv_round2(k, m);
    println!("The Set Intersection as computed by {:?}: {:?}",p2.party_type,list)

}