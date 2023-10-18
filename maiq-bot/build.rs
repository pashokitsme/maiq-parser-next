use std::fs;
use std::path::Path;

use pulldown_cmark::html;
use pulldown_cmark::Parser;

fn main() {
  println!("cargo:rerun-if-changed=replies");
  build_info_build::build_script();
  for path in walkdir::WalkDir::new("replies")
    .max_depth(10)
    .into_iter()
    .filter(|entry| matches!(entry.as_ref().unwrap().path().extension().and_then(|s| s.to_str()), Some("md")))
    .filter_map(|e| e.ok().map(|e| e.into_path()))
  {
    let out_path = Path::new(&std::env::var("OUT_DIR").unwrap()).join(&path);
    let content = fs::read_to_string(&path).expect("unable to read md file");
    let md = Parser::new(&content);
    let mut buf = String::with_capacity(content.len());
    html::push_html(&mut buf, md);
    let buf = buf.replace("<p>", "").replace("</p>", "").replace("<br />", "");
    fs::create_dir_all(out_path.parent().unwrap()).expect("unable to create dir");
    fs::write(out_path, buf).expect("unable to write file");
  }
}
