import fs from "fs";
import module from "../module.json" with { type: "json" };
import path from "path";
import prompts from "prompts";

const dataPath = "C:\\FoundryVTT\\FoundryVTT-WindowsPortable-13.345\\Data";

const symlinkPath = path.resolve(dataPath, "modules", module.id);
const symlinkStats = fs.lstatSync(symlinkPath, { throwIfNoEntry: false });
if (symlinkStats) {
    const atPath = symlinkStats.isDirectory() ? "folder" : symlinkStats.isSymbolicLink() ? "symlink" : "file";
    const proceed: boolean = (
        await prompts({
            type: "confirm",
            name: "value",
            initial: false,
            message: `A "${module.id}" ${atPath} already exists in the "modules" subfolder. Replace with new symlink?`,
        })
    ).value;
    if (!proceed) {
        console.log("Aborting.");
        process.exit();
    }
}

try {
    if (symlinkStats?.isDirectory()) {
        fs.rmSync(symlinkPath, { recursive: true, force: true });
    } else if (symlinkStats) {
        fs.unlinkSync(symlinkPath);
    }
    fs.symlinkSync(path.resolve(process.cwd()), symlinkPath, "junction");
} catch (error) {
    if (error instanceof Error) {
        console.error(`An error was encountered trying to create a symlink: ${error.message}`);
        process.exit(1);
    }
}

console.log(`Symlink succesfully created at "${symlinkPath}"!`);
