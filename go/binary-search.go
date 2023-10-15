package solution

func binarySearch(arr []int, value int) int {
	low := 0
	high := len(arr)

	for low < high {
		mid := int((low + high) / 2)

		if arr[mid] == value {
			return mid
		} else if arr[mid] < value {
			low = mid + 1
		} else {
			high = mid
		}
	}

	return -1
}
