use im_ternary_tree::TernaryTreeList;
use std::cell::RefCell;
use std::fs::read_to_string;

use calcit_runner::{call_stack::CallStackList, load_core_snapshot, program, runner, snapshot, Calcit, CalcitErr, CalcitItems};

pub fn eval_code(snippet: String) -> Result<Calcit, String> {
  // program::clear_all_program_evaled_defs("app.main/main!".into(), "app.main/reload!".into(), false)?;

  let core_data = cirru_edn::parse(&snippet)?;
  let mut snapshot = snapshot::load_snapshot_data(&core_data, "calcit-internal://calcit-core.cirru")?;

  let core_snapshot = load_core_snapshot()?;
  // attach core
  for (k, v) in core_snapshot.files {
    snapshot.files.insert(k.to_owned(), v.to_owned());
  }

  // overwrite global states
  {
    let mut prgm = { program::PROGRAM_CODE_DATA.write().unwrap() };
    *prgm = program::extract_program_data(&snapshot)?;
  }

  let check_warnings: &RefCell<Vec<String>> = &RefCell::new(vec![]);

  // make sure builtin classes are touched
  runner::preprocess::preprocess_ns_def(
    calcit_runner::primes::CORE_NS.into(),
    calcit_runner::primes::BUILTIN_CLASSES_ENTRY.into(),
    calcit_runner::primes::BUILTIN_CLASSES_ENTRY.into(),
    None,
    check_warnings,
    &rpds::List::new_sync(),
  )
  .map_err(|e| e.msg)?;

  let v = calcit_runner::run_program("app.main".into(), "main!".into(), TernaryTreeList::Empty).map_err(|e| format!("{}", e))?;

  // println!("Result: {}", v);
  Ok(v)
}

pub fn console_log(xs: &CalcitItems, _call_stack: &CallStackList) -> Result<Calcit, CalcitErr> {
  let mut buffer = String::from("");
  for (idx, x) in xs.iter().enumerate() {
    if idx > 0 {
      buffer.push(' ');
    }
    buffer.push_str(&x.turn_string());
  }
  println!("{}", buffer);
  Ok(Calcit::Nil)
}

fn main() -> Result<(), String> {
  let snippet = read_to_string("examples/compact.cirru").map_err(|e| e.to_string())?;

  calcit_runner::builtins::register_import_proc("println", console_log);
  calcit_runner::builtins::register_import_proc("echo", console_log);

  match eval_code(snippet) {
    Ok(v) => println!("Result {}", v),
    Err(e) => {
      println!("Error: {}", e)
    }
  }
  Ok(())
}
