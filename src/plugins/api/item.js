import Vue from 'vue'
import Api from './index'

export default class Item {
  constructor() {
    this.api = new Api('itemService')
  }

  async addItem(payload) {
    try {
      const result = await this.api.call('add_item', payload)
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }

  async updateItem(payload) {
    try {
      const result = await this.api.call('update_item', payload)
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }

  async getItems(limit = 10, offset = 0, query = null) {
    try {
      const result = await this.api.call('get_items', { limit, offset, query })
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }

  async deleteItem(id) {
    try {
      const result = await this.api.call('delete_item', { id })
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }

  async getItem(id) {
    try {
      const result = await this.api.call('get_item', { id })
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }
}
