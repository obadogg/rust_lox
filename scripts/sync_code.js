const fs = require('fs');
const code = `export default \`${fs.readFileSync('test.lox', 'utf-8')}\``;
fs.writeFileSync('web/src/page/Playground/defaultCode.ts',code,'utf-8');
