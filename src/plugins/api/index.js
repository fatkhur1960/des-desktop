import { promisified } from 'tauri/api/tauri'
import Item from './item'
import Auth from './auth'
import User from './user'
import Sale from './sales'

export default class Api {
  constructor(serviceName) {
    this.serviceName = serviceName
  }

  async call(route, payload) {
    try {
      const res = await promisified({
        cmd: this.serviceName,
        route: route,
        payload: payload,
      })
      return res
    } catch (e) {
      throw e
    }
  }
}

export { Item, Auth, User, Sale }
