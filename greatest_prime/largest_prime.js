let number = 600851475143;
let prime_numbers = []
let copy_number = number

function is_prime(number) {
    if ([1, 2, 3, 5].includes(number)) return true
    else if (![1, 7, 3, 9].includes(number % 10)) return false
    for (let i = 2; i < Math.sqrt(number); i++) {
        if (number % i == 0) return false
    }
    return true
}

function find_largest_prime_factor(number) {
    for (let i = 2; i < Math.sqrt(number); i++) {
        if (is_prime(i) && copy_number % i == 0) {
            copy_number /= i
           if (copy_number == 1) return i
           i-=1
        }
    }
}

let result = find_largest_prime_factor(number)
console.log(result)

module.exports = is_prime
