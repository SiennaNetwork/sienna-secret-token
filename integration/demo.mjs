#!/usr/bin/env node
/* vim: set ts=2 sts=2 sw=2 et cc=100 */
// # SIENNA Vesting Contract Demo
//
// * [x] by using a local testnet container
// * [ ] that allows time to be fast-forwarded using `libfaketime`
// * this script demonstrates:
//   * [x] deploying and configuring the token and vesting contracts
//   * [x] making claims according to the initial schedule
//   * [x] checking unlocked funds without making a claim
//   * [x] splitting the Remaining Pool Tokens between multiple addresses
//   * [ ] reconfiguring the Remaining Pool Token split, preserving the total portion size
//   * [ ] adding new accounts to Advisor/Investor pools

import assert from 'assert'
import { fileURLToPath } from 'url'
import { resolve, dirname } from 'path'

import { say as sayer, loadJSON, SecretNetwork } from '@hackbg/fadroma'
import SNIP20Contract from '@hackbg/snip20'
import MGMTContract from '@hackbg/mgmt'
import RPTContract from '@hackbg/rpt'

const say = sayer.tag(() => new Date().toISOString()) // Timestamped logger

Error.stackTraceLimit = Infinity
prepare().then(deploy).then(test)

async function prepare () {
  // get schedule from file
  const schedule = loadJSON('../settings/schedule.json', import.meta.url) // Initial config

  // get mnemonic of admin wallet from file generated by localnet genesis
  const {mnemonic} = loadJSON(`../build/localnet/keys/ADMIN.json`, import.meta.url)

  // create agent wrapping the admin wallet (used to deploy and control the contracts)
  const ADMIN = await SecretNetwork.Agent.fromMnemonic({ say, name: 'ADMIN', mnemonic })
  const wallets = []
  const recipients = {}

  // create an agent for each recipient address (used to test claims)
  await Promise.all(schedule.pools.map(pool=>Promise.all(pool.accounts.map(async account=>{
    const { name } = account
    const agent = await SecretNetwork.Agent.fromKeyPair({say, name}) // create agent
    const { address } = agent
    account.address = address        // replace placeholder with real address
    wallets.push([address, 1000000]) // balance to cover gas costs
    recipients[name] = {agent}       // store agent
  }))))

  // TODO add agents for testing MGMT.AddAccount and RPT.Reconfigure

  // seed agent wallets so the network recognizes they exist
  await ADMIN.sendMany(wallets, 'create recipient wallets')

  // instantiate fadroma builder to build and deploy the contracts:
  const commit    = 'HEAD' // git ref
  const buildRoot = fileURLToPath(new URL('../build', import.meta.url))
  const outputDir = resolve(buildRoot, 'outputs')
  const builder   = new SecretNetwork.Builder({ say: say.tag('builder'), outputDir, agent: ADMIN })

  // proceed to the next stage with these handles:
  return { ADMIN, recipients, builder, schedule }
}

async function deploy ({ ADMIN, recipients, builder, schedule }) {

  const repo = dirname(dirname(fileURLToPath(import.meta.url)))

  // deploy token
  const tokenBuildConfig = { say, repo, packageName: 'snip20-reference-impl' }
      , tokenInit = { name:      "Sienna"
                    , symbol:    "SIENNA"
                    , decimals:  18
                    , admin:     ADMIN.address
                    , prng_seed: "insecure"
                    , config:    { public_total_supply: true } }
      , TOKEN = await builder.deploy(SNIP20Contract, tokenInit, tokenBuildConfig)

  // deploy mgmt
  const mgmtBuildConfig = { say, repo, packageName: 'sienna-mgmt' }
      , mgmtInit = { token: [TOKEN.address, TOKEN.hash], schedule }
      , MGMT = await builder.deploy(MGMTContract, mgmtInit, mgmtBuildConfig)

  // deploy rpt; TODO instantiate RPT from MGMT
  const rptBuildConfig = { say, repo, packageName: 'sienna-rpt' }
      , rptInit = { token:   [TOKEN.address, TOKEN.hash]
                  , mgmt:    [MGMT.address,  MGMT.hash]
                  , pool:    'MintingPool'
                  , account: 'RPT'
                  , config:  [[ADMIN.address, "2500000000000000000000"]] }
      , RPT = await builder.deploy(RPTContract, rptInit, rptBuildConfig)

  return { ADMIN, recipients, TOKEN, schedule, MGMT, RPT }

}

async function test ({ ADMIN, recipients, TOKEN, schedule, MGMT, RPT }) {

  // create viewing keys
  const vk = await Promise.all(Object.values(recipients).map(async recipient=>
    recipient.vk = await TOKEN.createViewingKey(recipient.agent, "entropy"))
  )

  // mgmt takes over token; TODO auto-acquire on init
  await MGMT.acquire(TOKEN)

  // update schedule to point at RPT contract
  schedule
    .pools.filter(x=>x.name==='MintingPool')[0]
    .accounts.filter(x=>x.name==='RPT')[0]
    .address = RPT.address
  await MGMT.configure(schedule)

  // launch the vesting
  await MGMT.launch()
}