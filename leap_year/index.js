let d = new Date("1901-01-01")

let result = 0;

while (d.getFullYear() < 2001) {
  let day = d.getDay()
  if (d.getDate() == 1 && day == 0) {
  result += 1
  }
  d.setDate(d.getDate()+1)
}

console.log(result)
