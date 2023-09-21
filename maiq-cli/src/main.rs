use maiq_parser_next::snapshot::*;

use tokio::sync::mpsc;

use owo_colors::OwoColorize;

#[tokio::main]
async fn main() {
  let (tx, mut rx) = mpsc::channel(32);

  let parser = maiq_parser_next::parser::PeriodicalParserBuilder::new()
    .add_url("https://rsp.chemk.org/4korp/today.htm")
    .unwrap()
    .on_update(tx)
    .build()
    .unwrap();

  _ = parser.start();
  let snapshot = rx.recv().await;
  print_snapshot(&snapshot.unwrap());
}

fn print_snapshot(s: &Snapshot) {
  println!("{} от {}\n", s.id(), s.date());
  for group in s.groups() {
    print_group(group);
    println!()
  }
}

fn print_group(g: &Group) {
  println!("Группа {} ({}) ({})", g.name().bright_white(), g.id().purple(), g.lectures().len());
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