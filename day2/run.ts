import fs from 'fs';

type choiceIndex = { [k: string]: number; }
type scoreIndex = { [k: string]: any; }
type translateIndex = { [k: string]: any; }

const choiceScore: choiceIndex = {
    'X': 1,
    'Y': 2,
    'Z': 3
};

const scoreTable: scoreIndex = {
    'A': {
        'X': 3,
        'Y': 6,
        'Z': 0
    },
    'B': {
        'X': 0,
        'Y': 3,
        'Z': 6
    },
    'C': {
        'X': 6,
        'Y': 0,
        'Z': 3
    }
};

const p2translation: translateIndex = {
    'X': {
        'A': 'Z',
        'B': 'X',
        'C': 'Y'
    },
    'Y': {
        'A': 'X',
        'B': 'Y',
        'C': 'Z'
    },
    'Z': {
        'A': 'Y',
        'B': 'Z',
        'C': 'X'
    }
}

function run() {
    const input = fs.readFileSync('./day2/input.txt', { encoding: 'utf8' });

    const rounds = input.split(/\r?\n/);
    const results = rounds.map((line) => {
        const score = choiceScore[line[2]] + scoreTable[line[0]][line[2]];
        return score;
    });
    console.log('Part 1 Score: ' + results.reduce((prev, cur) => cur + prev, 0));

    const p2results = rounds.map((line) => {
        const translated = p2translation[line[2]][line[0]];
        const score = choiceScore[translated] + scoreTable[line[0]][translated];
        return score;
    })
    console.log('Part 2 Score: ' + p2results.reduce((prev, cur) => cur + prev, 0));

}

run();