import test from 'ava'

import { IntegrationOS } from '../index.js'

test('Fetch a Customer', async (t) => {
  const integrate = new IntegrationOS("sk_test_1", {
    serverUrl: "http://localhost:1080"
  });

  let response = await integrate.customers("test::quickbooks::acme").list();

  t.truthy(response.unified.length)
})
