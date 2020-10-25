use std::collections::HashMap;

const SIZE: usize = 10;

fn main() {
    // creating array, then sorts it (sorting code is from original rust docs)
    let mut list: [f32; SIZE] = [8.0, 9.0, 10.0, 10.0, 10.0, 11.0, 11.0, 11.0, 12.0, 13.0];
    list.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // passing list as reference because we don't want to alter the list.
    let mean = calc_mean(&list);
    println!("Mean: {}", mean);

    let median = calc_median(&list, list.len());
    println!("Median: {}", median);

    let mode = calc_mode(list.iter());
    println!("{:?}", mode);


    let range = calc_range(&list);
    println!("{}", range);
}

fn calc_mean(list: &[f32; SIZE]) -> f32 {
    // iter creates iterator, fold takes initial value,
    // acc is accumulator and takes 0.0, x is value of current iteration, acc + x, result is next acc
    list.iter().fold(0.0, |acc, x| acc + x) / list.len() as f32
}

fn calc_median(list: &[f32; SIZE], len: usize) -> f32 {
    if list.len() % 2 == 0 {
        let middle = (list.len() as f32 + 1.0) / 2.0 - 1.0;
        (list[middle.floor() as usize] + list[middle.ceil() as usize]) / 2.0
    } else {
        list[((len as f32 + 1.0) / 2.0) as usize]
    }

}

// create generic to make iter a parameter
fn calc_mode<'a, I>(l: I) -> Option<u32>
where I: IntoIterator<Item = &'a f32>
{
    let mut m: HashMap<u32, u32> = HashMap::new();

    for n in l {
        let o = *n;
        let c = m.entry(o as u32).or_insert(0);
        *c += 1;
    }

    m.iter()
    .max_by(|a, b| a.1.cmp(&b.1))
    .map(|(k, _v)| *k)
}

fn calc_range(l: &[f32; SIZE]) -> f32 {
    l[l.len() - 1] - l[0]
}