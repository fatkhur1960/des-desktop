<template>
  <transition name="modal">
    <sui-modal size="mini" v-model="showLocal">
      <sui-modal-header>Edit Item</sui-modal-header>
      <sui-modal-content>
        <sui-modal-description>
          <div class="dimmable">
            <div v-if="itemEditLoading" class="ui active inverted dimmer">
              <div class="ui loader"></div>
            </div>
            <sui-form @submit.prevent="saveItem">
              <sui-form-field
                required
                :class="{ error: $v.form.item_name.$error }"
              >
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
                <sui-button negative @click.prevent="closeModal()">
                  Batal
                </sui-button>
                <sui-button
                  :loading="isLoading"
                  primary
                  content="Simpan"
                  type="submit"
                />
              </div>
            </sui-form>
          </div>
        </sui-modal-description>
      </sui-modal-content>
    </sui-modal>
  </transition>
</template>
<script>
import { required } from 'vuelidate/lib/validators'

export default {
  name: 'EditItem',
  props: {
    show: { type: Boolean, default: false },
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
    form: {
      item_name: '',
      item_desc: '',
    },
  }),
  validations: {
    form: {
      item_name: { required },
    },
  },
  watch: {
    item: function (val) {
      if (val != null) {
        this.form.item_name = this.item.item_name
        this.form.item_desc = this.item.item_desc
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
  methods: {
    closeModal() {
      this.$emit('close')
    },
    saveItem() {
      this.$v.form.$touch()
      if (this.$v.form.$pending || this.$v.form.$error) return

      this.isLoading = true
      this.$api.item
        .updateItem({
          id: this.item.id,
          item_name: this.form.item_name,
          item_desc: this.form.item_desc,
        })
        .then((res) => {
          this.isLoading = false
          this.form.item_name = null
          this.form.item_desc = null
          this.$v.form.$reset()

          this.onSaved()
          this.closeModal()

          this.$success(res.data)
        })
        .catch((_) => {
          this.isLoading = false
        })
    },
  },
}
</script>
<style lang="scss">
.modal-enter {
  opacity: 0;
}
.modal-leave-active {
  opacity: 0;
}
.modal-enter .modal-container,
.modal-leave-active .modal-container {
  -webkit-transform: scale(1.1);
  transform: scale(1.1);
}
</style>
