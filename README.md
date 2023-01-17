# JsonEditor-rs

Quick and dirty ncurses JSON array editor as Rust practice

Takes in an JSON array in the following format:
```
[
  {
    "name": "Some configurable string",
    "value": "Some configuration data"
  },
  {
    "name": "Some configurable int",
    "value": 15
  }
  ...
]
```

Whether you have additional fields in the JSON, like description, rights or additional data, doesn't matter.
This quick and dirty only reads from objects in a list, name and value.
Name is shown in the "name area" on the left of the screen followed by a colon.
Value area is on the right side of the colon and editable by the user.
