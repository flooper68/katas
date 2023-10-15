import { describe, expect, it } from "bun:test";
import { compress } from "./compress";

describe("compress", () => {
  it("returns compressed input string", () => {
    const result = compress("abcab");

    expect(result).toEqual([
      ["a", "b", "c"],
      [0, 1, 2, 0, 1],
    ]);
  });

  it("works for empty string", () => {
    const result = compress("");

    expect(result).toEqual([[], []]);
  });
});
