use std::io::stdin;


fn check_sub_arr(org_arr : &[i32], sub_arr: &[i32]) -> bool {
    let mut i:usize = 0;
    let mut j:usize = 0;
    while i < org_arr.len() - sub_arr.len() + 1 {
        println!("{}", i);
        if org_arr[i] != sub_arr[0] {
            i += 1;
            continue;
        }
        while j < sub_arr.len() {
            if sub_arr[j] != org_arr[i+j] {
                break;
            }
            j += 1;
        }
        if j == sub_arr.len() {
            return true;
        }
        j = 0;
        i += 1;
    }
    false
}

fn main() {
    // 1
    println!("&{}", check_sub_arr(&[1, 2,3,5,6,8, 10, 11], &[6,8,10]));

    // 2
    let mut line = String::new();
    println!("Please type your input:");
    stdin().read_line(&mut line);
    let slice = line.trim_right();
    let paragraph = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
    println!("Number of {} in the paragraph is: {}", slice, paragraph.split(slice).count() - 1);
}
