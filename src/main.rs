use pinyin::ToPinyin;
use std::{io::Write, path::PathBuf};

fn main() {
    let arg = std::env::args().collect::<Vec<String>>();
    if arg.len() != 2 {
        println!("Usage:\n.\"{}\" <FullFilePath>", arg[0]);
        return;
    }
    let path = {
        let temp = PathBuf::from(
            arg[1]
                .replace("\r\n", "\n")
                .trim_matches(&['"', '\'', '\\', '/', '\t', '\n', '\r']),
        );
        let temp = if temp.is_absolute() {
            temp
        } else {
            PathBuf::from(arg[0].clone()).join(temp)
        };
        if !temp.exists() && temp.is_file() {
            PathBuf::new()
        } else {
            temp
        }
    };
    if path.as_os_str().is_empty() {
        panic!();
    }

    let ext = path.extension().unwrap().to_str().unwrap();
    let output_path = PathBuf::from(format!(
        "{}_out.{}",
        path.to_str()
            .unwrap()
            .trim_end_matches(&format!(".{}", ext)),
        ext
    ));

    let mut out_file = std::fs::File::create(&output_path).unwrap();
    let file = std::fs::read_to_string(path).unwrap();
    let content = file.lines().collect::<Vec<&str>>();

    let mut collection: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    for line in content {
        if line.is_empty() {
            continue;
        }
        let sentence_py = line
            .to_pinyin()
            .map(|f| match f {
                Some(f) => f.with_tone_num_end(),
                None => {
                    println!("{}", line);
                    ""
                }
            })
            .collect::<Vec<&str>>()
            .join("");
        collection.insert(sentence_py, line.to_string());
    }

    let mut vec_collection = collection.into_iter().collect::<Vec<(String, String)>>();
    vec_collection.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    for (_, i) in vec_collection {
        out_file.write(format!("{}\n", i).as_bytes()).unwrap();
    }
    out_file.flush().unwrap();

    println!("{}\nFinished.", output_path.display());
}
