package solution


func isAscSorted(array []int) bool  {
  for i := 1; i < len(array) - 1; i++ {
    if array[i] > array[i + 1] {
      return false
    }
  }

  return true
}
