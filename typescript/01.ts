import * as fs from 'fs';

let data = fs.readFileSync("../input01").toString()
    .split("\n")
    .map(v => parseInt(v))

for (let i = 0; i < data.length; i++) {
    for (let u = i+1; u < data.length; u++) {
        if (data[i] + data[u] == 2020) {
            console.log(`Part 1: ${data[i]} * ${data[u]} = ${data[i] * data[u]}`);
        }
    }
}


for (let i = 0; i < data.length; i++) {
    for (let u = i+1; u < data.length; u++) {
        for (let y = u+1; y < data.length; y++) {
            if (data[i] + data[u] + data[y] == 2020) {
                console.log(`Part 1: ${data[i]} * ${data[u]} * ${data[y]} = ${data[i] * data[u] * data[y]}`);
            }
        }
    }
}
