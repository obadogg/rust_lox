const fs = require('fs');
const path = require('path');
const { interpret_lox } = require('../rs-package/lox_napi/lox_napi.node');

const code = fs.readFileSync(path.resolve('test.lox'), 'utf-8');

const now = Date.now();
interpret_lox(code);
console.log(`耗时：${(Date.now() - now) / 1000}s`);
