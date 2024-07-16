const { string_to_md5 } = require('./pkg/test_rust.js');
console.log('from rust md5:', string_to_md5('Hello, World!'))