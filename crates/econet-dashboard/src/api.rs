use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::storage::{band_for_score, load_latest_shard, EcoNode};

#[derive(Serialize)]
struct NodeView {
    node_id: String,
    contaminant: String,
    k_n: f64,
    ecoimpact_band: f64,
}

async fn list_nodes() -> Json<Vec<NodeView>> {
    let shard = load_latest_shard("data/ceim").ok().flatten();
    let mut out = Vec::new();
    if let Some(s) = shard {
        for n in s.nodes {
            let band = band_for_score(n.k_n);
            out.push(NodeView {
                node_id: n.node_id,
                contaminant: n.contaminant,
                k_n: n.k_n,
                ecoimpact_band: band,
            });
        }
    }
    Json(out)
}

pub fn app() -> Router {
    Router::new().route("/nodes", get(list_nodes))
}
