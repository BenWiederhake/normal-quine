const THE_MAGIC: &'static str = "const THE_MAGIC: &'static str = \"@\";

fn main() {
    println!(\"{}\", THE_MAGIC.replace('\\u{40}', THE_MAGIC.replace(\"\\\\\", \"\\\\\\\\\").replace(\"\\\"\", \"\\\\\\\"\").as_str()));
}";

fn main() {
    println!("{}", THE_MAGIC.replace('\u{40}', THE_MAGIC.replace("\\", "\\\\").replace("\"", "\\\"").as_str()));
}
