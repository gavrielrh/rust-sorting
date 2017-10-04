
extern crate csv;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::process;
use std::time::Instant;
use rand::Rng;
use csv::Writer;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record<'a> {
    sort: &'a str,
    data_type: &'a str,
    n: usize,
    ms: u64,
}

fn run() -> Result<(), Box<Error>> {
    let mut wtr = Writer::from_path("sort_data_i32.csv")?;

    let mut n = 1000;
    while n <= 125000 {
        let v_i32: Vec<i32> = rand::thread_rng().gen_iter::<i32>().take(n).collect();
        // let v_char: Vec<char> = rand::thread_rng()
        //     .gen_ascii_chars()
        //     .take(n)
        //     .collect();

        let mut v_i32_mutable = v_i32.clone();

        wtr.serialize(Record {
            sort: "Bubble Sort",
            data_type: "i32",
            n: n,
            ms: get_sort_time(v_i32.clone(), bubble_sort),
        })?;

        wtr.serialize(Record {
            sort: "Better Bubble Sort",
            data_type: "i32",
            n: n,
            ms: get_sort_time(v_i32.clone(), better_bubble_sort),
        })?;

        wtr.serialize(Record {
            sort: "Insertion Sort",
            data_type: "i32",
            n: n,
            ms: get_sort_time(v_i32.clone(), insertion_sort),
        })?;

        wtr.serialize(Record {
            sort: "Merge Sort",
            data_type: "i32",
            n: n,
            ms: get_sort_time(v_i32.clone(), merge_sort),
        })?;

        wtr.serialize(Record {
            sort: "Quick Sort",
            data_type: "i32",
            n: n,
            ms: get_sort_time_mutable(&mut v_i32_mutable, quick_sort),
        })?;

        wtr.serialize(Record {
            sort: "Heap Sort",
            data_type: "i32",
            n: n,
            ms: get_sort_time(v_i32.clone(), heap_sort),
        })?;

        n *= 5;
    }

    wtr.flush()?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn get_sort_time<T: Ord + Clone + Copy>(v: Vec<T>, f: fn(Vec<T>) -> Vec<T>) -> u64 {
    let start = Instant::now();
    f(v);
    start.elapsed().as_secs() * 1000 + start.elapsed().subsec_nanos() as u64 / 1_000_000
}

fn get_sort_time_mutable<T: Ord + Clone + Copy>(
    mut v: &mut Vec<T>,
    f: fn(&mut [T]) -> Vec<T>,
) -> u64 {
    let start = Instant::now();
    f(&mut v);
    start.elapsed().as_secs() * 1000 + start.elapsed().subsec_nanos() as u64 / 1_000_000
}

fn bubble_sort<T: Ord + Clone>(mut v: Vec<T>) -> Vec<T> {
    let n = v.len();
    loop {
        let mut swapped = false;
        for i in 1..n {
            if v[i - 1] > v[i] {
                v.swap(i - 1, i);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }

    v
}

fn better_bubble_sort<T: Ord + Clone>(mut v: Vec<T>) -> Vec<T> {
    let mut n = v.len();
    while n != 0 {
        let mut new_n = 0;
        for i in 1..n {
            if v[i - 1] > v[i] {
                v.swap(i - 1, i);
                new_n = i
            }
        }
        n = new_n;
    }

    v
}

fn insertion_sort<T: Ord + Clone>(mut v: Vec<T>) -> Vec<T> {
    let mut i = 1;
    while i < v.len() {
        let mut j = i;
        while j > 0 && v[j - 1] > v[j] {
            v.swap(j, j - 1);
            j = j - 1;
        }
        i = i + 1;
    }

    v
}

fn merge_sort<T: Ord + Clone>(v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    }

    let (left, right) = v.split_at(v.len() / 2);

    let left_sorted = merge_sort(left.to_vec());
    let right_sorted = merge_sort(right.to_vec());

    merge(&left_sorted, &right_sorted)
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left_sorted = left.to_vec();
    let mut right_sorted = right.to_vec();

    while !left_sorted.is_empty() && !right_sorted.is_empty() {
        if left_sorted[0] <= right_sorted[0] {
            result.push(left_sorted.remove(0));
        } else {
            result.push(right_sorted.remove(0));
        }
    }

    while !left_sorted.is_empty() {
        result.push(left_sorted.remove(0));
    }
    while !right_sorted.is_empty() {
        result.push(right_sorted.remove(0));
    }

    result
}

fn quick_sort<T: Ord + Clone + Copy>(v: &mut [T]) -> Vec<T> {
    let n = v.len();

    if n <= 1 {
        return v.to_vec();
    } else if n == 2 {
        if v[0] > v[v.len() - 1] {
            v.swap(0, 1);
        }

        return v.to_vec();
    } else {
        let mut pivot = 0;

        for i in 1..n {
            if v[pivot as usize] > v[i as usize] {
                v.swap(pivot as usize, i as usize);

                if i == pivot + 1 {
                    pivot = i;
                } else {
                    v.swap(i as usize, (pivot + 1) as usize);
                    pivot = pivot + 1;
                }
            }
        }

        quick_sort(&mut v[0..(pivot + 1) as usize]);
        quick_sort(&mut v[(pivot + 1) as usize..n]);

        v.to_vec()
    }
}

fn heap_sort<T: Ord + Clone>(mut v: Vec<T>) -> Vec<T> {
    let n = v.len() as i32;

    for i in (0..(n / 2)).rev() {
        heapify(&mut v, n, i);
    }

    for i in (0..n).rev() {
        v.swap(0, i as usize);

        heapify(&mut v, i, 0);
    }

    v
}

fn heapify<T: Ord + Clone>(mut v: &mut [T], n: i32, i: i32) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && v[left as usize] > v[largest as usize] {
        largest = left;
    }

    if right < n && v[right as usize] > v[largest as usize] {
        largest = right;
    }

    if largest != i {
        v.swap(i as usize, largest as usize);

        heapify(&mut v, n, largest);
    }
}
