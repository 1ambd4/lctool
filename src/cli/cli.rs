use crate::cmd::{Command, DataCommand, EditCommand, InfoCommand, ListCommand};
use anyhow::Result;

pub async fn cli_main() -> Result<()> {
    let mut cmd = clap::Command::new("leetcode")
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .author(clap::crate_authors!())
        .subcommands(vec![
            DataCommand::usage().display_order(1),
            EditCommand::usage().display_order(2),
            InfoCommand::usage().display_order(3),
            ListCommand::usage().display_order(4),
        ])
        .arg_required_else_help(true);

    let m = cmd.clone().get_matches();

    match m.subcommand() {
        Some(("data", m)) => Ok(DataCommand::handler(m).await?),
        Some(("edit", m)) => Ok(EditCommand::handler(m).await?),
        Some(("info", m)) => Ok(InfoCommand::handler(m).await?),
        Some(("list", m)) => Ok(ListCommand::handler(m).await?),
        _ => unreachable!(),
    }
}
