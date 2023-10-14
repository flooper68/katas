import { describe, expect, it } from "bun:test";
import { isAscSorted } from "./is-asc-sorted";

describe("isSorted", () => {
  it("returns true for sorted array", () => {
    const result = isAscSorted([0, 1, 2, 3]);

    expect(result).toBe(true);
  });

  it("returns true for sorted array with subsequent equal numbers", () => {
    const result = isAscSorted([0, 1, 1, 3]);

    expect(result).toBe(true);
  });

  it("returns true for empty array", () => {
    const result = isAscSorted([]);

    expect(result).toBe(true);
  });

  it("returns true for array with single element", () => {
    const result = isAscSorted([1]);

    expect(result).toBe(true);
  });

  it("returns false for unsorted array", () => {
    const result = isAscSorted([1, 3, 2]);

    expect(result).toBe(false);
  });

  it("returns false for desc sorted array", () => {
    const result = isAscSorted([3, 2, 1]);

    expect(result).toBe(false);
  });
});
