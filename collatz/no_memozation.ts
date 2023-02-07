function find_collatz_sequence(num: number, result = 1) :number  {
   if(num===1) return 1
   if(num%2===0) num = num/2
	 else  num = 3*num+1
	 return find_collatz_sequence(num,result+1)+1
}

let result = [0,0]

for(let i = 2; i<Math.pow(10,6); i++) {
 let collatz  = find_collatz_sequence(i)
 if(collatz>result[1]) result = [i,collatz] 
}

console.log(result) 
