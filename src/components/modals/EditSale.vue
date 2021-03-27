<template>
  <sui-modal size="mini" v-model="showLocal">
    <sui-modal-header>Edit Penjualan</sui-modal-header>
    <sui-modal-content>
      <sui-modal-description>
        <sui-form @submit.prevent="saveItem" spellcheck="false">
          <sui-form-field>
            <label>Nama Item</label>
            <input type="text" v-model="form.item_name" readonly />
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
    item: {
      type: Object,
      required: false,
      default: {},
    },
  },
  data: () => ({
    itemEditLoading: true,
    isLoading: false,
    lang: langID,
    form: {
      id: null,
      sale_value: '',
      item_name: '',
      ts: '',
    },
  }),
  watch: {
    item: function (val) {
      if (val != null) {
        this.form.id = this.item.id
        this.form.item_name = this.item.item_name
        this.form.sale_value = this.item.sale_value
        this.form.ts = this.item.ts
        this.itemEditLoading = false
      }
    },
  },
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
      sale_value: { required },
      ts: { required },
    },
  },
  methods: {
    closeModal() {
      this.$emit('close')
    },
    saveItem() {
      this.$v.form.$touch()
      if (this.$v.form.$pending || this.$v.form.$error) return

      const ts = this.$moment(this.form.ts).format('YYYY-MM-DDTHH:mm:ss')

      this.isLoading = true
      this.$api.sale
        .updateSale({
          id: this.item.id,
          sale_value: parseInt(this.form.sale_value),
          ts,
        })
        .then((res) => {
          this.form.id = null
          this.form.sale_value = null
          this.form.item_name = null
          this.form.ts = null
          this.$v.form.$reset()
          this.onSaved()
          this.$success(res.data)

          setTimeout(() => {
            this.isLoading = false
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
