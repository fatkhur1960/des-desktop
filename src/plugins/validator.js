export function validUsername(value) {
  var illegalChars = /\W/
  return !illegalChars.test(value)
}
