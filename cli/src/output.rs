pub trait Output {
    fn format_plain(&self) -> String;
    fn format_json(&self) -> String;
}

pub fn write_output(output: &impl Output, format: String) {
    match format.as_str() {
        "json" => println!("{}", output.format_json()),
        _ => println!("{}", output.format_plain()),
    }
}
