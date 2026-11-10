import createClient from "openapi-fetch";

/** @type {import("./index.d.ts").createPakuvarausClient} */
export const createPakuvarausClient = (options) => createClient(options);
