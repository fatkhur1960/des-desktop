<template>
  <sui-modal size="mini" v-model="showLocal">
    <sui-modal-header>Tambahkan Item</sui-modal-header>
    <sui-modal-content>
      <sui-modal-description>
        <sui-form @submit.prevent="saveItem">
          <sui-form-field required :class="{ error: $v.form.item_name.$error }">
            <label>Nama Item</label>
            <input
              type="text"
              placeholder="Nama Item"
              v-model="$v.form.item_name.$model"
            />
            <span class="error-desc" v-if="$v.form.item_name.$error">
              Tidak boleh kosong
            </span>
          </sui-form-field>
          <sui-form-field>
            <label>Deskripsi</label>
            <input
              type="text"
              placeholder="Deskripsi"
              v-model="form.item_desc"
            />
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

export default {
  name: 'AddItem',
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
    form: {
      item_name: '',
      item_desc: '',
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
      item_name: { required },
    },
  },
  methods: {
    closeModal() {
      this.$emit('close')
    },
    saveItem() {
      this.$v.form.$touch()
      if (this.$v.form.$pending || this.$v.form.$error) return

      this.isLoading = true
      this.$api.item
        .addItem({
          item_name: this.form.item_name,
          item_desc: this.form.item_desc,
        })
        .then((res) => {
          this.form.item_name = null
          this.form.item_desc = null
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
