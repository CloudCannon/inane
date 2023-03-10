const fs = require("fs");
const path = require("path");

const cwd = process.cwd();
const package_name = path.basename(cwd);
const [os, cpu] = process.argv.slice(2);

if (!os || !cpu) {
    console.error("Script os and cpu as positional arguments");
    process.exit(1);
}

const version = process.env.GIT_VERSION;
if (!version) {
    console.error("Script expected a GIT_VERSION environment variable");
    process.exit(1);
}

fs.writeFileSync(path.join(cwd, "package.json"), JSON.stringify({
    name: package_name,
    version,
    description: `The platform-specific binary for inane on ${os}/${cpu}`,
    license: "MIT",
    repository: {
        type: "git",
        url: "git+https://github.com/cloudcannon/inane.git"
    },
    author: "CloudCannon",
    os: [ os ],
    cpu: [ cpu ],
}));

fs.writeFileSync(path.join(cwd, "README.md"), `# Inane

The platform-specific binary for inane on ${os}/${cpu}
`);