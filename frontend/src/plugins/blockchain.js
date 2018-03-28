import * as Exonum from 'exonum-client'
import bigInt from 'big-integer'
import axios from 'axios'

const ATTEMPTS = 10
const ATTEMPT_TIMEOUT = 500
const NETWORK_ID = 0
const PROTOCOL_VERSION = 0
const SERVICE_ID = 521

const SystemTime = Exonum.newType({
  fields: [
    { name: 'secs', type: Exonum.Uint64 },
    { name: 'nanos', type: Exonum.Uint32 }
  ]
})
const DNA = Exonum.newType({
  fields: [
    { name: 'dna', type: Exonum.Uint32 }
  ]
})
const Owl = Exonum.newType({
  fields: [
    { name: 'name', type: Exonum.String },
    { name: 'dna', type: Exonum.Uint32 }
  ]
})

const CREATE_USER_TX = [
  { name: 'public_key', type: Exonum.PublicKey },
  { name: 'name', type: Exonum.String }
]
const MAKE_OWL_TX = [
  { name: 'public_key', type: Exonum.PublicKey },
  { name: 'name', type: Exonum.String },
  { name: 'father_id', type: Exonum.Hash },
  { name: 'mother_id', type: Exonum.Hash },
  { name: 'seed', type: SystemTime }
]
const ISSUE_TX = [
  { name: 'public_key', type: Exonum.PublicKey },
  { name: 'seed', type: SystemTime }
]
const CREATE_ORDER_TX = [
  { name: 'public_key', type: Exonum.PublicKey },
  { name: 'owl_id', type: Exonum.Hash },
  { name: 'price', type: Exonum.Uint64 },
  { name: 'seed', type: SystemTime }
]
const ACCEPT_ORDER_TX = [
  {name: 'public_key', type: Exonum.PublicKey },
  {name: 'order_id', type: Exonum.Hash }
]

const TRANSACTIONS = [CREATE_USER_TX, MAKE_OWL_TX, ISSUE_TX, CREATE_ORDER_TX, ACCEPT_ORDER_TX]

function getMessageIdByTransaction(transaction) {
  return TRANSACTIONS.findIndex(element => element === transaction)
}

function createTransaction(transaction) {
  return Exonum.newMessage({
    network_id: NETWORK_ID,
    protocol_version: PROTOCOL_VERSION,
    service_id: SERVICE_ID,
    message_id: getMessageIdByTransaction(transaction),
    fields: transaction
  })
}

function postTransaction(transaction, data, signature) {
  return axios.post('/api/services/cryptoowls/v1/transaction', {
    network_id: NETWORK_ID,
    protocol_version: PROTOCOL_VERSION,
    service_id: SERVICE_ID,
    message_id: getMessageIdByTransaction(transaction),
    body: data,
    signature: signature
  }).then(response => waitForAcceptance(response.data.tx_hash))
}

function getSystemTime() {
  const now = Date.now()
  const secs = bigInt(now).over(1000)
  const nanos = bigInt(now).minus(secs.multiply(1000)).multiply(1000000)

  return {
    secs: secs.toString(),
    nanos: nanos.valueOf()
  }
}

function waitForAcceptance(hash) {
  let attempt = ATTEMPTS

  return (function makeAttempt() {
    return axios.get(`/api/explorer/v1/transactions/${hash}`).then(response => {
      if (response.data.type === 'committed') {
        return response.data
      } else {
        if (--attempt > 0) {
          return new Promise((resolve) => {
            setTimeout(resolve, ATTEMPT_TIMEOUT)
          }).then(makeAttempt)
        } else {
          throw new Error('Transaction has not been found')
        }
      }
    })
  })()
}

module.exports = {
  install(Vue) {
    Vue.prototype.$blockchain = {
      createUser: name => {
        const keyPair = Exonum.keyPair()

        const TxCreateWallet = createTransaction(CREATE_USER_TX)

        const data = {
          public_key: keyPair.publicKey,
          name: name
        }

        const signature = TxCreateWallet.sign(keyPair.secretKey, data)

        return postTransaction(CREATE_USER_TX, data, signature).then(() => keyPair)
      },

      makeOwl: (keyPair, name, father, mother) => {
        const TxMakeOwl = createTransaction(MAKE_OWL_TX)

        const data = {
          public_key: keyPair.publicKey,
          name: name,
          father_id: father,
          mother_id: mother,
          seed: getSystemTime()
        }

        const signature = TxMakeOwl.sign(keyPair.secretKey, data)

        return postTransaction(MAKE_OWL_TX, data, signature)
      },

      issue: (keyPair) => {
        const TxIssue = createTransaction(ISSUE_TX)

        const data = {
          public_key: keyPair.publicKey,
          seed: getSystemTime()
        }

        const signature = TxIssue.sign(keyPair.secretKey, data)

        return postTransaction(ISSUE_TX, data, signature)
      },

      createOrder: (keyPair, owl, price) => {
        const TxCreateOrder = createTransaction(CREATE_ORDER_TX)

        const data = {
          public_key: keyPair.publicKey,
          owl_id: owl,
          price: price,
          seed: getSystemTime()
        }

        const signature = TxCreateOrder.sign(keyPair.secretKey, data)

        return postTransaction(CREATE_ORDER_TX, data, signature)
      },

      acceptOrder: (keyPair, order) => {
        const TxAcceptOrder = createTransaction(ACCEPT_ORDER_TX)

        const data = {
          public_key: keyPair.publicKey,
          order_id: order
        }

        const signature = TxAcceptOrder.sign(keyPair.secretKey, data)

        return postTransaction(ACCEPT_ORDER_TX, data, signature)
      },

      getUsers: () => {
        return axios.get('/api/services/cryptoowls/v1/users').then(response => response.data)
      },

      getUser: publicKey => {
        return axios.get(`/api/services/cryptoowls/v1/user/${publicKey}`).then(response => response.data)
      },

      getOwls: () => {
        return axios.get('/api/services/cryptoowls/v1/owls').then(response => response.data)
      },

      getUserOwls: publicKey => {
        return axios.get(`/api/services/cryptoowls/v1/user/${publicKey}/owls`).then(response => response.data)
      },

      getOwl: hash => {
        return axios.get(`/api/services/cryptoowls/v1/owl/${hash}`).then(response => response.data)
      },

      getOrders: hash => {
        return axios.get(`/api/services/cryptoowls/v1/owl/${hash}/orders`).then(response => response.data)
      },

      getBlocks: latest => {
        let suffix = !isNaN(latest) ? '&latest=' + latest : ''

        return axios.get(`/api/explorer/v1/blocks?count=10${suffix}`).then(response => response.data.blocks)
      },

      getBlock: height => {
        return axios.get(`/api/explorer/v1/blocks/${height}`).then(response => response.data)
      },

      getTransaction: hash => {
        return axios.get(`/api/explorer/v1/transactions/${hash}`).then(response => response.data)
      },

      splitDNA: dna => {
        const buffer = DNA.serialize({dna: dna})
        const appearanceHex = Exonum.uint8ArrayToHexadecimal(new Uint8Array(buffer.slice(3)))
        const appearanceBinary = Exonum.hexadecimalToBinaryString(appearanceHex)

        return {
          color: Exonum.uint8ArrayToHexadecimal(new Uint8Array(buffer.slice(0, 3))),
          appearance: {
            eyes: parseInt(appearanceBinary.slice(0, 2), 2),
            wings: parseInt(appearanceBinary.slice(2, 4), 2),
            chest: parseInt(appearanceBinary.slice(4, 6), 2),
            tail: parseInt(appearanceBinary.slice(6, 8), 2)
          }
        }
      },

      getOwlHash: owl => Owl.hash(owl)
    }
  }
}
