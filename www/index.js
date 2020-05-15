import { CsvFile } from "webcsv"
const data = require("../tests/AGNC.csv")
let csv = CsvFile.new(data)
console.log(csv.get(0, 10))