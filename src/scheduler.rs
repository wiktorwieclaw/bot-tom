use std::sync::Arc;

use anyhow::Context;
use delay_timer::prelude::*;
use serenity::prelude::Context as SerenityContext;

use crate::{configuration::Configuration, task};

#[tracing::instrument("Spawning scheduler", skip_all)]
pub fn spawn_scheduler(
    configuration: Arc<Configuration>,
    ctx: Arc<SerenityContext>,
) -> anyhow::Result<TaskInstancesChain> {
    let cenzo = task::cenzo::build_cenzo(Arc::clone(&configuration), Arc::clone(&ctx))
        .context("Failed to build cenzo task")?;
    let scheduler = DelayTimer::new()
        .insert_task(cenzo)
        .context("Failed to insert barka task")?;
    Ok(scheduler)
}
