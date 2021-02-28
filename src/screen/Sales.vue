<template>
  <div class="ui stackable grid">
    <div class="column">
      <div class="ui vertical basic clearing page-title">
        <h1 class="ui left floated header">Data Penjualan</h1>
        <sui-button primary @click.native="toggle">
          Tambahkan Penjualan
        </sui-button>
      </div>
      <AddSaleModal :show="open" @close="toggle" :onSaved="getItems" />
      <div class="dimmable" style="margin: 1em 0;">
        <div v-if="itemLoading" class="ui active inverted dimmer">
          <div class="ui loader"></div>
        </div>
        <table class="ui single line table">
          <thead>
            <tr>
              <th width="40">#</th>
              <th>Nama Item</th>
              <th>Terjual</th>
              <th>Periode</th>
              <th width="80">Aksi</th>
            </tr>
          </thead>
          <tbody>
            <template v-if="items.length > 0">
              <tr v-for="(item, index) in items" :key="item.id">
                <td>{{ getOverallIndex(index) }}</td>
                <td>{{ item.item_name }}</td>
                <td>{{ item.sale_value }}</td>
                <td>{{ $moment(item.ts).format('MMMM YYYY') }}</td>
                <td>
                  <sui-button
                    compact
                    color="yellow"
                    icon="edit"
                    @click.prevent="editItem(item)"
                  />
                  <sui-button
                    compact
                    negative
                    icon="trash"
                    @click="confirmDelete(item.id, item.item_name)"
                  />
                </td>
              </tr>
            </template>
            <template v-else>
              <tr><td colspan="4">Belum ada data</td></tr>
            </template>
          </tbody>
          <tfoot v-if="totalPage > 1">
            <tr>
              <th :colspan="5">
                <div class="pagination-wrapper">
                  <slot
                    name="pagination"
                    :currentPage="currentPage"
                    :totalPage="totalPage"
                  >
                    <div class="ui pagination small menu">
                      <a
                        v-for="(item, idx) in paginationArray"
                        :class="setPaginationItemClass(item)"
                        :key="idx"
                        class="item"
                        @click="gotoPage(item)"
                      >
                        {{ item }}
                      </a>
                    </div>
                  </slot>
                </div>
              </th>
            </tr>
          </tfoot>
        </table>
        <EditSale
          :show="editModal"
          @close="toggleModal()"
          :onSaved="onUpdated"
          :item="targetItem"
        />
      </div>
    </div>
  </div>
</template>
<script>
import AddSaleModal from '@/components/modals/AddSale'
import EditSale from '@/components/modals/EditSale'

export default {
  components: { AddSaleModal, EditSale },
  data() {
    return {
      open: false,
      items: [],
      itemLoading: false,
      totalPage: 0,
      perPage: 10,
      editModal: false,
      currentPage: 1,
      activeModal: 0,
      targetItem: {},
    }
  },
  computed: {
    paginationArray() {
      return this.$paginationArray(this.currentPage, this.totalPage)
    },
  },
  beforeMount() {
    this.getItems()
  },
  methods: {
    getOverallIndex: function (index) {
      return this.$itemNum(this.currentPage, this.perPage, index)
    },
    toggle() {
      this.open = !this.open
    },
    toggleModal: function () {
      this.editModal = !this.editModal
    },
    setPaginationItemClass(item) {
      return {
        disabled: !this.isClickableItemPagination(item),
        active: item == this.currentPage,
      }
    },
    isClickableItemPagination(item) {
      return item != this.currentPage && item != '...'
    },
    gotoPage(item) {
      if (!this.isClickableItemPagination(item)) return
      this.currentPage = item
      this.getItems((item - 1) * this.perPage)
    },
    confirmDelete(id, item) {
      this.$modal.show('dialog', {
        title: 'Hapus Penjualan',
        text: `Anda yakin ingin menghapus penjualan '${item}'?`,
        buttons: [
          {
            title: 'Batalkan',
            handler: () => {
              this.$modal.hide('dialog')
            },
          },
          {
            title: 'Ya',
            default: true,
            handler: () => {
              this.deleteItem(id)
              this.$modal.hide('dialog')
            },
          },
        ],
      })
    },
    onUpdated() {
      this.getItems()
      this.targetItem = {}
    },
    editItem(item) {
      this.toggleModal()
      this.$api.sale
        .getSale(item.id)
        .then((res) => {
          this.targetItem = res.data
        })
        .catch((_) => {})
    },
    deleteItem(id) {
      this.$api.sale
        .deleteSale(id)
        .then((res) => {
          this.getItems()
          this.$success(res.data)
          this.itemLoading = false
        })
        .catch((_) => {
          this.itemLoading = false
        })
    },
    getItems(offset = 0) {
      this.itemLoading = true
      this.$api.sale
        .getSales(this.perPage, offset)
        .then((res) => {
          let entries = res.data.entries
          this.items = entries
          this.totalPage = Math.ceil(res.data.count / this.perPage)
          this.itemLoading = false
        })
        .catch((_) => {
          this.itemLoading = false
        })
    },
  },
}
</script>
