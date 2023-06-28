import { InteractionCollector, Message, PartialMessage, TextChannel } from "discord.js";
import { CompoundClient } from "../structs/CompoundClient";
import { EmbedBuilder } from "@discordjs/builders";
import { Logger } from "../structs/Logger.js";

interface TrackedEntry {
    originMessageId: string;
    originMessageContent: string;
    originMessageAuthorId: string;
    relayMessage: Message;
    collector: any;
}

export class Ruin {
    private client: CompoundClient;
    private guildId = "933683767908913222";
    private candidateChannelId = "960791596100648990";
    private crosspostChannelId = "1031739800862081084";
    private registryChannelId = "1001500979755241603";
    private approvedRole = "934564453461131324";

    private trackedEntries: {[key: string]: TrackedEntry} = {}

    constructor(client: CompoundClient) {
        this.client = client;
    }

    public async start() {
        Logger.info("Ruin module started!")
        this.client.on("messageCreate", async (message) => {
            if (!message.guild || message.guild.id != this.guildId) return;
            try {
                if (message.channel.id == this.candidateChannelId) this.handleRegistry(message);
            } catch (err) {
                Logger.error(err as any);
            }
        })

        this.client.on("messageUpdate", async (oldMessage, newMessage) => {
            if (!newMessage.guild || newMessage.guild.id != this.guildId) return;
            try {
                if (newMessage.channel.id == this.candidateChannelId) this.handleRegistryUpdate(newMessage);
            } catch (err) {
                Logger.error(err as any);
            }
        })

        this.client.on("messageDelete", async (message) => {
            if (!message.guild || message.guild.id != this.guildId) return;

            // Handle disposing tracked entries
            if (message.id in this.trackedEntries) {
                let entry = this.trackedEntries[message.id];
                entry.collector.stop();
                await entry.relayMessage.delete();
                delete this.trackedEntries[message.id];
            }
        })
    }

    private trimContent(content: string) {
        if (content.length > 2000) {
            return content.substring(0, 1900) + `..[${content.length - 1900} more]`;
        }

        return content;
    }

    private async handleRegistryUpdate(message: Message | PartialMessage) {
        if (this.trackedEntries[message.id] == null) return;
        if (message.partial) await message.fetch();

        let entry = this.trackedEntries[message.id];
        if (entry.originMessageContent == message.content) return;
        // Edit the sent message contents
        if (entry.relayMessage != null) {
            await entry.relayMessage.edit({
                content: this.trimContent(message.content ? message.content : "No content...? How?")
            })
        }
    }

    private async handleRegistry(message: Message) {
        if (!message.content.startsWith("╭──────────────────.★..─╮") ||
            !message.content.endsWith("╰─..★.──────────────────╯")) return;

        await message.react("📓");

        const embed = new EmbedBuilder()
            .setTitle("Character Submission")
            .setTimestamp(new Date())
            .setAuthor({
                name: message.author.username,
                iconURL: message.author.displayAvatarURL()
            }).setFields([
                {
                    name: "Original message",
                    value: `[Here](${message.url})`,
                    inline: true
                },
                {
                    name: "Account Age",
                    value: `${message.author.createdAt.toDateString()}`,
                    inline: true
                },
                {
                    name: "Guild Age",
                    value: message.member?.joinedAt?.toDateString() || "nul :(",
                    inline: true
                }
            ]).setFooter({
                text: `ID: ${message.author.id}`
            })

        let post = await (message.client.channels.cache.get(this.crosspostChannelId) as TextChannel).send({
            content: this.trimContent(message.content),
            embeds: [embed],
            // Approve, Deny
            components: [
                {
                    type: 1,
                    components: [
                        {
                            type: 2,
                            style: 3,
                            label: "Approve",
                            customId: "approve"
                        },
                        {
                            type: 2,
                            style: 4,
                            label: "Deny",
                            customId: "deny"
                        }
                    ]
                }
            ]
        });

        // Add collectors for the buttons
        const collector = post.createMessageComponentCollector({
            time: 1000 * 60 * 60 * 48
        });

        this.trackedEntries[message.id] = {
            originMessageId: message.id,
            originMessageContent: message.content,
            originMessageAuthorId: message.author.id,
            relayMessage: post,
            collector: collector
        };

        collector.on("collect", async (interaction) => {
            if (interaction.customId == "approve") {
                // Add the approved role
                let member = await interaction.guild.members.fetch(this.trackedEntries[message.id].originMessageAuthorId);
                await member.roles.add(this.approvedRole);

                await message.reply({
                    content: `Your character's been approved! Please post it in <#${this.registryChannelId}>`
                })
            }
            
            if (interaction.customId == "deny") {
                await message.reply({
                    content: `Your character's been denied.`
                })
            }

            delete this.trackedEntries[message.id];
            await post.delete();
            collector.stop();
        });
    }
}