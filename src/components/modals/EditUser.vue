<template>
  <transition name="modal">
    <sui-modal size="mini" v-model="showLocal">
      <sui-modal-header>Edit Admin</sui-modal-header>
      <sui-modal-content>
        <sui-modal-description>
          <div class="dimmable">
            <div v-if="itemEditLoading" class="ui active inverted dimmer">
              <div class="ui loader"></div>
            </div>
            <sui-form @submit.prevent="saveItem">
              <sui-form-field
                required
                :class="{ error: $v.form.full_name.$error }"
              >
                <label>Nama Lengkap</label>
                <input
                  type="text"
                  v-model="$v.form.full_name.$model"
                  placeholder="Username"
                />
                <div class="error-desc" v-if="$v.form.full_name.$error">
                  Tidak boleh kosong
                </div>
              </sui-form-field>
              <sui-form-field
                required
                :class="{ error: $v.form.username.$error }"
              >
                <label>Username</label>
                <input
                  type="text"
                  placeholder="Username"
                  v-model="$v.form.username.$model"
                />
                <div class="error-desc" v-if="$v.form.username.$error">
                  <div v-if="!$v.form.username.required">
                    Username tidak boleh kosong!
                  </div>
                  <div v-if="!$v.form.username.validUsername">
                    Username tidak valid!
                  </div>
                  <div v-if="!$v.form.username.minLength">
                    Username harus terdiri dari
                    {{ $v.form.username.$params.minLength.min }} karakter!
                  </div>
                </div>
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
import { required, minLength } from 'vuelidate/lib/validators'
import { validUsername } from '@/plugins/validator'

export default {
  name: 'EditUser',
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
      full_name: { required },
      username: { required, validUsername, minLength: minLength(6) },
    },
  }),
  validations: {
    form: {
      full_name: { required },
      username: { required, validUsername, minLength: minLength(6) },
    },
  },
  watch: {
    item: function (val) {
      if (val != null) {
        this.form.full_name = this.item.full_name
        this.form.username = this.item.username
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
      this.$api.user
        .updateUser({
          id: this.item.id,
          full_name: this.form.full_name,
          username: this.form.username,
        })
        .then((res) => {
          this.isLoading = false
          this.form.full_name = null
          this.form.username = null
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
