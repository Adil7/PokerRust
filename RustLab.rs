fn main(){

    let s1 = String::from("this is a string ");
    let s2 = String::from("concat");
    let s3 = string_concat(&s1, &s2);
    println!("{}", s3);

    let nums = [1.5, 2.7, 3.6, 4.9, 5.8];
    let range_tuple = (0, 4);
    let avg = sub_array_average(&nums, range_tuple);
    println!("Average: {}", avg);

    let mut vals = [1, 2, -7, -8, 0, 0];
    let res = array_signum(&mut vals);
    println!("Array Signum: {:?}", res);

}

fn string_concat<'a>(s1: &'a str, s2: &'a str) -> String {
    format!("{}{}", &s1, &s2)
}

fn sub_array_average(arr: &[f64], range: (usize, usize)) -> f64 {
    let mut sum: f64 = 0.0;
    let len = arr.len();
    let mut start = range.0;
    let end = range.1;
    while start < end {
        sum += arr[start];
        start += 1;
    }
    sum / (len as f64)
}

fn array_signum<'a>(arr: &mut [i32]) -> &[i32] {
    let len = arr.len();
    let mut i = 0;
    while i < len {
        if arr[i] > 0 {arr[i] = 1;}
        else if arr[i] < 0 {arr[i] = -1;}
        else {arr[i] = 0};
        i += 1;
    }
    arr 
}