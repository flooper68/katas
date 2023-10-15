export function twoCrystalBalls(breaks: boolean[]): number {
  const stepSize = Math.floor(Math.sqrt(breaks.length));
  let step = 0;

  while (step * stepSize < breaks.length) {
    if (breaks[step * stepSize]) {
      break;
    }

    step++;
  }

  if (step * stepSize >= breaks.length) {
    step -= 1;
  }

  for (
    let i = step * stepSize;
    i < (step + 1) * stepSize && i < breaks.length;
    i++
  ) {
    if (breaks[i]) {
      return i;
    }
  }

  return -1;
}
