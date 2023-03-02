import { ContextMenuBase } from "./structs/bases/ContextMenu.js";
import { SlashCommandBase } from "./structs/bases/SlashCommand.js";

export type GenericObject = {
    [key: string]: any;
};

export type Command = SlashCommandBase | ContextMenuBase;