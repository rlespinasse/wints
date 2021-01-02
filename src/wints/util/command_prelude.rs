use clap::{AppSettings, SubCommand};

pub type App = clap::App<'static, 'static>;
pub type Arg = clap::Arg<'static, 'static>;

pub fn subcommand(name: &'static str) -> App {
    SubCommand::with_name(name).settings(&[
        AppSettings::UnifiedHelpMessage,
        AppSettings::DeriveDisplayOrder,
        AppSettings::DontCollapseArgsInUsage,
    ])
}

pub fn arg(name: &'static str) -> Arg {
    Arg::with_name(name)
}
