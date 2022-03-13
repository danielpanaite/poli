"use strict";

let str1 = ["Mario", "Fabio", "Gatto", "wow", "a", "ma"];

for(const s of str1){
    if((s.length) < 2){
        console.log("");
        continue;
    }
    console.log(s.slice(0,2) + s.slice(s.length-2,s.length));
}