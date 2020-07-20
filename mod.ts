import {
    loadPlugin,
    importFromPlugin,
} from "https://denopkg.com/Srinivasa314/calcite-ts@1.0/calcite.ts";

await loadPlugin("deno_digest_plugin", "file://target/debug/");

export const multiply = importFromPlugin('multiply') as (a: number, b: number) => number
export const welcome = importFromPlugin('welcome') as (name: string) => string
