function uiPluginInstall(Vue, options) {
  const notifSuccess = function (message) {
    Vue.notify({
      group: 'app',
      type: 'success',
      title: 'Info!',
      text: message,
    })
  }

  const notifError = function (message) {
    Vue.notify({
      group: 'app',
      type: 'error',
      title: 'Error!',
      text: message,
    })
  }

  const pagination = function (currentPage, totalPage) {
    var delta = 2,
      range = [],
      rangeWithDots = [],
      l

    range.push(1)
    for (let i = currentPage - delta; i <= currentPage + delta; i++) {
      if (i < totalPage && i > 1) {
        range.push(i)
      }
    }
    if (totalPage != 1) range.push(totalPage)

    for (let i of range) {
      if (l) {
        if (i - l === 2) {
          rangeWithDots.push(l + 1)
        } else if (i - l !== 1) {
          rangeWithDots.push('...')
        }
      }
      rangeWithDots.push(i)
      l = i
    }

    return rangeWithDots
  }

  const itemNum = function (currentPage, perPage, idx) {
    return currentPage * perPage - perPage + idx + 1
  }

  //Install Vue plugin
  Vue.success = notifSuccess
  Vue.error = notifError
  Vue.paginationArray = pagination
  Vue.itemNum = itemNum
  Object.defineProperties(Vue.prototype, {
    $itemNum: {
      get: () => Vue.itemNum,
    },
    $success: {
      get: () => Vue.success,
    },
    $error: {
      get: () => Vue.error,
    },
    $paginationArray: {
      get: () => Vue.paginationArray,
    },
  })
}

export default uiPluginInstall
