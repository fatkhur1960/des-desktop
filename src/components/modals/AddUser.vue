<template>
  <sui-modal size="mini" v-model="showLocal">
    <sui-modal-header>Tambahkan Admin</sui-modal-header>
    <sui-modal-content>
      <sui-modal-description>
        <sui-form @submit.prevent="saveItem" spellcheck="false">
          <sui-form-field required :class="{ error: $v.form.full_name.$error }">
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
          <sui-form-field required :class="{ error: $v.form.username.$error }">
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
          <sui-form-field required :class="{ error: $v.form.password.$error }">
            <label>Password</label>
            <input
              type="password"
              placeholder="Password"
              v-model="$v.form.password.$model"
            />
            <div class="error-desc" v-if="$v.form.password.$error">
              <div v-if="!$v.form.password.required">
                Password tidak boleh kosong!
              </div>
              <div v-if="!$v.form.password.minLength">
                Password minimal harus terdiri dari
                {{ $v.form.password.$params.minLength.min }} karakter!
              </div>
            </div>
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
import { required, minLength } from 'vuelidate/lib/validators'
import { validUsername } from '@/plugins/validator'

export default {
  name: 'AddUser',
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
      full_name: '',
      username: '',
      password: '',
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
      full_name: { required },
      username: { required, validUsername, minLength: minLength(6) },
      password: { required, minLength: minLength(6) },
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
        .addUser({
          full_name: this.form.full_name,
          username: this.form.username,
          password: this.form.password,
        })
        .then((res) => {
          this.form.full_name = null
          this.form.username = null
          this.form.password = null
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
