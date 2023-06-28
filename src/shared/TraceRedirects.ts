import got from "got";

export interface Redirect {
    url: string;
    statusCode: number;
    to: string;
}

export class TraceRedirects {
    public static LIMIT: number = 10;

    public static async trace(url: string): Promise<Redirect[]> {
        let currentUrl = url;
        let redirectCount = 0;
        let redirects: Redirect[] = [];

        while (redirectCount < this.LIMIT) {
            const response = await got.get(currentUrl, {
                followRedirect: false,
                throwHttpErrors: false
            });

            if (this.isRedirect(response.statusCode)) {
                redirects.push({
                    url: currentUrl,
                    statusCode: response.statusCode,
                    to: response.headers.location as string
                });
            } else {
                break;
            }
        }

        return redirects;
    }

    public static isRedirect(code: number): boolean {
        return code === 301 || code === 302;
    }
}