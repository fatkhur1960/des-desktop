<template>
  <div>
    <sui-button primary @click.native="toggle">Add Item</sui-button>
    <sui-modal size="mini" v-model="open">
      <sui-modal-header>Add Item</sui-modal-header>
      <sui-modal-content>
        <sui-modal-description>
          <sui-form>
            <sui-form-field>
              <label>Item Name</label>
              <input placeholder="Item Name" />
            </sui-form-field>
          </sui-form>
        </sui-modal-description>
      </sui-modal-content>
      <sui-modal-actions>
        <sui-button negative @click.native="toggle"> Cancel </sui-button>
        <sui-button positive @click.native="toggle"> Save </sui-button>
      </sui-modal-actions>
    </sui-modal>
    <table class="ui single line table">
      <thead>
        <tr>
          <th>Name</th>
          <th>Premium Plan</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>John Lilki</td>
          <td>No</td>
        </tr>
        <tr>
          <td>Jamie Harington</td>
          <td>Yes</td>
        </tr>
        <tr>
          <td>Jill Lewis</td>
          <td>Yes</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
<script>
export default {
  data() {
    return { open: false };
  },
  methods: {
    toggle() {
      this.open = !this.open;
    },
  },
};
</script>
<style scoped>
</style>