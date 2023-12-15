use std::env;

use maiq_parser_next::parser::SnapshotParserBuilder;
use maiq_parser_next::prelude::*;

use owo_colors::OwoColorize;

#[tokio::main]
async fn main() {
  pretty_env_logger::init();
  let parser = SnapshotParserBuilder::new()
    .with_today_url("https://rsp.chemk.org/4korp/today.htm")
    .unwrap()
    .build::<SnapshotParser4>()
    .unwrap();

  let today = parser.fetch_today().await;
  let snapshot = today.unwrap().unwrap().0;

  if env::args().len() > 1 {
    print_group(snapshot.group(&env::args().nth(1).unwrap()).expect("no such group"));
    return;
  }
  print_snapshot(&snapshot);
}

fn print_snapshot(s: &Snapshot) {
  println!("{} от {}\n", s.id().purple(), s.date().bright_white());
  for group in s.groups() {
    print_group(group);
    println!()
  }
}

fn print_group(g: &Group) {
  println!("Группа {} ({}) ({})", g.name().bright_white(), g.lectures().len(), g.id().purple());
  for lecture in g.lectures() {
    print!("\t");
    if let Some(ref num) = lecture.order() {
      print!("{} ", format!("#{}", num).bright_white());
    }
    if let Some(sub) = lecture.subgroup() {
      print!("{} ", format!("(п. {sub})").green())
    }
    print!("{} ", lecture.name());

    if let Some(classroom) = lecture.classroom().as_ref() {
      print!("{} {}", "в".bright_white(), classroom.green());
    }

    if let Some(teacher) = lecture.teacher().as_ref() {
      print!(". Преподаватель: {}", teacher.green())
    }
    println!()
  }
}
