'use strict'
const { fixedFunc } = require('./index.node');

const delay = () => new Promise(resolve => setTimeout(resolve, 100));

(async () => {
    for (let i=0; i<1000000; i++) {
        fixedFunc(() => {})
        if (i % 10000 === 0) {
            const memUsage = process.memoryUsage().heapUsed / 1024 / 1024;
            console.log(`Mem usage: ${memUsage.toFixed(2)} MB`)
            await delay()
        }
    }
})()