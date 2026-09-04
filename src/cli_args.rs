pub const CALCIT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_cli() -> clap::ArgMatches {
  clap::App::new("Calcit Runner")
    .version(CALCIT_VERSION)
    .author("Jon. <jiyinyiyong@gmail.com>")
    .about("Calcit Runner")
    .arg(
      clap::Arg::with_name("emit-js")
        .help("emit js rather than interpreting")
        .long("emit-js")
        .takes_value(false),
    )
    .arg(
      clap::Arg::with_name("emit-ir")
        .help("emit EDN representation of program to program-ir.cirru")
        .long("emit-ir")
        .takes_value(false),
    )
    .arg(
      clap::Arg::with_name("once")
        .help("run once and exit")
        .long("once")
        .takes_value(false),
    )
    .arg(
      clap::Arg::with_name("eval")
        .help("eval a snippet")
        .short('e')
        .long("eval")
        .takes_value(true),
    )
    .arg(
      clap::Arg::with_name("dep")
        .help("add dependency")
        .short('d')
        .long("dep")
        .multiple(true)
        .takes_value(true),
    )
    .arg(
      clap::Arg::with_name("emit-path")
        .help("emit directory for js, defaults to `js-out/`")
        .long("emit-path")
        .takes_value(true),
    )
    .arg(
      clap::Arg::with_name("init-fn")
        .help("overwrite `init_fn`")
        .long("init-fn")
        .takes_value(true),
    )
    .arg(
      clap::Arg::with_name("reload-fn")
        .help("overwrite `reload_fn`")
        .long("reload-fn")
        .takes_value(true),
    )
    .arg(
      clap::Arg::with_name("entry")
        .help("overwrite with config entry")
        .long("entry")
        .takes_value(true),
    )
    .arg(
      clap::Arg::with_name("input")
        .help("entry file path, defaults to calcit.cirru")
        .default_value("calcit.cirru")
        .index(1),
    )
    .get_matches()
}
