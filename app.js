const fs = require("fs");

const easytier = require("./");

const config = fs.readFileSync("./app.yaml", "utf8");

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
console.log(1, 2);

let check_config = easytier.parseConfig(config);

if (check_config === 0) {
  console.log("Configuration is valid.");
} else {
  console.error("Configuration is invalid.");
  process.exit(1);
}

async function main() {
  let result = easytier.runNetworkService(config);
  if (result === 0) {
    console.log("Network service started successfully.");
    while (true) {
      await sleep(3000);
      let networkInfos = easytier.collectNetworkInfos();
      networkInfos.forEach((info) => {
        console.log(`Network Name: ${info.key},${info.value}`);
      });
    }
  } else {
    console.error("Failed to start network service.");
    process.exit(1);
  }
}

main().catch((err) => {
  console.error("Error:", err);
  process.exit(1);
});
