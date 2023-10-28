use serde::Deserialize;
use std::collections::HashMap;

enum ByFilename {
    Name,
    Extension,
}

fn deserialize_by_filename<'de, D>(
    deserializer: D,
    by: ByFilename,
) -> Result<HashMap<String, String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let default = match by {
        ByFilename::Name => IconTheme::get_default_icons_by_name(),
        ByFilename::Extension => IconTheme::get_default_icons_by_extension(),
    };
    HashMap::<_, _>::deserialize(deserializer)
        .map(|input| default.into_iter().chain(input).collect())
}

fn deserialize_by_name<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserialize_by_filename(deserializer, ByFilename::Name)
}

fn deserialize_by_extension<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserialize_by_filename(deserializer, ByFilename::Extension)
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct IconTheme {
    #[serde(deserialize_with = "deserialize_by_name")]
    pub name: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_by_extension")]
    pub extension: HashMap<String, String>,
    pub filetype: ByType,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct ByType {
    pub dir: String,
    pub file: String,
    pub pipe: String,
    pub socket: String,
    //pub executable: String,
    pub device_char: String,
    pub device_block: String,
    pub special: String,
    pub symlink_dir: String,
    pub symlink_file: String,
}

impl Default for IconTheme {
    fn default() -> Self {
        IconTheme {
            name: Self::get_default_icons_by_name(),
            extension: Self::get_default_icons_by_extension(),
            filetype: ByType::default(),
        }
    }
}

impl Default for ByType {
    fn default() -> ByType {
        ByType {
            dir: "\u{f115}".into(),          // 
            file: "\u{f016}".into(),         // 
            pipe: "\u{f731}".into(),         // 
            socket: "\u{f6a7}".into(),       // 
            //executable: "\u{f489}".into(),   // 
            symlink_dir: "\u{f481}".into(),  // 
            symlink_file: "\u{f481}".into(), // 
            device_char: "\u{e601}".into(),  // 
            device_block: "\u{fc29}".into(), // ﰩ
            special: "\u{f2dc}".into(),      // 
        }
    }
}

impl ByType {
    pub fn unicode() -> Self {
        ByType {
            dir: "\u{1f4c2}".into(),
            file: "\u{1f4c4}".into(),
            pipe: "\u{1f4e9}".into(),
            socket: "\u{1f4ec}".into(),
            //executable: "\u{1f3d7}".into(),
            symlink_dir: "\u{f481}".into(),
            symlink_file: "\u{1f516}".into(),
            device_char: "\u{1f5a8}".into(),
            device_block: "\u{1f4bd}".into(),
            special: "\u{1f4df}".into(),
        }
    }
}

impl IconTheme {
    pub fn unicode() -> Self {
        IconTheme {
            name: HashMap::new(),
            extension: HashMap::new(),
            filetype: ByType::unicode(),
        }
    }

    // pub only for testing in icons.rs
    pub fn get_default_icons_by_name() -> HashMap<String, String> {
        // Note: filenames must be lower-case
        [
            ("authorized_keys", "\u{e60a}"),    // ""
        ]
        .iter()
        .map(|&s| (s.0.to_owned(), s.1.to_owned()))
        .collect::<HashMap<_, _>>()
    }

    // pub only for testing in icons.rs
    pub fn get_default_icons_by_extension() -> HashMap<String, String> {
        // Note: extensions must be lower-case
        [
            ("1", "\u{f02d}"),               // ""
            ("2", "\u{f02d}"),               // ""
            ("3", "\u{f02d}"),               // ""
            ("4", "\u{f02d}"),               // ""
            ("5", "\u{f02d}"),               // ""
            ("6", "\u{f02d}"),               // ""
            ("7", "\u{f02d}"),               // ""
            ("8", "\u{f02d}"),               // ""
            ("7z", "\u{f410}"),              // ""
            ("aac", "\u{f001}"),             // ""
            ("ace", "\u{f410}"),             // ""
            ("ai", "\u{e7b4}"),              // ""
            ("alz", "\u{f410}"),             // ""
            ("ape", "\u{f001}"),             // ""
            ("apk", "\u{e70e}"),             // ""
            ("arc", "\u{f410}"),             // ""
            ("arj", "\u{f410}"),             // ""
            ("asc", "\u{f023}"),             // ""
            ("asf", "\u{f03d}"),             // ""
            ("asm", "\u{e614}"),             // ""
            ("asp", "\u{f121}"),             // ""
            ("a", "\u{e624}"),               // ""
            ("au", "\u{f001}"),              // ""
            ("avi", "\u{f03d}"),             // ""
            ("avro", "\u{e60b}"),            // ""
            ("awk", "\u{f489}"),             // ""
            ("bak", "\u{f006f}"),            // "󰁯"
            ("bash_history", "\u{f489}"),    // ""
            ("bash_profile", "\u{f489}"),    // ""
            ("bashrc", "\u{f489}"),          // ""
            ("bash", "\u{f489}"),            // ""
            ("bak", "\u{f56e}"),             // ""
            ("bat", "\u{f17a}"),             // ""
            ("bin", "\u{f489}"),             // ""
            ("bio", "\u{f0411}"),            // "󰐑"
            ("bmp", "\u{f1c5}"),             // ""
            ("bz2", "\u{f410}"),             // ""
            ("bz", "\u{f410}"),              // ""
            ("cab", "\u{f410}"),             // ""
            ("cc", "\u{e61d}"),              // ""
            ("cfg", "\u{e615}"),             // ""
            ("cgm", "\u{f1c5}"),             // ""
            ("cjs", "\u{e74e}"),             // ""
            ("class", "\u{e738}"),           // ""
            ("cljs", "\u{e76a}"),            // ""
            ("clj", "\u{e768}"),             // ""
            ("cls", "\u{e600}"),             // ""
            ("cl", "\u{f671}"),              // ""
            ("cmd", "\u{f489}"),             // ""
            ("coffee", "\u{f0f4}"),          // ""
            ("conf", "\u{e615}"),            // ""
            ("cpio", "\u{f410}"),            // ""
            ("cpp", "\u{e61d}"),             // ""
            ("cp", "\u{e61d}"),              // ""
            ("cshtml", "\u{f1fa}"),          // ""
            ("csh", "\u{f489}"),             // ""
            ("csproj", "\u{f031b}"),         // "󰌛"
            ("css", "\u{e749}"),             // ""
            ("cs", "\u{f031b}"),             // "󰌛"
            ("csv", "\u{f1c3}"),             // ""
            ("csx", "\u{f031b}"),            // "󰌛"
            ("cts", "\u{e628}"),             // ""
            ("c++", "\u{e61d}"),             // ""
            ("c", "\u{e61e}"),               // ""
            ("cue", "\u{f001}"),             // ""
            ("cxx", "\u{e61d}"),             // ""
            ("dart", "\u{e798}"),            // ""
            ("dat", "\u{f1c0}"),             // ""
            ("db", "\u{f1c0}"),              // ""
            ("deb", "\u{f410}"),             // ""
            ("desktop", "\u{f108}"),         // ""
            ("diff", "\u{f440}"),            // ""
            ("dll", "\u{f17a}"),             // ""
            ("dockerfile", "\u{f308}"),      // ""
            ("doc", "\u{f1c2}"),             // ""
            ("docx", "\u{f1c2}"),            // ""
            ("ds_store", "\u{f179}"),        // ""
            ("d", "\u{e7af}"),               // ""
            ("dump", "\u{f1c0}"),            // ""
            ("dz", "\u{f410}"),              // ""
            ("ear", "\u{f410}"),             // ""
            ("ebuild", "\u{f30d}"),          // ""
            ("eclass", "\u{f30d}"),          // ""
            ("ebook", "\u{e28b}"),           // ""
            ("editorconfig", "\u{e615}"),    // ""
            ("ejs", "\u{e618}"),             // ""
            ("elc", "\u{f0172}"),            // "󰅲"
            ("elf", "\u{f489}"),             // ""
            ("elm", "\u{e62c}"),             // ""
            ("el", "\u{f671}"),              // ""
            ("emf", "\u{f1c5}"),             // ""
            ("env", "\u{f462}"),             // ""
            ("eot", "\u{f031}"),             // ""
            ("epub", "\u{e28a}"),            // ""
            ("erb", "\u{e73b}"),             // ""
            ("erl", "\u{e7b1}"),             // ""
            ("exe", "\u{f17a}"),             // ""
            ("exs", "\u{e62d}"),             // ""
            ("ex", "\u{e62d}"),              // ""
            ("fish", "\u{f489}"),            // ""
            ("flac", "\u{f001}"),            // ""
            ("flc", "\u{f03d}"),             // ""
            ("fli", "\u{f03d}"),             // ""
            ("flv", "\u{f03d}"),             // ""
            ("fmi", "\u{f279}"),             // ""
            ("font", "\u{f031}"),            // ""
            ("fpl", "\u{f0411}"),            // "󰐑"
            ("fsi", "\u{e7a7}"),             // ""
            ("fs", "\u{e7a7}"),              // ""
            ("fsx", "\u{e7a7}"),             // ""
            ("gdoc", "\u{f1c2}"),            // ""
            ("gemfile", "\u{e21e}"),         // ""
            ("gemspec", "\u{e21e}"),         // ""
            ("gform", "\u{f298}"),           // ""
            ("gif", "\u{f1c5}"),             // ""
            ("git", "\u{f1d3}"),             // ""
            ("go", "\u{e626}"),              // ""
            ("gradle", "\u{e70e}"),          // ""
            ("graph", "\u{f278}"),           // ""
            ("gsheet", "\u{f1c3}"),          // ""
            ("gslides", "\u{f1c4}"),         // ""
            ("guardfile", "\u{e21e}"),       // ""
            ("gz", "\u{f410}"),              // ""
            ("hbs", "\u{e60f}"),             // ""
            ("heic", "\u{f1c5}"),            // ""
            ("heif", "\u{f1c5}"),            // ""
            ("heix", "\u{f1c5}"),            // ""
            ("hh", "\u{e61d}"),              // ""
            ("hl", "\u{f277}"),              // ""
            ("hpp", "\u{e61d}"),             // ""
            ("hs", "\u{e777}"),              // ""
            ("html", "\u{f13b}"),            // ""
            ("htm", "\u{f13b}"),             // ""
            ("h", "\u{e61e}"),               // ""
            ("hxx", "\u{e61d}"),             // ""
            ("ico", "\u{f1c5}"),             // ""
            ("image", "\u{f1c5}"),           // ""
            ("img", "\u{f1c0}"),             // ""
            ("iml", "\u{e7b5}"),             // ""
            ("info", "\u{f489}"),            // ""
            ("ini", "\u{e615}"),             // ""
            ("ipynb", "\u{e606}"),           // ""
            ("iso", "\u{f1c0}"),             // ""
            ("j2", "\u{e000}"),              // ""
            ("jar", "\u{e738}"),             // ""
            ("java", "\u{e738}"),            // ""
            ("jinja", "\u{e000}"),           // ""
            ("jl", "\u{e624}"),              // ""
            ("jpeg", "\u{f1c5}"),            // ""
            ("jpg", "\u{f1c5}"),             // ""
            ("jsonc", "\u{e60b}"),           // ""
            ("json", "\u{e60b}"),            // ""
            ("js", "\u{e74e}"),              // ""
            ("jsx", "\u{e7ba}"),             // ""
            ("key", "\u{e60a}"),             // ""
            ("ksh", "\u{f489}"),             // ""
            ("ldb", "\u{f1c0}"),             // ""
            ("ld", "\u{e624}"),              // ""
            ("less", "\u{e758}"),            // ""
            ("lha", "\u{f410}"),             // ""
            ("lhs", "\u{e777}"),             // ""
            ("license", "\u{f48a}"),         // ""
            ("list", "\u{f03a}"),            // ""
            ("lisp", "\u{f671}"),            // ""
            ("localized", "\u{f179}"),       // ""
            ("lock", "\u{f023}"),            // ""
            ("log", "\u{f18d}"),             // ""
            ("lss", "\u{e749}"),             // ""
            ("lua", "\u{e620}"),             // ""
            ("lz4", "\u{f410}"),             // ""
            ("lzh", "\u{f410}"),             // ""
            ("lzma", "\u{f410}"),            // ""
            ("lzo", "\u{f410}"),             // ""
            ("lz", "\u{f410}"),              // ""
            ("m2v", "\u{f03d}"),             // ""
            ("m3u8", "\u{f0411}"),           // "蘿"
            ("m3u", "\u{f0411}"),            // "蘿"
            ("m4a", "\u{f001}"),             // ""
            ("m4v", "\u{f03d}"),             // ""
            ("magnet", "\u{f076}"),          // ""
            ("man", "\u{f02d}"),             // ""
            ("markdown", "\u{f48a}"),        // ""
            ("md", "\u{f48a}"),              // ""
            ("midi", "\u{f001}"),            // ""
            ("mid", "\u{f001}"),             // ""
            ("mjpeg", "\u{f1c5}"),           // ""
            ("mjpg", "\u{f1c5}"),            // ""
            ("mjs", "\u{e74e}"),             // ""
            ("mk", "\u{f085}"),              // ""
            ("mka", "\u{f001}"),             // ""
            ("mkd", "\u{f48a}"),             // ""
            ("mkv", "\u{f03d}"),             // ""
            ("mng", "\u{f1c5}"),             // ""
            ("mobi", "\u{e28b}"),            // ""
            ("mov", "\u{f03d}"),             // ""
            ("mp3", "\u{f001}"),             // ""
            ("mp4", "\u{f03d}"),             // ""
            ("mp4v", "\u{f03d}"),            // ""
            ("mpc", "\u{f001}"),             // ""
            ("mpeg", "\u{f03d}"),            // ""
            ("mpg", "\u{f03d}"),             // ""
            ("msi", "\u{f17a}"),             // ""
            ("mts", "\u{e628}"),             // ""
            ("mustache", "\u{e60f}"),        // ""
            ("nix", "\u{f313}"),             // ""
            ("npmignore", "\u{e71e}"),       // ""
            ("nuv", "\u{f1c5}"),             // ""
            ("oga", "\u{f001}"),             // ""
            ("ogg", "\u{f001}"),             // ""
            ("ogm", "\u{f03d}"),             // ""
            ("ogv", "\u{f03d}"),             // ""
            ("ogx", "\u{f03d}"),             // ""
            ("old", "\u{f56e}"),             // ""
            ("opus", "\u{f001}"),            // ""
            ("orig", "\u{f006f}"),           // "󰁯"
            ("otf", "\u{f031}"),             // ""
            ("o", "\u{e624}"),               // ""
            ("pbm", "\u{f1c5}"),             // ""
            ("pcx", "\u{f1c5}"),             // ""
            ("pdf", "\u{f1c1}"),             // ""
            ("pem", "\u{f805}"),             // ""
            ("pgm", "\u{f1c5}"),             // ""
            ("phar", "\u{e608}"),            // ""
            ("php", "\u{e608}"),             // ""
            ("php", "\u{e73d}"),             // ""
            ("pkg", "\u{f187}"),             // ""
            ("plist", "\u{f302}"),           // ""
            ("pls", "\u{f0411}"),            // "󰐑"
            ("pl", "\u{e769}"),              // ""
            ("pm", "\u{e769}"),              // ""
            ("png", "\u{f1c5}"),             // ""
            ("ppm", "\u{f1c5}"),             // ""
            ("ppt", "\u{f1c4}"),             // ""
            ("pptx", "\u{f1c4}"),            // ""
            ("procfile", "\u{e21e}"),        // ""
            ("properties", "\u{e60b}"),      // ""
            ("pro", "\u{e7a1}"),             // ""
            ("ps1", "\u{f489}"),             // ""
            ("psd", "\u{e7b8}"),             // ""
            ("pub", "\u{e60a}"),             // ""
            ("pxm", "\u{f1c5}"),             // ""
            ("pyc", "\u{e606}"),             // ""
            ("py", "\u{e606}"),              // ""
            ("qt", "\u{f03d}"),              // ""
            ("rakefile", "\u{e21e}"),        // ""
            ("rar", "\u{f410}"),             // ""
            ("ra", "\u{f001}"),              // ""
            ("razor", "\u{f1fa}"),           // ""
            ("rb", "\u{e21e}"),              // ""
            ("rdata", "\u{f25d}"),           // ""
            ("rdb", "\u{e76d}"),             // ""
            ("rdoc", "\u{f48a}"),            // ""
            ("rds", "\u{f25d}"),             // ""
            ("readme", "\u{f48a}"),          // ""
            ("rl", "\u{f11c}"),              // ""
            ("rlib", "\u{e7a8}"),            // ""
            ("rmd", "\u{f48a}"),             // ""
            ("rm", "\u{f03d}"),              // ""
            ("rmvb", "\u{f03d}"),            // ""
            ("rpm", "\u{f410}"),             // ""
            ("rproj", "\u{fac5}"),           // "鉶"
            ("rspec_parallel", "\u{e21e}"),  // ""
            ("rspec_status", "\u{e21e}"),    // ""
            ("rspec", "\u{e21e}"),           // ""
            ("rss", "\u{f09e}"),             // ""
            ("rs", "\u{e7a8}"),              // ""
            ("rtf", "\u{f15c}"),             // ""
            ("rubydoc", "\u{e73b}"),         // ""
            ("r", "\u{f07d4}"),              // "󰟔"
            ("ru", "\u{e21e}"),              // ""
            ("rz", "\u{f410}"),              // ""
            ("sass", "\u{e603}"),            // ""
            ("scala", "\u{e737}"),           // ""
            ("scpt", "\u{f302}"),            // ""
            ("scss", "\u{e603}"),            // ""
            ("scss", "\u{e749}"),            // ""
            ("shell", "\u{f489}"),           // ""
            ("sh", "\u{f489}"),              // ""
            ("sig", "\u{e60a}"),             // ""
            ("slim", "\u{e73b}"),            // ""
            ("sln", "\u{e70c}"),             // ""
            ("so", "\u{e624}"),              // ""
            ("spx", "\u{f001}"),             // ""
            ("sqlite3", "\u{e7c4}"),         // ""
            ("sql", "\u{f1c0}"),             // ""
            ("srt", "\u{f02d}"),             // ""
            ("styl", "\u{e600}"),            // ""
            ("stylus", "\u{e600}"),          // ""
            ("sublime-package", "\u{e7aa}"), // ""
            ("sublime-session", "\u{e7aa}"), // ""
            ("sub", "\u{f02d}"),             // ""
            ("s", "\u{e614}"),               // ""
            ("svg", "\u{f1c5}"),             // ""
            ("svgz", "\u{f1c5}"),            // ""
            ("svelte", "\u{e697}"),          // ""
            ("swift", "\u{e755}"),           // ""
            ("swm", "\u{f410}"),             // ""
            ("swp", "\u{e62b}"),             // ""
            ("sym", "\u{e624}"),             // ""
            ("t7z", "\u{f410}"),             // ""
            ("tar", "\u{f410}"),             // ""
            ("taz", "\u{f410}"),             // ""
            ("tbz2", "\u{f410}"),            // ""
            ("tbz", "\u{f410}"),             // ""
            ("tex", "\u{e600}"),             // ""
            ("tga", "\u{f1c5}"),             // ""
            ("tgz", "\u{f410}"),             // ""
            ("tiff", "\u{f1c5}"),            // ""
            ("tif", "\u{f1c5}"),             // ""
            ("tlz", "\u{f410}"),             // ""
            ("toml", "\u{e60b}"),            // ""
            ("torrent", "\u{f048d}"),        // "󰒍"
            ("trash", "\u{f1f8}"),           // ""
            ("ts", "\u{e628}"),              // ""
            ("tsx", "\u{e7ba}"),             // ""
            ("ttc", "\u{f031}"),             // ""
            ("ttf", "\u{f031}"),             // ""
            ("t", "\u{e769}"),               // ""
            ("twig", "\u{e61c}"),            // ""
            ("txt", "\u{f15c}"),             // ""
            ("txz", "\u{f410}"),             // ""
            ("tzo", "\u{f410}"),             // ""
            ("tzst", "\u{f410}"),            // ""
            ("tz", "\u{f410}"),              // ""
            ("video", "\u{f03d}"),           // ""
            ("vim", "\u{e62b}"),             // ""
            ("vlc", "\u{f0411}"),            // "󰐑"
            ("vob", "\u{f03d}"),             // ""
            ("vue", "\u{f0844}"),            // "󰡄"
            ("war", "\u{f410}"),             // ""
            ("wav", "\u{f001}"),             // ""
            ("webm", "\u{f008}"),            // ""
            ("webm", "\u{f03d}"),            // ""
            ("webp", "\u{f1c5}"),            // ""
            ("wim", "\u{f410}"),             // ""
            ("windows", "\u{f17a}"),         // ""
            ("wma", "\u{f001}"),             // ""
            ("wmv", "\u{f03d}"),             // ""
            ("woff2", "\u{f031}"),           // ""
            ("woff", "\u{f031}"),            // ""
            ("wpl", "\u{f0411}"),            // "󰐑"
            ("xbm", "\u{f1c5}"),             // ""
            ("xbps", "\u{f187}"),            // ""
            ("xcf", "\u{f1c5}"),             // ""
            ("xls", "\u{f1c3}"),             // ""
            ("xlsx", "\u{f1c3}"),            // ""
            ("xml", "\u{e619}"),             // ""
            ("xml", "\u{f121}"),             // ""
            ("xpm", "\u{f1c5}"),             // ""
            ("xspf", "\u{f001}"),            // ""
            ("xul", "\u{e619}"),             // ""
            ("xul", "\u{f269}"),             // ""
            ("xwd", "\u{f1c5}"),             // ""
            ("xz", "\u{f410}"),              // ""
            ("yaml", "\u{e60b}"),            // ""
            ("yml", "\u{e60b}"),             // ""
            ("yuv", "\u{f1c5}"),             // ""
            ("zig", "\u{e6a9}"),             // ""
            ("zip", "\u{f410}"),             // ""
            ("zoo", "\u{f410}"),             // ""
            ("zshrc", "\u{f489}"),           // ""
            ("zsh-theme", "\u{f489}"),       // ""
            ("zsh", "\u{f489}"),             // ""
            ("zst", "\u{f410}"),             // ""
            ("z", "\u{f410}"),               // ""
        ]
        .iter()
        .map(|&s| (s.0.to_owned(), s.1.to_owned()))
        .collect::<HashMap<_, _>>()
    }
}

#[cfg(test)]
mod tests {
    use super::IconTheme;
    use crate::theme::Theme;

    fn partial_default_yaml() -> &'static str {
        r#"---
name:
  .trash: 
  .cargo: 
  .emacs.d: 
  a.out: 
extension:
  go: 
  hs: 
  rs: 
filetype:
  dir: 
  file: 
  pipe: 
  socket: 
  symlink-dir: 
  symlink-file: 
  device-char: 
  device-block: 󰜫
  special: 
"#
    }

    fn check_partial_yaml(def: &IconTheme, yaml: &IconTheme) {
        assert_eq!(def.filetype.dir, yaml.filetype.dir,);
    }

    #[test]
    fn test_default_theme() {
        let def = IconTheme::default();
        let yaml = Theme::with_yaml(partial_default_yaml()).unwrap();
        check_partial_yaml(&def, &yaml);
    }

    #[test]
    fn test_tmp_partial_default_theme_file() {
        use std::fs::File;
        use std::io::Write;
        let dir = assert_fs::TempDir::new().unwrap();
        let theme = dir.path().join("icon.yaml");
        let mut file = File::create(&theme).unwrap();
        writeln!(file, "{}", partial_default_yaml()).unwrap();
        let def = IconTheme::default();
        let decoded = Theme::from_path(theme.to_str().unwrap()).unwrap();
        check_partial_yaml(&def, &decoded);
    }

    #[test]
    fn test_empty_theme_return_default() {
        // Must contain one field at least
        // ref https://github.com/dtolnay/serde-yaml/issues/86
        let empty: IconTheme = Theme::with_yaml("  ").unwrap();
        let default = IconTheme::default();
        check_partial_yaml(&empty, &default);
    }

    #[test]
    fn test_partial_theme_return_default() {
        // Must contain one field at least
        // ref https://github.com/dtolnay/serde-yaml/issues/86
        let empty: IconTheme = Theme::with_yaml("filetype:\n  dir: ").unwrap(); //  is the default value
        let default = IconTheme::default();
        check_partial_yaml(&empty, &default);
    }

    #[test]
    fn test_serde_dir_from_yaml() {
        // Must contain one field at least
        // ref https://github.com/dtolnay/serde-yaml/issues/86
        let empty: IconTheme = Theme::with_yaml("filetype:\n  dir: ").unwrap();
        assert_eq!(empty.filetype.dir, "");
    }

    #[test]
    fn test_custom_icon_by_name() {
        // When a user sets to use 📦-icon for a cargo.toml file,
        let theme: IconTheme = Theme::with_yaml("name:\n  cargo.toml: 📦").unwrap();
        // 📦-icon should be used for a cargo.toml file.
        assert_eq!(theme.name.get("cargo.toml").unwrap(), "📦");
    }

    #[test]
    fn test_default_icon_by_name_with_custom_entry() {
        // When a user sets to use 📦-icon for a cargo.toml file,
        let theme: IconTheme = Theme::with_yaml("name:\n  cargo.toml: 📦").unwrap();
        // the default icon  should be used for a cargo.lock file.
        assert_eq!(theme.name.get("cargo.lock").unwrap(), "\u{e7a8}");
    }

    #[test]
    fn test_custom_icon_by_extension() {
        // When a user sets to use 🦀-icon for *.rs files,
        let theme: IconTheme = Theme::with_yaml("extension:\n  rs: 🦀").unwrap();
        // 🦀-icon should be used for *.rs files.
        assert_eq!(theme.extension.get("rs").unwrap(), "🦀");
    }

    #[test]
    fn test_default_icon_by_extension_with_custom_entry() {
        // When a user sets to use 🦀-icon for *.rs files,
        let theme: IconTheme = Theme::with_yaml("extension:\n  rs: 🦀").unwrap();
        // the default icon  should be used for *.go files.
        assert_eq!(theme.extension.get("go").unwrap(), "\u{e627}");
    }
}
