use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ProofMessage {
    ok: bool,
    source: &'static str,
}

fn main() {
    let message = ProofMessage {
        ok: true,
        source: "cinder-proof",
    };

    println!("{}", serde_json::to_string(&message).unwrap());
}
