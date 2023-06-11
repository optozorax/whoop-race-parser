use std::collections::BTreeMap;
use std::fs;

fn read_times(path: &std::path::PathBuf) -> BTreeMap<String, Vec<f64>> {
    let mut result: BTreeMap<String, Vec<f64>> = Default::default();
    for folder in fs::read_dir(path).unwrap().map(|x| x.unwrap()) {
        if folder.path().ends_with(".DS_Store") {
            continue;
        }
        let mut times = vec![];
        for file in fs::read_dir(folder.path()).unwrap().map(|x| x.unwrap()) {
            let s = fs::read_to_string(file.path()).unwrap();
            let iter = s.split('\n').skip_while(|x| !x.starts_with("Lap# ")).skip(1).filter(|x| !x.is_empty());
            for line in iter {
                let time = line.split(' ').filter(|x| !x.is_empty()).nth(1).unwrap().parse::<f64>().unwrap();
                // actually wrong results
                if !line.starts_with("//") {
                    times.push(time);
                }
            }
        }
        result.insert(folder.path().file_name().unwrap().to_str().unwrap().to_string(), times);
    }
    result
}

fn main() {
    #[cfg(target_os = "macos")] let gonki_path = "/Users/optozorax/Yandex.Disk.localized/my/gonki";
    #[cfg(target_os = "linux")] let gonki_path = "unknown";
    #[cfg(target_os = "windows")] let gonki_path = "C:/Users/1/YandexDisk/my/gonki";

    for folder in fs::read_dir(gonki_path).unwrap().map(|x| x.unwrap()) {
        if folder.path().ends_with(".DS_Store") {
            continue;
        }
        let folder_name = folder.path().file_name().unwrap().to_str().unwrap().to_string();
        let day_values = read_times(&folder.path());

        println!("------------------------------------------------------\nFolder: {folder_name}");
        println!();

        for (name, values) in day_values {
            println!("Day: {folder_name}/{name}");
            println!("Values: {}", values.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
            println!();
            println!();
        }
    }
}
