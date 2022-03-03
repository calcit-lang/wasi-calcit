pub const CALCIT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_cli<'a>() -> clap::ArgMatches<'a> {
  clap::App::new("Calcit Runner")
    .version(CALCIT_VERSION)
    .author("Jon. <jiyinyiyong@gmail.com>")
    .about("Calcit Runner")
    .arg(
      clap::Arg::with_name("eval")
        .help("eval a snippet")
        .short("e")
        .long("eval")
        .takes_value(true),
    )
    .arg(
      clap::Arg::with_name("dep")
        .help("add dependency")
        .short("d")
        .long("dep")
        .multiple(true)
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
        .help("entry file path, defaults to compact.cirru")
        .default_value("compact.cirru")
        .index(1),
    )
    .get_matches()
}
