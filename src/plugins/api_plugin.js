function apiPlugin(Vue, options) {
  //Install Vue plugin
  Vue.api = options
  Object.defineProperties(Vue.prototype, {
    $api: {
      get: () => Vue.api,
    },
  })
}

export default apiPlugin
