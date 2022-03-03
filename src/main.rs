use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::time::Instant;

use im_ternary_tree::TernaryTreeList;

use calcit::{
  builtins,
  call_stack::CallStackList,
  cli_args,
  primes::{Calcit, CalcitErr, CalcitItems},
  program, runner, snapshot, util, ProgramEntries,
};

fn main() -> Result<(), String> {
  builtins::effects::init_effects_states();

  builtins::register_import_proc("echo", echo);
  builtins::register_import_proc("println", echo);

  let cli_matches = cli_args::parse_cli();
  let entry_path = Path::new(cli_matches.value_of("input").unwrap()).to_owned();

  println!("calcit version: {}", cli_args::CALCIT_VERSION);

  let core_snapshot = calcit::load_core_snapshot()?;

  let mut snapshot = snapshot::gen_default(); // placeholder data

  if let Some(snippet) = cli_matches.value_of("eval") {
    match snapshot::create_file_from_snippet(snippet) {
      Ok(main_file) => {
        snapshot.files.insert(String::from("app.main").into(), main_file);
      }
      Err(e) => return Err(e),
    }
    if let Some(cli_deps) = cli_matches.values_of("dep") {
      for module_path in cli_deps {
        let module_data = calcit::load_module(module_path, entry_path.parent().unwrap())?;
        for (k, v) in &module_data.files {
          snapshot.files.insert(k.to_owned(), v.to_owned());
        }
      }
    }
  } else {
    // load entry file
    let content = fs::read_to_string(&entry_path).unwrap_or_else(|_| panic!("expected Cirru snapshot: {:?}", entry_path));

    let data = cirru_edn::parse(&content)?;
    // println!("reading: {}", content);
    snapshot = snapshot::load_snapshot_data(&data, entry_path.to_str().unwrap())?;

    // config in entry will overwrite default configs
    if let Some(entry) = cli_matches.value_of("entry") {
      if snapshot.entries.contains_key(&*entry) {
        println!("running entry: {}", entry);
        snapshot.configs = snapshot.entries[entry].to_owned();
      } else {
        return Err(format!("unknown entry `{}` among {:?}", entry, snapshot.entries.keys()));
      }
    }

    // attach modules
    for module_path in &snapshot.configs.modules {
      let module_data = calcit::load_module(module_path, entry_path.parent().unwrap())?;
      for (k, v) in &module_data.files {
        snapshot.files.insert(k.to_owned(), v.to_owned());
      }
    }
  }
  let init_fn = cli_matches.value_of("init-fn").or(Some(&snapshot.configs.init_fn)).unwrap();
  let reload_fn = cli_matches.value_of("reload-fn").or(Some(&snapshot.configs.reload_fn)).unwrap();
  let (init_ns, init_def) = util::string::extract_ns_def(init_fn)?;
  let (reload_ns, reload_def) = util::string::extract_ns_def(reload_fn)?;
  let entries: ProgramEntries = ProgramEntries {
    init_fn: init_fn.into(),
    reload_fn: reload_fn.into(),
    init_def: init_def.into(),
    init_ns: init_ns.into(),
    reload_ns: reload_ns.into(),
    reload_def: reload_def.into(),
  };

  // attach core
  for (k, v) in core_snapshot.files {
    snapshot.files.insert(k.to_owned(), v.to_owned());
  }

  // now global states
  {
    let mut prgm = { program::PROGRAM_CODE_DATA.write().unwrap() };
    *prgm = program::extract_program_data(&snapshot)?;
  }

  let check_warnings: &RefCell<Vec<String>> = &RefCell::new(vec![]);

  // make sure builtin classes are touched
  runner::preprocess::preprocess_ns_def(
    calcit::primes::CORE_NS.into(),
    calcit::primes::BUILTIN_CLASSES_ENTRY.into(),
    calcit::primes::BUILTIN_CLASSES_ENTRY.into(),
    None,
    check_warnings,
    &rpds::List::new_sync(),
  )
  .map_err(|e| e.msg)?;

  let started_time = Instant::now();

  let v = calcit::run_program(entries.init_ns.to_owned(), entries.init_def, TernaryTreeList::Empty).map_err(|e| {
    for w in e.warnings {
      eprintln!("{}", w);
    }
    e.msg
  })?;

  let duration = Instant::now().duration_since(started_time);
  println!("took {}ms: {}", duration.as_micros() as f64 / 1000.0, v);
  Ok(())
}

pub fn echo(xs: &CalcitItems, _call_stack: &CallStackList) -> Result<Calcit, CalcitErr> {
  let mut s = String::from("");
  for (idx, x) in xs.into_iter().enumerate() {
    if idx > 0 {
      s.push(' ');
    }
    s.push_str(&x.turn_string());
  }
  println!("{}", s);
  Ok(Calcit::Nil)
}
