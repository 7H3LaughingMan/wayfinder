import { compilePack, TYPE_COLLECTION_MAP } from "foundryvtt-cli";

async function transformEntry(entry: any) {
    entry._key = `!${TYPE_COLLECTION_MAP[entry._type]}!${entry._id}`;
    delete entry._type;
}

await compilePack("packs/src/wayfinder-macros", "packs/wayfinder-macros", { transformEntry });
