#![feature(str_escape)]

const THE_MAGIC: &'static str = "#![feature(str_escape)]\n\nconst THE_MAGIC: &\'static str = \"@\";\n\nfn main() {\n    println!(\"{}\", THE_MAGIC.replace(\'\\u{40}\', THE_MAGIC.escape_debug().as_str()));\n}";

fn main() {
    println!("{}", THE_MAGIC.replace('\u{40}', THE_MAGIC.escape_debug().as_str()));
}
