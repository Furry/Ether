import "colors";

export class Logger {
    public static get time() {
        return new Date().toLocaleTimeString();
    }

    public static warn(message: string) {
        console.log(`${`[${Logger.time}]`.green} ${`[WARN]`.yellow} ${message}`);
    }

    public static error(message: string) {
        console.log(`${`[${Logger.time}]`.green} ${`[ERROR]`.red} ${message}`);
    }

    public static info(message: string) {
        console.log(`${`[${Logger.time}]`} ${`[INFO]`.blue} ${message}`);
    }

    public static debug(message: string) {
        console.log(`${`[${Logger.time}]`.green} ${`[DEBUG]`.magenta} ${message}`);
    }
}