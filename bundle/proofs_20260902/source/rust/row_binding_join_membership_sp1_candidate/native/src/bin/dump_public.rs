//! Print the frozen row-96 membership public oracle as hex, for digest freezing.
fn main() {
    let public = zeebeam_row_binding_membership_native::expected_row96_public_values();
    println!("{}", public.iter().map(|b| format!("{b:02x}")).collect::<String>());
}
