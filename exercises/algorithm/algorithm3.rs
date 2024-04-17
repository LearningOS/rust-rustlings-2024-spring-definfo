/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

// selection sort
// fn sort<T>(array: &mut [T] where T: std::cmp::PartialOrd {
    // let len = array.len();
    // for i in 0..(len - 1) {
        // let mut ith_index = i;
        // for j in (i + 1)..len {
            // if array[j] < array[ith_index] {
                // ith_index = j;
            // }
        // }
        // array.swap(i, ith_index);
    // }
// }

// bubble sort
// fn sort<T>(array: &mut [T]) where T: std::cmp::PartialOrd {
    // let len = array.len();
    // for i in 0..len {
        // for j in 0..(len - i - 1) {
            // if array[j] > array[j + 1] {
                // array.swap(j, j + 1);
            // }
        // }
    // }
// }

// insertion sort
// fn sort<T>(array: &mut [T]) where T: Copy + std::cmp::PartialOrd {
    // let len = array.len();
    // if len < 2 { return; }
    // for i in 1..len {
        // let mut key = array[i];
        // let mut j = i - 1;
        // while j >= 0 && array[j] > key {
            // array.swap(j, j + 1);
            // // deal with overflow
            // if j > 0 {
                // j -= 1;
            // } else {
                // break;
            // }
        // } 
    // }
// }

// quick sort using stl
// fn sort<T>(array: &mut [T]) where T: std::cmp::Ord {
    // array.sort();
// }


// heap sort using stl
// #![feature(sort_internals)]
// fn sort<T>(array: &mut [T]) where T: std::cmp::PartialOrd {
    // core::slice::heapsort(array, |a, b| a < b);
// }

// heap sort
fn heaptify<T>(array: &mut [T], start: usize, end: usize) where T: std::cmp::PartialOrd {
    let mut largest = start;

    let mut left_child = largest * 2 + 1;
    if left_child < end && array[left_child] > array[largest] {
        largest = left_child;
    }
    let right_child = left_child + 1;
    if right_child < end && array[right_child] > array[largest] {
        largest = right_child;
    }

    if largest != start {
        array.swap(start, largest);
        heaptify(array, largest, end);
    }
}
fn sort<T>(array: &mut [T]) where T: std::cmp::PartialOrd {
    let len =  array.len();
    // build large root heap
    for i in (0..len / 2).rev() {
        heaptify(array, i, len);
    }
    for i in (1..len).rev() {
        array.swap(0, i);
        heaptify(array, 0, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}