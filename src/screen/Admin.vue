<template>
  <div class="ui stackable grid">
    <div class="column">
      <div class="ui vertical basic clearing page-title">
        <h1 class="ui left floated header">Data Admin</h1>
        <sui-button primary @click.native="toggle">Tambahkan Admin</sui-button>
      </div>
      <AddUserModal :show="open" @close="toggle" :onSaved="getItems" />
      <div class="dimmable" style="margin: 1em 0;">
        <div v-if="itemLoading" class="ui active inverted dimmer">
          <div class="ui loader"></div>
        </div>
        <table class="ui single line table">
          <thead>
            <tr>
              <th width="40">#</th>
              <th>Nama Lengkap</th>
              <th>Username</th>
              <th>Aktif</th>
              <th>Terakhir Login</th>
              <th width="80">Aksi</th>
            </tr>
          </thead>
          <tbody>
            <template v-if="items.length > 0">
              <tr v-for="(item, index) in items" :key="item.id">
                <td>{{ getOverallIndex(index) }}</td>
                <td>{{ item.full_name }}</td>
                <td>{{ item.username }}</td>
                <td v-if="item.active" class="positive">
                  <i class="icon checkmark"></i>
                  Aktif
                </td>
                <td v-else class="error">
                  <i class="icon ban"></i>
                  Tidak Aktif
                </td>
                <td>
                  {{
                    $moment($moment.utc(item.last_login).toDate())
                      .local()
                      .format('ddd, DD MMM YYYY - HH:mm')
                  }}
                </td>
                <td>
                  <sui-button
                    compact
                    color="yellow"
                    icon="edit"
                    @click.prevent="editItem(item)"
                  />
                  <sui-button
                    v-if="item.active"
                    compact
                    color="orange"
                    icon="ban"
                    @click.prevent="confirmBlock(item)"
                  />
                  <sui-button
                    v-else
                    compact
                    color="green"
                    icon="checkmark"
                    @click.prevent="confirmBlock(item)"
                  />
                  <sui-button
                    compact
                    negative
                    icon="trash"
                    @click="confirmDelete(item.id, item.full_name)"
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
              <th :colspan="6">
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
        <EditUser
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
import AddUserModal from '@/components/modals/AddUser'
import EditUser from '@/components/modals/EditUser'

export default {
  components: { AddUserModal, EditUser },
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
        title: 'Hapus Admin',
        text: `Anda yakin ingin menghapus admin '${item}'?`,
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
    confirmBlock(item) {
      const text = item.active ? 'menonaktifkan' : 'mengaktifkan'
      const title = !item.active ? 'Aktifkan' : 'Nonaktifkan'
      this.$modal.show('dialog', {
        title: `${title} Admin`,
        text: `Anda yakin ingin ${text} '${item.full_name}'?`,
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
              this.updateItem(item)
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
    updateItem(item) {
      const val = !item.active
      const text = val ? 'diaktifkan' : 'dinonaktifkan'
      this.$api.user
        .updateUser({ id: item.id, active: val })
        .then((_) => {
          this.$success(`${item.full_name} ${text}`)
          this.getItems()
        })
    },
    editItem(item) {
      this.toggleModal()
      this.$api.user
        .getUser(item.id)
        .then((res) => {
          this.targetItem = res.data
        })
        .catch((_) => {})
    },
    deleteItem(id) {
      this.$api.user
        .deleteUser(id)
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
      this.$api.user
        .getUsers(this.perPage, offset)
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
