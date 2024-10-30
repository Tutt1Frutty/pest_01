use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;


fn main() -> anyhow::Result<()> {
    let got = Grammar::parse(Rule::file, "-320.05,-273\n");
    println!("{:?}", got);

    Ok(())
}
