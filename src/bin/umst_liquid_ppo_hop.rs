// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Stdin hop-graph JSON → Burn [`BurnLiquidPPOAgent::step_and_learn`] receipt on stdout.
//! Fail closed on I=0 occupancy (licence not to look). Never physics GREEN / CHEM_ECO.

use std::io::{self, Read};
use umst_manifold::ai::hop_graph_liquid_ppo::{step_and_learn_hop_graph, HopGraph};

fn main() {
    let mut raw = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut raw) {
        eprintln!("umst-liquid-ppo-hop: stdin: {e}");
        std::process::exit(2);
    }
    let graph: HopGraph = match serde_json::from_str(raw.trim()) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("umst-liquid-ppo-hop: json: {e}");
            std::process::exit(2);
        }
    };
    let receipt = step_and_learn_hop_graph(&graph);
    println!("{}", serde_json::to_string_pretty(&receipt).unwrap());
    if receipt.occupancy_refused {
        std::process::exit(0);
    }
    if !receipt.step_and_learn_ran {
        std::process::exit(1);
    }
}
