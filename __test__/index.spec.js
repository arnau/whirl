const test = require('ava')
const { fromToml, fromSketchToml } = require('../index')

test('parse generic TOML', (t) => {
  const actual = fromToml('number = 42')
  const expected = { number: 42 }

  t.deepEqual(actual, expected)
})

test('parse sketch TOML', (t) => {
  const actual = fromSketchToml(`
type = "Sketch"
caption = "Bold man"
id = "12206bd06e46db3fc1c79bced449bc3844c6ea5b90c457e626e506e923a2beb67532"
image = "./image.png"
author = "arnau"
date = 2019-08-03

[[tools]]
id = "ipadpro"
name = "iPad Pro"

[[tools]]
id = "sketches"
name = "Sketches Pro"
url = "https://tayasui.com/sketches/"
  `)
  const expected = {
    caption: 'Bold man',
    id: '12206bd06e46db3fc1c79bced449bc3844c6ea5b90c457e626e506e923a2beb67532',
    image: './image.png',
    author: 'arnau',
    date: new Date('2019-08-03'),
    tools: [{
      id: 'ipadpro',
      name: 'iPad Pro'

    }, {
      id: 'sketches',
      name: 'Sketches Pro',
      url: 'https://tayasui.com/sketches/'

    }]
  }

  t.deepEqual(actual, expected)
})
