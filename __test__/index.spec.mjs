import test from 'ava'

import { encodeAccessKey, decodeAccessKey, generateId } from '../index.js'

test('round trip access key encode/decode', (t) => {
  const accessKeyData = {
    id: "foo",
    namespace: "bar",
    eventType: "baz",
    group: "qux",
    eventPath: "quux",
    eventObjectIdPath: "corge",
    timestampPath: "grault",
    parentAccessKey: "garply",
  }
  const accessKey = {
    environment: "test",
    eventType: "id",
    version: 1,
    data: accessKeyData,
  }

  const password = "32KFFT_i4UpkJmyPwY2TGzgHpxfXs7zS"
  const encoded = encodeAccessKey(accessKey, password)

  const decoded = decodeAccessKey(encoded, password)
  t.deepEqual(decoded, accessKey)
})

const checkPasswordLengthError = (t, accessKey, password) => {
  const error = new Error(`Password must be 32 characters in length. Given password is ${password.length} characters long`)
  error.code = 'GenericFailure'

  let err = t.throws(() => {
    encodeAccessKey(accessKey, password)
  }, { instanceOf: Error });
  t.deepEqual(err, error)

  err = t.throws(() => {
    decodeAccessKey('foo', password)
  }, { instanceOf: Error });
  t.deepEqual(err, error)
}

test('incorrect length password throws error', t => {
  const accessKeyData = {
    id: "foo",
    namespace: "bar",
    eventType: "baz",
    group: "qux",
    eventPath: "quux",
    eventObjectIdPath: "corge",
    timestampPath: "grault",
    parentAccessKey: "garply",
  }
  const accessKey = {
    environment: "test",
    eventType: "id",
    version: 1,
    data: accessKeyData,
  }


  let password = "32KFFT_i4UpkJmyPwY2TGzgHpxfXs7zSS"
  checkPasswordLengthError(t, accessKey, password)

  password = "32KFFT_i4UpkJmyPwY2TGzgHpxfXs7z"
  checkPasswordLengthError(t, accessKey, password)
})

test('can generate an id with proper prefix', t => {
  let prefix = 'evt';

  let id = generateId(prefix)
  let splitId = id.split('::')
  t.deepEqual(splitId.length, 3)
  t.deepEqual(splitId[0], prefix)
})

test('invalid prefix for generate id throws error', t => {
  let invalidPrefix = 'foo';

  const error = new Error(`Invalid id prefix: ${invalidPrefix}`)
  error.code = 'GenericFailure'

  let err = t.throws(() => {
    generateId(invalidPrefix)
  }, { instanceOf: Error });
  t.deepEqual(err, error)
})