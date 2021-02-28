<template>
  <table class="ui single line table">
    <thead>
      <tr>
        <th width="40">#</th>
        <th>Nama Item</th>
        <th>Deskripsi</th>
        <th width="80">Aksi</th>
      </tr>
    </thead>
    <tbody>
      <template v-if="items.length > 0">
        <tr v-for="(item, index) in items" :key="item.id">
          <td>{{ index + 1 }}</td>
          <td>{{ item.item_name }}</td>
          <td>{{ item.item_desc }}</td>
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
        <th :colspan="4">
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
</template>
