<template>
  <div class="ui left visible vertical inverted sidebar labeled icon menu">
    <router-link to="/home" class="item">
      <i class="home icon"></i>
      Home
    </router-link>
    <router-link to="/item" class="item">
      <i class="archive icon"></i>
      Item
    </router-link>
    <router-link to="/sales" class="item">
      <i class="shopping bag icon"></i>
      Sales
    </router-link>
    <router-link to="/predict" class="item">
      <i class="chart line icon"></i>
      Predict
    </router-link>
    <router-link to="/admin" class="item">
      <i class="users icon"></i>
      Admin
    </router-link>
    <a class="item" @click.prevent="logoutModal">
      <i class="sign-out alternate icon"></i>
      Logout
    </a>
  </div>
</template>
<script>
export default {
  props: {
    logoutModal: {
      type: Function,
      required: true,
    }
  }
};
</script>
<style>
.router-link-exact-active {
  color: #ffca28 !important;
}
</style>