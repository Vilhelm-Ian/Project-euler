let prime_numbers = []

function is_prime(number) {
    if ([1, 2, 3, 5].includes(number)) return true
    else if (![1, 7, 3, 9].includes(number % 10)) return false
    for (let i = 2; i <= Math.sqrt(number); i++) {
        if (number % i == 0) return false
    }
    return true
}

module.exports = is_prime

for (let i = 2; prime_numbers.length < 10001; i++) {
    if (is_prime(i)) prime_numbers.push(i)

}
console.log(prime_numbers[prime_numbers.length - 1])
