let is_prime = require("./largest_prime")

it("5", () => {
expect(is_prime(5)).toBe(true)
})

it("3", () => {
expect(is_prime(3)).toBe(true)
})

it("7", () => {
expect(is_prime(7)).toBe(true)
})

it("16", () => {
   expect(is_prime(16)).toBe(false)
})

it("18", () => {
expect(is_prime(89)).toBe(true)
})
