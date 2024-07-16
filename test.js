const crypto = require('crypto');

function stringToMd5(input) {
  const md5Hash = crypto.createHash('md5');
  md5Hash.update(input+'yyds');
  return md5Hash.digest('hex');
}

const input = 'Hello, World!';
const md5Value = stringToMd5(input);
console.log('MD5 value:', md5Value);
