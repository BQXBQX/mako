const assert = require("assert");
const { parseBuildResult, trim } = require("../../../scripts/test-utils");
const { files } = parseBuildResult(__dirname);

let content = files["p__index-async.css"];

assert(content.includes(`flex: 1 1;`), "should enable flexBugs by default");
