use error_chain::error_chain;
// use std::option::Option;
// use std::{
//   fmt::{Display, Formatter, Result},
//   io,
// };
use visdom::Vis;

error_chain! {
  foreign_links {
      Io(std::io::Error);
      HttpRequest(reqwest::Error);
      Other(visdom::types::BoxDynError);
  }
}

// struct Chapter {
//   name: Option<String>,
//   number: String,
//   location: String,
// }

// impl Display for Chapter {
//   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//     let a = Some(self.name);

//     write!(f, "({})", self.number)
//   }
// }

#[tokio::main]
async fn main() -> Result<()> {
  let res = reqwest::get("https://readberserk.com/").await?;
  println!("Status: {}", res.status());
  println!("Headers:\n{:#?}", res.headers());

  let html = res.text().await?;
  let root = Vis::load(html)?;
  let table = root.find("table");
  println!("{}", table.text());

  Ok(())
}
