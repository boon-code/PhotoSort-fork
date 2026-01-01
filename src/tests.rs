use super::*;

// Ensure that all legacy strings still parse correctly

#[test]
fn test_legacy_analysis_mode_only_exif() {
    let expect = vec![AnalysisMode::Exif];
    let input = "only_exif";
    let actual = AnalysisType::from_str(input).expect("Parsing this input is expected to succeed");
    assert_eq!(expect.as_slice(), actual.iter().as_slice());
}

#[test]
fn test_legacy_analysis_mode_only_name() {
    let expect = vec![AnalysisMode::Name];
    let input = "only_name";
    let actual = AnalysisType::from_str(input).expect("Parsing this input is expected to succeed");
    assert_eq!(expect.as_slice(), actual.iter().as_slice());
}

#[test]
fn test_legacy_analysis_mode_exif_then_name() {
    let expect = vec![AnalysisMode::Exif, AnalysisMode::Name];
    let input = "exif_then_name";
    let actual = AnalysisType::from_str(input).expect("Parsing this input is expected to succeed");
    assert_eq!(expect.as_slice(), actual.iter().as_slice());
}

#[test]
fn test_legacy_analysis_mode_name_then_exif() {
    let expect = vec![AnalysisMode::Name, AnalysisMode::Exif];
    let input = "name_then_exif";
    let actual = AnalysisType::from_str(input).expect("Parsing this input is expected to succeed");
    assert_eq!(expect.as_slice(), actual.iter().as_slice());
}

// Ensure the new format (f.e. exif,name to represent exif_then_name) works as well

#[test]
fn test_new_analysis_mode_only_exif() {
    let expect = vec![AnalysisMode::Exif];
    let input = "exif";
    let actual = AnalysisType::from_str(input).expect("Parsing this input is expected to succeed");
    assert_eq!(expect.as_slice(), actual.iter().as_slice());
}

#[test]
fn test_new_analysis_mode_only_name() {
    let expect = vec![AnalysisMode::Name];
    let input = "name";
    let actual = AnalysisType::from_str(input).expect("Parsing this input is expected to succeed");
    assert_eq!(expect.as_slice(), actual.iter().as_slice());
}

#[test]
fn test_new_analysis_mode_exif_then_name() {
    let expect = vec![AnalysisMode::Exif, AnalysisMode::Name];
    let input = "exif,name";
    let actual = AnalysisType::from_str(input).expect("Parsing this input is expected to succeed");
    assert_eq!(expect.as_slice(), actual.iter().as_slice());
}

#[test]
fn test_new_analysis_mode_name_then_exif() {
    let expect = vec![AnalysisMode::Name, AnalysisMode::Exif];
    let input = "name,exif";
    let actual = AnalysisType::from_str(input).expect("Parsing this input is expected to succeed");
    assert_eq!(expect.as_slice(), actual.iter().as_slice());
}

// Ensure empty lists are rejected

#[test]
fn test_empty_fails() {
    let _ = AnalysisType::from_str("")
        .expect_err("Empty lists must fail; at least one analysis mode must be selected");
    let _ = AnalysisType::from_str(",")
        .expect_err("',' must fail; at least one analysis mode must be selected");
}

// Ensure analysis modes can only be listed once

#[test]
fn test_multiple_elements_fails() {
    let _ = AnalysisType::from_str("name,exif,name")
        .expect_err("Must fail, as name is specified twice");
}
