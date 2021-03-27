<template>
  <sui-modal size="mini" v-model="showLocal">
    <sui-modal-header>Tambahkan Penjualan</sui-modal-header>
    <sui-modal-content>
      <sui-modal-description>
        <sui-form @submit.prevent="saveItem" spellcheck="false">
          <sui-form-field required :class="{ error: $v.form.item_id.$error }">
            <label>Nama Item</label>
            <Autocomplete ref="input" @itemSelected="onItemSelected" />
            <span class="error-desc" v-if="$v.form.item_id.$error">
              Tidak boleh kosong
            </span>
          </sui-form-field>
          <sui-form-field
            required
            :class="{ error: $v.form.sale_value.$error }"
          >
            <label>Jumlah Penjualan</label>
            <input
              type="number"
              placeholder="Jumlah Penjualan"
              v-model="$v.form.sale_value.$model"
            />
            <span class="error-desc" v-if="$v.form.sale_value.$error">
              Tidak boleh kosong
            </span>
          </sui-form-field>
          <sui-form-field required :class="{ error: $v.form.ts.$error }">
            <label>Periode</label>
            <datepicker
              minimum-view="month"
              format="MMMM yyyy"
              placeholder="Periode"
              v-model="$v.form.ts.$model"
              :language="lang"
            />
            <span class="error-desc" v-if="$v.form.ts.$error">
              Tidak boleh kosong
            </span>
          </sui-form-field>
          <div class="right-btn">
            <sui-button negative @click.prevent="closeModal">Batal</sui-button>
            <sui-button
              :loading="isLoading"
              primary
              content="Simpan"
              type="submit"
            />
          </div>
        </sui-form>
      </sui-modal-description>
    </sui-modal-content>
  </sui-modal>
</template>
<script>
import { required } from 'vuelidate/lib/validators'
import Autocomplete from '../Autocomplete'
import Datepicker from 'vuejs-datepicker'
import langID from '@/plugins/id'

export default {
  name: 'AddSale',
  components: { Autocomplete, Datepicker },
  props: {
    show: {
      type: Boolean,
      default: false,
      required: true,
    },
    onSaved: {
      type: Function,
      required: true,
    },
  },
  data: () => ({
    isLoading: false,
    lang: langID,
    search: '',
    form: {
      item_id: '',
      sale_value: '',
      ts: '',
    },
  }),
  computed: {
    showLocal: {
      get: function () {
        return this.show
      },
      set: function (val) {
        if (!val) this.closeModal()
      },
    },
  },
  validations: {
    form: {
      item_id: { required },
      sale_value: { required },
      ts: { required },
    },
  },
  methods: {
    closeModal() {
      this.$emit('close')
    },
    onItemSelected(item) {
      this.form.item_id = item.id
    },
    saveItem() {
      this.$v.form.$touch()
      if (this.$v.form.$pending || this.$v.form.$error) return

      const ts = this.$moment(this.form.ts).format('YYYY-MM-DDTHH:mm:ss')

      this.isLoading = true
      this.$api.sale
        .addSale({
          item_id: this.form.item_id,
          sale_value: parseInt(this.form.sale_value),
          ts,
        })
        .then((res) => {
          this.form.item_id = null
          this.form.sale_value = null
          this.form.ts = null
          this.$v.form.$reset()
          this.onSaved()
          this.$success(res.data)

          setTimeout(() => {
            this.isLoading = false
            this.$refs['input'].search = ''
            this.closeModal()
          }, 100)
        })
        .catch((_) => {
          this.isLoading = false
        })
    },
  },
}
</script>
