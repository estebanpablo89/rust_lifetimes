fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }
        if lang == current {
            found = true;
        }
    }

    languages.last().unwrap()
}

fn last_language (languages: &[String]) -> &str {
    languages.last().unwrap()
}

fn longest<'a>(long_a: &'a str, long_b: &'a str) -> &'a str {
    if long_a.len() > long_b.len() {
        long_a
    } else {
        long_b
    }
}

fn main() {
    let language = vec![
        String::from("Rust"),
        String::from("Python"),
        String::from("JavaScript"),
    ];

    let result = next_language(&language, "Python");

    println!("{}", result);
    
}
