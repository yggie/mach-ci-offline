# Mithril CI Offline

The offline version of [mithril-ci](https://github.com/yggie/mithril-ci), as
long as you have a modern browser, it should just work! It simply contains the
latest offline ready distribution from
[mithril-ci](https://github.com/yggie/mithril-ci) to be used with
[mithril](https://github.com/yggie/mithril) for active development.


## Usage

The repository provides a simple script to generate an offline compatible
version of the CI, simply run:

```
./generate_index.rb path/to/sample.logs
```

This will generate a compiled html file in the `generated/` directory, the file
name will be returned from the script. This makes it easy to do something like:

```
./generate_index.rb path/to/sample.logs | xargs open
```

Which will open the generated file in a browser.

## TODOs

* Truly offline – A few dependencies for the application are served over a CDN,
  for it to be truly offline, this should be included statically.
* Allow option for generating unique files – At present all generated files
  have the same filename, which means previous contents will be replaced. We
  might want to keep it in the future.
* File path independendence – The generated template still uses relative
  filepaths to retrieve the application scripts, this should be inlined.

## License

The software is distributed under the MIT License.
