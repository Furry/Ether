import { MongoClient, WithId } from "mongodb";
import { GenericObject } from "../types.js";

// Represents a MongoDB Database
export class Database {
    public state: "connected" | "disconnected" = "disconnected";
    public client: MongoClient;

    constructor(connection: string, public db: string) {
        this.client = new MongoClient(connection);
    }

    public async connect() {
        await this.client.connect();
        this.state = "connected";
    }

    public async fetch<T>(collection: string, query: GenericObject): Promise<T | null> {
        if (this.state === "disconnected") {
            throw new Error("Database is not connected");
        }

        const db = this.client.db(this.db);
        const data = await db.collection(collection).findOne({ ...query});
        return data as (WithId<T> & T) | null;
    }

    public async fetchAll<T>(collection: string, query: GenericObject): Promise<T[]> {
        if (this.state === "disconnected") {
            throw new Error("Database is not connected");
        }

        const db = this.client.db();
        const data = await db.collection(collection).find({ ...query }).toArray();
        return data as (WithId<T> & T)[];
    }

    public async insert(collection: string, data: GenericObject) {
        if (this.state === "disconnected") {
            throw new Error("Database is not connected");
        }

        const db = this.client.db();
        return await db.collection(collection).insertOne({ ...data });
    }

    public async update(collection: string, query: GenericObject, data: GenericObject) {
        if (this.state === "disconnected") {
            throw new Error("Database is not connected");
        }

        const db = this.client.db();
        return await db.collection(collection).updateOne({ ...query }, { $set: { ...data } });
    }

    public async updateMany(collection: string, query: GenericObject, data: GenericObject) {
        if (this.state === "disconnected") {
            throw new Error("Database is not connected");
        }

        const db = this.client.db();
        return await db.collection(collection).updateMany({ ...query }, { $set: { ...data } });
    }
}