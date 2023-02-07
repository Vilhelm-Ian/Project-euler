let arr = [];

for (let i = 0; i < 21; i++) {
   let row = [];
   for (let z = 0; z < 21; z++) {
      if (z === 0) {
         row.push(1);
         continue;
      }
      if (i === 0) {
         row.push(1);
         continue;
      }
      let above = arr[i - 1][z];
      let previous = row[z - 1];
      row.push(above + previous);
   }
   arr.push(row);
}

console.log(arr.at(-1).at(-1));
