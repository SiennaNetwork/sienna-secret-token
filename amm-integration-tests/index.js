const FactoryContract = require('./contract').FactoryContract
const ContractInstantiationInfo = require('./contract').ContractInstantiationInfo

const SecretNetworkAgent = require('../integra/lib/agent')
const say = require('../integra/lib/say').tag(`${new Date().toISOString()}`)

async function run_tests() {
    const { client, factory } = await setup()

    console.log(await factory.get_exchange_pair("invalid address"))
}

async function setup() {
  const snip20_wasm = 'snip20-reference-impl.wasm'
  const exchange_wasm = 'dex.wasm'
  const lp_token_wasm = 'amm_lp_token.wasm'

  const client = await SecretNetworkAgent.fromKeyPair({say, name: "test-client"})

  const exchange_upload = await client.upload({binary: exchange_wasm})
  const lp_token_upload = await client.upload({binary: lp_token_wasm})

  const exchange_contract_info = new ContractInstantiationInfo(exchange_upload.transactionHash, exchange_upload.codeId)
  const lp_token_contract_info = new ContractInstantiationInfo(lp_token_upload.transactionHash, lp_token_upload.codeId)

  const factory = await FactoryContract.instantiate(say, lp_token_contract_info, exchange_contract_info)

  return { client, factory }
}

await run_tests()
