import Vue from 'vue'
import Api from './index'

export default class Item {
  constructor() {
    this.api = new Api('userService')
  }

  async addUser(payload) {
    try {
      const result = await this.api.call('add_user', payload)
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }

  async updateUser(payload) {
    try {
      const result = await this.api.call('update_user', payload)
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }

  async updatePassword(payload) {
    try {
      const result = await this.api.call('update_password', payload)
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }

  async getUsers(limit = 10, offset = 0) {
    try {
      const result = await this.api.call('get_users', { limit, offset })
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }

  async deleteUser(id) {
    try {
      const result = await this.api.call('delete_user', { id })
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }

  async getUser(id) {
    try {
      const result = await this.api.call('get_user', { id })
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }
}
