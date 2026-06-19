import { render } from "preact";
import { App } from "./app";
import { navigateWithArma } from "./bridge/host";
import "./styles.css";

document.addEventListener("click", (event) => {
    const target = event.target;
    if (!(target instanceof Element)) {
        return;
    }

    const link = target.closest<HTMLAnchorElement>("a[href]");
    const href = link?.getAttribute("href");
    if (!href?.startsWith("arma://")) {
        return;
    }

    event.preventDefault();
    navigateWithArma(href);
});

const root = document.getElementById("app");

if (!root) {
    throw new Error("Forge web UI root element was not found.");
}

render(<App />, root);
