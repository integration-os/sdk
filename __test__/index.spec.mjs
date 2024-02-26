import test from 'ava'

import { IntegrationOS } from '../index.js'


test('fetch customer', async (t) => {

  const integrate = new IntegrationOS("sk_test_1_5wOEHNTqN8h6R7ErKHg_p9OPK1JE9AHPO3LrSUcmmIC-uZnKv1Z7ijPE2YrsURTS-830GyV_vQQhZeHG_Gc6Vd9A5SsJGg4Es5RqjjUoMQsDw8uWobpbCtBoAVsic08BFG6kf557lrytTGIo7563ikQfUqUfE2aO_ILg5tbYyZYZBMHd46GRTRwfa0_PhjhCFCpXnPE", {
    serverUrl: "https://development-api.integrationos.com/v1/unified"
  });

  const response = await integrate.customers("test::shopify::andrew-test-0d1866cc98").list();

  console.log(response);

  const getResponse = await integrate.customers("test::shopify::andrew-test-0d1866cc98").get("7497524773048");

  console.log(getResponse);

})
