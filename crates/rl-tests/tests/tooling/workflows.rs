use rl_tooling::workflows::try_generate;

#[test]
fn generate_check_workflow() {
    let temp = tempfile::tempdir().unwrap();

    try_generate(temp.path(), true, false, false, false, false, None).unwrap();

    let file = temp.path().join("check.yml");

    assert!(file.exists());

    let content = std::fs::read_to_string(file).unwrap();

    assert!(content.contains("name: RL Check"));
    assert!(content.contains("rl-check"));
}

#[test]
fn generate_package_workflow() {
    let temp = tempfile::tempdir().unwrap();

    try_generate(temp.path(), false, true, false, false, false, None).unwrap();

    let file = temp.path().join("release.yml");

    assert!(file.exists());

    let content = std::fs::read_to_string(file).unwrap();

    assert!(content.contains("name: Release"));
    assert!(content.contains("rl-package"));
}

#[test]
fn generate_all_workflows() {
    let temp = tempfile::tempdir().unwrap();

    try_generate(temp.path(), true, true, false, false, false, None).unwrap();

    assert!(temp.path().join("check.yml").exists());
    assert!(temp.path().join("release.yml").exists());
}

#[test]
fn does_not_override_existing_workflow() {
    let temp = tempfile::tempdir().unwrap();

    std::fs::create_dir_all(temp.path()).unwrap();

    std::fs::write(temp.path().join("check.yml"), "existing").unwrap();

    let result = try_generate(temp.path(), true, false, false, false, false, None);

    assert!(result.is_err());

    let content = std::fs::read_to_string(temp.path().join("check.yml")).unwrap();

    assert_eq!(content, "existing");
}

#[test]
fn generate_test_workflow_with_version() {
    let temp = tempfile::tempdir().unwrap();

    try_generate(
        temp.path(),
        false,
        false,
        true,
        false,
        false,
        Some("v2.2.0".to_string()),
    )
    .unwrap();

    let file = temp.path().join("test.yml");

    assert!(file.exists());

    let content = std::fs::read_to_string(file).unwrap();

    assert!(content.contains("name: RL Test"));
    assert!(content.contains("rl-lang/rl-test@main"));
    assert!(content.contains("version: v2.2.0"));
}

#[test]
fn generate_transpile_and_format_workflows() {
    let temp = tempfile::tempdir().unwrap();

    try_generate(temp.path(), false, false, false, true, true, None).unwrap();

    for (name, marker) in [
        ("transpile.yml", "rl-lang/rl-transpile@main"),
        ("format.yml", "rl-lang/rl-format@main"),
    ] {
        let file = temp.path().join(name);
        assert!(file.exists());

        let content = std::fs::read_to_string(file).unwrap();

        assert!(content.contains(marker));
        assert!(content.contains("version: latest"));
    }
}
