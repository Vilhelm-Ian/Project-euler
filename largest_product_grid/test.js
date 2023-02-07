//let is_prime = require("./index.js)
let index = require("./index.js")
it("4", () => {
    expect(2 + 2).toBe(4)
})


it("diagonal", () => {
    expect(index.multiply_diagonal_right([
        [1, 2, 3],
        [2, 2, 2],
        [4, 5, 2]
    ], 3, {
        horizontal_position: 0,
        vertical_position: 0
    })).toBe(4)
})

it("diagonal_with_zero", () => {
    expect(index.multiply_diagonal_right([
        [0, 0, 0, 0, 0],
        [0, 1, 2, 3, 0],
        [0, 2, 2, 2, 0],
        [0, 4, 5, 2, 0],
        [0, 0, 0, 0, 0]
    ], 3, {
        horizontal_position: 1,
        vertical_position: 2
    })).toBe(0)
})

it("diagonal_with_zero_right", () => { // 1,2,10i
    expect(index.multiply_diagonal_right([
        [0, 0, 0, 0, 0],
        [0, 1, 2, 3, 0],
        [0, 2, 2, 2, 0],
        [0, 4, 5, 2, 0],
        [0, 0, 0, 0, 0]
    ], 3, {
        horizontal_position: 1,
        vertical_position: 1
    })).toBe(4)
})


it("diagonal_with_zero_left", () => {
    expect(index.multiply_diagonal_left([
        [0, 0, 0, 0, 0],
        [0, 1, 2, 3, 0],
        [0, 2, 2, 2, 0],
        [0, 4, 5, 2, 0],
        [0, 0, 0, 0, 0]
    ], 3, {
        horizontal_position: 1,
        vertical_position: 2
    })).toBe(0)
})

it("diagonal_with_left", () => { // 1, 2, 10
    expect(index.multiply_diagonal_left([
        [0, 0, 0, 0, 0],
        [0, 1, 2, 3, 0],
        [0, 2, 2, 2, 0],
        [0, 4, 5, 2, 0],
        [0, 0, 0, 0, 0]
    ], 3, {
        horizontal_position: 1,
        vertical_position: 3
    })).toBe(24)
})

it("diagonal_with_left_no_zeros", () => {
    expect(index.multiply_diagonal_left([
        [1, 2, 3],
        [2, 2, 2],
        [4, 5, 2]
    ], 3, {
        horizontal_position: 0,
        vertical_position: 2
    })).toBe(24)
})


it("diagonal find the largest", () => {
    expect(index.find_the_largest([
        [0, 0, 0, 0, 0],
        [0, 1, 2, 3, 0],
        [0, 2, 2, 2, 0],
        [0, 4, 5, 2, 0],
        [0, 0, 0, 0, 0]
    ], 3, index.multiply_diagonal_right)).toBe(4)
})

it("down", () => { // 1, 2, 10
    expect(index.multiply_down([
        [0, 0, 0, 0, 0],
        [0, 1, 2, 3, 0],
        [0, 2, 2, 2, 0],
        [0, 4, 5, 2, 0],
        [0, 0, 0, 0, 0]
    ], 3, {
        horizontal_position: 1,
        vertical_position: 3
    })).toBe(12)
})

it("right", () => { // 1, 2, 10
    expect(index.multiply_right([
        [0, 0, 0, 0, 0],
        [0, 1, 2, 3, 0],
        [0, 2, 2, 2, 0],
        [0, 4, 5, 2, 0],
        [0, 0, 0, 0, 0]
    ], 3, {
        horizontal_position: 1,
        vertical_position: 1
    })).toBe(6)
})

