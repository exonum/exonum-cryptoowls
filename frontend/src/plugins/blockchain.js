import * as Exonum from 'exonum-client'
import * as proto from '../../proto/proto.js'
import bigInt from 'big-integer'
import axios from 'axios'

const SERVICE_ID = 521
const CREATE_USER_TX_ID = 0
const MAKE_OWL_TX_ID = 2
const ISSUE_TX_ID = 4
const CREATE_AUCTION_TX_ID = 1
const MAKE_BID_TX_ID = 3

const ATTEMPTS = 10
const ATTEMPT_TIMEOUT = 500

const DNA = Exonum.newType(proto.exonum.DNA)
const Owl = Exonum.newType(proto.exonum.examples.cryptoowls.CryptoOwl)
const Auction = Exonum.newType(proto.exonum.examples.cryptoowls.Auction)

function getSystemTime() {
  const now = Date.now()
  const secs = bigInt(now).over(1000)
  const nanos = bigInt(now).minus(secs.multiply(1000)).multiply(1000000)

  return {
    seconds: secs.valueOf(),
    nanos: nanos.valueOf()
  }
}

function waitForAcceptance(response) {
  let attempt = ATTEMPTS

  if (response.data.debug) {
    throw new Error(response.data.description)
  }

  return (function makeAttempt() {
    return axios.get(`/api/explorer/v1/transactions?hash=${response.data.tx_hash}`).then(response => {
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
        const TxCreateWallet = Exonum.newTransaction({
          author: keyPair.publicKey,
          service_id: SERVICE_ID,
          message_id: CREATE_USER_TX_ID,
          schema: proto.exonum.examples.cryptoowls.CreateUser
        })

        // Transaction data
        const data = {
          name: name
        }

        // Send transaction into blockchain
        return TxCreateWallet.send('/api/explorer/v1/transactions', data, keyPair.secretKey)
        .then(() => keyPair)

      },

      makeOwl: (keyPair, name, father, mother) => {
        // Describe transaction to make new owl
        const TxMakeOwl = Exonum.newTransaction({
          author: keyPair.publicKey,
          service_id: SERVICE_ID,
          message_id: MAKE_OWL_TX_ID,
          schema: proto.exonum.examples.cryptoowls.MakeOwl
        })


        // Transaction data
        const data = {
          name: name,
          father_id: {data: Exonum.hexadecimalToUint8Array(father)},
          mother_id: {data: Exonum.hexadecimalToUint8Array(mother)},
          seed: getSystemTime()
        }

        // Send transaction into blockchain
        return TxMakeOwl.send('/api/explorer/v1/transactions', data, keyPair.secretKey)
      },

      issue: (keyPair) => {
        // Describe transaction to issue funds
        const TxIssue = Exonum.newTransaction({
          service_id: SERVICE_ID,
          author: keyPair.publicKey,
          message_id: ISSUE_TX_ID,
          schema: proto.exonum.examples.cryptoowls.Issue
        })

        // Transaction data
        const data = {
          seed: getSystemTime()
        }

        // Send transaction into blockchain
        return TxIssue.send('/api/explorer/v1/transactions', data, keyPair.secretKey)
      },

      createAuction: (keyPair, owl, price, duration) => {
        // Describe transaction to create auction
        const TxCreateAuction = Exonum.newTransaction({
          author: keyPair.publicKey,
          service_id: SERVICE_ID,
          message_id: CREATE_AUCTION_TX_ID,
          schema: proto.exonum.examples.cryptoowls.CreateAuction
        })

        // Transaction data
        const data = {
          owl_id: { data: Exonum.hexadecimalToUint8Array(owl) },
          start_price: price,
          duration: duration
        }

        // Send transaction into blockchain
        return TxCreateAuction.send('/api/explorer/v1/transactions', data, keyPair.secretKey)
      },

      makeBid: (keyPair, auction, price) => {
        // Describe transaction to make bid
        const TxMakeBid = Exonum.newTransaction({
          service_id: SERVICE_ID,
          message_id: MAKE_BID_TX_ID,
          author: keyPair.publicKey,
          schema: proto.exonum.examples.cryptoowls.MakeBid
        })

        // Transaction data
        const data = {
          auction_id: auction,
          value: price
        }

        // Send transaction into blockchain
        return TxMakeBid.send('/api/explorer/v1/transactions', data, keyPair.secretKey)
      },

      getUsers: () => {
        return axios.get('/api/services/cryptoowls/v1/users').then(response => response.data)
      },

      getUser: publicKey => {
        return axios.get(`/api/services/cryptoowls/v1/user?pub_key=${publicKey}`).then(response => {
          if (response.data === 'User not found') {
            throw new Error(response.data)
          }
          return response.data
        })
      },

      getAuctions:() => {
        return axios.get('/api/services/cryptoowls/v1/auctions').then(response => response.data)
      },

      getUserAuctions: publicKey => {
        return axios.get(`/api/services/cryptoowls/v1/user/auctions?pub_key=${publicKey}`).then(response => response.data)
      },

      getAuction: id => {
        return axios.get(`/api/services/cryptoowls/v1/auction?id=${id}`).then(response => response.data)
      },

      getBids: id => {
        return axios.get(`/api/services/cryptoowls/v1/auction/bids?id=${id}`).then(response => response.data)
      },

      getOwls: () => {
        return axios.get('/api/services/cryptoowls/v1/owls').then(response => response.data)
      },

      getUserOwls: publicKey => {
        return axios.get(`/api/services/cryptoowls/v1/user/owls?pub_key=${publicKey}`).then(response => response.data)
      },

      getOwl: hash => {
        return axios.get(`/api/services/cryptoowls/v1/owl?id=${hash}`).then(response => response.data)
      },

      getBlocks: latest => {
        let suffix = !isNaN(latest) ? '&latest=' + latest : ''

        return axios.get(`/api/explorer/v1/blocks?count=10${suffix}`).then(response => response.data.blocks)
      },

      getBlock: height => {
        return axios.get(`/api/explorer/v1/block?height=${height}`).then(response => response.data)
      },

      getTransaction: hash => {
        return axios.get(`/api/explorer/v1/transactions?hash=${hash}`).then(response => response.data)
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

      getOwlHash: (owl) => Owl.hash(owl),
      getAuctionHash: auction => Auction.hash(auction)
    }
  }
}
