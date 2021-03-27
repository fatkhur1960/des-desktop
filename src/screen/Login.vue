<template>
  <div class="login">
    <div class="ui aligned grid">
      <div class="column">
        <form class="ui form bos-form" @submit.prevent="doLogin">
          <div class="ui segment container">
            <div class="header">
              <h2>Masuk</h2>
            </div>
            <div
              class="required field"
              :class="{ error: $v.form.username.$error }"
            >
              <div class="ui icon input">
                <input
                  type="text"
                  name="username"
                  v-model.trim="$v.form.username.$model"
                  placeholder="Username"
                />
              </div>
              <div class="error-msg" v-if="$v.form.username.$error">
                <div class="error" v-if="!$v.form.username.required">
                  Username tidak boleh kosong
                </div>
              </div>
            </div>
            <div
              class="required field"
              :class="{ error: $v.form.password.$error }"
            >
              <div class="ui icon input">
                <input
                  :type="show2 ? 'text' : 'password'"
                  name="password"
                  v-model.trim="$v.form.password.$model"
                  placeholder="Password"
                />
              </div>
              <div class="error-msg" v-if="$v.form.password.$error">
                <div class="error" v-if="!$v.form.password.required">
                  Password tidak boleh kosong
                </div>
              </div>
            </div>
            <sui-button
              fluid
              large
              :loading="loading"
              type="submit"
              color="orange"
              content="Masuk"
            />
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script>
import { required } from 'vuelidate/lib/validators'

export default {
  name: 'Login',
  data() {
    return {
      loading: false,
      show2: false,
      form: {
        username: '',
        password: '',
      },
    }
  },
  validations: {
    form: {
      username: { required },
      password: { required },
    },
  },
  mounted() {
    this.form.username = 'admin'
    this.form.password = '123123'
  },
  methods: {
    doLogin() {
      this.$v.form.$touch()
      if (this.$v.form.$pending || this.$v.form.$error) return

      this.loading = true
      this.$api.auth
        .authorize({
          username: this.form.username,
          password: this.form.password,
        })
        .then((res) => {
          this.$router.replace('/home')
          this.loading = false
        })
        .catch((e) => {
          console.log(e)
          this.loading = false
        })
    },
  },
}
</script>

<style lang="scss" scoped>
.login {
  height: 100vh;
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow: hidden;
}
</style>
