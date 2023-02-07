for(let i = 1; i < 1000; i++) {
   for(let z = 1; z< 1000; z++) {
      let a = Math.pow(i,2)-Math.pow(z,2)
      let b = 2*i*z
      let c = Math.pow(i,2)+Math.pow(z,2)
      if(a+b+c===1000) console.log(a*b*c)
   }
} 
