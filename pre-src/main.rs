const THE_MAGIC: &'static str = "@";

fn main() {
    print!("{}", THE_MAGIC.replace('\u{40}', THE_MAGIC.replace("\\", "\\\\").replace("\"", "\\\"").as_str()));
}
