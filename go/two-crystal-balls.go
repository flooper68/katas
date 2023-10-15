package solution

import "math"

func twoCrystalBalls(breaks []bool) int {

	stepSize := int(math.Floor(math.Sqrt(float64(len(breaks)))))
	step := 0

	for step*stepSize < len(breaks) {
		if breaks[step*stepSize] {
			break
		}

		step++
	}

	if step*stepSize >= len(breaks) {
		step -= 1
	}

	for i := step * stepSize; i < (step+1)*stepSize && i < len(breaks); i++ {
		if breaks[i] {
			return i
		}
	}

	return -1

}
