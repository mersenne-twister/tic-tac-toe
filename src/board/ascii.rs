use super::TicTac;

pub fn get_frame(ascii: Option<TicTac>, frame: usize) -> &'static str {
    assert!(frame <= 7);
    match ascii {
        Some(TicTac::X) => X[frame],
        Some(TicTac::O) => O[frame],
        None => BLANK,
    }
}

const X: [&str; 8] = [
    r#"    Y88b   d88P    "#,
    r#"     Y88b d88P     "#,
    r#"      Y88o88P      "#,
    r#"       Y888P       "#,
    r#"       d888b       "#,
    r#"      d88888b      "#,
    r#"     d88P Y88b     "#,
    r#"    d88P   Y88b    "#,
];

const O: [&str; 8] = [
    r#"     .d88888b.     "#,
    r#"    d88P" "Y88b    "#,
    r#"    888     888    "#,
    r#"    888     888    "#,
    r#"    888     888    "#,
    r#"    888     888    "#,
    r#"    Y88b. .d88P    "#,
    r#"     "Y88888P"     "#,
];

const BLANK: &str = "                   ";
