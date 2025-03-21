fn bigass(arr: &mut [i32]) {
    let mut gap = arr.len();
    let mut swapped = true;
    while gap > 1 || swapped {
        gap = (gap * 10) / 13;
        if gap < 1 {
            gap = 1;
        }
        swapped = false;
        // forward pass
        for i in 0..arr.len() - gap {
            if arr[i] > arr[i + gap] {
                arr.swap(i, i + gap);
                swapped = true;
            }
        }
        // backward pass
        for i in (gap..arr.len()).rev() {
            if arr[i - gap] > arr[i] {
                arr.swap(i - gap, i);
                swapped = true;
            }
        }
    }
}
fn main() {
    let mut arr = [5, 1, 4, 2, 8, 0, 3];
    println!("Before: {:?}", arr);
    bigass(&mut arr);    
    println!("After: {:?}", arr);
}