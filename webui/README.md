# Forge Web UI

Preact proof of concept for the Forge in-game browser UI.

## Commands

```sh
npm install
npm run dev
npm run build
npm run build:arma
```

The app uses hash routes so local browser-control loading can navigate without a web server:

- `#/`
- `#/about`

The normal production build writes static files to `dist/`. The Arma build writes a browser bootstrap to `arma/crate/addons/webui/ui/index.html` and copies the compiled stylesheet/script to `ui/_site`. The bootstrap loads those files with `A3API.RequestFile`, which avoids `CT_WEBBROWSER` local-subresource blocking.
