import type { Client, ClientOptions } from "openapi-fetch";
import type { paths } from "./schema.js";

export type * from "./schema.js";

export type PakuvarausClient = Client<paths>;

/**
 * Create a client for the pakuvaraus API. Pass `baseUrl`, the API origin
 * such as `https://pakuvaraus.tietokilta.fi`, and optionally a custom `fetch`.
 */
export declare function createPakuvarausClient(
  options?: ClientOptions,
): PakuvarausClient;
