use task3_3_template::participants::unique_count;

fn names(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[test]
fn empty_list_has_no_unique_names() {
    assert_eq!(unique_count(&names(&[])), 0);
}

#[test]
fn one_name_counts_as_one_participant() {
    assert_eq!(unique_count(&names(&["Аня"])), 1);
}

#[test]
fn all_distinct_names_are_counted() {
    assert_eq!(unique_count(&names(&["Аня", "Борис", "Вера"])), 3);
}

#[test]
fn one_repeated_name_counts_once() {
    assert_eq!(unique_count(&names(&["Аня", "Аня", "Аня"])), 1);
}

#[test]
fn separated_repetitions_count_once() {
    assert_eq!(
        unique_count(&names(&["Аня", "Борис", "Аня", "Вера", "Борис"])),
        3
    );
}

#[test]
fn names_are_compared_case_sensitively() {
    assert_eq!(unique_count(&names(&["Rust", "rust", "Rust"])), 2);
}

#[test]
fn unicode_names_are_counted() {
    assert_eq!(unique_count(&names(&["猫", "猫", "Кот"])), 2);
}

#[test]
fn empty_name_is_a_unique_value() {
    assert_eq!(unique_count(&names(&["", ""])), 1);
}

#[test]
fn names_with_different_whitespace_are_distinct() {
    assert_eq!(unique_count(&names(&["Аня", " Аня", "Аня "])), 3);
}

#[test]
fn repeated_unicode_names_count_once() {
    assert_eq!(unique_count(&names(&["Жанна", "Жанна", "Жанна"])), 1);
}

#[test]
fn mixed_repetitions_have_correct_unique_count() {
    assert_eq!(unique_count(&names(&["a", "b", "c", "a", "d", "b"])), 4);
}

#[test]
fn input_names_are_not_modified() {
    let input = names(&["Аня", "Борис", "Аня"]);
    let _ = unique_count(&input);
    assert_eq!(input, names(&["Аня", "Борис", "Аня"]));
}
