use std::{
    collections::HashMap,
    io::{self, BufRead},
};
use utils::{read_int, read_string};

fn main() {
    let mut book: HashMap<String, String> = HashMap::new();
    let entries = read_int();

    (0..entries).for_each(|_| {
        let entries: Vec<String> = read_string()
            .split(" ")
            .take(2)
            .map(|x| x.to_string())
            .collect();

        let name = entries[0].to_string();
        let phone = entries[1].to_string();

        book.insert(name, phone);
    });

    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().trim().to_string())
        .for_each(|line| {
            let phone = book.get(&line);

            let res = match phone {
                Some(v) => format!("{line}={v}"),
                None => "Not found".to_string(),
            };

            println!("{res}");
        });
}
