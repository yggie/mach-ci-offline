# Mach Test Browser (Standalone version)

The standalone version of
[mach-test-browser](https://github.com/yggie/mach-test-browser), as long as you
have a modern browser, it should just work! It simply contains the latest
distribution from
[mach-test-browser](https://github.com/yggie/mach-test-browser) to be used with
[mach](https://github.com/yggie/mach) for active development.


## Usage

To start the standalone server, just run:

```
cargo run
```

This will start the app server on http://localhost:8888. By default, it will
search for a file called `default.log` in the `public/` directory.

## TODOs

* Allow option for generating unique files – At present all generated files
  have the same filename, which means previous contents will be replaced. We
  might want to keep it in the future.
* File path independendence – The generated template still uses relative
  filepaths to retrieve the application scripts, this should be inlined.

## License

The software is distributed under the [MIT License](/LICENSE).
