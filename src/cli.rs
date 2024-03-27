use clap::{Arg, Command, ArgAction};

pub fn build_cli() -> Command {
    Command::new("Plexisort")
        .version("1.0")
        .author("Richard Chukwu <richinex@gmail.com>")
        .about("Organizes files by metadata")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .action(ArgAction::Set)
            .num_args(1))
        .arg(Arg::new("source")
            .long("source")
            .value_name("SOURCE_DIR")
            .help("Sets the source directory(s)")
            .action(ArgAction::Append)
            .num_args(1..))
        .arg(Arg::new("destination")
            .long("destination")
            .value_name("DEST_DIR")
            .help("Sets the destination directory")
            .action(ArgAction::Set)
            .num_args(1))
        .arg(Arg::new("dry-run")
            .long("dry-run")
            .help("Runs the organizer without making any changes")
            .action(ArgAction::Set))
        .arg(Arg::new("undo")
            .long("undo")
            .help("Reverts the last set of file movements")
            .action(ArgAction::Set))
}