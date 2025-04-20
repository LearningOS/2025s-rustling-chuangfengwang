/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T])
where
    T: PartialOrd + Copy,
{
    let len = array.len();
    // 如果数组长度小于等于1，无需排序
    if len <= 1 {
        return;
    }

    for i in 0..len {
        // 标志位，检查是否发生交换
        let mut swapped = false;
        // 每次遍历后，最大的元素会冒泡到最后，所以可以减少内层循环次数
        for j in 0..len - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                swapped = true;
            }
        }

        // 如果这一轮没有发生交换，说明数组已经有序
        if !swapped {
            break;
        }
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