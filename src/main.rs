use anyhow::Result;
use comparator::ApiComparator;

use crate::git::{CrateRepo, GitBackend};

mod ast;
mod comparator;
mod git;
mod glue;
mod public_api;

fn main() -> Result<()> {
    let mut repo = CrateRepo::current()?;

    let current = glue::extract_api()?;

    dbg!("Checking out to default branch");
    repo.switch_to(git::DEFAULT_BRANCH_NAME)?;

    let previous = glue::extract_api()?;

    dbg!("Checking out back to initial branch");
    repo.switch_back()?;

    let api_comparator = ApiComparator::new(previous, current);

    let diagnosis = api_comparator.run();

    dbg!(diagnosis);

    Ok(())
}
