<template>
  <div id="app">
    <notifications
      group="app"
      position="top right"
      classes="vue-notification"
    />
    <v-dialog />
    <template v-if="$route.meta.sidebar">
      <Sidebar :logoutModal="toggle" />
      <sui-modal size="mini" v-model="open">
        <sui-modal-header>Keluar</sui-modal-header>
        <sui-modal-content>
          <sui-modal-description>
            <p>Anda yakin ingin keluar?</p>
          </sui-modal-description>
        </sui-modal-content>
        <sui-modal-actions>
          <sui-button negative @click.native="toggle">Batal</sui-button>
          <sui-button positive @click.native="logout">Ya</sui-button>
        </sui-modal-actions>
      </sui-modal>
      <div class="content-view">
        <vue-page-transition name="fade-in-left">
          <router-view />
        </vue-page-transition>
      </div>
    </template>
    <router-view v-else></router-view>
  </div>
</template>

<style lang="scss">
@import url('assets/app.scss');
</style>

<script>
import Sidebar from "@/components/Sidebar";
export default {
  name: "App",
  components: { Sidebar },
  data() {
    return { open: false };
  },
  methods: {
    toggle() {
      this.open = !this.open;
    },
    logout() {
      this.open = false;
      this.$router.replace("/");
    },
  },
};
</script>