import got from "got";

export interface SearchHeader {
    user_id: string;
    account_type: string;
    short_limit: string;
    long_limit: string;
    long_remaining: number;
    short_remaining: number;
    status: number;
    results_requested: number;
    index: {number: {
        status: number,
        parent_id: number,
        id: number,
        results: number
    }};
    search_depth: string;
    minimum_similarity: number;
    query_image_display: string;
    query_image: string;
    results_returned: number;
}

export interface ResultData {
    ext_urls: string[];
    title: string;
    fa_id: number;
    author_name: string;
    author_url: string;
    e621_id?: number;
    creator: string;
    material: string;
    characters: string;
    source: string;
    da_id: string;
    as_project: string;
    created_at?: Date;
    pawoo_id?: number;
    pawoo_user_acct: string;
    pawoo_user_username: string;
    pawoo_user_display_name: string;
    sankaku_id?: number;
    danbooru_id?: number;
    gelbooru_id?: number;
    anidb_aid?: number;
    mal_id?: number;
    anilist_id?: number;
    part: string;
    year: string;
    est_time: string;
    pixiv_id?: number;
    member_name: string;
    member_id?: number;
    bcy_id?: number;
    member_link_id?: number;
    bcy_type: string;
    imdb_id: string;
}

export interface PostHeader {
    similarity: string;
    thumbnail: string;
    index_id: number;
    index_name: string;
    dupes: number;
    hidden: number;
}

export interface SearchEntry {
    header: PostHeader,
    data: ResultData
}

export interface SearchResponse {
    header: SearchHeader;
    results: SearchEntry[];
}

export class ReverseImageSearch {
    public static async search(url: string, nsfw: boolean = false): Promise<SearchResponse | null> {
        const response = await got.get(
            `https://saucenao.com/search.php?db=999&output_type=2&api_key=${process.env.SAUCENAO_KEY}&testmode=1&numres=16&hide=${nsfw ? 0 : 3}&url=${url}`
        );

        return JSON.parse(response.body);
    }

    public static firstImageLink(content: string): string | null {
        const regex = /https?:\/\/[^\s]+\.(?:(?:png)|(?:jpg)|(?:gif)|(?:webp))/g
        const matches = content.match(regex);
        if (!matches) return null;
        return matches[0];
    }
}