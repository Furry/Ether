import { ActivityOptions, ActivityType } from "discord.js";
import { CompoundClient } from "./CompoundClient.js";

export class Intervals {
    public static startStatus(client: CompoundClient) {
        const statuses = [
            `/help <3`,
            `a good movie!`,
            `a train wreck.`
        ];

        
        let i = 0;
        setInterval(() => {
            client.user?.setActivity(statuses[i], {
                type: ActivityType.Playing,
                name: statuses[i]
            });
            i = ++i % statuses.length;
        }, 30000);
    }
}