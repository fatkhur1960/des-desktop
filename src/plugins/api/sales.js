import Vue from 'vue'
import Api from './index'

export default class Item {
  constructor() {
    this.api = new Api('saleService')
  }

  async addSale(payload) {
    try {
      const result = await this.api.call('add_sale', payload)
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }

  async updateSale(payload) {
    try {
      const result = await this.api.call('update_sale', payload)
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }

  async getSales(limit = 10, offset = 0) {
    try {
      const result = await this.api.call('get_sales', { limit, offset })
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }

  async getSaleHistories(limit = 10, offset = 0, { id }) {
    try {
      const result = await this.api.call('get_sale_histories', {
        limit,
        offset,
        id,
      })
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }

  async deleteSale(id) {
    try {
      const result = await this.api.call('delete_sale', { id })
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }

  async getSale(id) {
    try {
      const result = await this.api.call('get_sale', { id })
      return result
    } catch (e) {
      Vue.error(e)
      throw e
    }
  }
}
