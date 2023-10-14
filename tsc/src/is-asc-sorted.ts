export function isAscSorted(array: number[]): boolean {
  for (let i = 0; i < array.length; i++) {
    if (array[i] > array[i + 1]) {
      return false;
    }
  }

  return true;
}
