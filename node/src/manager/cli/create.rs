use std::sync::Arc;

use graph::prelude::Error;
use graph_store_postgres::SubgraphStore;

use crate::manager::core;

pub fn run(store: Arc<SubgraphStore>, name: String) -> Result<(), Error> {
    let name = core::create::run(store, name, true)?;

    println!("created subgraph {}", name);

    Ok(())
}
