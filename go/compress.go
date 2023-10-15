package solution


func compress(input string) ([]int, []rune)  {
  indexes := make([]int, len(input))
  chars := make([]rune, 0)
  indexMap := make(map[rune]int)

  for i, c := range input {
    if indexMap[c] == 0 {
      indexMap[c] = len(indexMap) + 1
      chars = append(chars, c)
    }

    indexes[i] = indexMap[c]
  }

  return indexes, chars
}
