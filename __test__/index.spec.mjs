import test from 'ava'

import { Retcher } from '../index.js'

test('retcher works', async (t) => {
  const retcher = new Retcher({});

  const response = await retcher.fetch('https://jindrich.bar');

  const text = await response.text();
  text.includes('barjin') ? t.pass() : t.fail();
})

test('json method works', async (t) => {
  const retcher = new Retcher({});

  const response = await retcher.fetch('http://httpbin.org/json');

  const json = await response.json();
  json.slideshow.author === 'Yours Truly' ? t.pass() : t.fail();
})
