use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Transfer {
    from: String,
    to: String,
}
