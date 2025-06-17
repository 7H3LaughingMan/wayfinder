import { execSync } from "child_process";

execSync("node C:\\FoundryVTT\\FoundryVTT-WindowsPortable-13.345\\App\\resources\\app\\main.js", {
    cwd: "C:\\FoundryVTT\\FoundryVTT-WindowsPortable-13.345\\App",
    stdio: "inherit",
});
