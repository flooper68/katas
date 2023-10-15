import { describe, expect, test } from "bun:test";
import { binarySearch } from "./binary-search";

describe("binarySearch", () => {
  test("should return the index of the value if it exists in the array", () => {
    expect(binarySearch([1, 2, 3, 4, 5], 3)).toBe(2);
    expect(binarySearch([1, 2], 2)).toBe(1);
    expect(binarySearch([1], 1)).toBe(0);
    expect(binarySearch([], 1)).toBe(-1);
    expect(binarySearch([1, 2], 3)).toBe(-1);
  });
});
