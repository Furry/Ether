export enum CommandType {
    Slash = "slash",
    ContextMenu = "contextmenu"
}

export interface BaseCommand {
    name: string;
    type: CommandType;
}