use clap::{Parser, Subcommand, Args};

const LAST_LONG_HELP: &str = "If specified, will only return messages that were added in the \
provided range. Format is {NUMBER}{GRANULARITY}, where granularity can be either \
s (seconds), m (minutes), h (hours) or d(days). For example, '1d' translates to 'fetch messages \
added in the last day (24 hours)'.";

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CLIParser {
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Subcommand)]
pub enum Commands {
    Add(AddArgs),
    Delete(DeleteArgs),
    List(ListArgs),
    Daemon
}

#[derive(Args)]
pub struct AddArgs {
    pub text: String,
    pub notify_after: Option<String>,
}


#[derive(Args)]
pub struct DeleteArgs {
    pub id: i32,
}


#[derive(Args)]
pub struct ListArgs {
    #[arg(long_help=LAST_LONG_HELP)]
    pub last: Option<String>,
    #[arg(short='l', long="limit", help="Max number of messages to fetch")]
    pub limit: Option<i64>,
    #[arg(short='o', long="offset", help="Number of messages to skip from result set")]
    pub offset: Option<i64>
}
