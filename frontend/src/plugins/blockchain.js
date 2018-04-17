import * as Exonum from 'exonum-client'
import bigInt from 'big-integer'
import axios from 'axios'

const NETWORK_ID = 0
const PROTOCOL_VERSION = 0
const SERVICE_ID = 521
const CREATE_USER_TX_ID = 0
const MAKE_OWL_TX_ID = 1
const ISSUE_TX_ID = 2
const CREATE_ORDER_TX_ID = 3
const ACCEPT_ORDER_TX_ID = 4

const ATTEMPTS = 10
const ATTEMPT_TIMEOUT = 500

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
const Order = Exonum.newType({
  fields: [
    { name: 'public_key', type: Exonum.PublicKey },
    { name: 'owl_id', type: Exonum.Hash },
    { name: 'status', type: Exonum.String },
    { name: 'price', type: Exonum.Uint64 }
  ]
})

function getSystemTime() {
  const now = Date.now()
  const secs = bigInt(now).over(1000)
  const nanos = bigInt(now).minus(secs.multiply(1000)).multiply(1000000)

  return {
    secs: secs.toString(),
    nanos: nanos.valueOf()
  }
}

function waitForAcceptance(response) {
  let attempt = ATTEMPTS

  if (response.data.debug) {
    throw new Error(response.data.description)
  }

  return (function makeAttempt() {
    return axios.get(`/api/explorer/v1/transactions/${response.data.tx_hash}`).then(response => {
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
        // Generate new signing key pair
        const keyPair = Exonum.keyPair()

        // Describe transaction to create new user
        const TxCreateWallet = Exonum.newMessage({
          network_id: NETWORK_ID,
          protocol_version: PROTOCOL_VERSION,
          service_id: SERVICE_ID,
          message_id: CREATE_USER_TX_ID,
          fields: [
            { name: 'public_key', type: Exonum.PublicKey },
            { name: 'name', type: Exonum.String }
          ]
        })

        // Transaction data
        const data = {
          public_key: keyPair.publicKey,
          name: name
        }

        // Sign transaction with user's secret key
        const signature = TxCreateWallet.sign(keyPair.secretKey, data)

        // Send transaction into blockchain
        return axios.post('/api/services/cryptoowls/v1/transaction', {
          network_id: NETWORK_ID,
          protocol_version: PROTOCOL_VERSION,
          service_id: SERVICE_ID,
          message_id: CREATE_USER_TX_ID,
          body: data,
          signature: signature
        }).then(waitForAcceptance).then(() => keyPair)
      },

      makeOwl: (keyPair, name, father, mother) => {
        // Describe transaction to make new owl
        const TxMakeOwl = Exonum.newMessage({
          network_id: NETWORK_ID,
          protocol_version: PROTOCOL_VERSION,
          service_id: SERVICE_ID,
          message_id: MAKE_OWL_TX_ID,
          fields: [
            { name: 'public_key', type: Exonum.PublicKey },
            { name: 'name', type: Exonum.String },
            { name: 'father_id', type: Exonum.Hash },
            { name: 'mother_id', type: Exonum.Hash },
            { name: 'seed', type: SystemTime }
          ]
        })

        // Transaction data
        const data = {
          public_key: keyPair.publicKey,
          name: name,
          father_id: father,
          mother_id: mother,
          seed: getSystemTime()
        }

        // Sign transaction with user's secret key
        const signature = TxMakeOwl.sign(keyPair.secretKey, data)

        // Send transaction into blockchain
        return axios.post('/api/services/cryptoowls/v1/transaction', {
          network_id: NETWORK_ID,
          protocol_version: PROTOCOL_VERSION,
          service_id: SERVICE_ID,
          message_id: MAKE_OWL_TX_ID,
          body: data,
          signature: signature
        }).then(waitForAcceptance)
      },

      issue: (keyPair) => {
        // Describe transaction to issue funds
        const TxIssue = Exonum.newMessage({
          network_id: NETWORK_ID,
          protocol_version: PROTOCOL_VERSION,
          service_id: SERVICE_ID,
          message_id: ISSUE_TX_ID,
          fields: [
            { name: 'public_key', type: Exonum.PublicKey },
            { name: 'seed', type: SystemTime }
          ]
        })

        // Transaction data
        const data = {
          public_key: keyPair.publicKey,
          seed: getSystemTime()
        }

        // Sign transaction with user's secret key
        const signature = TxIssue.sign(keyPair.secretKey, data)

        // Send transaction into blockchain
        return axios.post('/api/services/cryptoowls/v1/transaction', {
          network_id: NETWORK_ID,
          protocol_version: PROTOCOL_VERSION,
          service_id: SERVICE_ID,
          message_id: ISSUE_TX_ID,
          body: data,
          signature: signature
        }).then(waitForAcceptance)
      },

      createOrder: (keyPair, owl, price) => {
        // Describe transaction to place new order
        const TxCreateOrder = Exonum.newMessage({
          network_id: NETWORK_ID,
          protocol_version: PROTOCOL_VERSION,
          service_id: SERVICE_ID,
          message_id: CREATE_ORDER_TX_ID,
          fields: [
            { name: 'public_key', type: Exonum.PublicKey },
            { name: 'owl_id', type: Exonum.Hash },
            { name: 'price', type: Exonum.Uint64 },
            { name: 'seed', type: SystemTime }
          ]
        })

        // Transaction data
        const data = {
          public_key: keyPair.publicKey,
          owl_id: owl,
          price: price,
          seed: getSystemTime()
        }

        // Sign transaction with user's secret key
        const signature = TxCreateOrder.sign(keyPair.secretKey, data)

        // Send transaction into blockchain
        return axios.post('/api/services/cryptoowls/v1/transaction', {
          network_id: NETWORK_ID,
          protocol_version: PROTOCOL_VERSION,
          service_id: SERVICE_ID,
          message_id: CREATE_ORDER_TX_ID,
          body: data,
          signature: signature
        }).then(waitForAcceptance)
      },

      acceptOrder: (keyPair, order) => {
        // Describe transaction to accept order
        const TxAcceptOrder = Exonum.newMessage({
          network_id: NETWORK_ID,
          protocol_version: PROTOCOL_VERSION,
          service_id: SERVICE_ID,
          message_id: ACCEPT_ORDER_TX_ID,
          fields: [
            {name: 'public_key', type: Exonum.PublicKey },
            {name: 'order_id', type: Exonum.Hash }
          ]
        })

        // Transaction data
        const data = {
          public_key: keyPair.publicKey,
          order_id: order
        }

        // Sign transaction with user's secret key
        const signature = TxAcceptOrder.sign(keyPair.secretKey, data)

        // Send transaction into blockchain
        return axios.post('/api/services/cryptoowls/v1/transaction', {
          network_id: NETWORK_ID,
          protocol_version: PROTOCOL_VERSION,
          service_id: SERVICE_ID,
          message_id: ACCEPT_ORDER_TX_ID,
          body: data,
          signature: signature
        }).then(waitForAcceptance)
      },

      getUsers: () => {
        return axios.get('/api/services/cryptoowls/v1/users').then(response => response.data)
      },

      getUser: publicKey => {
        return axios.get(`/api/services/cryptoowls/v1/user/${publicKey}`).then(response => {
          if (response.data === 'User not found') {
            throw new Error(response.data)
          }
          return response.data
        })
      },

      getUserOrders: publicKey => {
        return axios.get(`/api/services/cryptoowls/v1/user/${publicKey}/orders`).then(response => response.data)
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

      /**
       * Convert DNA, represented as Uint32 number, into byte array of 4 element
       * Each element is a Uint8 number
       * First three elements represents owl's color in RGB
       * Fourth element is converted into binary representation, divided into 4 equal parts
       * Each part represents one of owl's body parts: eyes, wings, chest, tails
       * Each body part can be in one of 4 possible variants
       * @param dna
       * @returns {{color: *, appearance: {eyes: number, wings: number, chest: number, tail: number}}}
       */
      splitDNA: dna => {
        // Convert DNA into byte array
        const buffer = DNA.serialize({dna: dna})

        // First three elements is an owl color in RGB
        const color = Exonum.uint8ArrayToHexadecimal(new Uint8Array(buffer.slice(0, 3)))

        // Convert fourth element into binary string
        const appearance = Exonum.uint8ArrayToBinaryString(new Uint8Array(buffer.slice(3)))

        return {
          color: color,
          appearance: {
            eyes: parseInt(appearance.slice(0, 2), 2),
            wings: parseInt(appearance.slice(2, 4), 2),
            chest: parseInt(appearance.slice(4, 6), 2),
            tail: parseInt(appearance.slice(6, 8), 2)
          }
        }
      },

      getOwlHash: owl => Owl.hash(owl),

      getOrderHash: order => Order.hash(order)
    }
  }
}
