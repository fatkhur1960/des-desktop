<template>
  <div class="ui stackable grid">
    <div class="column">
      <div class="ui grid vertical basic clearing page-title">
        <div class="seven wide column">
          <h1 class="ui left floated header">Prediksi</h1>
        </div>
        <!-- <sui-button primary @click.native="toggle">Tambahkan Admin</sui-button> -->
        <div class="nine wide column">
          <sui-form spellcheck="false">
            <sui-form-fields fields="three">
              <sui-form-field>
                <Autocomplete ref="input" @itemSelected="onItemSelected" />
              </sui-form-field>
              <sui-form-field>
                <input
                  type="number"
                  v-model="next"
                  min="1"
                  max="24"
                  required
                  placeholder="(N) bulan ke depan"
                />
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
      <div v-if="alpha" style="text-align: center; margin-top: 3em">
        <h2 style="margin-bottom: .5em">Prediksi {{ targetItem.item_name }}</h2>
        <p>Best Alpha: {{ alpha }}</p>
      </div>
    </div>
  </div>
</template>
<script>
import LineChart from '../components/lineChart'
import Autocomplete from '../components/Autocomplete'

export default {
  components: { LineChart, Autocomplete },
  data: () => ({
    chartData: {},
    itemLoading: false,
    targetItem: null,
    next: 1,
    alpha: null,
  }),
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
      return months[index]
    },
    getRandomColor() {
      return Math.floor(Math.random() * 16777215).toString(16)
    },
    onItemSelected(item) {
      this.targetItem = item ?? null
    },
    submit() {
      this.getItems(this.targetItem.id, parseInt(this.next))
      return
    },
    reset() {
      this.targetItem = null
      this.$refs['input'].search = ''
      this.getItems()
      return
    },
    getItems(id = null, next = null) {
      this.itemLoading = true
      this.$api.sale
        .predict({ id, next })
        .then((res) => {
          let forecast = res.data.forecast
          let real = res.data.real
          this.alpha = res.data.alpha

          let labels = forecast.map((s) => {
            let dt = new Date(s.date)
            return `${this.getMonth(dt.getMonth())} ${dt.getFullYear()}`
          })

          console.log(labels)
          console.log(forecast)
          console.log(real)

          let datasets = [
            {
              label: 'Real',
              backgroundColor: `#${this.getRandomColor()}`,
              data: real.map((s) => {
                return s.sale_value
              }),
            },
            {
              label: 'Forecast',
              backgroundColor: `#${this.getRandomColor()}`,
              data: forecast.map((s) => {
                return s.sale_value
              }),
            },
          ]

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
