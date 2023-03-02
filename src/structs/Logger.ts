export class Logger {
    public static get time() {
        return new Date().toLocaleTimeString();
    }

    public static warn(message: string) {
        console.log(`[${Logger.time}] [WARN] ${message}`);
    }

    public static error(message: string) {
        console.log(`[${Logger.time}] [ERROR] ${message}`);
    }

    public static info(message: string) {
        console.log(`[${Logger.time}] [INFO] ${message}`);
    }

    public static debug(message: string) {
        console.log(`[${Logger.time}][DEBUG] ${message}`);
    }
}