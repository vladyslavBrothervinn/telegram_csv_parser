use anyhow::anyhow;
use telegram_csv_parser::*;

#[test]
pub fn rule_quoted_string_test() -> anyhow::Result<()> {
    let successful_parse =
        CSVParser::parse(Rule::quoted_string, "\"Добрий вечір, пане господарю\"")?
            .next()
            .ok_or_else(|| anyhow!("Value not found"))?;

    assert_eq!(
        successful_parse.as_str(),
        "\"Добрий вечір, пане господарю\""
    );

    let successful_parse = CSVParser::parse(Rule::quoted_string, "\"\"");

    assert!(successful_parse.is_ok());

    let successful_parse = CSVParser::parse(Rule::quoted_string, "");

    assert!(successful_parse.is_err());

    Ok(())
}

#[test]
pub fn rule_value_test() -> anyhow::Result<()> {
    let successful_parse = CSVParser::parse(Rule::value, "2022-01-03 10:57:14+00:00")?
        .next()
        .ok_or_else(|| anyhow!("Value not found"))?;

    assert_eq!(successful_parse.as_str(), "2022-01-03 10:57:14+00:00");
    assert_eq!(successful_parse.as_span().start(), 0);
    assert_eq!(successful_parse.as_span().end(), 25);

    dbg!(successful_parse);

    let successful_parse = CSVParser::parse(Rule::value, ",")?
        .next()
        .ok_or_else(|| anyhow!("Value not found"))?;

    assert_eq!(successful_parse.as_str(), "");

    dbg!(successful_parse);

    let successful_parse = CSVParser::parse(Rule::value, "12,122")?
        .next()
        .ok_or_else(|| anyhow!("Value not found"))?;

    assert_eq!(successful_parse.as_str(), "12");

    Ok(())
}

#[test]
pub fn rule_row_test() -> anyhow::Result<()> {
    let successful_parse = CSVParser::parse(Rule::row, "-273.15,-31")?
        .next()
        .ok_or_else(|| anyhow!("Row not found"))?;

    assert_eq!(successful_parse.as_str(), "-273.15,-31");
    assert_eq!(successful_parse.as_span().start(), 0);
    assert_eq!(successful_parse.as_span().end(), 11);

    dbg!(successful_parse);

    let successful_parse = CSVParser::parse(Rule::row, "");

    assert!(successful_parse.is_ok());

    let successful_parse = CSVParser::parse(Rule::row, "smth,,smth");

    assert!(successful_parse.is_ok());

    Ok(())
}

#[test]
pub fn rule_file_test() -> anyhow::Result<()> {
    let successful_parse = CSVParser::parse(
        Rule::file,
        "4,145678,2021-09-16 05:20:01+00:00,,1053373318,,\"Доброго ранку, без питань\",text,{}\n",
    );

    assert!(successful_parse.is_ok());

    //dbg!(successful_parse);

    // assert_eq!(
    //     successful_parse.as_str(),
    //     "4,145678,2021-09-16 05:20:01+00:00,,1053373318,,\"Доброго ранку, без питань\",text,,{}"
    // );

    // dbg!(successful_parse);

    let successful_parse = CSVParser::parse(Rule::file, "");

    assert!(successful_parse.is_ok());

    let successful_parse = CSVParser::parse(Rule::file, "/");

    assert!(successful_parse.is_err());

    Ok(())
}
