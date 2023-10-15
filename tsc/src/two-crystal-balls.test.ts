import { describe, expect, test } from "bun:test";
import { twoCrystalBalls } from "./two-crystal-balls";

describe("twoCrystalBalls", () => {
  test("", () => {
    expect(twoCrystalBalls([false, false, true])).toBe(2);
    expect(
      twoCrystalBalls([false, false, false, false, false, false, false, true])
    ).toBe(7);
    expect(
      twoCrystalBalls([
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        true,
      ])
    ).toBe(9);
    expect(
      twoCrystalBalls([
        false,
        false,
        false,
        false,
        false,
        false,
        true,
        true,
        true,
        true,
      ])
    ).toBe(6);
    expect(twoCrystalBalls([false, true])).toBe(1);
    expect(twoCrystalBalls([true, true])).toBe(0);
    expect(twoCrystalBalls([])).toBe(-1);
    expect(twoCrystalBalls([false, false])).toBe(-1);
  });
});
