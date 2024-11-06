use pest::Parser;
use anyhow::anyhow;
use pest_01::*;

#[test]
fn basic_test()-> anyhow::Result<()> {
    ///тестуємо числовий ввід field
    let pair = Grammar::parse(Rule::field, "-320.05")?.next().ok_or_else(
        ||anyhow!("no pair")
    )?;

    assert_eq!(pair.as_str(), "-320.05");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 7);

///тест на неправильний ввід
    let pair = Grammar::parse(Rule::field, "x");
    assert!(pair.is_err());
///тест на порожній ввід
    let pair = Grammar::parse(Rule::field, "");
    assert!(pair.is_err());
    Ok(())
}

#[test]
fn record_test()-> anyhow::Result<()> {
    ///тестуємо числовий ввід record
    let pair = Grammar::parse(Rule::record, "-320.05,88")?.next().ok_or_else(
        ||anyhow!("no pair")
    )?;

    assert_eq!(pair.as_str(), "-320.05,88");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 10);

    ///тест на неправильний ввід
    let pair = Grammar::parse(Rule::record, "x");
    assert!(pair.is_err());
    ///тест на порожній ввід
    let pair = Grammar::parse(Rule::record, "");
    assert!(pair.is_err());
    Ok(())
}

#[test]
fn file_test()-> anyhow::Result<()> {
    ///тестуємо файл з одни записом
    let pair = Grammar::parse(Rule::file, "-320.05,88\n")?.next().ok_or_else(
        ||anyhow!("no pair")
    )?;

    assert_eq!(pair.as_str(), "-320.05,88\n");

    ///тест на неправильний ввід
    let pair = Grammar::parse(Rule::file, "x");
    assert!(pair.is_err());
    ///тест на некоректний формат
    let pair = Grammar::parse(Rule::file, "-320.05 88\n");
    assert!(pair.is_err());
    ///тест на відсутність нового рядка
    let pair = Grammar::parse(Rule::file, "-320.05,88");
    assert!(pair.is_err());
    Ok(())
}

#[test]
fn file_test_2()-> anyhow::Result<()> {
    ///тестуємо файл з одни записом
    let pair = Grammar::parse(Rule::file, "-320.05,88\n,-144.66,88\n")?.next().ok_or_else(
        || anyhow!("no pair")
    )?;

    assert_eq!(pair.as_str(), "-320.05,88\n,-144.66,88\n");
}
