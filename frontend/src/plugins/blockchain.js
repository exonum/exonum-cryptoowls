import * as Exonum from 'exonum-client'
import * as bigInt from 'big-integer'
import axios from 'axios'

const NETWORK_ID = 0
const PROTOCOL_VERSION = 0
const SERVICE_ID = 521

const SystemTime = Exonum.newType({
  fields: [
    { name: 'secs', type: Exonum.Uint64 },
    { name: 'nanos', type: Exonum.Uint32 }
  ]
})

const CREATE_USER_TX = [
  {name: 'public_key', type: Exonum.PublicKey},
  {name: 'name', type: Exonum.String}
]
const MAKE_OWL_TX = [
  {name: 'public_key', type: Exonum.PublicKey},
  {name: 'name', type: Exonum.String},
  {name: 'father_id', type: Exonum.Hash},
  {name: 'mother_id', type: Exonum.Hash}
]
const ISSUE_TX = [
  {name: 'public_key', type: Exonum.PublicKey},
  {name: 'current_time', type: SystemTime}
]
const CREATE_ORDER_TX = [
  {name: 'public_key', type: Exonum.PublicKey},
  {name: 'owl_id', type: Exonum.Hash},
  {name: 'price', type: Exonum.Uint64},
  {name: 'current_time', type: SystemTime}
]
const ACCEPT_ORDER_TX = [
  {name: 'public_key', type: Exonum.PublicKey},
  {name: 'order_id', type: Exonum.Hash}
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
  })
}

function getSystemTime() {
  const now = Date.now()
  const secs = bigInt(now).over(1000)
  const nanos = bigInt(now).minus(secs.multiply(1000)).multiply(1000000)

  return {
    secs: secs.valueOf(),
    nanos: nanos.toString()
  }
}

function getData(url) {
  return axios.get(url).then(response => response.data)
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
          mother_id: mother
        }

        const signature = TxMakeOwl.sign(keyPair.secretKey, data)

        return postTransaction(MAKE_OWL_TX, data, signature)
      },

      issue: (keyPair) => {
        const TxIssue = createTransaction(ISSUE_TX)

        const data = {
          public_key: keyPair.publicKey,
          current_time: getSystemTime()
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
          current_time: getSystemTime()
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
      }
    }
  }
}
