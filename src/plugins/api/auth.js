import Vue from 'vue'
import Api from './index'

export default class Auth {
  constructor() {
    this.api = new Api('authService')
  }

  async authorize(payload) {
    try {
      const result = await this.api.call('authorize', payload)
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }
}
