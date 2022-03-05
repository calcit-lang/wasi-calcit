use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use im_ternary_tree::TernaryTreeList;

use calcit::{
  builtins, call_stack,
  call_stack::CallStackList,
  cli_args, codegen,
  codegen::emit_js::gen_stack,
  codegen::COMPILE_ERRORS_FILE,
  primes::{Calcit, CalcitErr, CalcitItems},
  program, runner, snapshot, util, ProgramEntries,
};

pub struct CLIOptions {
  entry_path: PathBuf,
  emit_path: String,
  emit_js: bool,
  emit_ir: bool,
}

fn main() -> Result<(), String> {
  builtins::effects::init_effects_states();

  builtins::register_import_proc("echo", echo);
  builtins::register_import_proc("println", echo);

  let cli_matches = cli_args::parse_cli();
  let cli_options = CLIOptions {
    // has default value
    entry_path: Path::new(cli_matches.value_of("input").unwrap()).to_owned(),
    emit_path: cli_matches.value_of("emit-path").or(Some("js-out")).unwrap().to_owned(),
    emit_js: cli_matches.is_present("emit-js"),
    emit_ir: cli_matches.is_present("emit-ir"),
  };
  let mut eval_once = cli_matches.is_present("once");

  println!("calcit version: {}", cli_args::CALCIT_VERSION);

  let core_snapshot = calcit::load_core_snapshot()?;

  let mut snapshot = snapshot::gen_default(); // placeholder data

  if let Some(snippet) = cli_matches.value_of("eval") {
    eval_once = true;
    match snapshot::create_file_from_snippet(snippet) {
      Ok(main_file) => {
        snapshot.files.insert(String::from("app.main").into(), main_file);
      }
      Err(e) => return Err(e),
    }
    if let Some(cli_deps) = cli_matches.values_of("dep") {
      for module_path in cli_deps {
        let module_data = calcit::load_module(module_path, cli_options.entry_path.parent().unwrap())?;
        for (k, v) in &module_data.files {
          snapshot.files.insert(k.to_owned(), v.to_owned());
        }
      }
    }
  } else {
    // load entry file
    let content =
      fs::read_to_string(&cli_options.entry_path).unwrap_or_else(|_| panic!("expected Cirru snapshot: {:?}", cli_options.entry_path));

    let data = cirru_edn::parse(&content)?;
    // println!("reading: {}", content);
    snapshot = snapshot::load_snapshot_data(&data, cli_options.entry_path.to_str().unwrap())?;

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
      let module_data = calcit::load_module(module_path, cli_options.entry_path.parent().unwrap())?;
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

  let task = if cli_options.emit_js {
    run_codegen(&entries, &cli_options.emit_path, false)
  } else if cli_options.emit_ir {
    run_codegen(&entries, &cli_options.emit_path, true)
  } else {
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
  };

  if eval_once {
    task?;
  } else {
    // error are only printed in watch mode
    match task {
      Ok(_) => {}
      Err(e) => {
        eprintln!("\nfailed to run, {}", e);
      }
    }
  }

  runner::track::exit_when_cleared();
  Ok(())
}

fn run_codegen(entries: &ProgramEntries, emit_path: &str, ir_mode: bool) -> Result<(), String> {
  let started_time = Instant::now();

  if ir_mode {
    builtins::effects::modify_cli_running_mode(builtins::effects::CliRunningMode::Ir)?;
  } else {
    builtins::effects::modify_cli_running_mode(builtins::effects::CliRunningMode::Js)?;
  }

  let code_emit_path = Path::new(emit_path);
  if !code_emit_path.exists() {
    let _ = fs::create_dir(code_emit_path);
  }

  let js_file_path = code_emit_path.join(format!("{}.js", COMPILE_ERRORS_FILE)); // TODO mjs_mode

  let check_warnings: &RefCell<Vec<String>> = &RefCell::new(vec![]);
  gen_stack::clear_stack();

  // preprocess to init
  match runner::preprocess::preprocess_ns_def(
    entries.init_ns.to_owned(),
    entries.init_def.to_owned(),
    entries.init_def.to_owned(),
    None,
    check_warnings,
    &rpds::List::new_sync(),
  ) {
    Ok(_) => (),
    Err(failure) => {
      eprintln!("\nfailed preprocessing, {}", failure);
      call_stack::display_stack(&failure.msg, &failure.stack)?;

      let _ = fs::write(
        &js_file_path,
        format!(
          "export default \"Preprocessing failed:\\n{}\";",
          failure.msg.trim().escape_default()
        ),
      );
      return Err(failure.msg);
    }
  }

  // preprocess to reload
  match runner::preprocess::preprocess_ns_def(
    entries.reload_ns.to_owned(),
    entries.reload_def.to_owned(),
    entries.init_def.to_owned(),
    None,
    check_warnings,
    &rpds::List::new_sync(),
  ) {
    Ok(_) => (),
    Err(failure) => {
      eprintln!("\nfailed preprocessing, {}", failure);
      call_stack::display_stack(&failure.msg, &failure.stack)?;
      return Err(failure.msg);
    }
  }

  let warnings = check_warnings.to_owned().into_inner();
  throw_on_js_warnings(&warnings, &js_file_path)?;

  // clear if there are no errors
  let no_error_code = String::from("export default null;");
  if !(js_file_path.exists() && fs::read_to_string(&js_file_path).unwrap() == no_error_code) {
    let _ = fs::write(&js_file_path, no_error_code);
  }

  if ir_mode {
    match codegen::gen_ir::emit_ir(&*entries.init_fn, &*entries.reload_fn, emit_path) {
      Ok(_) => (),
      Err(failure) => {
        eprintln!("\nfailed codegen, {}", failure);
        call_stack::display_stack(&failure, &gen_stack::get_gen_stack())?;
        return Err(failure);
      }
    }
  } else {
    // TODO entry ns
    match codegen::emit_js::emit_js(&entries.init_ns, emit_path) {
      Ok(_) => (),
      Err(failure) => {
        eprintln!("\nfailed codegen, {}", failure);
        call_stack::display_stack(&failure, &gen_stack::get_gen_stack())?;
        return Err(failure);
      }
    }
  }
  let duration = Instant::now().duration_since(started_time);
  println!("took {}ms", duration.as_micros() as f64 / 1000.0);
  Ok(())
}

fn throw_on_js_warnings(warnings: &[String], js_file_path: &Path) -> Result<(), String> {
  if !warnings.is_empty() {
    let mut content: String = String::from("");
    for message in warnings {
      println!("{}", message);
      content = format!("{}\n{}", content, message);
    }

    let _ = fs::write(&js_file_path, format!("export default \"{}\";", content.trim().escape_default()));
    Err(format!(
      "Found {} warnings, codegen blocked. errors in {}.js",
      warnings.len(),
      COMPILE_ERRORS_FILE,
    ))
  } else {
    Ok(())
  }
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
