<template>
  <div class="ui stackable grid">
    <div class="column">
      <div class="ui grid vertical basic clearing page-title">
        <div class="ten wide column">
          <h1 class="ui left floated header">Dashboard</h1>
        </div>
        <!-- <sui-button primary @click.native="toggle">Tambahkan Admin</sui-button> -->
        <div class="six wide column">
          <sui-form spellcheck="false">
            <sui-form-fields fields="two">
              <sui-form-field>
                <Autocomplete ref="input" @itemSelected="onItemSelected" />
              </sui-form-field>
              <sui-form-field>
                <sui-button
                  @click.prevent="submit"
                  primary
                  content="Cari"
                  icon="find"
                />
                <sui-button
                  color="orange"
                  content="Reset"
                  @click.prevent="reset"
                  icon="close"
                />
              </sui-form-field>
            </sui-form-fields>
          </sui-form>
        </div>
      </div>
      <div class="dimmable" style="margin: 1em 0;">
        <div v-if="itemLoading" class="ui active inverted dimmer">
          <div class="ui loader"></div>
        </div>
        <line-chart v-if="!itemLoading" :height="180" :chart-data="chartData" />
      </div>
    </div>
  </div>
</template>
<script>
import LineChart from '../components/chart'
import Autocomplete from '../components/Autocomplete'

export default {
  components: { LineChart, Autocomplete },
  data: () => ({ chartData: {}, itemLoading: true, targetItem: null }),
  mounted() {
    this.getItems()
  },
  computed: {
    myStyles() {
      return {
        height: '350px',
        position: 'relative',
      }
    },
  },
  methods: {
    fillData({ labels, datasets }) {
      this.chartData = {
        labels,
        datasets,
      }
    },
    getMonth(index) {
      const months = [
        '',
        'Jan',
        'Feb',
        'Mar',
        'Apr',
        'Mei',
        'Jun',
        'Jul',
        'Agu',
        'Sep',
        'Okt',
        'Nov',
        'Des',
      ]
      return months[parseInt(index)]
    },
    getRandomColor() {
      return Math.floor(Math.random() * 16777215).toString(16)
    },
    onItemSelected(item) {
      this.targetItem = item ?? null
    },
    submit() {
      this.getItems(this.targetItem.id)
      return
    },
    reset() {
      this.targetItem = null
      this.$refs['input'].search = ''
      this.getItems()
      return
    },
    getItems(id = null) {
      this.$api.sale
        .getSaleHistories(10, 0, { id })
        .then((res) => {
          let entries = res.data.entries
          let labels = entries
            .map((item) => {
              return item.sales.map(
                (s) => `${this.getMonth(s.month)} ${s.year}`,
              )
            })
            .sort((a, b) => a.length < b.length)[0]
          let datasets = entries.map((item) => {
            return {
              label: item.item_name,
              backgroundColor: `#${this.getRandomColor()}`,
              data: item.sales.map((sale) => {
                return sale.sale_value
              }),
            }
          })
          this.fillData({ labels, datasets })
          this.itemLoading = false
        })
        .catch((_) => {
          this.itemLoading = false
        })
    },
  },
}
</script>
