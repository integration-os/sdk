import('dotenv/config');

import test from 'ava'

import { IntegrationOS } from '../index.js'

test.before(t => {
  const requiredEnvVars = [
    'INTEGRATIONOS_SECRET_KEY',
    'INTEGRATIONOS_CONNECTION_KEY',
    'INTEGRATIONOS_TESTING_MODEL'
  ];

  requiredEnvVars.forEach(envVar => {
    if (!process.env[envVar]) {
      t.fail(`${envVar} is not set`);
    }
  });
});

test('List model entities', async (t) => {
  const integrate = new IntegrationOS(process.env.INTEGRATIONOS_SECRET_KEY);

  let response = await integrate[process.env.INTEGRATIONOS_TESTING_MODEL](process.env.INTEGRATIONOS_CONNECTION_KEY).list();

  t.truthy(response.unified)
  t.truthy(response.meta)
  t.truthy(response.headers)
})
