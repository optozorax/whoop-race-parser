use ordered_float::OrderedFloat;
use std::fs;
use inline_python::python;

type of64 = OrderedFloat<f64>;

fn median(numbers: &mut [of64]) -> of64 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn read_times(path: &str) -> Vec<of64> {
    let mut times: Vec<of64> = vec![];
    for file in fs::read_dir(path).unwrap().map(|x| x.unwrap()) {
        let s = fs::read_to_string(file.path()).unwrap();
        let iter = s.split('\n').skip_while(|x| !x.starts_with("Lap# ")).skip(1).filter(|x| !x.is_empty());
        for line in iter {
            let time = line.split(' ').filter(|x| !x.is_empty()).nth(1).unwrap().parse::<f64>().unwrap();
            times.push(time.try_into().unwrap());
        }
    }
    times
}

fn filter_times(times: &mut [of64]) -> Vec<of64> {
    let median_time = median(times);
    times.iter().filter(|x| **x < median_time * 1.5).copied().collect::<Vec<_>>()
}

fn main() {
    let mut times2 = read_times("C:/Users/1/YandexDisk/my/gonki/home_1");
    let mut times = read_times("C:/Users/1/YandexDisk/my/gonki/home_1_2");
    // let mut files_count = 0;

    let min_time = *times.iter().min().unwrap();
    let max_time = *times.iter().max().unwrap();
    let avg_time = times.iter().sum::<of64>() / times.len() as f64;
    let median_time = median(&mut times);

    // filteresd extra values
    let times_filtered = filter_times(&mut times);
    let times2_filtered = filter_times(&mut times2);
    let max_time_filtered = times_filtered.iter().max().unwrap();

    let bin_size = 0.5;
    let min_start = (min_time / bin_size).trunc() * bin_size;
    let max_end_filtered = (max_time_filtered / bin_size).ceil() * bin_size;
    let bins_count = (max_end_filtered - min_start) / bin_size;

    // println!("Count of flied batteries: {}", files_count);
    println!("Count of laps: {}", times.len());
    println!("Min time is: {}", min_time);
    println!("Max time is: {}", max_time);
    println!("Avg time is: {}", avg_time);
    println!("Median time is: {}", median_time);

    let times_filtered = times_filtered.into_iter().map(|x| x.into_inner()).collect::<Vec<_>>();
    let times2_filtered = times2_filtered.into_iter().map(|x| x.into_inner()).collect::<Vec<_>>();
    let times = times.into_iter().map(|x| x.into_inner()).collect::<Vec<_>>();

    python! {
        import matplotlib.pyplot as pyplot
        import matplotlib.pyplot as plt
        import numpy as np

        pyplot.locator_params(axis='x', nbins='bins_count * 2)

        n, bins, patches = plt.hist(
            'times2_filtered,
            color="lightgreen",
            ec="black",
            bins=np.arange('min_start, 'max_end_filtered, 'bin_size),
            histtype="stepfilled",
            alpha=0.2,
            density=True,
        )

        plt.hist(
            'times_filtered,
            color="blue",
            ec="black",
            bins=np.arange('min_start, 'max_end_filtered, 'bin_size),
            histtype="stepfilled",
            alpha=0.2,
            density=True,
        )

        print("Max bin: %s".format(max(n)))

        plt.grid()
        plt.show()

        plt.hist(
            'times,
            color="lightgreen",
            ec="black",
            histtype="stepfilled",
            bins=50,
        )
        plt.grid()
        plt.show()
    }
}
