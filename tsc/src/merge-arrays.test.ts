import { describe, expect, it } from "bun:test";
import { mergeArrays } from "./merge-arrays";

describe("mergeArrays", () => {
  it("should merge arrays", () => {
    const nums1 = [1, 2, 3, 0, 0, 0];
    const m = 3;
    const nums2 = [2, 5, 6];
    const n = 3;

    mergeArrays(nums1, m, nums2, n);

    expect(nums1).toEqual([1, 2, 2, 3, 5, 6]);
  });

  it("should merge arrays given empty first array", () => {
    const nums1 = [0];
    const m = 0;
    const nums2 = [1];
    const n = 1;

    mergeArrays(nums1, m, nums2, n);

    expect(nums1).toEqual([1]);
  });

  it("should merge arrays given empty second array", () => {
    const nums1 = [1];
    const m = 1;
    const nums2 = [0];
    const n = 0;

    mergeArrays(nums1, m, nums2, n);

    expect(nums1).toEqual([1]);
  });
});
