/*
  Say hello
*/

local body = std.parseJson(std.extVar('body'));

{
  "title": "Greeting",
  "message": "Hello, " + body.name + "!"
}
