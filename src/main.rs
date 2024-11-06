use pest_derive::Parser;
use pest::Parser;
use anyhow::anyhow;
use pest_01::*;

fn main() -> anyhow::Result<()> {
    Ok(())
}
//
// #[cfg(test)]
// mod tests{
//     use super::*;
//     #[test]
//     fn basic_test()-> anyhow::Result<()> {
//         let pair = Grammar::parse(Rule::field, "-320.05")?.next().ok_or_else(
//             ||anyhow!("no pair")
//         )?;
//
//         assert_eq!(pair.as_str(), "-320.05");
//         assert_eq!(pair.as_span().start(), 0);
//         assert_eq!(pair.as_span().end(), 7);
//
//
//         let pair = Grammar::parse(Rule::field, "x");
//         assert!(pair.is_err());
//
//         let pair = Grammar::parse(Rule::field, "x");
//         assert!(pair.is_err());
//         Ok(())
//     }
//
//     #[test]
//     fn record_test()-> anyhow::Result<()> {
//         let pair = Grammar::parse(Rule::record, "-320.05,88")?.next().ok_or_else(
//             ||anyhow!("no pair")
//         )?;
//
//         assert_eq!(pair.as_str(), "-320.05,88");
//         assert_eq!(pair.as_span().start(), 0);
//         assert_eq!(pair.as_span().end(), 10);
//
//         Ok(())
//     }
//}