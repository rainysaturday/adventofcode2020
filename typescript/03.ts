import * as fs from 'fs';

let data = fs.readFileSync("../input03").toString()
    .split("\n")

let r1d1trees = 0;
let r3d1trees = 0;
let r5d1trees = 0;
let r7d1trees = 0;
let r1d2trees = 0;
for (let i = 0; i < data.length; i++) {
    const line = data[i];
    if (line[(i*1) % line.length] == '#') {
        r1d1trees++;
    }
    if (line[(i*3) % line.length] == '#') {
        r3d1trees++;
    }
    if (line[(i*5) % line.length] == '#') {
        r5d1trees++;
    }
    if (line[(i*7) % line.length] == '#') {
        r7d1trees++;
    }
    if (i % 2 == 0 && line[(i/2) % line.length] == '#') {
        r1d2trees++;
    }
}
console.log("Part1 trees: ", r3d1trees);
console.log("Path2 trees: ", (r1d1trees * r1d2trees * r3d1trees * r5d1trees * r7d1trees));