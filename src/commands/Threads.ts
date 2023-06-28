import { ChannelType, CommandInteraction, ComponentType, EmbedBuilder, Guild, GuildMember, GuildTextBasedChannel, GuildTextThreadManager, Interaction, PermissionFlagsBits, PermissionsBitField, SlashCommandBuilder, TextBasedChannel, ThreadChannel, ThreadMemberManager, User } from "discord.js";
import { CommandType } from "../structs/bases/BaseCommand.js";
import { SlashCommandBase } from "../structs/bases/SlashCommand.js";
import { CompoundClient } from "../structs/CompoundClient.js";

export class Threads implements SlashCommandBase {
    type =  CommandType.Slash;
    name = "threads";
    description = "Displays a list of threads the user is in.";


    public async execute(interaction: Interaction) {
        if (!interaction.isChatInputCommand()) return;
        if (!interaction.member || !interaction.memberPermissions || !interaction.guild) return;


        // ! Only enable if resyncing 
        // await this.syncThreads(interaction.guild);
    
        const hasPermissionForOthers = interaction.memberPermissions.has(PermissionsBitField.Flags.ManageMessages);
        const user = (interaction.options.getUser("user", false) && hasPermissionForOthers) ?
            interaction.options.getUser("user", false) ||
            interaction.user : interaction.user;

        const member = interaction.guild.members.cache.get(user.id);

        if (!member) {
            await interaction.reply({
                content: "User not found.",
                ephemeral: true
            });
            return;
        } else if (!hasPermissionForOthers && member.id !== interaction.user.id) {
            await interaction.reply({
                content: "You do not have permission to view other users' threads.",
                ephemeral: true
            });
            return;
        }

        let threads = await CompoundClient.instance.db.direct.collection("threads").find({
            members: {
                $in: [user.id]
            }
        }).toArray();

        // Sort by last message time
        const formattedThreads = threads.sort((a, b) => {
            if (a.lastMessageAt === null) return 1;
            if (b.lastMessageAt === null) return -1;
            return b.lastMessageAt - a.lastMessageAt;
        }).map((thread) => {
            return `<#${thread.id}> - ${thread.name}`
        });

        await this.page(interaction, formattedThreads, member)
    };

    public async page(interaction: CommandInteraction, threads: string[], member: GuildMember) {
        const maxLength = 2048;
        const nestedArray: string[][] = [];

        let currentSize = 0;
        let stack: string[] = [];
        for (const thread of threads) {
            if (currentSize + thread.length > maxLength) {
                nestedArray.push(stack);
                stack = [];
                currentSize = 0;
            }
            stack.push(thread);
            currentSize += thread.length;
        }

        nestedArray.push(stack);

        let currentIndex = 0;
        const embed = new EmbedBuilder()
            .setTitle(`Threads for ${member.displayName} (${threads.length})`)
            .setDescription(nestedArray[currentIndex].join("\n"))
            .setFooter({
                text: `Page ${currentIndex + 1} of ${nestedArray.length}`
            });

        const message = await interaction.reply({
            embeds: [embed],
            ephemeral: true,
            components: nestedArray.length > 1 ? [
                {
                    type: 1,
                    components: [
                        {
                            type: 2,
                            style: 1,
                            label: "Previous",
                            customId: "previous-collector"
                        },
                        {
                            type: 2,
                            style: 1,
                            label: "Next",
                            customId: "next-collector"
                        }
                    ]
                }
            ] : []
        });

        if (nestedArray.length <= 1) return;

        const collector = message.createMessageComponentCollector({
            componentType: ComponentType.Button,
            filter: (i) => i.user.id === interaction.user.id,
            time: 360000
        });

        collector.on("collect", async (i) => {
            if (i.customId === "previous-collector") {
                currentIndex--;
                if (currentIndex < 0) currentIndex = nestedArray.length - 1;
            } else if (i.customId === "next-collector") {
                currentIndex++;
                if (currentIndex > nestedArray.length - 1) currentIndex = 0;
            }

            embed.setDescription(nestedArray[currentIndex].join("\n"));
            embed.setFooter({
                text: `Page ${currentIndex + 1} of ${nestedArray.length}`
            });

            await i.update({
                embeds: [embed]
            });
        });

        // Delete the collector when ended
        collector.on("end", (x) => {
            embed.setDescription("This message is now inactive.")
            embed.setFooter({
                text: `I can't pay attention forever..`
            })

            x.first()?.editReply({
                components: [],
                embeds: [embed]
            })
        });

    }

    public async syncThreads(guild: Guild) {
        const threads = await guild.channels.cache.filter((channel) => {
            return channel.isThread();
        })

        // for (const pairs of threads.entries()) {
        //     const thread = pairs[1] as ThreadChannel;
        //     await thread.members.fetch();
        //     console.log("Insert")
        //     CompoundClient.instance.db.direct.collection("threads").updateOne({id: thread.id}, {
        //         $set: {
        //             id: thread.id,
        //             name: thread.name,
        //             members: thread.members.cache.map(m => m.id),
        //             archivedAt: thread.archiveTimestamp,
        //             createdAt: thread.createdTimestamp,
        //             lastMessage: thread.lastMessage,
        //             lastMessageAt: thread.lastPinAt?.getTime() || null,
        //             creator: thread.ownerId,
        //             disableArchive: false
        //         }
        //     }, { upsert: true })
        // }

        for (const pairs of guild.channels.cache.entries()) {
            let channel = pairs[1] as any;
            // Return if channel isn't of guildtextchannel

            if (channel.type != ChannelType.GuildText) continue;
            const tm = channel.threads as GuildTextThreadManager<TextBasedChannel>;
            const threadcol = await tm.fetchArchived({
                fetchAll: true
            })
            
            for (const pairs of threadcol.threads.entries()) {
                const thread = pairs[1] as ThreadChannel;
                await thread.members.fetch();
                CompoundClient.instance.db.direct.collection("threads").updateOne({id: thread.id}, {
                    $set: {
                        id: thread.id,
                        name: thread.name,
                        members: thread.members.cache.map(m => m.id),
                        archivedAt: thread.archiveTimestamp,
                        createdAt: thread.createdTimestamp,
                        lastMessage: thread.lastMessage,
                        lastMessageAt: thread.lastPinAt?.getTime() || null,
                        creator: thread.ownerId,
                        disableArchive: false
                    }
                }, { upsert: true })
            }
        }
    }

    public builder() {
        return new SlashCommandBuilder()
            .setName(this.name)
            .setDMPermission(false)
            .setDescription(this.description)
            .setDefaultMemberPermissions(PermissionFlagsBits.SendMessages)
            .addUserOption(option => 
                option.setName("user")
                .setDescription("The user to view threads for")
                .setRequired(false))
    }
}