<template>
  <div class="autocomplete">
    <input
      type="text"
      placeholder="Pilih Item"
      @input="onChange"
      @focus="isOpen = true"
      @keyup.down="onArrowDown"
      @keyup.up="onArrowUp"
      @keyup.enter="onEnter"
      required
      v-model="search"
    />

    <ul v-if="isOpen" class="autocomplete-results">
      <template v-if="items.length == 0">
        <li class="autocomplete-result no-result">Tidak ada item</li>
      </template>
      <template v-else>
        <li class="autocomplete-result" v-for="(item, i) in items" :key="i">
          <a
            @click="setResult(item)"
            :class="{ 'is-active': i === arrowCounter }"
          >
            {{ item.item_name }}
          </a>
        </li>
      </template>
    </ul>
  </div>
</template>
<script>
export default {
  data() {
    return {
      isOpen: false,
      items: [],
      isLoading: false,
      arrowCounter: -1,
      localSearch: null,
      search: '',
    }
  },
  mounted() {
    document.addEventListener('click', this.handleClickOutside)
  },
  destroyed() {
    document.removeEventListener('click', this.handleClickOutside)
  },
  methods: {
    onChange() {
      this.$emit('input', this.search)
      this.$api.item.getItems(10, 0, this.search).then((res) => {
        this.items = res.data.entries
      })
    },
    setResult(result) {
      this.search = result.item_name
      this.isOpen = false
      this.$emit('itemSelected', result)
    },
    handleClickOutside(evt) {
      if (!this.$el.contains(evt.target)) {
        this.isOpen = false
        this.arrowCounter = -1
      }
    },
    onArrowDown() {
      if (this.arrowCounter < this.items.length) {
        this.arrowCounter = this.arrowCounter + 1
      }
    },
    onArrowUp() {
      if (this.arrowCounter > 0) {
        this.arrowCounter = this.arrowCounter - 1
      }
    },
    onEnter() {
      this.setResult(this.items[this.arrowCounter])
      this.isOpen = false
      this.arrowCounter = -1
      return
    },
  },
}
</script>
<style lang="scss">
.autocomplete {
  position: relative;

  input {
    transition: opacity 0.1s ease-in-out;

    &:focus {
      border-bottom-color: #fff !important;
      border-bottom-left-radius: 0px !important;
      border-bottom-right-radius: 0px !important;
    }
  }
}

.autocomplete-results {
  padding: 0;
  margin: 0;
  border: 1px solid #eeeeee;
  max-height: 140px;
  overflow: auto;
  position: absolute;
  top: 100%;
  width: 100%;
  display: block;
  border-color: #96c8da;
  -webkit-box-shadow: 0 2px 3px 0 rgba(34, 36, 38, 0.15);
  box-shadow: 0 2px 3px 0 rgba(34, 36, 38, 0.15);
  transition: opacity 0.1s ease;
  background: #fff;
  border-top: 0 !important;
  border-radius: 0 0 0.28571429rem 0.28571429rem;
  z-index: 13;
  transition: opacity 0.1s ease-in-out;
}

.autocomplete-result {
  list-style: none;

  a {
    height: auto;
    text-align: left;
    border-top: none;
    line-height: 1em;
    color: rgba(0, 0, 0, 0.87);
    padding: 0.78571429rem 1.14285714rem !important;
    font-size: 1rem;
    text-transform: none;
    font-weight: 400;
    -webkit-box-shadow: none;
    box-shadow: none;
    border-top: 1px solid #fafafa;
    padding: 0.78571429rem 1.14285714rem !important;
    white-space: normal;
    word-wrap: normal;
    display: block;
    will-change: transform, opacity;

    &:hover {
      background: rgba(0, 0, 0, 0.05);
      color: rgba(0, 0, 0, 0.95);
      z-index: 13;
      cursor: pointer;
    }

    &.is-active {
      background: rgba(0, 0, 0, 0.05);
      color: rgba(0, 0, 0, 0.95);
      z-index: 13;
      cursor: pointer;
    }
  }
}
.no-result {
  text-align: center;
  height: auto;
  border-top: none;
  line-height: 1em;
  color: rgba(0, 0, 0, 0.87);
  padding: 0.78571429rem 1.14285714rem !important;
  font-size: 1rem;
  text-transform: none;
  font-weight: 400;
  -webkit-box-shadow: none;
  box-shadow: none;
  border-top: 1px solid #fafafa;
  padding: 0.78571429rem 1.14285714rem !important;
  white-space: normal;
  word-wrap: normal;
  display: block;
  will-change: transform, opacity;

  &:hover {
    color: rgba(0, 0, 0, 0.87);
    background: #fff;
  }
}
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s;
}
.fade-enter, .fade-leave-to /* .fade-leave-active below version 2.1.8 */ {
  opacity: 0;
}
</style>
