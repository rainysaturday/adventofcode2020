import * as fs from 'fs';

let data = fs.readFileSync("../input02").toString()
    .split("\n")

let valid1 = 0;
let valid2 = 0;

for (let line of data) {
    let g = /(\d+)-(\d+) (.): (.+)/.exec(line);
    if (g == undefined) {
        console.log("Bad line? " + line);
    } else {
        let min = +g[1];
        let max = +g[2];
        let char = g[3];
        let pass = g[4];

        let charMap = new Map<string, number>();
        pass.split('').forEach(c => {
            charMap.set(c, charMap.get(c) == undefined ? 1 : charMap.get(c)! + 1);
        });
        let count = charMap.get(char);
        if (count == undefined) {
            count = 0;
        }
        console.log("Pass: " + pass + " and charmap: ", charMap, g)
        if (count >= min && count <= max) {
            valid1++;
        }

        let pos1 = min - 1;
        let pos2 = max - 1;
        if ((pass[pos1] == char || pass[pos2] == char) && pass[pos1] != pass[pos2]) {
            valid2++;
        }
    }
}

console.log(`We have ${valid1} valid passwords for day1`);
console.log(`We have ${valid2} valid passwords for day2`);