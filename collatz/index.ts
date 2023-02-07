function find_collatz_sequence(num: number, result = 1) :number  {
   if(num===1) return result
   if(num%2===0) num = num/2
	 else  num = 3*num+1
	 return newColaltz(num,result+1)+1
}



const memoizeCollatz = () => {
   interface Collatz {
      [key: string]: number
   }
   let cache: Collatz = {}
   type ObjectKey = keyof typeof cache
   return (num: number, result = 1) => {
      let key = num as ObjectKey
      if(num in cache) {
         return cache[num]
      }
      else {
         let length_of_sequence = find_collatz_sequence(num) 
         cache[key] = length_of_sequence
         return length_of_sequence
      }
   }
}
 
 
const newColaltz = memoizeCollatz()



let result = [0,0]

for(let i = 2; i<Math.pow(10,6); i++) {
 let collatz  = newColaltz(i)
 if(collatz>result[1]) result = [i,collatz] 
}

console.log(result) 
