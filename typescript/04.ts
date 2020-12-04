import * as fs from 'fs';

let data = fs.readFileSync("../input04").toString()
    .split("\n\n")
    .map(pass => pass.split("\n").join(" "))

const expectedFields = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
//    "cid",
]

let valid = 0;
for (let passport of data) {
    let items = new Map<string, string>();
    passport.split(" ")
        .forEach(prop => {
            let parts = prop.split(":");
            items.set(parts[0], parts[1]);
        });

        let count = 0
        for (let field of expectedFields) {
            if (items.has(field)) {
                count++;
            }
        }
        if (count == 7) {
            valid++;
    }
}  


console.log("Part1, valid: " + valid);


valid = 0;
for (let passport of data) {
    let items = new Map<string, string>();
    passport.split(" ").forEach(prop => {
        let parts = prop.split(":");
        items.set(parts[0], parts[1]);
    });

    let count = 0
    for (let field of expectedFields) {
        if (items.has(field)) {
            count++;
        }
    }

    if (count != 7) {
        continue;
    }

    // Validate byr
    if (!valid_limit(items, "byr", 1920, 2002)
        || !valid_limit(items, "iyr", 2010, 2020)
        || !valid_limit(items, "eyr", 2020, 2030)
    ) {
        continue;
    }

    let hgt = items.get("hgt")!;
    if (hgt.endsWith("cm")) {
        let val = +hgt.replace("cm", "");
        if (val < 150 || val > 193) {
            continue
        }
    } else if (hgt.endsWith("in")) {
        let val = +hgt.replace("in", "");
        if (val < 59 || val > 76) {
            continue
        }
    } else {
        continue;
    }

    if (!/#[0-9a-f]{6}/.exec(items.get("hcl")!)) {
        continue;
    }
    
    if (!/(amb|blu|brn|gry|grn|hzl|oth)/.exec(items.get("ecl")!)) {
        continue;
    }
    
    let pid = items.get("pid")!;
    if (+pid == NaN || pid.length != 9) {
        continue;
    }

    valid++;
}  

function valid_limit(items: Map<string, string>, field: string, lower: number, upper: number): boolean {
    let value = +items.get(field)!;
    if (value < lower || value > upper) {
        return false;
    }
    return true;
}

console.log("Part2, valid: " + valid);