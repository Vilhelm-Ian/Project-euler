let a = `7
3
1
6
7
1
7
6
5
3
1
3
3
`

let arr = a.split("").filter(number=> number !== "\n").length

console.log(arr)
