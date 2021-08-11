## Hot Curry(culum Vitæ)

Hot Curry is a simple CV generator that expects:

- A template (written in [Tera](https://tera.netlify.app/docs/) a powerful template engine which syntax is extremely close to [Jinja](https://jinja.palletsprojects.com/en/3.0.x/))
- And a "data" file in Yaml, Json, or Toml, containing your CV

Running the `hot-curry` command you can then generate an HTML file and/or a PDF file containing your CV.

### Usage

1. You need to create a `hot_curry.toml` file, this will contain all the configuration needed by Hot Curry to generate your CV. A typical config file looks like this:

```toml
[source]
# Path to the source that contains the CV's data, can be json, toml, or yml/yaml
path = "./source.yml"

[template]
# Path or URL to the html file to render the source data, must use Tera's syntax
source = "./render.html"
# If your template is hosted you can use the following syntax:
# source = {url = "https://..."}

[output]
# The path, doesn't include extension
path = "./cv"
# The output type(s): pdf or html (more output types could be implemented in the future)
types = ["html", "pdf"]
```

_Generating pdf files requires Chromium or Chrome installed and accessible from your PATH._

2. You can now create your own "render" template (or use an existing one from the [hot-curry-themes](https://github.com/gaku-sei/hot-curry-themes/) repository):

```html
<div>{{ name }}'s CV</div>
```

3. And some data:

```yaml
name = Kévin
```

4. Run `hot-curry`, the `test.pdf` and `test.html` files have been generated for you.

5. You want the perfect style and need to check what your CV looks like when the `render.html` template changes? You can use the [watchexec](https://github.com/watchexec/watchexec) binary: `watchexec -e html -r hot-curry`. This command will automatically rebuild your CV and you can check what the new version looks like in browser.

### Benefits

Hot Curry can come very handy when you need to translate your CV in several languages, or if you want to refresh the style without changing the data inside, or on the contrary, add a new work experience without copy pasting HTML code here and there.

It's also very useful when you need to "distribute" your CV in different formats.
