const collatz = require("./collatz")

const numArray = collatz(30) //Any number
let maxNumber = null //Suppor variable

numArray.forEach((number) => {
  number > maxNumber ? (maxNumber = number) : maxNumber
})

console.log(maxNumber)
