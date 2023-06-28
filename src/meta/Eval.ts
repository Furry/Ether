import readline from "readline";
import { CompoundClient } from "../structs/CompoundClient.js";
import { Logger } from "../structs/Logger.js";

export class Evaluator {
    public static async EvalLoop(client: CompoundClient) {
        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout
        });

        rl.on("line", async (input) => {
            Logger.info("Evaluating: " + `"${input}"`);
            try {
                eval(input);
            } catch (err) {
                Logger.error(err as any);
            }
        });
    }
}