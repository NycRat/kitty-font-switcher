#[derive(serde::Deserialize, Debug)]
struct Font<'a> {
    font_family: &'a str,
    bold_font: Option<&'a str>,
    italic_font: Option<&'a str>,
    bold_italic_font: Option<&'a str>,
    font_size: f32,
}

fn main() {
    let fonts_file_path = std::format!("{}/.config/fonts.json", std::env::var("HOME").unwrap());
    let fonts_file =
        std::fs::read_to_string(fonts_file_path).expect("Could not open ~/.config/fonts.json");
    let fonts: Vec<Font> =
        serde_json::from_str(&fonts_file).expect("Config file in the wrong format");

    let rev = std::env::args().nth(1).unwrap_or_default() == "-d";

    let config_file_path = "/Users/avah/.config/kitty/font.conf";
    let config_file_contents = std::fs::read_to_string(config_file_path).unwrap();
    let font_index: usize = config_file_contents
        .split_once("\n")
        .unwrap()
        .0
        .split_once(" ")
        .unwrap()
        .1
        .parse()
        .unwrap();

    let new_font_index;
    if rev {
        if font_index != 0 {
            new_font_index = font_index - 1;
        } else {
            new_font_index = fonts.len() - 1;
        }
    } else {
        new_font_index = (font_index + 1) % fonts.len();
    }

    let new_font = &fonts[new_font_index];

    let new_config_file_contents = std::format!(
        "# {}\n\
        font_family {}\n\
        bold_font {}\n\
        italic_font {}\n\
        bold_italic_font {}\n\
        font_size {}\n",
        new_font_index,
        new_font.font_family,
        new_font.bold_font.unwrap_or("auto"),
        new_font.italic_font.unwrap_or("auto"),
        new_font.bold_italic_font.unwrap_or("auto"),
        new_font.font_size,
    );

    std::fs::write(config_file_path, new_config_file_contents).unwrap();
}
