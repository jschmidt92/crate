import { copyFile, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = dirname(fileURLToPath(import.meta.url));
const projectRoot = resolve(scriptDir, "..");
const repoRoot = resolve(projectRoot, "..");
const distDir = resolve(projectRoot, "dist");
const targetDir = resolve(repoRoot, "arma/client/addons/webui/ui");
const siteDir = resolve(targetDir, "_site");

function findFirstAsset(html, pattern, label) {
    const match = html.match(pattern);
    if (!match) {
        throw new Error(`Unable to find ${label} asset in Vite index.html.`);
    }
    return match[1].replace(/^\.\//, "");
}

const sourceHtml = await readFile(resolve(distDir, "index.html"), "utf8");
const cssAsset = findFirstAsset(sourceHtml, /<link\s+rel="stylesheet"[^>]*href="([^"]+)"/, "CSS");
const jsAsset = findFirstAsset(sourceHtml, /<script\s+type="module"[^>]*src="([^"]+)"/, "JS");

await rm(targetDir, { recursive: true, force: true });
await mkdir(siteDir, { recursive: true });
await copyFile(resolve(distDir, cssAsset), resolve(siteDir, "style.css"));
await copyFile(resolve(distDir, jsAsset), resolve(siteDir, "script.js"));

const indexHtml = `<!doctype html>
<html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Forge</title>
        <script>
            Promise.all([
                A3API.RequestFile("forge\\\\forge_client\\\\addons\\\\webui\\\\ui\\\\_site\\\\style.css"),
                A3API.RequestFile("forge\\\\forge_client\\\\addons\\\\webui\\\\ui\\\\_site\\\\script.js"),
            ]).then(([css, js]) => {
                const style = document.createElement("style");
                style.textContent = css;
                document.head.appendChild(style);

                const script = document.createElement("script");
                script.text = js;
                document.head.appendChild(script);
            }).catch((error) => {
                document.body.textContent = "Forge WebUI failed to load packaged assets.";
            });
        </script>
    </head>
    <body>
        <div id="app"></div>
    </body>
</html>
`;

await writeFile(resolve(siteDir, "index.html"), indexHtml);

console.log(`Packaged Arma UI: ${targetDir}`);
