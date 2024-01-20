#[macro_export]
/// Time the execution of a statement and print the result to stdout.
/// The statement can be an expression that returns a value.
macro_rules! time_it {
    ($comment:literal => $stmt:stmt) => {{
        time_it!(concat!($comment, "") => {$stmt})
    }};
    (at once | $comment:literal => $stmt:stmt) => {{
        time_it!(at once | concat!($comment, "") => {$stmt})
    }};
    ($comment:expr => $stmt:stmt) => {{
        use std::io::Write;
        print!("{} => ", $comment);
        let _ = std::io::stdout().flush();
        let start = std::time::Instant::now();
        let result = { $stmt };
        let duration = start.elapsed();
        println!("{:?}", duration);
        result
    }};
    (at once | $comment:expr => $stmt:stmt) => {{
        use std::io::Write;
        let start = std::time::Instant::now();
        let result = { $stmt };
        let duration = start.elapsed();
        println!("{} => {:?}", $comment, duration);
        result
    }};
}

#[macro_export]
macro_rules! write_items_to_file {
    ($items:expr) => {{
        use $crate::time_it;
        use $crate::debug;
        use std::fs::File;
        let var_name_with_spaces = stringify!($items).replace("_", " ");
        let comment = format!("writing {var_name_with_spaces} to ron and json files");
        time_it!(at once | comment => {
            let output_path_ron = format!("temp/{}.ron", stringify!($items));
            let output_path_json = format!("temp/{}.json", stringify!($items));

            let mut file_ron = File::create(output_path_ron).unwrap();
            let mut file_json = File::create(output_path_json).unwrap();

            let items_pretty_ron = ron::ser::to_string_pretty(&$items, ron::ser::PrettyConfig::default()).unwrap();
            let items_pretty_json = ureq::serde_json::to_string_pretty(&$items).unwrap();

            file_ron.write_all(items_pretty_ron.as_bytes()).unwrap();
            file_json.write_all(items_pretty_json.as_bytes()).unwrap();

            debug!(format!("temp/{}.ron", stringify!($items)), format!("temp/{}.json", stringify!($items)));
        });
    }};
}

#[macro_export]
macro_rules! debug {
    ($val:expr) => {
        #[cfg(debug_assertions)]
        {
            dbg!($val)
        }
    };
    ($($val:expr),+ $(,)?) => {
        #[cfg(debug_assertions)]
        {
            dbg!($($val),+)
        }
    };
}
