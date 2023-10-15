export function binarySearch(array: number[], value: number) {
  let bottom = 0;
  let top = array.length;

  while (bottom < top) {
    const middle = Math.floor((bottom + top) / 2);

    if (array[middle] === value) {
      return middle;
    } else if (array[middle] < value) {
      bottom = middle + 1;
    } else {
      top = middle;
    }
  }

  return -1;
}
