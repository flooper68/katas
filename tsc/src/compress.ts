/**
 * 'abcab' => [['a', 'b', 'c'], [0, 1, 2, 0, 1]]
 */
export function compress(input: string) {
  const chars: string[] = [];
  const indexes: number[] = [];
  const indexMap: Record<string, number> = {};

  for (const char of input) {
    if (indexMap[char] === undefined) {
      indexMap[char] = chars.length;
      chars.push(char);
    }

    indexes.push(indexMap[char]);
  }
  return [chars, indexes];
}
