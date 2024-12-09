import test from 'ava'

import { Retcher } from '../index.js'
const retcher = new Retcher();

test('retcher works', async (t) => {
  const response = await retcher.fetch('https://jindrich.bar');

  const text = await response.text();
  text.includes('barjin') ? t.pass() : t.fail();
})

test('json method works', async (t) => {
  const response = await retcher.fetch('https://httpbin.org/json');

  const json = await response.json();
  json.slideshow.author === 'Yours Truly' ? t.pass() : t.fail();
})

test('headers work', async (t) => {
  const response = await retcher.fetch(
    'https://httpbin.org/headers', 
    { headers: { 'Retch-Test': 'foo' } }
  );
  const json = await response.json();

  json.headers['Retch-Test'] ? t.pass() : t.fail();
})

test('request body works', async (t) => {
  const response = await retcher.fetch(
    'https://httpbin.org/post', 
    { 
      method: 'POST',
      body: [...new TextEncoder().encode('{"Retch-Test":"foořžš"}')],
      headers: { 'Content-Type': 'application/json' }
    }
  );
  const json = await response.json();

  json.data === '{"Retch-Test":"foořžš"}' ? t.pass() : t.fail();
})
