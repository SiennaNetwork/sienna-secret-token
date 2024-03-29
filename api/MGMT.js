import { SecretNetwork, loadSchemas } from '@hackbg/fadroma'

export const schema = loadSchemas(import.meta.url, {
  initMsg:     './mgmt/init.json',
  queryMsg:    './mgmt/query.json',
  queryAnswer: './mgmt/response.json',
  handleMsg:   './mgmt/handle.json'
})

export default class MGMT extends SecretNetwork.Contract.withSchema(schema) {

  /** query contract status */
  get status () { return this.q.status() }

  /** query current schedule */
  get schedule () { return this.q.get_schedule() }

  /** take over a SNIP20 token */
  acquire = async snip20 => {
    const tx1 = await snip20.setMinters([this.address])
    const tx2 = await snip20.changeAdmin(this.address)
    return [tx1, tx2]
  }

  /** load a schedule */
  configure = schedule =>
    this.tx.configure({ schedule })

  /** launch the vesting */
  launch = () =>
    this.tx.launch()

  /** claim accumulated portions */
  claim = claimant =>
    this.tx.claim({}, claimant)

  /** see how much is claimable by someone at a certain time */
  progress = (address, time = + new Date()) => {
    time = Math.floor(time / 1000) // convert JS msec to CosmWasm seconds
    //console.debug(`querying progress for ${address} at ${time} seconds`)
    return this.q.progress({ address, time })
  }

  /** add a new account to a pool */
  add = (pool_name, account) =>
    this.tx.add_account({ pool_name, account })

  /** set the admin */
  setOwner = (new_admin) =>
    this.tx.set_owner({new_admin})
}
