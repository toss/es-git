const path = require('path');
const { getSha } = require('../index');

describe('esgit - getSha', () => {
  it('sha를 가져올 수 있다.', () => {
    const sha = getSha('main', { dir: path.resolve(__dirname, '..') });

    expect(typeof sha).toBe('string');
  })
})
