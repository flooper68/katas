import { describe, expect, test } from "bun:test";
import { addNode } from "./add-node";

describe("addNode", () => {
  test("should create a new node if root is null", () => {
    const root = null;

    const result = addNode(root, 5);

    expect(result).toEqual({ value: 5, left: null, right: null });
  });

  test("should add a node to left one level deep", () => {
    const root = {
      value: 10,
      left: null,
      right: null,
    };

    const result = addNode(root, 5);

    expect(result).toEqual({
      value: 10,
      left: {
        value: 5,
        left: null,
        right: null,
      },
      right: null,
    });
  });

  test("should add a node to right one level deep", () => {
    const root = {
      value: 10,
      left: null,
      right: null,
    };

    const result = addNode(root, 11);

    expect(result).toEqual({
      value: 10,
      left: null,
      right: {
        value: 11,
        left: null,
        right: null,
      },
    });
  });

  test("should add a node to right if values are equal", () => {
    const root = {
      value: 10,
      left: null,
      right: null,
    };

    const result = addNode(root, 10);

    expect(result).toEqual({
      value: 10,
      left: null,
      right: {
        value: 10,
        left: null,
        right: null,
      },
    });
  });

  test("should add a node to correct place for a deeper tree", () => {
    const root = {
      value: 10,
      left: {
        value: 5,
        left: {
          value: 2,
          left: null,
          right: null,
        },
        right: null,
      },
      right: {
        value: 15,
        left: null,
        right: null,
      },
    };

    const result = addNode(root, 7);

    expect(result).toEqual({
      value: 10,
      left: {
        value: 5,
        left: {
          value: 2,
          left: null,
          right: null,
        },
        right: {
          value: 7,
          left: null,
          right: null,
        },
      },
      right: {
        value: 15,
        left: null,
        right: null,
      },
    });
  });
});
