import * as fs from 'fs';

let data = fs.readFileSync("../input05").toString()
    .split("\n")

const WIDTH = 8;
const LENGTH = 128;

function getPosition(origPath: string, path: string, rowLow: number, rowHigh: number, colLow: number, colHigh: number): {col: number, row: number} {
    const rowMiddle = (rowLow + rowHigh) / 2;
    const colMiddle = (colLow + colHigh) / 2;
    if (path.length == 0) {
        let retVal = {
            col: origPath[origPath.length - 1] == 'L' ? colLow : colHigh - 1,
            row: origPath[origPath.length - 4] == 'F' ? rowLow : rowHigh - 1
        };
        return retVal;
    }

    switch (path[0]) {
        case "F": rowHigh = rowMiddle; break;
        case "B": rowLow = rowMiddle; break;
        case "L": colHigh = colMiddle; break;
        case "R": colLow = colMiddle; break;
    }

    return getPosition(origPath, path.substr(1), rowLow, rowHigh, colLow, colHigh);
}

let ids = [];
for (let line of data) {
    let {col, row} = getPosition(line, line, 0, LENGTH, 0, WIDTH);
    ids.push(col + (row * WIDTH));
}
ids = ids.sort((a, b) => a - b);

console.log("Part1, max value: ", ids[ids.length - 1]);

for (let i = 1; i < ids.length; i++) {
    if (ids[i] - ids[i - 1] == 2) {
        console.log("Part 2: Gap detected at " + i + " from " + ids[i-1] + " to " + ids[i] + " meaning my seat is at id " + (ids[i] - 1));
    }
}
